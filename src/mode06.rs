//! OBD service 06: the ECU's own self-test results with their limits (wiki `obd2-mode06`,
//! `honda-mode06`). CAN cars use standardized monitor IDs and scaling, including per-cylinder misfire
//! counts; K-line cars report raw manufacturer tests.

use crate::adapter::{Adapter, Transport};
use anyhow::Result;
use crate::obd;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TestResult {
    pub ecu: String,
    pub monitor: String,
    pub test: String,
    pub unit: &'static str,
    pub value: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    /// pass, FAIL, not_run (all zero: not run since codes were cleared), or unknown (unknown scaling).
    pub verdict: &'static str,
    pub raw: String,
}

/// Misfire counters for one cylinder (0 = all cylinders): `average` is the weighted average over the
/// last 10 drives (TID 0B), `current` this drive (TID 0C). Counts, not a pass/fail test.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Misfire {
    pub ecu: String,
    pub cylinder: u8,
    pub average: Option<f64>,
    pub current: Option<f64>,
}

#[derive(Debug, Default, Serialize)]
pub struct Tests {
    /// False when no ECU answered service 06 (e.g. an EV).
    pub supported: bool,
    pub misfire: Vec<Misfire>,
    pub results: Vec<TestResult>,
    pub errors: Vec<String>,
}

/// (scale, offset, unit, signed) for a Unit and Scaling ID (J1979 Appendix E as in the wiki table).
fn uasid(id: u8) -> Option<(f64, f64, &'static str, bool)> {
    Some(match id {
        0x01 => (1.0, 0.0, "raw", false),
        0x02 => (0.1, 0.0, "raw", false),
        0x03 => (0.01, 0.0, "raw", false),
        0x04 => (0.001, 0.0, "raw", false),
        0x05 => (0.0000305, 0.0, "ratio", false),
        0x06 => (0.000305, 0.0, "ratio", false),
        0x07 => (0.25, 0.0, "rpm", false),
        0x08 => (0.01, 0.0, "km/h", false),
        0x09 => (1.0, 0.0, "km/h", false),
        0x0A => (0.122, 0.0, "mV", false),
        0x0B => (0.001, 0.0, "V", false),
        0x0C => (0.01, 0.0, "V", false),
        0x0D => (0.00390625, 0.0, "mA", false),
        0x0E => (0.001, 0.0, "A", false),
        0x0F => (0.01, 0.0, "A", false),
        0x10 => (1.0, 0.0, "ms", false),
        0x11 => (100.0, 0.0, "ms", false),
        0x12 => (1.0, 0.0, "s", false),
        0x13 => (1.0, 0.0, "mΩ", false),
        0x14 => (1.0, 0.0, "Ω", false),
        0x15 => (1.0, 0.0, "kΩ", false),
        0x16 => (0.1, -40.0, "°C", false),
        0x17 => (0.01, 0.0, "kPa", false),
        0x18 => (0.0117, 0.0, "kPa", false),
        0x19 => (0.079, 0.0, "kPa", false),
        0x1A => (1.0, 0.0, "kPa", false),
        0x1B => (10.0, 0.0, "kPa", false),
        0x1C => (0.01, 0.0, "°", false),
        0x1D => (0.5, 0.0, "°", false),
        0x1E => (0.0000305, 0.0, "λ", false),
        0x1F => (0.05, 0.0, "A/F", false),
        0x20 => (0.00390625, 0.0, "ratio", false),
        0x21 => (1.0, 0.0, "mHz", false),
        0x22 => (1.0, 0.0, "Hz", false),
        0x23 => (1.0, 0.0, "kHz", false),
        0x24 => (1.0, 0.0, "counts", false),
        0x25 => (1.0, 0.0, "km", false),
        0x26 => (0.1, 0.0, "mV/ms", false),
        0x27 => (0.01, 0.0, "g/s", false),
        0x28 => (1.0, 0.0, "g/s", false),
        0x29 => (0.25, 0.0, "Pa/s", false),
        0x2A => (0.001, 0.0, "kg/h", false),
        0x2B => (1.0, 0.0, "switches", false),
        0x2C => (0.01, 0.0, "g/cyl", false),
        0x2D => (0.01, 0.0, "mg/stroke", false),
        0x2E => (1.0, 0.0, "true/false", false),
        0x2F => (0.01, 0.0, "%", false),
        0x30 => (0.001526, 0.0, "%", false),
        0x31 => (0.001, 0.0, "L", false),
        0x32 => (0.0000305, 0.0, "in", false),
        0x33 => (0.00024414, 0.0, "λ", false),
        0x34 => (1.0, 0.0, "min", false),
        0x35 => (10.0, 0.0, "ms", false),
        0x36 => (0.01, 0.0, "g", false),
        0x37 => (0.1, 0.0, "g", false),
        0x38 => (1.0, 0.0, "g", false),
        0x39 => (0.01, -327.68, "%", false),
        0x3A => (0.001, 0.0, "g", false),
        0x3B => (0.0001, 0.0, "g", false),
        0x3C => (0.1, 0.0, "µs", false),
        0x3D => (0.01, 0.0, "mA", false),
        0x3E => (0.00006103516, 0.0, "mm²", false),
        0x3F => (0.01, 0.0, "L", false),
        0x40 => (1.0, 0.0, "ppm", false),
        0x41 => (0.01, 0.0, "µA", false),
        0x81 => (1.0, 0.0, "raw", true),
        0x82 => (0.1, 0.0, "raw", true),
        0x83 => (0.01, 0.0, "raw", true),
        0x84 => (0.001, 0.0, "raw", true),
        0x85 => (0.0000305, 0.0, "ratio", true),
        0x86 => (0.000305, 0.0, "ratio", true),
        0x87 => (1.0, 0.0, "ppm", true),
        0x8A => (0.122, 0.0, "mV", true),
        0x8B => (0.001, 0.0, "V", true),
        0x8C => (0.01, 0.0, "V", true),
        0x8D => (0.00390625, 0.0, "mA", true),
        0x8E => (0.001, 0.0, "A", true),
        0x90 => (1.0, 0.0, "ms", true),
        0x96 => (0.1, 0.0, "°C", true),
        0x99 => (0.1, 0.0, "kPa", true),
        0x9C => (0.01, 0.0, "°", true),
        0x9D => (0.5, 0.0, "°", true),
        0xA8 => (1.0, 0.0, "g/s", true),
        0xA9 => (0.25, 0.0, "Pa/s", true),
        0xAD => (0.01, 0.0, "mg/stroke", true),
        0xAE => (0.1, 0.0, "mg/stroke", true),
        0xAF => (0.01, 0.0, "%", true),
        0xB0 => (0.003052, 0.0, "%", true),
        0xB1 => (2.0, 0.0, "mV/s", true),
        0xFC => (0.01, 0.0, "kPa", true),
        0xFD => (0.001, 0.0, "kPa", true),
        0xFE => (0.25, 0.0, "Pa", true),
        _ => return None,
    })
}

fn monitor_name(mid: u8) -> String {
    let bank_sensor = |base: u8| format!("bank {} sensor {}", (mid - base) / 4 + 1, (mid - base) % 4 + 1);
    match mid {
        0x01..=0x10 => format!("O2 sensor {}", bank_sensor(0x01)),
        0x21..=0x24 => format!("catalyst bank {}", mid - 0x20),
        0x31..=0x34 => format!("EGR bank {}", mid - 0x30),
        0x35..=0x38 => format!("VVT bank {}", mid - 0x34),
        0x39 => "EVAP 0.150 in leak (cap off)".into(),
        0x3A => "EVAP 0.090 in leak".into(),
        0x3B => "EVAP 0.040 in leak".into(),
        0x3C => "EVAP 0.020 in leak".into(),
        0x3D => "EVAP purge flow".into(),
        0x41..=0x50 => format!("O2 sensor heater {}", bank_sensor(0x41)),
        0x61..=0x64 => format!("heated catalyst bank {}", mid - 0x60),
        0x71..=0x74 => format!("secondary air {}", mid - 0x70),
        0x81..=0x84 => format!("fuel system bank {}", mid - 0x80),
        0x85..=0x86 => format!("boost pressure bank {}", mid - 0x84),
        0x90..=0x91 => format!("NOx absorber bank {}", mid - 0x8F),
        0x98..=0x99 => format!("NOx catalyst bank {}", mid - 0x97),
        0xA1 => "misfire (all cylinders)".into(),
        0xA2..=0xAD => format!("misfire cylinder {}", mid - 0xA1),
        0xB0..=0xB1 => format!("particulate filter bank {}", mid - 0xAF),
        _ => format!("manufacturer monitor 0x{mid:02X}"),
    }
}

fn test_name(tid: u8) -> String {
    match tid {
        0x01 => "rich→lean threshold voltage".into(),
        0x02 => "lean→rich threshold voltage".into(),
        0x03 => "low voltage for switch time".into(),
        0x04 => "high voltage for switch time".into(),
        0x05 => "rich→lean switch time".into(),
        0x06 => "lean→rich switch time".into(),
        0x07 => "minimum sensor voltage".into(),
        0x08 => "maximum sensor voltage".into(),
        0x09 => "time between transitions".into(),
        0x0A => "sensor period".into(),
        0x0B => "misfire count, 10-drive average".into(),
        0x0C => "misfire count, this drive".into(),
        _ => format!("manufacturer test 0x{tid:02X}"),
    }
}

fn round4(v: f64) -> f64 {
    (v * 10_000.0).round() / 10_000.0
}

fn word(b: &[u8], i: usize) -> u16 {
    u16::from_be_bytes([b[i], b[i + 1]])
}

/// One CAN service 06 reply (`46` then 9-byte records `MID TID UASID value min max`).
pub fn decode_can(ecu: &str, data: &[u8]) -> (Vec<TestResult>, Vec<Misfire>) {
    let mut results = Vec::new();
    let mut misfire: Vec<Misfire> = Vec::new();
    let Some(records) = data.strip_prefix(&[0x46][..]) else { return (results, misfire) };
    for r in records.as_chunks::<9>().0 {
        let (mid, tid, id) = (r[0], r[1], r[2]);
        let raw = [word(r, 3), word(r, 5), word(r, 7)];
        let scale = uasid(id);
        let conv = |w: u16| scale.map(|(k, off, _, signed)| round4(if signed { w as i16 as f64 } else { w as f64 } * k + off));
        if (0xA1..=0xAD).contains(&mid) && matches!(tid, 0x0B | 0x0C) {
            let cylinder = mid - 0xA1;
            let idx = match misfire.iter().position(|m| m.cylinder == cylinder) {
                Some(i) => i,
                None => {
                    misfire.push(Misfire { ecu: ecu.into(), cylinder, average: None, current: None });
                    misfire.len() - 1
                }
            };
            if tid == 0x0B {
                misfire[idx].average = conv(raw[0]);
            } else {
                misfire[idx].current = conv(raw[0]);
            }
            continue;
        }
        let (value, min, max) = (conv(raw[0]), conv(raw[1]), conv(raw[2]));
        let verdict = match (value, min, max) {
            _ if raw == [0, 0, 0] => "not_run",
            (Some(v), Some(lo), Some(hi)) if lo <= v && v <= hi => "pass",
            (Some(_), Some(_), Some(_)) => "FAIL",
            _ => "unknown",
        };
        results.push(TestResult {
            ecu: ecu.into(),
            monitor: monitor_name(mid),
            test: test_name(tid),
            unit: scale.map(|s| s.2).unwrap_or(""),
            value,
            min,
            max,
            verdict,
            raw: r.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" "),
        });
    }
    (results, misfire)
}

/// Honda's 2005 CR-V service 06 table (wiki `honda-mode06`): TID → (label, conversion of raw).
/// (label, unit, conversion from the raw value).
type Label = (&'static str, &'static str, fn(f64) -> f64);

fn honda_crv(tid: u8) -> Option<Label> {
    let evap: fn(f64) -> f64 = |x| x * 0.1 - 3276.8;
    Some(match tid {
        0x62 => ("catalyst: downstream O2 moving average (P0420)", "", |x| x),
        0x76 => ("A/F sensor response (P0133)", "", |x| x),
        0x78 => ("A/F sensor output during A/F control (P1172)", "mA", |x| x * 0.00390625 - 128.0),
        0x70..=0x72 => ("A/F sensor heater current (P0135)", "A", |x| x * 0.01),
        0x18 | 0x19 => ("rear O2 sensor heater current (P0141)", "A", |x| x * 0.01),
        0x48 => ("intake air leak at idle (P2279)", "L/min", |x| x * 0.1),
        0x4D..=0x4F => ("thermostat: coolant temperature (P0128)", "°C", |x| x - 40.0),
        0x50 => ("thermostat: estimated minus actual coolant temperature (P0128)", "°C", |x| x),
        0x81 | 0x82 | 0x88 | 0x8B | 0x8D | 0x8F | 0x90..=0x92 | 0x94 | 0x95 => ("EVAP: fuel tank pressure test", "mmHg", evap),
        0x93 | 0x96 | 0x97 => ("EVAP: pressure change rate", "mmHg/min", |x| x * 0.1),
        _ => return None,
    })
}

/// One K-line service 06 message: `46 TID TLTCID value limit`; TLTCID bit 7 set = the limit is a minimum.
pub fn decode_kline(ecu: &str, data: &[u8], profile: Option<&str>) -> Vec<TestResult> {
    let [0x46, tid, tlt, a, b, c, d, ..] = *data else { return vec![] };
    if tid % 0x20 == 0 {
        return vec![];
    }
    let (value, limit) = (u16::from_be_bytes([a, b]) as f64, u16::from_be_bytes([c, d]) as f64);
    let minimum = tlt & 0x80 != 0;
    let pass = if minimum { value >= limit } else { value <= limit };
    let label = if profile == Some("crv-2004") { honda_crv(tid) } else { None };
    let (test, unit, conv): (String, &'static str, fn(f64) -> f64) = match label {
        Some((name, unit, f)) => (name.to_string(), unit, f),
        None => (format!("manufacturer test 0x{tid:02X}, component 0x{:02X}", tlt & 0x7F), "raw", |x| x),
    };
    let (v, l) = (round4(conv(value)), round4(conv(limit)));
    vec![TestResult {
        ecu: ecu.into(),
        monitor: format!("test 0x{tid:02X}"),
        test,
        unit,
        value: Some(v),
        min: minimum.then_some(l),
        max: (!minimum).then_some(l),
        verdict: if value == 0.0 && limit == 0.0 { "not_run" } else if pass { "pass" } else { "FAIL" },
        raw: data.iter().map(|x| format!("{x:02X}")).collect::<Vec<_>>().join(" "),
    }]
}

/// Walk the supported-ID bitmaps (06 00, 06 20, …) and read every supported monitor / test.
pub fn read<T: Transport>(a: &mut Adapter<T>, profile: Option<&str>) -> Result<Tests> {
    let can = a.bus().is_can();
    let mut out = Tests::default();
    let mut ids: Vec<u8> = Vec::new();
    let mut base = 0u8;
    loop {
        let msgs = match a.query(&format!("06{base:02X}")) {
            Ok(m) => m,
            Err(e) => {
                out.errors.push(format!("06{base:02X}: {e}"));
                break;
            }
        };
        let mut more = false;
        for m in msgs.iter().filter(|m| m.data.get(..2) == Some(&[0x46, base][..])) {
            let bitmap = if can { m.data.get(2..6) } else { m.data.get(3..7) };
            if let Some(bits) = bitmap {
                out.supported = true;
                let found = obd::supported_pids(base, bits);
                more |= base < 0xE0 && found.contains(&(base + 0x20));
                ids.extend(found.into_iter().filter(|id| id % 0x20 != 0));
            }
        }
        if !more {
            break;
        }
        base += 0x20;
    }
    ids.sort();
    ids.dedup();
    for id in ids {
        match a.query(&format!("06{id:02X}")) {
            Ok(msgs) => {
                for m in msgs.iter().filter(|m| m.data.first() == Some(&0x46)) {
                    if can {
                        let (r, mf) = decode_can(&m.ecu, &m.data);
                        out.results.extend(r);
                        out.misfire.extend(mf);
                    } else {
                        out.results.extend(decode_kline(&m.ecu, &m.data, profile));
                    }
                }
            }
            Err(e) => out.errors.push(format!("06{id:02X}: {e}")),
        }
    }
    out.misfire.sort_by_key(|m| m.cylinder);
    a.ensure_connected()?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn losing_the_adapter_during_the_self_tests_is_an_error() {
        let mut sim = crate::sim::Sim::new("odyssey-2016").unwrap();
        sim.fail("0600", "adapter_disconnected", usize::MAX);
        let mut a = crate::adapter::Adapter::connect(sim).unwrap();
        assert!(read(&mut a, None).is_err());
    }

    fn hexs(s: &str) -> Vec<u8> {
        let s: String = s.split_whitespace().collect();
        (0..s.len() / 2).map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap()).collect()
    }

    #[test]
    fn misfire_records_become_per_cylinder_counts_without_a_verdict() {
        // Toyota capture (wiki obd2-mode06): 46 | A2 0B 24 0000 0000 FFFF | A2 0C 24 0000 0000 FFFF
        let (results, misfire) = decode_can("7E8", &hexs("46 A2 0B 24 0000 0000 FFFF A2 0C 24 0000 0000 FFFF"));
        assert!(results.is_empty(), "misfire counters are not pass/fail tests");
        assert_eq!(misfire, vec![Misfire { ecu: "7E8".into(), cylinder: 1, average: Some(0.0), current: Some(0.0) }]);
        let (_, m) = decode_can("7E8", &hexs("46 A5 0B 24 003B 0000 FFFF A5 0C 24 0060 0000 FFFF"));
        assert_eq!((m[0].cylinder, m[0].average, m[0].current), (4, Some(59.0), Some(96.0)));
    }

    #[test]
    fn signed_scaled_catalyst_test_passes_within_limits() {
        let (r, _) = decode_can("7E8", &hexs("46 21 A9 86 02E5 02D0 7FFF"));
        assert_eq!(r[0].monitor, "catalyst bank 1");
        assert_eq!(r[0].test, "manufacturer test 0xA9");
        assert_eq!((r[0].value, r[0].min), (Some(0.226), Some(0.2196)));
        assert_eq!(r[0].verdict, "pass");
    }

    #[test]
    fn o2_tests_use_standard_names_units_and_limits() {
        let (r, _) = decode_can("7E8", &hexs("46 01 01 0A 0BB0 0BB0 0BB0 01 05 10 0048 0000 0064 01 85 24 0096 004B FFFF"));
        assert_eq!(r.len(), 3);
        assert_eq!((r[0].monitor.as_str(), r[0].test.as_str(), r[0].unit, r[0].value), ("O2 sensor bank 1 sensor 1", "rich→lean threshold voltage", "mV", Some(365.024)));
        assert_eq!((r[1].value, r[1].verdict), (Some(72.0), "pass"));
    }

    #[test]
    fn verdicts_cover_fail_not_run_and_unknown_scaling() {
        let (r, _) = decode_can("7E8", &hexs("46 21 80 24 0010 0000 000F 21 81 24 0000 0000 0000 21 82 77 0001 0000 0002"));
        assert_eq!(r.iter().map(|x| x.verdict).collect::<Vec<_>>(), vec!["FAIL", "not_run", "unknown"]);
        assert_eq!(r[2].value, None);
    }

    #[test]
    fn signed_values_compare_as_signed() {
        // UASID 8D (signed mA): value FF00 = −1 mA, min FE00 = −2 mA, max 0000: passes
        let (r, _) = decode_can("7E8", &hexs("46 01 81 8D FF00 FE00 0000"));
        assert_eq!(r[0].verdict, "pass");
        assert!(r[0].value.unwrap() < 0.0);
    }

    #[test]
    fn partial_trailing_record_is_ignored() {
        let (r, _) = decode_can("7E8", &hexs("46 21 80 24 0010 0000 00FF 21 81"));
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn k_line_record_uses_the_limit_type_bit_and_honda_labels() {
        // ISO 15031-5 sample: TLTCID 84 = minimum limit, 0010 >= 0000 passes; 16 = maximum, 0032 > 0020 fails
        let r = decode_kline("10", &hexs("46 02 84 0010 0000"), None);
        assert_eq!(r[0].verdict, "pass");
        let r = decode_kline("10", &hexs("46 02 16 0032 0020"), None);
        assert_eq!(r[0].verdict, "FAIL");
        // Honda CR-V thermostat TID 4D: raw − 40 °C, minimum limit
        let r = decode_kline("10", &hexs("46 4D 81 0082 0046"), Some("crv-2004"));
        assert_eq!((r[0].test.as_str(), r[0].unit, r[0].value, r[0].min, r[0].verdict), ("thermostat: coolant temperature (P0128)", "°C", Some(90.0), Some(30.0), "pass"));
    }

    #[test]
    fn reads_the_simulated_odyssey_misfire_counters() {
        let mut a = crate::adapter::Adapter::connect(crate::sim::Sim::new("odyssey-2016").unwrap()).unwrap();
        let t = read(&mut a, Some("odyssey-2016")).unwrap();
        assert!(t.supported);
        let cyl3 = t.misfire.iter().find(|m| m.cylinder == 3).unwrap();
        assert_eq!(cyl3.current, Some(180.0));
        assert!(t.results.iter().any(|r| r.monitor == "catalyst bank 1" && r.verdict == "pass"));
    }

    #[test]
    fn an_ev_without_service_06_reports_unsupported() {
        let mut a = crate::adapter::Adapter::connect(crate::sim::Sim::new("kona-ev-2020").unwrap()).unwrap();
        assert!(!read(&mut a, Some("kona-ev-2020")).unwrap().supported);
    }

    #[test]
    fn reads_the_simulated_crv_k_line_tests() {
        let mut a = crate::adapter::Adapter::connect(crate::sim::Sim::new("crv-2004").unwrap()).unwrap();
        let t = read(&mut a, Some("crv-2004")).unwrap();
        assert!(t.results.iter().any(|r| r.test.contains("catalyst")), "{:?}", t.results);
        assert!(t.misfire.is_empty());
    }
}
