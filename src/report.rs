//! A full diagnostic report: generic scan + vehicle-specific data + findings.

pub use crate::findings::Severity;
use crate::adapter::{Adapter, Transport};
use crate::findings::{self, Finding};
use crate::obd::Reading;
use crate::scan::{self, Scan};
use crate::store::{NewSession, Store, StoredDtc, StoredReading};
use crate::vehicles::{self, Profile};
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Summary {
    pub headline: String,
    pub critical: usize,
    pub warning: usize,
    pub advice: usize,
    pub info: usize,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub vehicle: Option<&'static Profile>,
    /// History key: profile id when the vehicle is known (stable whether or not the VIN reads), else VIN, else "unknown".
    pub vehicle_key: String,
    pub summary: Summary,
    pub findings: Vec<Finding>,
    pub scan: Scan,
    /// Vehicle-specific data beyond generic OBD-II (e.g. EV battery).
    pub extended: Vec<Reading>,
}

pub fn build<T: Transport>(a: &mut Adapter<T>, hint: Option<&str>) -> Result<Report> {
    let scan = scan::scan(a)?;
    let vehicle = scan.vin.as_deref().and_then(vehicles::detect).or_else(|| hint.and_then(vehicles::by_id));
    let vehicle_key = vehicle.map(|v| v.id.to_string()).or_else(|| scan.vin.clone()).unwrap_or_else(|| "unknown".into());
    let (extended, mut extra) = match vehicle {
        Some(v) => vehicles::extended(v, a),
        None => (vec![], vec![]),
    };
    let mut findings = findings::generic(&scan);
    if let Some(v) = vehicle {
        vehicles::annotate(v, &scan, &extended, &mut findings);
    }
    findings.append(&mut extra);
    findings.sort_by_key(|f| f.severity);
    let count = |s| findings.iter().filter(|f| f.severity == s).count();
    let mut summary = Summary { critical: count(Severity::Critical), warning: count(Severity::Warning), advice: count(Severity::Advice), info: count(Severity::Info), headline: String::new() };
    summary.headline = format!(
        "{}: {} critical, {} warning, {} advice, {} info; {} trouble code(s)",
        vehicle.map(|v| v.name).unwrap_or("Unknown vehicle"), summary.critical, summary.warning, summary.advice, summary.info, scan.dtcs.len()
    );
    Ok(Report { vehicle, vehicle_key, summary, findings, scan, extended })
}

impl Report {
    pub fn stored_readings(&self) -> Vec<StoredReading> {
        // Bare keys only for the legislated engine ECU address, so a key never moves to another module.
        let key = |k: &str, ecu: &str| if ecu == "7E8" || ecu == "10" { k.to_string() } else { format!("{k}@{ecu}") };
        let row = |source: &str, key: String, r: &Reading| StoredReading { source: source.into(), key, value: r.value, unit: r.unit.into() };
        let mut out = Vec::new();
        if let Some(v) = self.scan.battery_voltage {
            out.push(StoredReading { source: "adapter".into(), key: "battery_voltage".into(), value: v, unit: "V".into() });
        }
        for l in &self.scan.live {
            out.push(row("live", key(&l.reading.key, &l.ecu), &l.reading));
        }
        for ff in &self.scan.freeze_frames {
            out.extend(ff.readings.iter().map(|r| row("freeze", key(&r.key, &ff.ecu), r)));
        }
        out.extend(self.extended.iter().map(|r| row("extended", r.key.clone(), r)));
        out
    }

    pub fn stored_dtcs(&self) -> Vec<StoredDtc> {
        self.scan.dtcs.iter().map(|d| StoredDtc { code: d.code.clone(), kind: d.kind.into(), ecu: d.ecu.clone() }).collect()
    }
}
pub fn save(db: &Store, r: &Report, kind: &str, odometer_mi: Option<f64>, note: Option<String>) -> Result<i64> {
    let odometer_mi = odometer_mi.or_else(|| r.extended.iter().find(|x| x.key == "odometer_mi").map(|x| x.value));
    db.save_session(&NewSession {
        vehicle: r.vehicle_key.clone(),
        kind: kind.into(),
        at: None,
        odometer_mi,
        note,
        report: serde_json::to_value(r)?,
        readings: r.stored_readings(),
        dtcs: r.stored_dtcs(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::Adapter;
    use crate::sim::Sim;

    fn build_for(sim: &str, hint: Option<&str>) -> Report {
        let mut a = Adapter::connect(Sim::new(sim).unwrap()).unwrap();
        build(&mut a, hint).unwrap()
    }

    #[test]
    fn vehicle_detected_from_vin_is_keyed_by_profile() {
        let r = build_for("odyssey-2016", None);
        assert_eq!(r.vehicle.unwrap().id, "odyssey-2016");
        assert_eq!(r.vehicle_key, "odyssey-2016");
    }

    #[test]
    fn unrecognised_vin_is_the_key() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.set_vin(0x7E8, "1HGCM82633A004352");
        let r = build(&mut Adapter::connect(sim).unwrap(), None).unwrap();
        assert_eq!(r.vehicle_key, "1HGCM82633A004352");
    }

    #[test]
    fn hint_used_when_no_vin_and_keyed_by_profile() {
        let r = build_for("crv-2004", Some("crv-2004"));
        assert_eq!(r.vehicle.unwrap().id, "crv-2004");
        assert_eq!(r.vehicle_key, "crv-2004");
    }

    #[test]
    fn unknown_vehicle_without_vin_or_hint() {
        let r = build_for("crv-2004", None);
        assert!(r.vehicle.is_none());
        assert_eq!(r.vehicle_key, "unknown");
    }

    #[test]
    fn findings_sorted_most_severe_first_with_summary() {
        let r = build_for("odyssey-2016", None);
        assert!(r.findings.windows(2).all(|w| w[0].severity <= w[1].severity));
        assert_eq!(r.summary.warning, r.findings.iter().filter(|f| f.severity == Severity::Warning).count());
        assert!(r.summary.headline.contains("warning"), "{}", r.summary.headline);
    }

    #[test]
    fn kona_report_includes_battery_tpms_and_odometer() {
        let r = build_for("kona-ev-2020", Some("kona-ev-2020"));
        let get = |k: &str| r.extended.iter().find(|x| x.key == k).map(|x| x.value);
        assert_eq!(get("soh"), Some(99.2));
        assert_eq!(get("soc_bms"), Some(36.5));
        assert_eq!(get("cell_delta_v"), Some(0.02));
        assert_eq!(get("cell_v_98"), Some(3.62));
        assert_eq!(get("odometer_mi"), Some(40000.0));
        assert_eq!(get("tire_fl_psi"), Some(38.0));
        assert_eq!(get("aux_12v"), Some(12.56));
        assert_eq!(r.scan.ecus, vec!["7EB", "7EC"]);
        assert!(r.findings.is_empty(), "{:?}", r.findings);
    }

    #[test]
    fn kona_read_failures_become_info_findings() {
        let mut sim = Sim::new("kona-ev-2020").unwrap();
        sim.silence(0x7A8);
        let mut a = Adapter::connect(sim).unwrap();
        let r = build(&mut a, Some("kona-ev-2020")).unwrap();
        let f = r.findings.iter().find(|f| f.title.contains("Vehicle-specific")).unwrap();
        assert!(f.detail.contains("7A0 22C00B"), "{}", f.detail);
        assert!(r.extended.iter().any(|x| x.key == "soh"));
    }

    #[test]
    fn readings_flatten_for_history() {
        let r = build_for("odyssey-2016", None);
        let rows = r.stored_readings();
        assert!(rows.iter().any(|x| x.source == "live" && x.key == "rpm"));
        assert!(rows.iter().any(|x| x.source == "adapter" && x.key == "battery_voltage"));
        assert!(rows.iter().any(|x| x.source == "freeze" && x.key == "rpm"));
        assert_eq!(r.stored_dtcs().len(), 3);
    }

    #[test]
    fn keys_do_not_depend_on_which_ecus_answered() {
        let r = build_for("kona-ev-2020", Some("kona-ev-2020"));
        let keys: Vec<_> = r.stored_readings().into_iter().filter(|x| x.source == "live").map(|x| x.key).collect();
        assert!(keys.contains(&"distance_since_clear@7EB".to_string()) && keys.contains(&"distance_since_clear@7EC".to_string()), "{keys:?}");
    }

    #[test]
    fn secondary_ecu_readings_are_keyed_by_ecu() {
        let r = build_for("odyssey-2016", None);
        let keys: Vec<_> = r.stored_readings().into_iter().filter(|x| x.source == "live" && x.key.starts_with("speed")).map(|x| x.key).collect();
        assert_eq!(keys, vec!["speed", "speed@7E9"]);
    }
}
