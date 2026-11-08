//! Rule-based findings from a generic scan. Thresholds: wiki `diagnostic-thresholds`.

use crate::dtc;
use crate::scan::Scan;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Critical,
    Warning,
    Advice,
    Info,
}

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub severity: Severity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub title: String,
    pub detail: String,
    pub recommendation: String,
}

fn finding(severity: Severity, title: impl Into<String>, detail: impl Into<String>, recommendation: impl Into<String>) -> Finding {
    Finding { severity, code: None, title: title.into(), detail: detail.into(), recommendation: recommendation.into() }
}

/// The engine ECU's value (see `scan::engine_ecu`) when it reports one, else the first module's.
fn live(s: &Scan, key: &str) -> Option<f64> {
    let engine = crate::scan::engine_ecu(s.ecus.iter().map(String::as_str));
    let mine = s.live.iter().filter(|r| r.reading.key == key);
    mine.clone().find(|r| Some(r.ecu.as_str()) == engine).or_else(|| mine.clone().next()).map(|r| r.reading.value)
}

/// Codes whose fault can destroy the engine or the high-voltage system if the car keeps being driven.
const STOP_DRIVING: &[(&str, &str)] = &[
    ("P0217", "Engine overheating. Stop driving; let it cool, check coolant level and the cooling fan."),
    ("P0218", "Transmission over-temperature. Stop driving and let it cool; check the fluid level and condition."),
    ("P0524", "Engine oil pressure too low. Stop the engine now and check the oil level before driving any further."),
    ("P0AA6", "High-voltage isolation fault: high voltage may be leaking to the body. Do not drive or charge; have an EV-qualified technician inspect it."),
];

/// P0300 (random) and P0301–P0312 (one cylinder), plus P0313/P0314; P0315 and up are other faults.
pub fn is_misfire(code: &str) -> bool {
    ("P0300"..="P0314").contains(&code)
}

const MISFIRE_ADVICE: &str = "If the check-engine light is FLASHING, stop driving: a severe misfire overheats and destroys the catalytic converter. Otherwise check the spark plug, ignition coil and injector on the named cylinder.";

const CONTINUOUS: [&str; 3] = ["misfire", "fuel_system", "components"];

#[derive(Debug, Clone, Serialize)]
pub struct Inspection {
    /// Would an OBD-based emissions (I/M) inspection most likely pass right now?
    pub likely_pass: bool,
    pub reasons: Vec<String>,
}

fn incomplete_monitors(s: &Scan) -> Vec<&'static str> {
    let mut v: Vec<&str> = s.monitors.iter().flat_map(|m| m.status.monitors.iter()).filter(|m| m.incomplete).map(|m| m.name).collect();
    v.sort();
    v.dedup();
    v
}

/// The one readiness rule (US I/M practice; some states are stricter): continuous monitors must be ready;
/// of the others, MY1996–2000 may have two incomplete and MY2001+ one.
fn readiness_problems(incomplete: &[&str], model_year: Option<u16>) -> Option<String> {
    let allowed = if model_year.is_some_and(|y| y <= 2000) { 2 } else { 1 };
    if let Some(c) = incomplete.iter().find(|m| CONTINUOUS.contains(m)) {
        return Some(format!("the {c} self-test is not ready"));
    }
    let others: Vec<&str> = incomplete.iter().copied().filter(|m| !CONTINUOUS.contains(m)).collect();
    (others.len() > allowed).then(|| format!("{} self-tests are not ready ({}); at most {allowed} allowed", others.len(), others.join(", ")))
}

/// US OBD I/M rules of thumb: MIL off, continuous monitors ready, at most one (MY2001+) or two
/// (MY1996–2000) other monitors incomplete; many states also fail a car with permanent codes.
pub fn inspection(s: &Scan, model_year: Option<u16>) -> Option<Inspection> {
    if s.monitors.is_empty() {
        return None;
    }
    let mut reasons = Vec::new();
    if !s.codes_complete {
        reasons.push("the trouble-code read was incomplete, so permanent codes may be missing".to_string());
    }
    if s.monitors.iter().any(|m| m.status.mil_on) {
        reasons.push("the check-engine light is on".to_string());
    }
    reasons.extend(readiness_problems(&incomplete_monitors(s), model_year));
    let permanent: Vec<&str> = s.dtcs.iter().filter(|d| d.kind == "permanent").map(|d| d.code.as_str()).collect();
    if !permanent.is_empty() {
        reasons.push(format!("permanent code(s) {} (many states fail these until the car clears them itself)", permanent.join(", ")));
    }
    Some(Inspection { likely_pass: reasons.is_empty(), reasons })
}

/// Findings from service 06: misfiring cylinders (by count) and self-tests outside their limits.
pub fn tests(t: &crate::mode06::Tests) -> Vec<Finding> {
    let mut out = Vec::new();
    for m in t.misfire.iter().filter(|m| m.cylinder > 0) {
        let (cur, avg) = (m.current.unwrap_or(0.0), m.average.unwrap_or(0.0));
        let sev = if cur >= 100.0 { Severity::Warning } else if cur >= 20.0 || avg >= 50.0 { Severity::Advice } else { continue };
        out.push(finding(
            sev,
            format!("Cylinder {} misfiring", m.cylinder),
            format!("{cur:.0} misfires this drive; {avg:.0} on average over recent drives (zero is normal; a code typically sets around a few hundred in one drive)."),
            "Swap this cylinder's ignition coil with a neighbour's, drive, and run `carlink tests` again: if the count moves with the coil, replace the coil; if not, check the spark plug, then the injector and compression.",
        ));
    }
    for r in t.results.iter().filter(|r| r.verdict == "FAIL") {
        out.push(finding(
            Severity::Warning,
            format!("Self-test failing: {} — {}", r.monitor, r.test),
            format!("Value {} {} is outside its limits (min {}, max {}).", r.value.map(|v| v.to_string()).unwrap_or_default(), r.unit, r.min.map(|v| v.to_string()).unwrap_or("-".into()), r.max.map(|v| v.to_string()).unwrap_or("-".into())),
            "The car's own test of this system is failing even if no code has set yet; diagnose that system.",
        ));
    }
    out
}

/// The self-test that must report complete after a repair before a code can be called fixed.
pub fn monitor_for(code: &str) -> &'static str {
    match code {
        c if is_misfire(c) => "misfire",
        "P0420" | "P0421" | "P0430" | "P0431" => "catalyst",
        c if ("P0440"..="P0469").contains(&c) || c.starts_with("P145") || c.starts_with("P245") => "evap",
        "P0135" | "P0141" | "P0147" | "P0155" | "P0161" | "P0167" => "oxygen_sensor_heater",
        c if ("P0030"..="P0064").contains(&c) => "oxygen_sensor_heater",
        c if ("P0130"..="P0167").contains(&c) || ("P2195"..="P2199").contains(&c) => "oxygen_sensor",
        "P0171" | "P0172" | "P0174" | "P0175" => "fuel_system",
        c if ("P0400"..="P0409").contains(&c) || c.starts_with("P001") || c.starts_with("P002") => "egr_vvt",
        _ => "components",
    }
}

fn f_from_c(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

pub fn generic(s: &Scan, model_year: Option<u16>) -> Vec<Finding> {
    let mut out = Vec::new();
    if s.monitors.iter().any(|m| m.status.mil_on) {
        out.push(finding(Severity::Warning, "Check-engine light (MIL) is on", "An emissions-related fault is confirmed.", "Read the codes below; clear them only after the cause is fixed."));
    }

    let mut codes: Vec<&str> = s.dtcs.iter().map(|d| d.code.as_str()).collect();
    codes.sort();
    codes.dedup();
    for code in codes {
        let entries: Vec<_> = s.dtcs.iter().filter(|d| d.code == code).collect();
        let kinds: Vec<&str> = entries.iter().map(|d| d.kind).collect();
        let info = dtc::lookup(code);
        let what = info.description.clone().unwrap_or_else(|| "manufacturer-specific code".into());
        let stop = STOP_DRIVING.iter().find(|(c, _)| *c == code).map(|(_, advice)| *advice);
        let severity = if stop.is_some() {
            Severity::Critical
        } else if kinds.iter().all(|k| *k == "pending") {
            Severity::Advice
        } else {
            Severity::Warning
        };
        out.push(Finding {
            severity,
            code: Some(code.into()),
            title: format!("{code} — {what}"),
            detail: format!("{} ({}); reported by ECU {} as {}.", info.system, if info.manufacturer_specific { "manufacturer-specific" } else { "SAE generic" }, entries[0].ecu, kinds.join(", ")),
            recommendation: if let Some(advice) = stop {
                advice.into()
            } else if is_misfire(code) {
                MISFIRE_ADVICE.into()
            } else if kinds == ["pending"] {
                "Pending: seen once, not yet confirmed. Re-scan after another drive.".into()
            } else {
                "Diagnose the cause using the freeze frame and the vehicle notes before clearing.".into()
            },
        });
    }

    let incomplete = incomplete_monitors(s);
    if !incomplete.is_empty() {
        let fails = readiness_problems(&incomplete, model_year).is_some();
        let (sev, verdict) = if fails {
            (Severity::Advice, "As things stand the car would NOT pass an emissions (I/M) inspection.")
        } else {
            (Severity::Info, "Within the usual inspection allowance, but some states are stricter.")
        };
        out.push(finding(sev, "Readiness monitors incomplete", format!("Not ready: {}. {verdict}", incomplete.join(", ")),
            "Normal after codes are cleared or the battery is disconnected; they complete with a few days of mixed city and highway driving. Re-scan before an inspection."));
    }

    let running = live(s, "rpm").is_some_and(|r| r > 400.0);
    if let Some(v) = s.battery_voltage {
        if running && v < 12.6 {
            out.push(finding(Severity::Warning, "Alternator may not be charging", format!("{v:.1} V with the engine running — no higher than a resting battery."), "Turn on the headlights and blower and re-check: below 13.5 V with loads on means test the alternator, belt and battery cables."));
        } else if running && v < 13.2 {
            out.push(finding(Severity::Info, "Charging voltage on the low side", format!("{v:.1} V with the engine running."), "Often normal: Honda (and other) charging control lowers the voltage at light load. Turn on the headlights and blower; it should rise above 13.5 V. If it does not, test the alternator."));
        } else if running && v > 15.0 {
            out.push(finding(Severity::Warning, "Charging voltage high", format!("{v:.1} V with the engine running."), "Over-charging damages the battery and electronics; have the regulator/alternator checked."));
        } else if !running && v <= 12.2 {
            out.push(finding(Severity::Warning, "12 V battery weak", format!("{v:.1} V at rest (healthy is 12.6+)."), "Charge and load-test the 12 V battery; check for parasitic drain."));
        } else if !running && v <= 12.4 {
            out.push(finding(Severity::Advice, "12 V battery partly discharged", format!("{v:.1} V at rest."), "Drive or charge it, then re-check after the car has rested a few hours."));
        }
    }

    if let Some(c) = live(s, "coolant_temp") {
        if c > 110.0 {
            out.push(finding(Severity::Critical, "Engine overheating", format!("Coolant {c:.0} °C ({:.0} °F).", f_from_c(c)), "Stop driving. Check coolant level (engine cold), fan operation and for leaks."));
        } else if c > 104.0 {
            out.push(finding(Severity::Warning, "Coolant temperature high", format!("Coolant {c:.0} °C ({:.0} °F); normal is 88–104 °C (190–220 °F).", f_from_c(c)), "Watch the gauge; check coolant level and radiator fan."));
        } else if c < 80.0 && live(s, "run_time").is_some_and(|t| t > 900.0) {
            out.push(finding(Severity::Advice, "Engine not warming up — possible stuck-open thermostat", format!("Coolant {c:.0} °C ({:.0} °F) after {:.0} min running.", f_from_c(c), live(s, "run_time").unwrap_or(0.0) / 60.0), "Usually a thermostat; it hurts fuel economy and heater output (often sets P0128)."));
        }
    }

    let fuel_system = live(s, "fuel_system");
    if let Some(code @ (8.0 | 16.0)) = fuel_system {
        let what = if code == 8.0 { "open loop because of a system fault" } else { "closed loop, but an oxygen sensor is faulty" };
        out.push(finding(Severity::Warning, "Fuel control is degraded", format!("The engine computer reports its fuel system as {what}."), "Look for oxygen/air-fuel sensor codes and check those sensors; fuel economy and emissions suffer until it is fixed."));
    }
    let closed_loop = fuel_system.is_none_or(|c| c == 2.0);
    for bank in [1, 2] {
        let (Some(st), Some(lt)) = (live(s, &format!("stft_b{bank}")), live(s, &format!("ltft_b{bank}"))) else { continue };
        let total = st + lt;
        if !running || !closed_loop || total.abs() <= 10.0 {
            continue;
        }
        let sev = if total.abs() >= 20.0 { Severity::Warning } else { Severity::Advice };
        let (dir, why) = if total > 0.0 { ("lean", "vacuum leak, weak fuel pump/injectors or a dirty MAF") } else { ("rich", "leaking injector, high fuel pressure or a faulty O2/MAF sensor") };
        out.push(finding(sev, format!("Engine bank {bank} running {dir}"), format!("Fuel trims total {total:+.1}% (short {st:+.1}%, long {lt:+.1}%)."), format!("Common causes: {why}. Normal is within ±10%.")));
    }

    if let Some(d) = live(s, "distance_with_mil").filter(|d| *d > 0.0) {
        out.push(finding(Severity::Info, "Driven with the check-engine light on", format!("{:.0} miles ({d:.0} km) since the MIL came on.", d * 0.621371), "Long distances with a misfire can damage the catalytic converter."));
    }
    if !s.codes_complete {
        // First of the warnings (the caller's sort is stable): every other conclusion depends on it.
        out.insert(0, finding(Severity::Warning, "Trouble-code read INCOMPLETE", s.errors.join("; "), "Some codes may be missing from this report. Run `carlink scan` again before drawing conclusions or clearing anything."));
    } else if !s.errors.is_empty() {
        out.push(finding(Severity::Info, "Scan incomplete", s.errors.join("; "), "Re-run with the ignition on; some modules may not support every request."));
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::Adapter;
    use crate::obd::decode_pid;
    use crate::scan::{EcuReading, scan};
    use crate::sim::Sim;

    fn base() -> Scan {
        let mut s = scan(&mut Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap()).unwrap();
        s.live.clear();
        s.monitors.clear();
        s.battery_voltage = None;
        s
    }

    fn with(mut s: Scan, pid: u8, data: &[u8]) -> Scan {
        for reading in crate::obd::decode_pid_all(pid, data) {
            s.live.push(EcuReading { ecu: "7E8".into(), reading });
        }
        s
    }

    fn titles(s: &Scan) -> Vec<(Severity, String)> {
        generic(s, None).into_iter().map(|f| (f.severity, f.title)).collect()
    }

    fn has(s: &Scan, sev: Severity, needle: &str) -> bool {
        titles(s).iter().any(|(v, t)| *v == sev && t.contains(needle))
    }

    #[test]
    fn inspection_verdict_follows_the_us_rules() {
        let odyssey = scan(&mut Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap()).unwrap();
        let i = inspection(&odyssey, Some(2016)).unwrap();
        assert!(!i.likely_pass);
        assert!(i.reasons.iter().any(|r| r.contains("check-engine light")));
        assert!(i.reasons.iter().any(|r| r.contains("permanent")));
        let mut clean = odyssey.clone();
        clean.dtcs.clear();
        clean.monitors[0].status = crate::obd::decode_monitor_status(&[0, 0x07, 0xE5, 0x04]).unwrap();
        assert!(inspection(&clean, Some(2016)).unwrap().likely_pass, "one incomplete non-continuous monitor is allowed");
        assert!(inspection(&base(), None).is_none_or(|i| i.likely_pass));
    }

    #[test]
    fn an_incomplete_code_read_cannot_be_a_likely_pass() {
        let mut s = base();
        s.monitors.push(crate::scan::EcuMonitors { ecu: "7E8".into(), status: crate::obd::decode_monitor_status(&[0, 0x07, 0x25, 0x00]).unwrap() });
        s.codes_complete = false;
        assert!(!inspection(&s, Some(2016)).unwrap().likely_pass);
    }

    #[test]
    fn repaired_codes_map_to_the_self_test_that_proves_them() {
        assert_eq!(monitor_for("P0303"), "misfire");
        assert_eq!(monitor_for("P0420"), "catalyst");
        assert_eq!(monitor_for("P1457"), "evap");
        assert_eq!(monitor_for("P0171"), "fuel_system");
        assert_eq!(monitor_for("P0141"), "oxygen_sensor_heater");
        assert_eq!(monitor_for("P0700"), "components");
    }

    #[test]
    fn healthy_scan_has_no_findings() {
        assert!(generic(&base(), None).is_empty());
    }

    #[test]
    fn odyssey_dtcs_mil_and_readiness() {
        let s = scan(&mut Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap()).unwrap();
        let f = generic(&s, None);
        assert!(f.iter().any(|x| x.severity == Severity::Warning && x.title.contains("Check-engine light")));
        let p0303 = f.iter().find(|x| x.code.as_deref() == Some("P0303")).unwrap();
        assert_eq!(p0303.severity, Severity::Warning);
        assert!(p0303.title.contains("Cylinder 3 Misfire"));
        assert!(p0303.detail.contains("stored, permanent"), "{}", p0303.detail);
        assert_eq!(f.iter().filter(|x| x.code.as_deref() == Some("P0303")).count(), 1);
        assert!(f.iter().any(|x| x.title.contains("Readiness") && x.detail.contains("evap")));
        assert!(f.iter().any(|x| x.title.contains("with the check-engine light on")));
    }

    #[test]
    fn pending_only_code_is_advice() {
        let mut s = base();
        s.dtcs.push(crate::scan::Dtc { code: "P0171".into(), kind: "pending", ecu: "7E8".into() });
        assert!(has(&s, Severity::Advice, "P0171"));
    }

    #[test]
    fn manufacturer_code_says_so() {
        let mut s = base();
        s.dtcs.push(crate::scan::Dtc { code: "P1457".into(), kind: "stored", ecu: "10".into() });
        assert!(has(&s, Severity::Warning, "P1457 — manufacturer-specific"));
    }

    #[test]
    fn two_incomplete_monitors_flag_inspection() {
        let mut s = base();
        s.monitors.push(crate::scan::EcuMonitors { ecu: "7E8".into(), status: crate::obd::decode_monitor_status(&[0, 0x07, 0x25, 0x21]).unwrap() });
        let f = generic(&s, None);
        let r = f.iter().find(|x| x.title.contains("Readiness")).unwrap();
        assert_eq!(r.severity, Severity::Advice);
        assert!(r.recommendation.contains("inspection"));
    }

    #[test]
    fn an_incomplete_continuous_monitor_fails_inspection() {
        let mut s = base();
        s.monitors.push(crate::scan::EcuMonitors { ecu: "7E8".into(), status: crate::obd::decode_monitor_status(&[0, 0x17, 0x25, 0x00]).unwrap() });
        let f = generic(&s, Some(2016));
        assert_eq!(f[0].severity, Severity::Advice);
        assert!(f[0].detail.contains("NOT pass"));
    }

    #[test]
    fn older_cars_may_have_two_incomplete_monitors() {
        let mut s = base();
        s.monitors.push(crate::scan::EcuMonitors { ecu: "7E8".into(), status: crate::obd::decode_monitor_status(&[0, 0x07, 0x25, 0x05]).unwrap() });
        assert_eq!(generic(&s, Some(2000))[0].severity, Severity::Info);
        assert_eq!(generic(&s, Some(2004))[0].severity, Severity::Advice);
    }

    #[test]
    fn exactly_12_2_volts_at_rest_is_weak() {
        let mut s = base();
        s.battery_voltage = Some(12.2);
        assert!(has(&s, Severity::Warning, "battery"));
    }

    #[test]
    fn incomplete_code_read_is_a_warning() {
        let mut s = base();
        s.codes_complete = false;
        s.errors.push("stored codes: adapter error: CAN ERROR".into());
        assert!(has(&s, Severity::Warning, "INCOMPLETE"));
    }

    #[test]
    fn temperatures_and_distances_are_also_in_us_units() {
        let s = with(base(), 0x05, &[40 + 112]);
        assert!(generic(&s, None)[0].detail.contains("°F"));
        let s = with(base(), 0x21, &[0x00, 0x64]);
        assert!(generic(&s, None)[0].detail.contains("miles"));
    }

    #[test]
    fn one_incomplete_monitor_is_info() {
        let mut s = base();
        s.monitors.push(crate::scan::EcuMonitors { ecu: "7E8".into(), status: crate::obd::decode_monitor_status(&[0, 0x07, 0x25, 0x04]).unwrap() });
        assert!(has(&s, Severity::Info, "Readiness"));
    }

    #[test]
    fn same_monitor_from_two_ecus_counts_once() {
        let mut s = base();
        for ecu in ["7E8", "7E9"] {
            s.monitors.push(crate::scan::EcuMonitors { ecu: ecu.into(), status: crate::obd::decode_monitor_status(&[0, 0x00, 0x04, 0x04]).unwrap() });
        }
        let f = generic(&s, None);
        let r = f.iter().find(|x| x.title.contains("Readiness")).unwrap();
        assert_eq!(r.severity, Severity::Info);
        assert!(r.detail.starts_with("Not ready: evap."), "{}", r.detail);
    }

    #[test]
    fn battery_voltage_engine_off() {
        let mut s = base();
        s.battery_voltage = Some(12.0);
        assert!(has(&s, Severity::Warning, "battery"));
        s.battery_voltage = Some(12.3);
        assert!(has(&s, Severity::Advice, "battery"));
        s.battery_voltage = Some(12.6);
        assert!(titles(&s).is_empty());
    }

    #[test]
    fn charging_voltage_engine_running() {
        let mut s = with(base(), 0x0C, &[0x0B, 0x40]);
        s.battery_voltage = Some(12.3);
        assert!(has(&s, Severity::Warning, "not be charging"));
        s.battery_voltage = Some(15.4);
        assert!(has(&s, Severity::Warning, "Charging voltage high"));
        s.battery_voltage = Some(14.2);
        assert!(titles(&s).is_empty());
    }

    #[test]
    fn honda_style_low_charging_voltage_is_only_info() {
        let mut s = with(base(), 0x0C, &[0x0B, 0x40]);
        s.battery_voltage = Some(12.9);
        assert!(has(&s, Severity::Info, "Charging voltage on the low side"));
        assert!(generic(&s, None)[0].recommendation.contains("13.5"));
    }

    #[test]
    fn engine_ecu_value_drives_findings_over_another_module() {
        let mut s = base();
        s.ecus = vec!["7E8".into(), "7E9".into()];
        s.live.push(EcuReading { ecu: "7E9".into(), reading: decode_pid(0x05, &[40 + 115]).unwrap() });
        s.live.push(EcuReading { ecu: "7E8".into(), reading: decode_pid(0x05, &[40 + 90]).unwrap() });
        assert!(titles(&s).is_empty(), "{:?}", titles(&s));
    }

    #[test]
    fn stop_driving_codes_are_critical() {
        for code in ["P0217", "P0524", "P0AA6"] {
            let mut s = base();
            s.dtcs.push(crate::scan::Dtc { code: code.into(), kind: "stored", ecu: "7E8".into() });
            assert!(has(&s, Severity::Critical, code), "{code}");
        }
    }

    #[test]
    fn non_misfire_p031x_codes_do_not_get_misfire_advice() {
        let mut s = base();
        s.dtcs.push(crate::scan::Dtc { code: "P0315".into(), kind: "stored", ecu: "7E8".into() });
        assert!(!generic(&s, None)[0].recommendation.to_lowercase().contains("flashing"));
    }

    #[test]
    fn misfire_codes_carry_the_flashing_light_warning() {
        let mut s = base();
        s.dtcs.push(crate::scan::Dtc { code: "P0302".into(), kind: "stored", ecu: "7E8".into() });
        let f = generic(&s, None);
        assert!(f[0].recommendation.to_lowercase().contains("flashing"), "{}", f[0].recommendation);
    }

    #[test]
    fn coolant_overheat_levels() {
        assert!(has(&with(base(), 0x05, &[40 + 112]), Severity::Critical, "overheating"));
        assert!(has(&with(base(), 0x05, &[40 + 107]), Severity::Warning, "Coolant"));
        assert!(titles(&with(base(), 0x05, &[40 + 90])).is_empty());
    }

    #[test]
    fn cold_coolant_after_long_run_suggests_thermostat() {
        let s = with(with(base(), 0x05, &[40 + 60]), 0x1F, &[0x04, 0x00]);
        assert!(has(&s, Severity::Advice, "thermostat"));
        assert!(titles(&with(base(), 0x05, &[40 + 60])).is_empty());
    }

    #[test]
    fn fuel_trim_lean_and_rich_only_when_running() {
        let run = |s| with(s, 0x0C, &[0x0B, 0x40]);
        // +12% STFT +10% LTFT = +22% lean
        let lean = run(with(with(base(), 0x06, &[143]), 0x07, &[141]));
        assert!(has(&lean, Severity::Warning, "bank 1 running lean"));
        // -12% total = rich advice on bank 2
        let rich = run(with(with(base(), 0x08, &[113]), 0x09, &[128]));
        assert!(has(&rich, Severity::Advice, "bank 2 running rich"));
        assert!(titles(&with(with(base(), 0x06, &[143]), 0x07, &[141])).is_empty());
    }

    #[test]
    fn faulty_fuel_control_is_a_warning() {
        assert!(has(&with(base(), 0x03, &[0x08, 0x00]), Severity::Warning, "Fuel control"));
        assert!(has(&with(base(), 0x03, &[0x10, 0x00]), Severity::Warning, "Fuel control"));
        assert!(titles(&with(base(), 0x03, &[0x02, 0x00])).is_empty());
    }

    #[test]
    fn fuel_trims_are_ignored_in_open_loop() {
        let lean_running = with(with(with(base(), 0x06, &[143]), 0x07, &[141]), 0x0C, &[0x0B, 0x40]);
        assert!(titles(&with(lean_running, 0x03, &[0x01, 0x00])).is_empty());
    }

    #[test]
    fn scan_errors_are_surfaced() {
        let mut s = base();
        s.errors.push("vin: adapter error: BUS ERROR".into());
        assert!(has(&s, Severity::Info, "incomplete"));
    }
}
