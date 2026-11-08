//! Setup checklist: which link between the computer and the car is broken, and what to do about it.

use crate::adapter::{Adapter, AdapterInfo, Transport};
use crate::problem;
use crate::vehicles::{self, Profile};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Check {
    pub name: &'static str,
    pub ok: bool,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_to_do: Option<&'static str>,
}

#[derive(Debug, Serialize)]
pub struct Probe {
    /// True when carlink can diagnose this car (a missing VIN does not make it false).
    pub ok: bool,
    /// The built-in simulator, not a real car.
    pub simulated: bool,
    pub checks: Vec<Check>,
    pub adapter: Option<AdapterInfo>,
    pub battery_voltage: Option<f64>,
    pub vin: Option<String>,
    pub vehicle: Option<&'static Profile>,
}

fn pass(name: &'static str, detail: impl Into<String>) -> Check {
    Check { name, ok: true, detail: detail.into(), what_to_do: None }
}

fn fail(name: &'static str, e: &anyhow::Error, fallback: &'static str) -> Check {
    let what_to_do = problem::find(e).map(|p| p.what_to_do).unwrap_or(fallback);
    Check { name, ok: false, detail: format!("{e:#}"), what_to_do: Some(what_to_do) }
}

/// Below this the adapter is powered by USB only: it is not in the car, or the OBD port has no power.
const MIN_PORT_VOLTAGE: f64 = 8.0;

pub fn probe<T: Transport>(t: T) -> Probe {
    let mut p = Probe { ok: false, simulated: t.simulated(), checks: vec![], adapter: None, battery_voltage: None, vin: None, vehicle: None };
    let mut a = match Adapter::open(t) {
        Ok(a) => a,
        Err(e) => {
            p.checks.push(fail("adapter", &e, problem::CHECK_USB));
            return p;
        }
    };
    p.checks.push(pass("adapter", format!("{} {}", a.info().device, a.info().stn.clone().unwrap_or_default()).trim().to_string()));
    p.battery_voltage = a.voltage().ok().flatten();
    match p.battery_voltage {
        Some(v) if v >= MIN_PORT_VOLTAGE => p.checks.push(pass("power_at_obd_port", format!("{v:.1} V"))),
        other => {
            p.checks.push(Check {
                name: "power_at_obd_port",
                ok: false,
                detail: format!("{} at the OBD port", other.map(|v| format!("{v:.1} V")).unwrap_or("no voltage reading".into())),
                what_to_do: Some("The adapter is not getting power from the car. Push it fully into the car's OBD port (under the dashboard, driver side). If it is in, check the fuse for the OBD/accessory socket."),
            });
            p.adapter = Some(a.info().clone());
            return p;
        }
    }
    if let Err(e) = a.detect_vehicle() {
        p.checks.push(fail("vehicle", &e, problem::CHECK_IGNITION));
        p.adapter = Some(a.info().clone());
        return p;
    }
    p.checks.push(pass("vehicle", format!("answering on {}", a.info().protocol)));
    p.ok = true;
    p.vin = crate::scan::vin(&mut a).ok().flatten();
    p.vehicle = p.vin.as_deref().and_then(vehicles::detect);
    p.checks.push(match &p.vin {
        Some(v) => pass("vin", v.clone()),
        None => Check {
            name: "vin",
            ok: false,
            detail: "the car did not report a VIN (normal for older cars)".into(),
            what_to_do: Some("Read the VIN from the windshield or the driver's door jamb and pass it with --vin, and pass --vehicle, on every command for this car."),
        },
    });
    p.adapter = Some(a.info().clone());
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::Sim;

    fn names(p: &Probe) -> Vec<(&str, bool)> {
        p.checks.iter().map(|c| (c.name, c.ok)).collect()
    }

    #[test]
    fn healthy_setup_passes_every_check() {
        let p = probe(Sim::new("odyssey-2016").unwrap());
        assert!(p.ok && p.simulated);
        assert_eq!(names(&p), vec![("adapter", true), ("power_at_obd_port", true), ("vehicle", true), ("vin", true)]);
        assert_eq!(p.vehicle.unwrap().id, "odyssey-2016");
    }

    #[test]
    fn adapter_not_plugged_into_the_car_is_named() {
        let p = probe(Sim::from_spec("odyssey-2016:not-in-car").unwrap());
        assert!(!p.ok);
        let failed = p.checks.iter().find(|c| !c.ok).unwrap();
        assert_eq!(failed.name, "power_at_obd_port");
        assert!(failed.what_to_do.unwrap().contains("OBD port"));
    }

    #[test]
    fn ignition_off_is_named() {
        let p = probe(Sim::from_spec("odyssey-2016:ignition-off").unwrap());
        let failed = p.checks.iter().find(|c| !c.ok).unwrap();
        assert_eq!(failed.name, "vehicle");
        assert!(failed.what_to_do.unwrap().contains("ignition"));
    }

    #[test]
    fn missing_vin_is_a_soft_check() {
        let p = probe(Sim::new("crv-2004").unwrap());
        assert!(p.ok);
        let vin = p.checks.iter().find(|c| c.name == "vin").unwrap();
        assert!(!vin.ok && vin.what_to_do.unwrap().contains("--vin"));
    }

    #[test]
    fn wrong_device_stops_at_the_first_check() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.identity = "Arduino";
        let p = probe(sim);
        assert_eq!(names(&p), vec![("adapter", false)]);
    }
}
