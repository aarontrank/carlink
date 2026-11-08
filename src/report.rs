//! A full diagnostic report: generic scan + vehicle-specific data + findings.

pub use crate::findings::Severity;
use crate::adapter::{Adapter, Transport};
use crate::findings::{self, Finding};
use crate::obd::Reading;
use crate::scan::{self, Scan};
use crate::store::{NewSession, Store, StoredDtc, StoredReading};
use crate::dtc;
use crate::mode06;
use crate::uds::{self, ModuleReport};
use crate::vehicles::{self, Profile, VinFacts};
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Summary {
    pub headline: String,
    /// True when this came from the built-in simulator: never present it as a real car.
    pub simulated: bool,
    pub critical: usize,
    pub warning: usize,
    pub advice: usize,
    pub info: usize,
}

/// What the caller told us about the car (`--vehicle`, `--vin`).
#[derive(Debug, Default, Clone, Copy)]
pub struct Ident<'a> {
    pub vehicle: Option<&'a str>,
    pub vin: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct VinInfo {
    pub vin: Option<String>,
    /// "car" when read over OBD, "--vin" when given on the command line.
    pub source: Option<&'static str>,
    pub make: Option<&'static str>,
    pub model_year: Option<u16>,
    pub check_digit_ok: Option<bool>,
}

/// One trouble code, merged across the services and ECUs that reported it.
#[derive(Debug, Serialize)]
pub struct CodeSummary {
    pub code: String,
    pub description: Option<String>,
    pub kinds: Vec<&'static str>,
    pub ecus: Vec<String>,
    pub system: &'static str,
    pub manufacturer_specific: bool,
    pub vehicle_note: Option<&'static str>,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub summary: Summary,
    pub findings: Vec<Finding>,
    pub codes: Vec<CodeSummary>,
    /// False when a code read failed: absence of a code in `codes` then proves nothing.
    pub codes_complete: bool,
    /// OBD emissions-inspection outlook (None when the car reports no readiness monitors, e.g. the Kona).
    pub inspection: Option<findings::Inspection>,
    /// Service 06 self-test results and per-cylinder misfire counts (CAN cars; `carlink tests` for K-line).
    pub tests: Option<mode06::Tests>,
    /// Trouble codes in the car's other modules (brakes, airbags, TPMS…), for profiles that list them.
    pub modules: Vec<ModuleReport>,
    pub vehicle: Option<&'static Profile>,
    /// History key: the VIN (from the car, else `--vin`). None means the scan cannot be saved.
    pub vehicle_key: Option<String>,
    /// Why this may not be the car the caller meant (VIN or profile contradicts `--vin`/`--vehicle`).
    /// `clear` refuses while this is set.
    pub identity_conflict: Option<String>,
    pub vin: VinInfo,
    pub scan: Scan,
    /// Vehicle-specific data beyond generic OBD-II (e.g. EV battery).
    pub extended: Vec<Reading>,
}

fn warning(title: &str, detail: String, recommendation: &str) -> Finding {
    Finding { severity: Severity::Warning, code: None, title: title.into(), detail, recommendation: recommendation.into() }
}

pub fn build<T: Transport>(a: &mut Adapter<T>, ident: &Ident) -> Result<Report> {
    let scan = scan::scan(a)?;
    let mut checks = Vec::new();
    let mut identity_conflict = None;
    let (vin, source) = match (scan.vin.clone(), ident.vin) {
        (Some(v), given) => {
            if given.is_some_and(|g| !g.eq_ignore_ascii_case(&v)) {
                let detail = format!("The car reports VIN {v}, not {}.", given.unwrap_or_default());
                checks.push(warning("Connected to a different car than --vin", detail.clone(),
                    "Make sure the adapter is plugged into the intended car; this scan is filed under the VIN the car reports."));
                identity_conflict = Some(detail);
            }
            (Some(v), Some("car"))
        }
        (None, Some(g)) => (Some(g.to_uppercase()), Some("--vin")),
        (None, None) => (None, None),
    };
    let by_vin = vin.as_deref().and_then(vehicles::detect);
    let by_flag = ident.vehicle.and_then(vehicles::by_id);
    if let (Some(v), Some(f)) = (by_vin, by_flag)
        && v.id != f.id
    {
        let detail = format!("--vehicle {} was given, but the VIN identifies a {}.", f.id, v.name);
        checks.push(warning("Vehicle profile does not match the VIN", detail.clone(),
            "Using the profile that matches the VIN. Check you are connected to the intended car."));
        identity_conflict.get_or_insert(detail);
    }
    let vehicle = by_vin.or(by_flag);
    let vehicle_key = vin.clone();
    if vin.is_none() {
        checks.push(Finding { severity: Severity::Info, code: None, title: "No VIN — this scan is not saved to history".into(),
            detail: "The car did not report a VIN and none was given. History is kept per VIN so different cars are never mixed.".into(),
            recommendation: "Read the VIN from the windshield (driver's side, bottom corner) or the driver's door jamb and pass it with --vin on every command for this car.".into() });
    }
    let facts = vin.as_deref().map(vehicles::vin_info);
    let (extended, mut extra) = match vehicle {
        Some(v) => vehicles::extended(v, a),
        None => (vec![], vec![]),
    };
    let modules: Vec<ModuleReport> = vehicle.map(|v| v.modules.iter().map(|m| uds::read_module(a, m)).collect()).unwrap_or_default();
    // K-line service 06 has no misfire counts, only raw maker tests: only `carlink tests` reads it there.
    let tests = a.bus().is_can().then(|| mode06::read(a, vehicle.map(|v| v.id))).transpose()?.filter(|t| t.supported);
    a.ensure_connected()?;
    let model_year = vin.as_deref().map(vehicles::vin_info).and_then(|f| f.model_year);
    let mut findings = findings::generic(&scan, model_year);
    findings.extend(module_findings(&modules));
    if let Some(t) = &tests {
        findings.extend(findings::tests(t));
    }
    if let Some(v) = vehicle {
        vehicles::annotate(v, &scan, &extended, tests.as_ref(), &mut findings);
    }
    findings.append(&mut extra);
    findings.append(&mut checks);
    findings.sort_by_key(|f| f.severity);
    let codes = summarize_codes(&scan, vehicle);
    let module_codes = modules.iter().flat_map(|m| &m.dtcs).filter(|d| d.state != "not_failed").count();
    let count = |s| findings.iter().filter(|f| f.severity == s).count();
    let name = match (vehicle, facts) {
        (Some(v), _) => v.name.to_string(),
        (None, Some(VinFacts { make: Some(m), model_year: Some(y), .. })) => format!("{y} {m}"),
        _ => "Unknown vehicle".to_string(),
    };
    let total = codes.len() + module_codes;
    let code_text = if scan.codes_complete && modules.iter().all(|m| m.error.is_none()) {
        format!("{total} trouble code(s)")
    } else {
        format!("code read INCOMPLETE, at least {total} trouble code(s)")
    };
    let simulated = scan.adapter.simulated;
    let summary = Summary {
        simulated,
        headline: format!(
            "{}{name}: {} critical, {} warning, {} advice, {} info; {code_text}",
            if simulated { "SIMULATOR — " } else { "" },
            count(Severity::Critical), count(Severity::Warning), count(Severity::Advice), count(Severity::Info)
        ),
        critical: count(Severity::Critical),
        warning: count(Severity::Warning),
        advice: count(Severity::Advice),
        info: count(Severity::Info),
    };
    let vin = VinInfo { make: facts.and_then(|f| f.make), model_year: facts.and_then(|f| f.model_year), check_digit_ok: facts.map(|f| f.check_digit_ok), vin, source };
    let codes_complete = scan.codes_complete && modules.iter().all(|m| m.error.is_none());
    let inspection = findings::inspection(&scan, model_year);
    Ok(Report { summary, findings, codes, codes_complete, inspection, tests, modules, vehicle, vehicle_key, identity_conflict, vin, scan, extended })
}

fn module_findings(modules: &[ModuleReport]) -> Vec<Finding> {
    let mut out = Vec::new();
    let silent: Vec<String> = modules.iter().filter(|m| !m.responded && m.error.is_none()).map(|m| format!("{} ({})", m.name, m.description)).collect();
    if !silent.is_empty() {
        out.push(Finding {
            severity: Severity::Info,
            code: None,
            title: "Some modules did not answer".into(),
            detail: format!("No answer from: {}.", silent.join("; ")),
            recommendation: "Their codes are unknown — not fitted, asleep, or not readable this way. Do not tell the person they have no faults; a warning light from one of these systems needs the factory tool.".into(),
        });
    }
    for m in modules {
        for d in &m.dtcs {
            let severity = match d.state {
                "active" | "stored" => Severity::Warning,
                "pending" => Severity::Advice,
                "history" => Severity::Info,
                _ => continue,
            };
            let base = d.code.split('-').next().unwrap_or(&d.code);
            let what = dtc::lookup(base).description.unwrap_or_else(|| "manufacturer-specific code".into());
            let mut recommendation = if m.clearable {
                format!("Diagnose with the factory service information for the {} module; after the repair, with the person's yes, carlink can clear it (`carlink clear --only {}`).", m.name, m.name)
            } else {
                format!("Diagnose with the factory service information. carlink does not clear the {} module: its codes are evidence the dealer needs, so leave them for the factory tool.", m.name)
            };
            if m.name == "SRS" {
                recommendation.insert_str(0, "Airbag fault: the airbags may not deploy in a crash. Get it repaired promptly. ");
            }
            out.push(Finding {
                severity,
                code: Some(d.code.clone()),
                title: format!("{} ({}): {} — {what}", m.name, m.description, d.code),
                detail: format!("{} code in the {} module.", d.state, m.name),
                recommendation,
            });
        }
        if let Some(e) = &m.error {
            out.push(Finding { severity: Severity::Info, code: None, title: format!("{} module could not be read", m.name), detail: e.clone(), recommendation: "Run the scan again; if it keeps failing, its codes are unknown.".into() });
        }
        if let Some(why) = &m.partial {
            out.push(Finding { severity: Severity::Info, code: None, title: format!("{} ({}): code list may be incomplete", m.name, m.description), detail: format!("{why}."), recommendation: "The codes shown are real; run the scan again to look for the rest.".into() });
        }
        if let Some(why) = &m.refused {
            out.push(Finding {
                severity: Severity::Info,
                code: None,
                title: format!("{} ({}): codes not readable by carlink", m.name, m.description),
                detail: format!("The module answered but refused: {why}."),
                recommendation: "Do not tell the person this module has no faults; if its warning light is on, it needs the factory tool.".into(),
            });
        }
    }
    out
}

fn summarize_codes(scan: &Scan, vehicle: Option<&'static Profile>) -> Vec<CodeSummary> {
    let mut out: Vec<CodeSummary> = Vec::new();
    for d in &scan.dtcs {
        if let Some(c) = out.iter_mut().find(|c| c.code == d.code) {
            if !c.kinds.contains(&d.kind) {
                c.kinds.push(d.kind);
            }
            if !c.ecus.contains(&d.ecu) {
                c.ecus.push(d.ecu.clone());
            }
            continue;
        }
        let info = dtc::lookup(&d.code);
        out.push(CodeSummary {
            vehicle_note: vehicle.and_then(|v| vehicles::dtc_note(v, &d.code)),
            code: d.code.clone(),
            description: info.description,
            kinds: vec![d.kind],
            ecus: vec![d.ecu.clone()],
            system: info.system,
            manufacturer_specific: info.manufacturer_specific,
        });
    }
    out
}

impl Report {
    pub fn stored_readings(&self) -> Vec<StoredReading> {
        // Bare keys only for the engine ECU, so a key never moves to another module.
        let engine = crate::scan::engine_ecu(self.scan.ecus.iter().map(String::as_str));
        let key = |k: &str, ecu: &str| if Some(ecu) == engine { k.to_string() } else { format!("{k}@{ecu}") };
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
        for m in self.tests.iter().flat_map(|t| &t.misfire) {
            let cyl = if m.cylinder == 0 { "all".to_string() } else { format!("cyl{}", m.cylinder) };
            for (suffix, v) in [("avg", m.average), ("current", m.current)] {
                if let Some(v) = v {
                    out.push(StoredReading { source: "mode06".into(), key: format!("misfire_{cyl}_{suffix}"), value: v, unit: "counts".into() });
                }
            }
        }
        out
    }

    pub fn stored_dtcs(&self) -> Vec<StoredDtc> {
        let obd = self.scan.dtcs.iter().map(|d| StoredDtc { code: d.code.clone(), kind: d.kind.into(), ecu: d.ecu.clone() });
        let modules = self.modules.iter().flat_map(|m| m.dtcs.iter().filter(|d| d.state != "not_failed").map(|d| StoredDtc { code: d.code.clone(), kind: d.state.into(), ecu: m.name.into() }));
        obd.chain(modules).collect()
    }
}
pub fn save(db: &Store, r: &Report, kind: &str, odometer_mi: Option<f64>, note: Option<String>) -> Result<i64> {
    let odometer_mi = odometer_mi.or_else(|| r.extended.iter().find(|x| x.key == "odometer_mi").map(|x| x.value));
    let Some(key) = r.vehicle_key.clone() else {
        return Err(crate::problem::problem("vin_required", "this car has no VIN on record, so it cannot be saved to history", crate::problem::GIVE_VIN));
    };
    db.save_session(&NewSession {
        vehicle: key,
        vin: r.vin.vin.clone(),
        profile: r.vehicle.map(|v| v.id.to_string()),
        kind: kind.into(),
        at: None,
        odometer_mi,
        note,
        report: serde_json::to_value(r)?,
        readings: r.stored_readings(),
        dtcs: r.stored_dtcs(),
        codes_complete: r.codes_complete,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::Adapter;
    use crate::sim::Sim;

    #[test]
    fn a_scan_that_loses_the_adapter_is_an_error_not_a_partial_report() {
        let mut sim = Sim::new("crv-2004").unwrap();
        sim.fail("0902", "adapter_disconnected", usize::MAX);
        let mut a = Adapter::connect(sim).unwrap();
        assert!(build(&mut a, &Ident::default()).is_err());
    }

    fn build_for(sim: &str, hint: Option<&str>) -> Report {
        let mut a = Adapter::connect(Sim::new(sim).unwrap()).unwrap();
        build(&mut a, &Ident { vehicle: hint, vin: None }).unwrap()
    }

    fn has_warning(r: &Report, needle: &str) -> bool {
        r.findings.iter().any(|f| f.severity == Severity::Warning && f.title.contains(needle))
    }

    #[test]
    fn vin_read_from_the_car_is_the_history_key() {
        let r = build_for("odyssey-2016", None);
        assert_eq!(r.vehicle.unwrap().id, "odyssey-2016");
        assert_eq!(r.vehicle_key.as_deref(), Some("5FNRL5H60GB000001"));
        assert_eq!(r.vin.source, Some("car"));
    }

    #[test]
    fn vin_given_on_the_command_line_keys_a_car_that_reports_none() {
        let mut a = Adapter::connect(Sim::new("crv-2004").unwrap()).unwrap();
        let r = build(&mut a, &Ident { vehicle: Some("crv-2004"), vin: Some("JHLRD78834C000001") }).unwrap();
        assert_eq!((r.vehicle_key.as_deref(), r.vin.source), (Some("JHLRD78834C000001"), Some("--vin")));
    }

    #[test]
    fn without_a_vin_there_is_no_key_and_nothing_is_saved() {
        let r = build_for("crv-2004", Some("crv-2004"));
        assert_eq!(r.vehicle.unwrap().id, "crv-2004");
        assert_eq!(r.vehicle_key, None);
        assert!(r.findings.iter().any(|f| f.title.contains("No VIN")), "{:?}", r.findings);
        let err = save(&Store::open_in_memory().unwrap(), &r, "scan", None, None).unwrap_err();
        assert_eq!(crate::problem::find(&err).unwrap().kind, "vin_required");
    }

    #[test]
    fn unknown_vehicle_without_vin_or_hint() {
        let r = build_for("crv-2004", None);
        assert!(r.vehicle.is_none());
        assert_eq!(r.vehicle_key, None);
    }

    #[test]
    fn unrecognised_vin_is_the_key_and_is_described() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.set_vin(0x7E8, "1HGCM82633A004352");
        let r = build(&mut Adapter::connect(sim).unwrap(), &Ident::default()).unwrap();
        assert_eq!(r.vehicle_key.as_deref(), Some("1HGCM82633A004352"));
        assert!(r.vehicle.is_none());
        assert_eq!((r.vin.make, r.vin.model_year), (Some("Honda"), Some(2003)));
        assert!(r.summary.headline.starts_with("SIMULATOR — 2003 Honda"), "{}", r.summary.headline);
    }

    #[test]
    fn vehicle_flag_contradicting_the_vin_is_a_warning() {
        let mut a = Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap();
        let r = build(&mut a, &Ident { vehicle: Some("crv-2004"), vin: None }).unwrap();
        assert_eq!(r.vehicle.unwrap().id, "odyssey-2016");
        assert!(has_warning(&r, "does not match"), "{:?}", r.findings);
        assert!(r.identity_conflict.is_some());
    }

    #[test]
    fn vin_flag_contradicting_the_car_is_a_warning() {
        let mut a = Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap();
        let r = build(&mut a, &Ident { vehicle: None, vin: Some("5FNRL5H60GB000002") }).unwrap();
        assert_eq!(r.vehicle_key.as_deref(), Some("5FNRL5H60GB000001"));
        assert!(has_warning(&r, "different car"), "{:?}", r.findings);
        assert!(r.identity_conflict.as_deref().unwrap().contains("5FNRL5H60GB000002"));
    }

    #[test]
    fn failed_code_read_marks_the_report_incomplete() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.inject("03", "CAN ERROR");
        sim.inject("03", "CAN ERROR");
        let r = build(&mut Adapter::connect(sim).unwrap(), &Ident::default()).unwrap();
        assert!(!r.codes_complete);
        assert!(build_for("odyssey-2016", None).codes_complete);
    }

    #[test]
    fn headline_says_when_the_code_read_was_incomplete() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.inject("03", "CAN ERROR");
        sim.inject("03", "CAN ERROR");
        let r = build(&mut Adapter::connect(sim).unwrap(), &Ident::default()).unwrap();
        assert!(r.summary.headline.contains("INCOMPLETE"), "{}", r.summary.headline);
        assert_eq!(r.findings[0].title, "Trouble-code read INCOMPLETE");
    }

    #[test]
    fn simulator_reports_say_so() {
        let r = build_for("odyssey-2016", None);
        assert!(r.summary.simulated);
        assert!(r.summary.headline.starts_with("SIMULATOR — "));
    }

    #[test]
    fn codes_are_listed_once_with_description_kinds_and_vehicle_note() {
        let r = build_for("odyssey-2016", None);
        let p0303 = r.codes.iter().find(|c| c.code == "P0303").unwrap();
        assert_eq!(p0303.kinds, vec!["stored", "permanent"]);
        assert_eq!(p0303.description.as_deref(), Some("Cylinder 3 Misfire Detected"));
        assert!(p0303.vehicle_note.unwrap().contains("25-061"));
        assert_eq!(r.codes.len(), 2);
    }

    #[test]
    fn summary_comes_first_in_the_json() {
        let v = serde_json::to_value(build_for("kona-ev-2020", Some("kona-ev-2020"))).unwrap();
        let keys: Vec<&String> = v.as_object().unwrap().keys().collect();
        assert_eq!(keys[..3], ["summary", "findings", "codes"]);
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
        assert!(r.findings.iter().all(|f| f.severity == Severity::Info || f.title.starts_with("IEB")), "{:?}", r.findings);
    }

    #[test]
    fn kona_read_failures_become_info_findings() {
        let mut sim = Sim::new("kona-ev-2020").unwrap();
        sim.silence(0x7A8);
        let mut a = Adapter::connect(sim).unwrap();
        let r = build(&mut a, &Ident { vehicle: Some("kona-ev-2020"), vin: None }).unwrap();
        let f = r.findings.iter().find(|f| f.title.contains("Vehicle-specific")).unwrap();
        assert!(f.detail.contains("7A0 22C00B"), "{}", f.detail);
        assert!(r.extended.iter().any(|x| x.key == "soh"));
    }

    #[test]
    fn kona_module_codes_are_read_reported_and_stored() {
        let r = build_for("kona-ev-2020", Some("kona-ev-2020"));
        let ieb = r.modules.iter().find(|m| m.name == "IEB").unwrap();
        assert!(ieb.responded);
        assert_eq!(ieb.dtcs[0].code, "C1611-00");
        assert!(r.findings.iter().any(|f| f.severity == Severity::Warning && f.title.starts_with("IEB")));
        assert!(r.stored_dtcs().iter().any(|d| d.ecu == "IEB" && d.kind == "stored"));
        assert!(r.codes_complete);
    }

    #[test]
    fn odyssey_scan_reads_misfire_counters_flags_cylinders_and_stores_them() {
        let r = build_for("odyssey-2016", None);
        let t = r.tests.as_ref().unwrap();
        assert_eq!(t.misfire.iter().find(|m| m.cylinder == 3).unwrap().current, Some(180.0));
        assert!(r.findings.iter().any(|f| f.severity == Severity::Warning && f.title.contains("Cylinder 3")), "{:?}", r.findings);
        assert!(r.findings.iter().any(|f| f.title.contains("VCM")), "{:?}", r.findings);
        assert!(r.stored_readings().iter().any(|x| x.key == "misfire_cyl3_current" && x.value == 180.0));
        assert!(build_for("crv-2004", Some("crv-2004")).tests.is_none(), "K-line service 06 is only read by `carlink tests`");
    }

    #[test]
    fn a_read_only_module_code_is_never_offered_for_clearing() {
        let mut sim = Sim::new("kona-ev-2020").unwrap();
        sim.add_module(0x7EC, false, &[([0x0A, 0x0A, 0x12], 0x08)]);
        let r = build(&mut Adapter::connect(sim).unwrap(), &Ident { vehicle: Some("kona-ev-2020"), vin: None }).unwrap();
        let f = r.findings.iter().find(|f| f.title.starts_with("BMS")).unwrap();
        assert!(!f.recommendation.contains("carlink clear"), "{}", f.recommendation);
    }

    #[test]
    fn modules_that_do_not_answer_are_named_as_unknown() {
        let mut sim = Sim::new("kona-ev-2020").unwrap();
        sim.silence(0x7D9);
        let r = build(&mut Adapter::connect(sim).unwrap(), &Ident { vehicle: Some("kona-ev-2020"), vin: None }).unwrap();
        let f = r.findings.iter().find(|f| f.title == "Some modules did not answer").unwrap();
        assert!(f.detail.contains("IEB"), "{}", f.detail);
    }

    #[test]
    fn kona_airbag_module_refusal_is_reported_not_hidden() {
        let r = build_for("kona-ev-2020", Some("kona-ev-2020"));
        let f = r.findings.iter().find(|f| f.title.starts_with("SRS")).unwrap();
        assert!(f.detail.contains("diagnostic session"), "{}", f.detail);
        assert!(r.codes_complete, "a refusal is not a failed read");
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
    fn report_records_whether_the_session_is_complete() {
        assert!(build_for("odyssey-2016", None).codes_complete);
    }

    #[test]
    fn secondary_ecu_readings_are_keyed_by_ecu() {
        let r = build_for("odyssey-2016", None);
        let keys: Vec<_> = r.stored_readings().into_iter().filter(|x| x.source == "live" && x.key.starts_with("speed")).map(|x| x.key).collect();
        assert_eq!(keys, vec!["speed", "speed@7E9"]);
    }
}
