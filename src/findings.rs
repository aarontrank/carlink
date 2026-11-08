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

fn live(s: &Scan, key: &str) -> Option<f64> {
    s.live.iter().find(|r| r.reading.key == key).map(|r| r.reading.value)
}

pub fn generic(s: &Scan) -> Vec<Finding> {
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
        let severity = if kinds.iter().all(|k| *k == "pending") { Severity::Advice } else { Severity::Warning };
        out.push(Finding {
            severity,
            code: Some(code.into()),
            title: format!("{code} — {what}"),
            detail: format!("{} ({}); reported by ECU {} as {}.", info.system, if info.manufacturer_specific { "manufacturer-specific" } else { "SAE generic" }, entries[0].ecu, kinds.join(", ")),
            recommendation: if kinds == ["pending"] {
                "Pending: seen once, not yet confirmed. Re-scan after another drive.".into()
            } else {
                "Diagnose the cause using the freeze frame and the vehicle notes before clearing.".into()
            },
        });
    }

    let mut incomplete: Vec<&str> = s.monitors.iter().flat_map(|m| m.status.monitors.iter()).filter(|m| m.incomplete).map(|m| m.name).collect();
    incomplete.sort();
    incomplete.dedup();
    if !incomplete.is_empty() {
        let sev = if incomplete.len() > 1 { Severity::Advice } else { Severity::Info };
        out.push(finding(sev, "Readiness monitors incomplete", format!("Not ready: {}.", incomplete.join(", ")),
            "Normal after codes are cleared or the battery is disconnected; complete them with mixed driving. A 2001+ car fails an emissions inspection with more than one incomplete."));
    }

    let running = live(s, "rpm").is_some_and(|r| r > 400.0);
    if let Some(v) = s.battery_voltage {
        if running && v < 13.0 {
            out.push(finding(Severity::Warning, "Charging voltage low", format!("{v:.1} V with the engine running (expected 13.5–14.7)."), "Have the alternator, belt and battery tested. Some cars lower voltage deliberately (smart charging); re-check at higher load."));
        } else if running && v > 15.0 {
            out.push(finding(Severity::Warning, "Charging voltage high", format!("{v:.1} V with the engine running."), "Over-charging damages the battery and electronics; have the regulator/alternator checked."));
        } else if !running && v < 12.2 {
            out.push(finding(Severity::Warning, "12 V battery weak", format!("{v:.1} V at rest (healthy is 12.6+)."), "Charge and load-test the 12 V battery; check for parasitic drain."));
        } else if !running && v < 12.4 {
            out.push(finding(Severity::Advice, "12 V battery partly discharged", format!("{v:.1} V at rest."), "Drive or charge it, then re-check after the car has rested a few hours."));
        }
    }

    if let Some(c) = live(s, "coolant_temp") {
        if c > 110.0 {
            out.push(finding(Severity::Critical, "Engine overheating", format!("Coolant {c:.0} °C."), "Stop driving. Check coolant level (engine cold), fan operation and for leaks."));
        } else if c > 104.0 {
            out.push(finding(Severity::Warning, "Coolant temperature high", format!("Coolant {c:.0} °C (normal 88–104)."), "Watch the gauge; check coolant level and radiator fan."));
        } else if c < 80.0 && live(s, "run_time").is_some_and(|t| t > 900.0) {
            out.push(finding(Severity::Advice, "Engine not warming up — possible stuck-open thermostat", format!("Coolant {c:.0} °C after {:.0} min running.", live(s, "run_time").unwrap_or(0.0) / 60.0), "Usually a thermostat; it hurts fuel economy and heater output (often sets P0128)."));
        }
    }

    for bank in [1, 2] {
        let (Some(st), Some(lt)) = (live(s, &format!("stft_b{bank}")), live(s, &format!("ltft_b{bank}"))) else { continue };
        let total = st + lt;
        if !running || total.abs() <= 10.0 {
            continue;
        }
        let sev = if total.abs() >= 20.0 { Severity::Warning } else { Severity::Advice };
        let (dir, why) = if total > 0.0 { ("lean", "vacuum leak, weak fuel pump/injectors or a dirty MAF") } else { ("rich", "leaking injector, high fuel pressure or a faulty O2/MAF sensor") };
        out.push(finding(sev, format!("Engine bank {bank} running {dir}"), format!("Fuel trims total {total:+.1}% (short {st:+.1}%, long {lt:+.1}%)."), format!("Common causes: {why}. Normal is within ±10%.")));
    }

    if let Some(d) = live(s, "distance_with_mil").filter(|d| *d > 0.0) {
        out.push(finding(Severity::Info, "Driven with the check-engine light on", format!("{d:.0} km since the MIL came on."), "Long distances with a misfire can damage the catalytic converter."));
    }
    if !s.errors.is_empty() {
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
        s.live.push(EcuReading { ecu: "7E8".into(), reading: decode_pid(pid, data).unwrap() });
        s
    }

    fn titles(s: &Scan) -> Vec<(Severity, String)> {
        generic(s).into_iter().map(|f| (f.severity, f.title)).collect()
    }

    fn has(s: &Scan, sev: Severity, needle: &str) -> bool {
        titles(s).iter().any(|(v, t)| *v == sev && t.contains(needle))
    }

    #[test]
    fn healthy_scan_has_no_findings() {
        assert!(generic(&base()).is_empty());
    }

    #[test]
    fn odyssey_dtcs_mil_and_readiness() {
        let s = scan(&mut Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap()).unwrap();
        let f = generic(&s);
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
        let f = generic(&s);
        let r = f.iter().find(|x| x.title.contains("Readiness")).unwrap();
        assert_eq!(r.severity, Severity::Advice);
        assert!(r.recommendation.contains("inspection"));
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
            s.monitors.push(crate::scan::EcuMonitors { ecu: ecu.into(), status: crate::obd::decode_monitor_status(&[0, 0x44, 0, 0]).unwrap() });
        }
        let f = generic(&s);
        let r = f.iter().find(|x| x.title.contains("Readiness")).unwrap();
        assert_eq!((r.severity, r.detail.as_str()), (Severity::Info, "Not ready: components."));
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
        s.battery_voltage = Some(12.6);
        assert!(has(&s, Severity::Warning, "Charging voltage low"));
        s.battery_voltage = Some(15.4);
        assert!(has(&s, Severity::Warning, "Charging voltage high"));
        s.battery_voltage = Some(14.2);
        assert!(titles(&s).is_empty());
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
    fn scan_errors_are_surfaced() {
        let mut s = base();
        s.errors.push("vin: adapter error: BUS ERROR".into());
        assert!(has(&s, Severity::Info, "incomplete"));
    }
}
