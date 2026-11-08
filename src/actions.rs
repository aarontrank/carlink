//! The only paths by which carlink sends anything beyond its fixed read sweep.

use crate::adapter::{Adapter, Transport};
use crate::obd;
use crate::report;
use crate::store::{NewSession, Store};
use crate::vehicles::Powertrain;
use serde_json::{Value, json};
use anyhow::{Result, bail};

/// OBD-II read services plus UDS ReadDataByIdentifier. Everything else (clears, writes, resets,
/// security access, routines, adapter AT/ST config) is refused: an allowlist fails closed.
const READ_SERVICES: &[u8] = &[0x01, 0x02, 0x03, 0x06, 0x07, 0x09, 0x0A, 0x22];

pub fn check_raw(cmd: &str) -> Result<String> {
    let hex: String = cmd.split_whitespace().collect::<String>().to_uppercase();
    if hex.is_empty() || !hex.len().is_multiple_of(2) || hex.len() > 14 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("raw request must be 1–7 bytes of hex, e.g. '010C' or '220101'");
    }
    let sid = u8::from_str_radix(&hex[..2], 16)?;
    if !READ_SERVICES.contains(&sid) {
        bail!("service {sid:02X} is not a read-only request; allowed: 01 02 03 06 07 09 0A 22 (use `carlink clear` to clear codes)");
    }
    if sid == 0x22 && hex.len() < 6 {
        bail!("service 22 needs a 2-byte data identifier, e.g. 220101");
    }
    Ok(hex)
}

/// Combustion engines must positively report 0 rpm; an EV reports no rpm at all.
pub fn check_engine_off<T: Transport>(a: &mut Adapter<T>, ev: bool) -> Result<()> {
    if ev {
        return Ok(());
    }
    let rpm = a.query("010C")?.into_iter().find_map(|m| match m.data[..] {
        [0x41, 0x0C, ref d @ ..] => obd::decode_pid(0x0C, d).map(|r| r.value),
        _ => None,
    });
    match rpm {
        None => bail!("could not confirm the engine is off (no rpm reading). Turn the engine off with the ignition ON; for an EV pass --vehicle so carlink knows it has no engine"),
        Some(r) if r > 0.0 => bail!("engine is running ({r} rpm); turn the engine off with the ignition ON, then retry"),
        _ => Ok(()),
    }
}

const AFTER_CLEAR: &str = "Readiness monitors are now incomplete; drive normally for several days before an emissions inspection. A code that returns means the fault is still present.";

/// Guard, saved pre-clear scan, service 04, best-effort rescan. Once service 04 has been sent,
/// nothing returns `Err`: the outcome (even "unknown") is reported and recorded so it is never re-run blindly.
pub fn clear_with_history<T: Transport>(a: &mut Adapter<T>, db: &Store, hint: Option<&str>, note: Option<String>) -> Result<Value> {
    let before = report::build(a, hint)?;
    check_engine_off(a, before.vehicle.is_some_and(|v| v.powertrain == Powertrain::Ev))?;
    let before_id = report::save(db, &before, "pre-clear", None, note.clone())?;
    let mut out = json!({"pre_clear_session_id": before_id, "codes_before": before.scan.dtcs});
    let mut problems: Vec<String> = Vec::new();
    match a.query("04") {
        Ok(msgs) => {
            let acked: Vec<&str> = msgs.iter().filter(|m| m.data.first() == Some(&0x44)).map(|m| m.ecu.as_str()).collect();
            if acked.is_empty() {
                bail!("no ECU acknowledged the clear request (it may refuse while the engine runs or with the ignition off)");
            }
            out["clear_status"] = json!("cleared");
            out["acknowledged_by"] = json!(acked);
            out["note"] = json!(AFTER_CLEAR);
        }
        Err(e) => {
            out["clear_status"] = json!("unknown");
            problems.push(format!("the clear request was sent but the reply was unreadable ({e}); the codes may or may not be cleared. Do not clear again — run `carlink scan` to see"));
        }
    }
    let rescan_hint = before.vehicle.map(|v| v.id).or(hint);
    let saved = match report::build(a, rescan_hint) {
        Ok(mut after) => {
            after.vehicle_key = before.vehicle_key.clone();
            out["codes_after"] = json!(after.scan.dtcs);
            report::save(db, &after, "clear", None, note)
        }
        Err(e) => {
            problems.push(format!("the follow-up scan failed ({e}); do not clear again, re-run `carlink scan` in a minute"));
            db.save_session(&NewSession {
                vehicle: before.vehicle_key.clone(), kind: "clear".into(), at: None, odometer_mi: None, note,
                report: json!({"clear_status": out["clear_status"], "problems": problems}), readings: vec![], dtcs: vec![],
            })
        }
    };
    match saved {
        Ok(id) => out["post_clear_session_id"] = json!(id),
        Err(e) => problems.push(format!("could not record the clear in history: {e}")),
    }
    out["problems"] = json!(problems);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::Adapter;
    use crate::sim::Sim;

    #[test]
    fn raw_allows_read_only_services_and_normalizes() {
        assert_eq!(check_raw("01 0c").unwrap(), "010C");
        assert_eq!(check_raw("220101").unwrap(), "220101");
        assert_eq!(check_raw("03").unwrap(), "03");
    }

    #[test]
    fn raw_refuses_writes_clears_resets_and_adapter_commands() {
        for bad in ["04", "1101", "2E0101AA", "3101FF00", "2701", "1003", "14FFFFFF", "ATPP0CSV23", "STSBR9600", "", "0", "zz", "2201", "01 02 03 04 05 06 07 08 09"] {
            assert!(check_raw(bad).is_err(), "{bad} should be refused");
        }
    }

    fn adapter(v: &str) -> Adapter<Sim> {
        Adapter::connect(Sim::new(v).unwrap()).unwrap()
    }

    fn sessions(db: &Store) -> Vec<String> {
        db.sessions(None, 10).unwrap().into_iter().map(|s| s.kind).collect()
    }

    #[test]
    fn engine_running_is_refused() {
        let mut a = adapter("odyssey-2016"); // sim idles at 720 rpm
        assert!(check_engine_off(&mut a, false).unwrap_err().to_string().contains("engine is running"));
    }

    #[test]
    fn unreadable_rpm_is_refused_and_mentions_vehicle_flag() {
        let mut a = adapter("crv-2004");
        a.transport_mut().remove_pid(0x10, 0x0C);
        let err = check_engine_off(&mut a, false).unwrap_err().to_string();
        assert!(err.contains("could not confirm") && err.contains("--vehicle"), "{err}");
        assert!(check_engine_off(&mut a, true).is_ok());
    }

    #[test]
    fn clear_without_acknowledgement_is_an_error() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("kona-ev-2020");
        a.transport_mut().ignore_clear = true;
        assert!(clear_with_history(&mut a, &db, Some("kona-ev-2020"), None).unwrap_err().to_string().contains("no ECU acknowledged"));
    }

    #[test]
    fn refused_clear_saves_nothing_and_sends_nothing() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("odyssey-2016");
        assert!(clear_with_history(&mut a, &db, None, None).is_err());
        assert!(sessions(&db).is_empty());
        assert!(!a.transport().log.contains(&"04".to_string()));
    }

    #[test]
    fn successful_clear_records_both_sessions_under_same_key() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("crv-2004");
        a.transport_mut().set_pid(0x10, 0x0C, &[0, 0]);
        let out = clear_with_history(&mut a, &db, Some("crv-2004"), None).unwrap();
        assert_eq!((out["clear_status"].as_str(), out["codes_after"].as_array().map(|v| v.len())), (Some("cleared"), Some(0)));
        assert_eq!(out["acknowledged_by"], json!(["10"]));
        let s = db.sessions(None, 5).unwrap();
        assert_eq!(s.iter().map(|x| (x.kind.as_str(), x.vehicle.as_str())).collect::<Vec<_>>(), vec![("clear", "crv-2004"), ("pre-clear", "crv-2004")]);
    }

    #[test]
    fn rescan_keeps_vehicle_key_when_vin_stops_reading() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("kona-ev-2020");
        a.transport_mut().set_vin(0x7EB, "KM8K53AG0LU000001");
        a.transport_mut().forget_vin_after_clear = true;
        clear_with_history(&mut a, &db, None, None).unwrap();
        assert!(db.sessions(None, 5).unwrap().iter().all(|s| s.vehicle == "kona-ev-2020"));
    }

    #[test]
    fn clear_is_recorded_even_when_rescan_fails() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("kona-ev-2020");
        a.transport_mut().ignition_off_after_clear = true;
        let out = clear_with_history(&mut a, &db, Some("kona-ev-2020"), None).unwrap();
        assert!(out["problems"][0].as_str().unwrap().contains("do not clear again"));
        assert_eq!(sessions(&db), vec!["clear", "pre-clear"]);
    }

    #[test]
    fn garbled_clear_reply_is_recorded_as_unknown_not_an_error() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("kona-ev-2020");
        a.transport_mut().garble_clear_reply = true;
        let out = clear_with_history(&mut a, &db, Some("kona-ev-2020"), None).unwrap();
        assert_eq!(out["clear_status"], "unknown");
        assert!(out["problems"][0].as_str().unwrap().contains("Do not clear again"));
        assert_eq!(sessions(&db), vec!["clear", "pre-clear"]);
    }
}
