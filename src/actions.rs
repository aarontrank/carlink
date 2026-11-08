//! The only paths by which carlink sends anything beyond its fixed read sweep.

use crate::adapter::{Adapter, READ_SERVICES, Transport};
use crate::obd;
use crate::report::{self, Ident, Report};
use crate::uds::{Module, Protocol};
use crate::vehicles::Profile;
use serde::Serialize;
use crate::store::{NewSession, Store};
use crate::vehicles::Powertrain;
use serde_json::{Value, json};
use crate::problem::problem;
use anyhow::Result;

/// Only `READ_SERVICES` pass. Everything else (clears, writes, resets, security access, routines,
/// adapter AT/ST config) is refused: an allowlist fails closed.
pub fn check_raw(cmd: &str) -> Result<String> {
    let hex: String = cmd.split_whitespace().collect::<String>().to_uppercase();
    if hex.is_empty() || !hex.len().is_multiple_of(2) || hex.len() > 14 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(problem("bad_input", "raw request must be 1–7 bytes of hex, e.g. '010C' or '220101'", "Fix the request and run it again."));
    }
    let sid = u8::from_str_radix(&hex[..2], 16)?;
    if !READ_SERVICES.contains(&sid) {
        let allowed: Vec<String> = READ_SERVICES.iter().map(|s| format!("{s:02X}")).collect();
        return Err(problem(
            "request_not_allowed",
            format!("service {sid:02X} is not a read-only request; allowed: {}", allowed.join(" ")),
            "carlink only sends read requests through `raw`. To clear codes use `carlink clear`.",
        ));
    }
    if sid == 0x22 && hex.len() < 6 {
        return Err(problem("bad_input", "service 22 needs a 2-byte data identifier, e.g. 220101", "Fix the request and run it again."));
    }
    Ok(hex)
}

/// Combustion engines must positively report 0 rpm; an EV reports no rpm at all. Any car reporting
/// rpm above 0 is refused, whatever profile it was identified as.
pub fn check_engine_off<T: Transport>(a: &mut Adapter<T>, ev: bool) -> Result<()> {
    let rpm = a.query("010C")?.into_iter().find_map(|m| match m.data[..] {
        [0x41, 0x0C, ref d @ ..] => obd::decode_pid(0x0C, d).map(|r| r.value),
        _ => None,
    });
    match rpm {
        None if ev => Ok(()),
        None => Err(problem(
            "engine_state_unknown",
            "could not confirm the engine is off (the car reported no rpm)",
            "Turn the engine OFF and leave the ignition ON, then run the command again. For an electric car pass --vehicle (e.g. --vehicle kona-ev-2020) so carlink knows it has no engine.",
        )),
        Some(r) if r > 0.0 => Err(problem(
            "engine_running",
            format!("the engine is running ({r} rpm)"),
            "Turn the engine OFF, turn the ignition back ON without starting, then run the command again.",
        )),
        _ => Ok(()),
    }
}

/// What a negative response code means, in words a technician understands.
pub fn nrc_meaning(nrc: u8) -> &'static str {
    match nrc {
        0x10 => "general reject",
        0x11 => "service not supported by this module",
        0x12 => "sub-function not supported",
        0x13 => "wrong message length or format",
        0x14 => "response too long",
        0x21 => "busy, repeat request",
        0x22 => "conditions not correct (often: engine must be off, ignition on, vehicle stopped)",
        0x24 => "request sequence error",
        0x31 => "request out of range",
        0x33 => "security access denied",
        0x7E | 0x7F => "not supported in the current diagnostic session",
        _ => "refused",
    }
}

/// What one clear request goes to: the emissions codes of the OBD ECUs (service 04), or one module (service 14).
#[derive(Debug, Clone, Copy)]
pub enum Target {
    Engine,
    Module(&'static Module),
}

impl Target {
    fn name(&self) -> &'static str {
        match self {
            Target::Engine => "engine",
            Target::Module(m) => m.name,
        }
    }
}

fn engine_has_codes(before: &Report) -> bool {
    before.scan.dtcs.iter().any(|d| d.kind != "permanent") || before.scan.monitors.iter().any(|m| m.status.mil_on)
}

fn modules_with_codes(before: &Report) -> Vec<&'static Module> {
    let modules: &'static [Module] = before.vehicle.map(|v| v.modules).unwrap_or(&[]);
    modules.iter().filter(|m| before.modules.iter().any(|r| r.name == m.name && r.dtcs.iter().any(|d| d.state != "not_failed"))).collect()
}

/// `--only` → targets. Empty means the engine/emissions codes only, and only if there are any (clearing
/// also resets readiness monitors). A module is cleared only when named, or with `all` (engine + every
/// module that has codes): the person must agree to each system by name, and some are safety systems.
pub fn resolve_targets(before: &Report, only: &[String]) -> Result<Vec<Target>> {
    let modules: &'static [Module] = before.vehicle.map(|v| v.modules).unwrap_or(&[]);
    let named = !only.is_empty() && !only.iter().any(|n| n.eq_ignore_ascii_case("all"));
    if !named && !before.codes_complete {
        return Err(problem(
            "scan_incomplete",
            "some trouble codes could not be read, so carlink cannot tell what needs clearing",
            "Run `carlink scan` again. If the code read keeps failing, name what to clear with --only (e.g. --only engine).",
        ));
    }
    if only.is_empty() {
        return Ok(engine_has_codes(before).then_some(Target::Engine).into_iter().collect());
    }
    if only.iter().any(|n| n.eq_ignore_ascii_case("all")) {
        return Ok(engine_has_codes(before).then_some(Target::Engine).into_iter().chain(modules_with_codes(before).into_iter().map(Target::Module)).collect());
    }
    let mut seen: Vec<String> = Vec::new();
    only.iter()
        .filter(|n| {
            let key = n.to_ascii_lowercase();
            let new = !seen.contains(&key);
            seen.push(key);
            new
        })
        .map(|name| {
            if name.eq_ignore_ascii_case("engine") {
                return Ok(Target::Engine);
            }
            modules.iter().find(|m| m.name.eq_ignore_ascii_case(name)).map(Target::Module).ok_or_else(|| {
                let valid: Vec<&str> = ["engine", "all"].into_iter().chain(modules.iter().map(|m| m.name)).collect();
                problem("bad_input", format!("'{name}' is not something carlink can clear on this car; valid: {}", valid.join(", ")), "Use names from the scan's modules[].name, or `engine`.")
            })
        })
        .collect()
}

#[derive(Debug, Serialize)]
pub struct Sent {
    pub target: &'static str,
    pub acknowledged_by: Vec<String>,
    pub refused: Vec<Value>,
    pub error: Option<String>,
}

fn send_clear<T: Transport>(a: &mut Adapter<T>, t: Target, before: &Report) -> Sent {
    let (result, sid) = match t {
        Target::Engine => (a.query("04"), 0x04),
        Target::Module(m) => {
            let used = before.modules.iter().find(|r| r.name == m.name).and_then(|r| r.protocol).unwrap_or(m.protocol);
            let request = match used {
                Protocol::Uds => "14FFFFFF",
                Protocol::Kwp => "14FF00",
            };
            (a.with_header(m.request, |a| a.query(request)), 0x14)
        }
    };
    let mut sent = Sent { target: t.name(), acknowledged_by: vec![], refused: vec![], error: None };
    match result {
        Ok(msgs) => {
            for m in msgs {
                match m.data[..] {
                    [first, ..] if first == sid + 0x40 => sent.acknowledged_by.push(m.ecu),
                    [0x7F, s, nrc, ..] if s == sid => sent.refused.push(json!({"ecu": m.ecu, "nrc": format!("{nrc:02X}"), "meaning": nrc_meaning(nrc)})),
                    _ => {}
                }
            }
        }
        Err(e) => sent.error = Some(format!("{e:#}")),
    }
    sent
}

/// Per target, from the fresh scan only: cleared / codes_remain / not_acknowledged / unverified.
/// A code still present is "back" (fault still there) only if the module holding it confirmed the clear;
/// otherwise the clear did not take there ("not_acknowledged"), which may be retried once.
fn target_status(t: Target, sent: &Sent, after: Option<&Report>) -> (&'static str, Vec<String>) {
    let Some(r) = after else { return ("unverified", vec![]) };
    let remaining: Vec<(String, String)> = match t {
        Target::Engine if !r.scan.codes_complete => return ("unverified", vec![]),
        Target::Engine => r.scan.dtcs.iter().filter(|d| d.kind != "permanent").map(|d| (d.code.clone(), d.ecu.clone())).collect(),
        Target::Module(m) => match r.modules.iter().find(|x| x.name == m.name) {
            Some(x) if x.error.is_none() && x.refused.is_none() && x.responded => {
                let reply = format!("{:03X}", u16::from_str_radix(m.request, 16).unwrap_or(0) + 8);
                x.dtcs.iter().filter(|d| d.state != "not_failed").map(|d| (d.code.clone(), reply.clone())).collect()
            }
            _ => return ("unverified", vec![]),
        },
    };
    let codes = remaining.iter().map(|(c, _)| c.clone()).collect();
    let back = remaining.iter().any(|(_, ecu)| sent.acknowledged_by.contains(ecu));
    let status = if remaining.is_empty() {
        "cleared"
    } else if back {
        "codes_remain"
    } else if sent.refused.is_empty() {
        "not_acknowledged"
    } else {
        "refused"
    };
    (status, codes)
}

const SEVERITY_ORDER: [&str; 5] = ["unverified", "codes_remain", "refused", "not_acknowledged", "cleared"];

/// Guard → saved pre-clear scan → explicit return to functional addressing → each clear request (sent
/// once) → fresh scan. Outcomes come only from that scan, never from the replies. After the first
/// request is sent nothing returns `Err`, so a caller can never mistake a sent clear for an unsent one.
pub fn clear_with_history<T: Transport>(a: &mut Adapter<T>, db: &Store, ident: &Ident, only: &[String], note: Option<String>) -> Result<Value> {
    let before = report::build(a, ident)?;
    if let Some(conflict) = &before.identity_conflict {
        return Err(problem(
            "vehicle_mismatch",
            format!("refusing to clear: {conflict}"),
            "The adapter may be in a different car than you think. Check which car it is plugged into and pass that car's VIN and vehicle; nothing was cleared.",
        ));
    }
    if before.vehicle_key.is_none() {
        return Err(problem("vin_required", "refusing to clear a car with no VIN on record", crate::problem::GIVE_VIN));
    }
    check_engine_off(a, before.vehicle.is_some_and(|v| v.powertrain == Powertrain::Ev))?;
    let targets = resolve_targets(&before, only)?;
    if targets.is_empty() {
        let pending: Vec<String> = modules_with_codes(&before).iter().map(|m| format!("{} ({})", m.name, m.description)).collect();
        let mut steps = vec!["There are no engine/emissions codes, so nothing was sent (clearing would only reset the readiness monitors).".to_string()];
        if !pending.is_empty() {
            steps.push(format!("These modules have codes: {}. To clear one, tell the person which system it is, get their yes, and run the clear again with --only <NAME>.", pending.join(", ")));
        }
        return Ok(json!({"clear_status": "nothing_to_clear", "next_steps": steps, "vehicle": identity(&before), "codes_before": before.codes}));
    }
    let before_id = report::save(db, &before, "pre-clear", None, note.clone())?;
    a.reset_addressing()?;

    let sent: Vec<Sent> = targets.iter().map(|t| send_clear(a, *t, &before)).collect();
    let mut problems: Vec<String> = sent.iter().filter_map(|s| s.error.as_ref().map(|e| format!("the reply to clearing {} was unreadable ({e})", s.target))).collect();

    let rescan = Ident { vehicle: before.vehicle.map(|v| v.id).or(ident.vehicle), vin: before.vin.vin.as_deref().or(ident.vin) };
    let after = match report::build(a, &rescan) {
        Ok(mut r) => {
            r.vehicle_key.clone_from(&before.vehicle_key);
            Some(r)
        }
        Err(e) => {
            problems.push(format!("the follow-up scan failed ({e})"));
            None
        }
    };
    let results: Vec<Value> = targets
        .iter()
        .zip(&sent)
        .map(|(t, s)| {
            let (status, remaining) = target_status(*t, s, after.as_ref());
            json!({"target": s.target, "status": status, "codes_remaining": remaining, "acknowledged_by": s.acknowledged_by, "refused": s.refused})
        })
        .collect();
    let status = SEVERITY_ORDER.iter().find(|st| results.iter().any(|r| r["status"] == **st)).copied().unwrap_or("unverified");

    let saved = match &after {
        Some(r) => report::save(db, r, "clear", None, note),
        None => db.save_session(&NewSession {
            vehicle: before.vehicle_key.clone().unwrap_or_default(),
            vin: before.vin.vin.clone(),
            profile: before.vehicle.map(|v| v.id.to_string()),
            kind: "clear".into(),
            note,
            report: json!({"clear_status": "unverified", "targets": results, "problems": problems}),
            codes_complete: false,
            ..Default::default()
        }),
    };
    let post_clear_session_id = match saved {
        Ok(id) => Some(id),
        Err(e) => {
            problems.push(format!("could not record the clear in history ({e})"));
            None
        }
    };
    let mut next_steps = next_steps(status, &results, before.vehicle);
    if status == "cleared" && targets.iter().any(|t| matches!(t, Target::Engine)) {
        let mut monitors: Vec<&str> = before.codes.iter().filter(|c| c.kinds.iter().any(|k| *k != "permanent")).map(|c| crate::findings::monitor_for(&c.code)).collect();
        monitors.sort();
        monitors.dedup();
        if !monitors.is_empty() {
            next_steps.push(format!(
                "Do not call the repair confirmed yet. It is confirmed when a later `carlink scan` shows the {} self-test(s) complete (inspection/readiness) and the code has not come back as pending or stored. That needs normal driving, sometimes several days (EVAP tests need a cold start).",
                monitors.join(", ")
            ));
        }
    }
    if targets.iter().any(|t| matches!(t, Target::Engine)) && !engine_has_codes(&before) {
        next_steps.insert(0, "There were no engine codes before the clear: it only reset the readiness monitors (and, on Hondas, learned idle values).".into());
    }
    Ok(json!({
        "clear_status": status,
        "simulated": before.summary.simulated,
        "next_steps": next_steps,
        "vehicle": identity(&before),
        "targets": results,
        "codes_before": before.codes,
        "codes_after": after.as_ref().map(|r| &r.codes),
        "modules_after": after.as_ref().map(|r| r.modules.iter().map(|m| json!({"name": m.name, "codes": m.dtcs.iter().map(|d| &d.code).collect::<Vec<_>>()})).collect::<Vec<_>>()),
        "problems": problems,
        "pre_clear_session_id": before_id,
        "post_clear_session_id": post_clear_session_id,
    }))
}

fn identity(r: &Report) -> Value {
    json!({"key": r.vehicle_key, "vin": r.vin.vin, "vin_source": r.vin.source, "profile": r.vehicle.map(|v| v.id), "name": r.vehicle.map(|v| v.name)})
}

fn next_steps(status: &str, results: &[Value], vehicle: Option<&Profile>) -> Vec<String> {
    let with = |st: &str| results.iter().filter(|r| r["status"] == st).map(|r| r["target"].as_str().unwrap_or("").to_string()).collect::<Vec<_>>().join(", ");
    match status {
        "cleared" => {
            let mut v = vec!["All codes are cleared. Readiness monitors restart: drive normally (city and highway) for several days before an emissions inspection.".to_string()];
            if results.iter().any(|r| r["target"] == "engine") {
                v.extend(vehicle.map(|p| p.after_clear).unwrap_or(&[]).iter().map(|s| s.to_string()));
            }
            v.push("Permanent codes (if any) stay until the car re-tests the repaired system and passes; that is normal.".into());
            v
        }
        "not_acknowledged" => vec![format!(
            "Clearing did not take effect for: {}. Nothing confirmed the request and the codes are still there. Check: ignition ON, engine OFF, car stopped. Then you may run `carlink clear` once more.",
            with("not_acknowledged")
        )],
        "refused" => vec![format!(
            "A module refused the clear ({}); see `targets[].refused` for the reason. Check: ignition ON, engine OFF, car stopped. Then you may run `carlink clear` once more.",
            with("refused")
        )],
        "codes_remain" => vec![format!(
            "Codes came back right after clearing for: {}. The fault is most likely still present — diagnose and repair it; clearing again will not help.",
            with("codes_remain")
        )],
        _ => vec![
            "The clear request WAS sent, but the result could not be confirmed. Do NOT run `carlink clear` again.".to_string(),
            "Wait one minute, then run `carlink scan` to see which codes are present.".to_string(),
        ],
    }
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
        assert_eq!(check_raw("1902FF").unwrap(), "1902FF");
        assert_eq!(check_raw("2102").unwrap(), "2102");
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
        let err = check_engine_off(&mut a, false).unwrap_err();
        let p = crate::problem::find(&err).unwrap();
        assert_eq!(p.kind, "engine_state_unknown");
        assert!(p.what_to_do.contains("--vehicle"));
        assert!(check_engine_off(&mut a, true).is_ok());
    }

    #[test]
    fn a_running_engine_is_refused_even_when_the_profile_says_ev() {
        let mut a = adapter("odyssey-2016");
        let err = check_engine_off(&mut a, true).unwrap_err();
        assert_eq!(crate::problem::find(&err).unwrap().kind, "engine_running");
    }

    const KONA_VIN: &str = "KM8K53AG0LU000001";
    const CRV_VIN: &str = "JHLRD78834C000001";

    /// The two simulated cars that report no VIN of their own get theirs from --vin, as in the shop.
    fn ident(v: &'static str) -> Ident<'static> {
        let vin = match v {
            "kona-ev-2020" => Some(KONA_VIN),
            "crv-2004" => Some(CRV_VIN),
            _ => None,
        };
        Ident { vehicle: Some(v), vin }
    }

    fn only(names: &[&str]) -> Vec<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    fn kind(e: anyhow::Error) -> &'static str {
        crate::problem::find(&e).map(|p| p.kind).unwrap_or("none")
    }

    #[test]
    fn unanswered_clear_is_reported_as_not_acknowledged() {
        let db = Store::open_in_memory().unwrap();
        let mut a = Adapter::connect(Sim::from_spec("odyssey-2016:engine-off").unwrap()).unwrap();
        a.transport_mut().ignore_clear = true;
        let out = clear_with_history(&mut a, &db, &Ident::default(), &[], None).unwrap();
        assert_eq!(out["clear_status"], "not_acknowledged");
        assert!(out["next_steps"][0].as_str().unwrap().contains("did not take effect"));
    }

    #[test]
    fn refused_clear_saves_nothing_and_sends_nothing() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("odyssey-2016");
        assert!(clear_with_history(&mut a, &db, &Ident::default(), &[], None).is_err());
        assert!(sessions(&db).is_empty());
        assert!(!a.transport().log.contains(&"04".to_string()));
    }

    #[test]
    fn no_vin_means_no_clear() {
        let db = Store::open_in_memory().unwrap();
        let mut a = Adapter::connect(Sim::from_spec("crv-2004:engine-off").unwrap()).unwrap();
        let err = clear_with_history(&mut a, &db, &Ident { vehicle: Some("crv-2004"), vin: None }, &[], None).unwrap_err();
        assert_eq!(kind(err), "vin_required");
        assert!(!a.transport().log.contains(&"04".to_string()));
    }

    #[test]
    fn a_different_car_than_requested_is_not_cleared() {
        let db = Store::open_in_memory().unwrap();
        let mut a = Adapter::connect(Sim::from_spec("odyssey-2016:engine-off").unwrap()).unwrap();
        let err = clear_with_history(&mut a, &db, &Ident { vehicle: None, vin: Some("5FNRL5H60GB000002") }, &[], None).unwrap_err();
        assert_eq!(kind(err), "vehicle_mismatch");
        let err = clear_with_history(&mut a, &db, &Ident { vehicle: Some("crv-2004"), vin: None }, &[], None).unwrap_err();
        assert_eq!(kind(err), "vehicle_mismatch");
        assert!(!a.transport().log.contains(&"04".to_string()));
        assert!(sessions(&db).is_empty());
    }

    #[test]
    fn verified_clear_records_both_sessions_under_same_key_with_vehicle_steps() {
        let db = Store::open_in_memory().unwrap();
        let mut a = Adapter::connect(Sim::from_spec("crv-2004:engine-off").unwrap()).unwrap();
        let out = clear_with_history(&mut a, &db, &ident("crv-2004"), &[], None).unwrap();
        assert_eq!(out["clear_status"], "cleared");
        assert_eq!(out["codes_after"], json!([]));
        assert_eq!(out["targets"][0]["acknowledged_by"], json!(["10"]));
        assert_eq!(out["vehicle"]["key"], CRV_VIN);
        assert!(out["next_steps"].as_array().unwrap().iter().any(|s| s.as_str().unwrap().contains("idle learn")));
        assert!(out["next_steps"].as_array().unwrap().iter().any(|s| s.as_str().unwrap().contains("catalyst, evap")), "{out}");
        let s = db.sessions(None, 5).unwrap();
        assert_eq!(s.iter().map(|x| (x.kind.as_str(), x.vehicle.as_str())).collect::<Vec<_>>(), vec![("clear", CRV_VIN), ("pre-clear", CRV_VIN)]);
    }

    #[test]
    fn functional_addressing_is_reasserted_right_before_the_clear() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("kona-ev-2020");
        let out = clear_with_history(&mut a, &db, &ident("kona-ev-2020"), &only(&["engine"]), None).unwrap();
        let log = a.transport().log.join(" ");
        assert!(log.contains("ATFCSM0 ATAR ATSH7DF 04"), "{log}");
        assert!(out["next_steps"][0].as_str().unwrap().contains("no engine codes"), "{out}");
    }

    #[test]
    fn codes_from_a_module_that_confirmed_the_clear_are_reported_as_still_present() {
        let db = Store::open_in_memory().unwrap();
        let mut a = Adapter::connect(Sim::from_spec("odyssey-2016:engine-off").unwrap()).unwrap();
        a.transport_mut().inject("04", "7E8 01 44");
        let out = clear_with_history(&mut a, &db, &Ident::default(), &[], None).unwrap();
        assert_eq!(out["clear_status"], "codes_remain");
        assert!(out["next_steps"][0].as_str().unwrap().contains("still present"));
    }

    #[test]
    fn codes_from_a_module_that_stayed_silent_are_not_called_a_live_fault() {
        let db = Store::open_in_memory().unwrap();
        let mut a = Adapter::connect(Sim::from_spec("odyssey-2016:engine-off").unwrap()).unwrap();
        a.transport_mut().inject("04", "7E9 01 44");
        let out = clear_with_history(&mut a, &db, &Ident::default(), &[], None).unwrap();
        assert_eq!(out["clear_status"], "not_acknowledged");
    }

    #[test]
    fn unreadable_clear_reply_with_codes_still_there_is_not_called_a_live_fault() {
        let db = Store::open_in_memory().unwrap();
        let mut a = Adapter::connect(Sim::from_spec("odyssey-2016:engine-off").unwrap()).unwrap();
        a.transport_mut().inject("04", "STOPPED");
        let out = clear_with_history(&mut a, &db, &Ident::default(), &[], None).unwrap();
        assert_eq!(out["clear_status"], "not_acknowledged");
        assert!(!out["next_steps"][0].as_str().unwrap().contains("still present"));
    }

    #[test]
    fn a_module_refusing_is_named_with_the_reason() {
        let db = Store::open_in_memory().unwrap();
        let mut a = Adapter::connect(Sim::from_spec("odyssey-2016:engine-off").unwrap()).unwrap();
        a.transport_mut().inject("04", "7E8 03 7F 04 22");
        let out = clear_with_history(&mut a, &db, &Ident::default(), &[], None).unwrap();
        assert_eq!(out["clear_status"], "refused");
        assert_eq!(out["targets"][0]["refused"][0]["ecu"], "7E8");
        assert!(out["targets"][0]["refused"][0]["meaning"].as_str().unwrap().contains("conditions not correct"));
    }

    #[test]
    fn default_clear_never_touches_modules_and_says_which_have_codes() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("kona-ev-2020");
        let out = clear_with_history(&mut a, &db, &ident("kona-ev-2020"), &[], None).unwrap();
        assert_eq!(out["clear_status"], "nothing_to_clear");
        assert!(out["next_steps"][1].as_str().unwrap().contains("IEB"), "{out}");
        assert!(!a.transport().log.iter().any(|c| c == "04" || c.starts_with("14")));
        assert!(sessions(&db).is_empty());
    }

    #[test]
    fn a_named_module_is_cleared_and_verified() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("kona-ev-2020");
        let out = clear_with_history(&mut a, &db, &ident("kona-ev-2020"), &only(&["ieb"]), None).unwrap();
        assert_eq!(out["clear_status"], "cleared", "{out}");
        assert_eq!(out["targets"][0]["target"], "IEB");
        let log = a.transport().log.join(" ");
        assert!(log.contains("ATSH7D1 ATCRA7D9 ATFCSH7D1 ATFCSD300000 ATFCSM1 14FFFFFF"), "{log}");
        assert!(!a.transport().log.contains(&"04".to_string()));
    }

    #[test]
    fn all_means_engine_and_every_module_with_codes() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("kona-ev-2020");
        let out = clear_with_history(&mut a, &db, &ident("kona-ev-2020"), &only(&["all"]), None).unwrap();
        assert_eq!(out["targets"].as_array().unwrap().iter().map(|t| t["target"].as_str().unwrap()).collect::<Vec<_>>(), vec!["IEB"]);
    }

    #[test]
    fn unknown_target_names_are_rejected_with_the_valid_list() {
        let db = Store::open_in_memory().unwrap();
        let err = clear_with_history(&mut adapter("kona-ev-2020"), &db, &ident("kona-ev-2020"), &only(&["flux"]), None).unwrap_err();
        assert!(err.to_string().contains("IEB") && err.to_string().contains("all"), "{err}");
    }

    #[test]
    fn module_clear_that_is_refused_is_reported_per_target() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("kona-ev-2020");
        a.transport_mut().inject("14FFFFFF", "7D9 03 7F 14 22");
        let out = clear_with_history(&mut a, &db, &ident("kona-ev-2020"), &only(&["IEB"]), None).unwrap();
        assert_eq!(out["clear_status"], "refused");
        assert_eq!(out["targets"][0]["refused"][0]["meaning"], "conditions not correct (often: engine must be off, ignition on, vehicle stopped)");
    }

    #[test]
    fn duplicate_names_are_cleared_once() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("kona-ev-2020");
        let out = clear_with_history(&mut a, &db, &ident("kona-ev-2020"), &only(&["IEB", "ieb"]), None).unwrap();
        assert_eq!(out["targets"].as_array().unwrap().len(), 1);
        assert_eq!(a.transport().log.iter().filter(|c| *c == "14FFFFFF").count(), 1);
    }

    #[test]
    fn unreadable_codes_block_an_unnamed_clear() {
        let db = Store::open_in_memory().unwrap();
        let mut sim = Sim::from_spec("odyssey-2016:engine-off").unwrap();
        sim.inject("03", "CAN ERROR");
        sim.inject("03", "CAN ERROR");
        let err = clear_with_history(&mut Adapter::connect(sim).unwrap(), &db, &Ident::default(), &[], None).unwrap_err();
        assert_eq!(kind(err), "scan_incomplete");
    }

    #[test]
    fn a_module_that_refuses_the_verification_read_is_unverified() {
        let db = Store::open_in_memory().unwrap();
        let mut a = adapter("kona-ev-2020");
        a.transport_mut().inject_where(0x7D1, Some("14FFFFFF"), "1902FF", "7D9 03 7F 19 22");
        let out = clear_with_history(&mut a, &db, &ident("kona-ev-2020"), &only(&["IEB"]), None).unwrap();
        assert_eq!(out["clear_status"], "unverified", "{out}");
    }

    #[test]
    fn a_module_read_over_kwp_is_cleared_over_kwp() {
        let db = Store::open_in_memory().unwrap();
        let mut sim = Sim::new("kona-ev-2020").unwrap();
        sim.add_module(0x7BB, true, &[([0x96, 0x20, 0x00], 0xA0)]);
        let mut a = Adapter::connect(sim).unwrap();
        clear_with_history(&mut a, &db, &ident("kona-ev-2020"), &only(&["HVAC"]), None).unwrap();
        assert!(a.transport().log.contains(&"14FF00".to_string()));
        assert!(!a.transport().log.contains(&"14FFFFFF".to_string()));
    }

    #[test]
    fn rescan_keeps_vehicle_key_when_vin_stops_reading() {
        let db = Store::open_in_memory().unwrap();
        let mut a = Adapter::connect(Sim::from_spec("odyssey-2016:engine-off").unwrap()).unwrap();
        a.transport_mut().forget_vin_after_clear = true;
        clear_with_history(&mut a, &db, &Ident::default(), &[], None).unwrap();
        assert!(db.sessions(None, 5).unwrap().iter().all(|s| s.vehicle == "5FNRL5H60GB000001"));
    }

    #[test]
    fn clear_is_recorded_as_unverified_when_rescan_fails() {
        let db = Store::open_in_memory().unwrap();
        let mut a = Adapter::connect(Sim::from_spec("odyssey-2016:engine-off").unwrap()).unwrap();
        a.transport_mut().ignition_off_after_clear = true;
        let out = clear_with_history(&mut a, &db, &Ident::default(), &[], None).unwrap();
        assert_eq!(out["clear_status"], "unverified");
        assert!(out["next_steps"][0].as_str().unwrap().contains("Do NOT run `carlink clear` again"));
        assert_eq!(sessions(&db), vec!["clear", "pre-clear"]);
        assert!(!db.sessions(None, 1).unwrap()[0].codes_complete);
    }

    #[test]
    fn garbled_clear_reply_is_settled_by_the_rescan() {
        let db = Store::open_in_memory().unwrap();
        let mut a = Adapter::connect(Sim::from_spec("odyssey-2016:engine-off").unwrap()).unwrap();
        a.transport_mut().append("04", "CAN ERROR");
        let out = clear_with_history(&mut a, &db, &Ident::default(), &[], None).unwrap();
        assert_eq!(out["clear_status"], "cleared");
        assert!(out["problems"][0].as_str().unwrap().contains("unreadable"));
        assert_eq!(a.transport().log.iter().filter(|c| *c == "04").count(), 1);
    }
}
