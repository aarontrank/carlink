use serde::Serialize;

pub fn decode_dtc(a: u8, b: u8) -> Option<String> {
    if a == 0 && b == 0 {
        return None;
    }
    let system = ['P', 'C', 'B', 'U'][(a >> 6) as usize];
    Some(format!("{system}{}{:X}{:02X}", (a >> 4) & 0x3, a & 0x0F, b))
}

pub fn decode_dtc_list(data: &[u8]) -> Vec<String> {
    data.as_chunks::<2>().0.iter().filter_map(|p| decode_dtc(p[0], p[1])).collect()
}

/// `base` is the bitmap PID itself (0x00, 0x20, …); bit 7 of the first byte is PID base+1.
pub fn supported_pids(base: u8, bitmap: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for (i, byte) in bitmap.iter().take(4).enumerate() {
        for bit in 0..8 {
            let pid = base as usize + i * 8 + bit + 1;
            if byte & (0x80 >> bit) != 0 && pid <= 0xFF {
                out.push(pid as u8);
            }
        }
    }
    out
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Reading {
    pub key: String,
    pub name: &'static str,
    pub value: f64,
    pub unit: &'static str,
    /// Meaning of an enumerated value (e.g. fuel system status), when the number alone says nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'static str>,
}

impl Reading {
    pub fn new(key: impl Into<String>, name: &'static str, value: f64, unit: &'static str) -> Reading {
        Reading { key: key.into(), name, value, unit, text: None }
    }
}

type Formula = fn(&[f64]) -> f64;

/// (pid, key, name, unit, bytes, formula over data bytes A,B,C,D)
const PIDS: &[(u8, &str, &str, &str, usize, Formula)] = &[
    (0x04, "engine_load", "Calculated engine load", "%", 1, |d| d[0] * 100.0 / 255.0),
    (0x05, "coolant_temp", "Engine coolant temperature", "°C", 1, |d| d[0] - 40.0),
    (0x06, "stft_b1", "Short-term fuel trim bank 1", "%", 1, |d| d[0] / 1.28 - 100.0),
    (0x07, "ltft_b1", "Long-term fuel trim bank 1", "%", 1, |d| d[0] / 1.28 - 100.0),
    (0x08, "stft_b2", "Short-term fuel trim bank 2", "%", 1, |d| d[0] / 1.28 - 100.0),
    (0x09, "ltft_b2", "Long-term fuel trim bank 2", "%", 1, |d| d[0] / 1.28 - 100.0),
    (0x0A, "fuel_pressure", "Fuel pressure (gauge)", "kPa", 1, |d| d[0] * 3.0),
    (0x0B, "map", "Intake manifold absolute pressure", "kPa", 1, |d| d[0]),
    (0x0C, "rpm", "Engine speed", "rpm", 2, |d| (d[0] * 256.0 + d[1]) / 4.0),
    (0x0D, "speed", "Vehicle speed", "km/h", 1, |d| d[0]),
    (0x0E, "timing_advance", "Timing advance", "° before TDC", 1, |d| d[0] / 2.0 - 64.0),
    (0x0F, "intake_air_temp", "Intake air temperature", "°C", 1, |d| d[0] - 40.0),
    (0x10, "maf", "Mass air flow", "g/s", 2, |d| (d[0] * 256.0 + d[1]) / 100.0),
    (0x11, "throttle", "Throttle position", "%", 1, |d| d[0] * 100.0 / 255.0),
    (0x1F, "run_time", "Run time since engine start", "s", 2, |d| d[0] * 256.0 + d[1]),
    (0x2C, "egr_commanded", "Commanded EGR", "%", 1, |d| d[0] * 100.0 / 255.0),
    (0x2D, "egr_error", "EGR error", "%", 1, |d| d[0] / 1.28 - 100.0),
    (0x2E, "evap_purge", "Commanded evaporative purge", "%", 1, |d| d[0] * 100.0 / 255.0),
    (0x21, "distance_with_mil", "Distance traveled with MIL on", "km", 2, |d| d[0] * 256.0 + d[1]),
    (0x2F, "fuel_level", "Fuel tank level", "%", 1, |d| d[0] * 100.0 / 255.0),
    (0x30, "warmups_since_clear", "Warm-ups since codes cleared", "count", 1, |d| d[0]),
    (0x31, "distance_since_clear", "Distance since codes cleared", "km", 2, |d| d[0] * 256.0 + d[1]),
    (0x32, "evap_vapor_pressure", "Evap system vapor pressure", "Pa", 2, |d| ((d[0] as i32 as i8 as f64) * 256.0 + d[1]) / 4.0),
    (0x33, "baro", "Barometric pressure", "kPa", 1, |d| d[0]),
    (0x3C, "cat_temp_b1s1", "Catalyst temperature bank 1 sensor 1", "°C", 2, |d| (d[0] * 256.0 + d[1]) / 10.0 - 40.0),
    (0x3D, "cat_temp_b2s1", "Catalyst temperature bank 2 sensor 1", "°C", 2, |d| (d[0] * 256.0 + d[1]) / 10.0 - 40.0),
    (0x3E, "cat_temp_b1s2", "Catalyst temperature bank 1 sensor 2", "°C", 2, |d| (d[0] * 256.0 + d[1]) / 10.0 - 40.0),
    (0x3F, "cat_temp_b2s2", "Catalyst temperature bank 2 sensor 2", "°C", 2, |d| (d[0] * 256.0 + d[1]) / 10.0 - 40.0),
    (0x42, "module_voltage", "Control module voltage", "V", 2, |d| (d[0] * 256.0 + d[1]) / 1000.0),
    (0x43, "absolute_load", "Absolute load value", "%", 2, |d| (d[0] * 256.0 + d[1]) * 100.0 / 255.0),
    (0x44, "commanded_afr", "Commanded air-fuel equivalence ratio", "λ", 2, |d| (d[0] * 256.0 + d[1]) / 32768.0),
    (0x45, "relative_throttle", "Relative throttle position", "%", 1, |d| d[0] * 100.0 / 255.0),
    (0x46, "ambient_temp", "Ambient air temperature", "°C", 1, |d| d[0] - 40.0),
    (0x47, "throttle_b", "Absolute throttle position B", "%", 1, |d| d[0] * 100.0 / 255.0),
    (0x49, "pedal_d", "Accelerator pedal position D", "%", 1, |d| d[0] * 100.0 / 255.0),
    (0x4A, "pedal_e", "Accelerator pedal position E", "%", 1, |d| d[0] * 100.0 / 255.0),
    (0x4C, "throttle_commanded", "Commanded throttle actuator", "%", 1, |d| d[0] * 100.0 / 255.0),
    (0x4D, "time_with_mil", "Time run with MIL on", "min", 2, |d| d[0] * 256.0 + d[1]),
    (0x4E, "time_since_clear", "Time since codes cleared", "min", 2, |d| d[0] * 256.0 + d[1]),
    (0x5A, "pedal_relative", "Relative accelerator pedal position", "%", 1, |d| d[0] * 100.0 / 255.0),
    (0x5B, "hybrid_battery_charge", "Hybrid/EV battery state of charge (not its health)", "%", 1, |d| d[0] * 100.0 / 255.0),
    (0x5C, "oil_temp", "Engine oil temperature", "°C", 1, |d| d[0] - 40.0),
    (0x5E, "fuel_rate", "Engine fuel rate", "L/h", 2, |d| (d[0] * 256.0 + d[1]) / 20.0),
    (0xA6, "odometer", "Odometer", "km", 4, |d| ((d[0] * 256.0 + d[1]) * 256.0 + d[2]) * 25.6 + d[3] / 10.0),
];

pub fn decode_pid(pid: u8, data: &[u8]) -> Option<Reading> {
    let &(_, key, name, unit, len, f) = PIDS.iter().find(|p| p.0 == pid)?;
    if data.len() < len {
        return None;
    }
    let d: Vec<f64> = data[..len].iter().map(|&b| b as f64).collect();
    let value = (f(&d) * 100.0).round() / 100.0;
    Some(Reading::new(key, name, value, unit))
}

// Sensor positions for PIDs 14–1B, 24–2B and 34–3B follow the layout PID 13 describes (two banks of four).
// A car that reports PID 1D instead (four banks of two) numbers them differently; see DECISIONS.md.
const O2_V: [&str; 8] = ["o2_b1s1_v", "o2_b1s2_v", "o2_b1s3_v", "o2_b1s4_v", "o2_b2s1_v", "o2_b2s2_v", "o2_b2s3_v", "o2_b2s4_v"];
const O2_TRIM: [&str; 8] = ["o2_b1s1_trim", "o2_b1s2_trim", "o2_b1s3_trim", "o2_b1s4_trim", "o2_b2s1_trim", "o2_b2s2_trim", "o2_b2s3_trim", "o2_b2s4_trim"];
const WB_LAMBDA: [&str; 8] = ["wb_b1s1_lambda", "wb_b1s2_lambda", "wb_b1s3_lambda", "wb_b1s4_lambda", "wb_b2s1_lambda", "wb_b2s2_lambda", "wb_b2s3_lambda", "wb_b2s4_lambda"];
const WB_V: [&str; 8] = ["wb_b1s1_v", "wb_b1s2_v", "wb_b1s3_v", "wb_b1s4_v", "wb_b2s1_v", "wb_b2s2_v", "wb_b2s3_v", "wb_b2s4_v"];
const WB_MA: [&str; 8] = ["wb_b1s1_ma", "wb_b1s2_ma", "wb_b1s3_ma", "wb_b1s4_ma", "wb_b2s1_ma", "wb_b2s2_ma", "wb_b2s3_ma", "wb_b2s4_ma"];

fn fuel_system_text(a: u8) -> Option<&'static str> {
    Some(match a {
        1 => "open loop: engine still cold",
        2 => "closed loop: using oxygen sensors (normal when warm)",
        4 => "open loop: high load or deceleration fuel cut",
        8 => "open loop: system failure",
        16 => "closed loop with a fault in at least one oxygen sensor",
        _ => return None,
    })
}

fn fuel_type_text(a: u8) -> Option<&'static str> {
    Some(match a {
        1 => "gasoline",
        2 => "methanol",
        3 => "ethanol",
        4 => "diesel",
        5 => "LPG",
        6 => "CNG",
        8 => "electric",
        9..=16 => "bi-fuel",
        17 => "hybrid gasoline",
        18 => "hybrid ethanol",
        19 => "hybrid diesel",
        20 => "hybrid electric",
        21 => "hybrid running electric and combustion",
        22 => "hybrid regenerative",
        _ => return None,
    })
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

/// Every reading a service 01 reply carries (most PIDs have one, the oxygen-sensor PIDs two).
pub fn decode_pid_all(pid: u8, d: &[u8]) -> Vec<Reading> {
    let b = |i: usize| d.get(i).map(|x| *x as f64);
    match pid {
        0x03 => d.first().map(|&a| Reading { text: fuel_system_text(a), ..Reading::new("fuel_system", "Fuel system status", a as f64, "code") }).into_iter().collect(),
        0x51 => d.first().map(|&a| Reading { text: fuel_type_text(a), ..Reading::new("fuel_type", "Fuel type", a as f64, "code") }).into_iter().collect(),
        0x14..=0x1B => match (b(0), b(1)) {
            (Some(a), Some(bb)) => {
                let n = (pid - 0x14) as usize;
                let mut v = vec![Reading::new(O2_V[n], "Oxygen sensor voltage", round2(a / 200.0), "V")];
                if bb != 255.0 {
                    v.push(Reading::new(O2_TRIM[n], "Short-term fuel trim from this oxygen sensor", round2(bb / 1.28 - 100.0), "%"));
                }
                v
            }
            _ => vec![],
        },
        0x24..=0x2B | 0x34..=0x3B => match (b(0), b(1), b(2), b(3)) {
            (Some(a), Some(bb), Some(c), Some(dd)) => {
                let n = (if pid < 0x34 { pid - 0x24 } else { pid - 0x34 }) as usize;
                let lambda = Reading::new(WB_LAMBDA[n], "Wideband sensor air-fuel equivalence ratio", ((a * 256.0 + bb) * 2.0 / 65536.0 * 1000.0).round() / 1000.0, "λ");
                let second = if pid < 0x34 {
                    Reading::new(WB_V[n], "Wideband sensor voltage", round2((c * 256.0 + dd) * 8.0 / 65536.0), "V")
                } else {
                    Reading::new(WB_MA[n], "Wideband sensor current", round2((c * 256.0 + dd) / 256.0 - 128.0), "mA")
                };
                vec![lambda, second]
            }
            _ => vec![],
        },
        _ => decode_pid(pid, d).into_iter().collect(),
    }
}

/// Every reading key service 01 can produce.
pub fn pid_keys() -> impl Iterator<Item = &'static str> {
    PIDS.iter().map(|p| p.1).chain(["fuel_system", "fuel_type"]).chain(O2_V).chain(O2_TRIM).chain(WB_LAMBDA).chain(WB_V).chain(WB_MA)
}

/// The first service 01 PID that can produce reading `key` (`pids_for_key` lists them all).
pub fn pid_for_key(key: &str) -> Option<u8> {
    pids_for_key(key).first().copied()
}

/// Every service 01 PID that can produce reading `key`.
pub fn pids_for_key(key: &str) -> Vec<u8> {
    snapshot_pids().filter(|p| decode_pid_all(*p, &[0x80; 4]).iter().any(|r| r.key == key)).collect()
}

/// PIDs worth snapshotting during a scan, in request order.
pub fn snapshot_pids() -> impl Iterator<Item = u8> {
    [0x03].into_iter().chain(PIDS.iter().map(|p| p.0)).chain(0x14..=0x1B).chain(0x24..=0x2B).chain(0x34..=0x3B).chain([0x51])
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Monitor {
    pub name: &'static str,
    pub incomplete: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MonitorStatus {
    pub mil_on: bool,
    pub dtc_count: u8,
    pub ignition: &'static str,
    pub monitors: Vec<Monitor>,
}

// Bit 4 was "A/C refrigerant" in early J1979 and is the gasoline particulate filter in current editions.
const SPARK: [&str; 8] = ["catalyst", "heated_catalyst", "evap", "secondary_air", "gpf_or_ac", "oxygen_sensor", "oxygen_sensor_heater", "egr_vvt"];
const COMPRESSION: [&str; 8] = ["nmhc_catalyst", "nox_scr", "reserved", "boost_pressure", "reserved", "exhaust_gas_sensor", "pm_filter", "egr_vvt"];

/// Service 01 PID 01, bytes A–D (SAE J1979).
pub fn decode_monitor_status(d: &[u8]) -> Option<MonitorStatus> {
    let [a, b, c, dd] = *d.get(..4)? else { return None };
    let compression = b & 0x08 != 0;
    let mut monitors = Vec::new();
    for (bit, name) in ["misfire", "fuel_system", "components"].into_iter().enumerate() {
        if b & (1 << bit) != 0 {
            monitors.push(Monitor { name, incomplete: b & (0x10 << bit) != 0 });
        }
    }
    let names = if compression { COMPRESSION } else { SPARK };
    for (bit, name) in names.into_iter().enumerate() {
        if c & (1 << bit) != 0 && name != "reserved" {
            monitors.push(Monitor { name, incomplete: dd & (1 << bit) != 0 });
        }
    }
    Some(MonitorStatus {
        mil_on: a & 0x80 != 0,
        dtc_count: a & 0x7F,
        ignition: if compression { "compression" } else { "spark" },
        monitors,
    })
}

/// 17 characters, digits and capitals except I, O and Q, and not one repeated character (a placeholder).
pub fn valid_vin(v: &str) -> bool {
    v.len() == 17
        && v.bytes().all(|b| b.is_ascii_digit() || (b.is_ascii_uppercase() && !matches!(b, b'I' | b'O' | b'Q')))
        && v.bytes().any(|b| b != v.as_bytes()[0])
}

/// North American check digit (position 9). Other markets may not use one, so a mismatch is a warning.
pub fn vin_check_digit_ok(v: &str) -> bool {
    const WEIGHTS: [u32; 17] = [8, 7, 6, 5, 4, 3, 2, 10, 0, 9, 8, 7, 6, 5, 4, 3, 2];
    let value = |c: u8| match c {
        b'0'..=b'9' => (c - b'0') as u32,
        b'A'..=b'H' => (c - b'A' + 1) as u32,
        b'J'..=b'N' => (c - b'J' + 1) as u32,
        b'P' => 7,
        b'R' => 9,
        b'S'..=b'Z' => (c - b'S' + 2) as u32,
        _ => 0,
    };
    let sum: u32 = v.bytes().zip(WEIGHTS).map(|(c, w)| value(c) * w).sum();
    let expected = match sum % 11 {
        10 => b'X',
        n => b'0' + n as u8,
    };
    v.as_bytes().get(8) == Some(&expected)
}

/// Service 09 PID 02 payload (after the 49 02 header): item count byte, then ASCII, possibly 00-padded.
pub fn decode_vin(d: &[u8]) -> Option<String> {
    let s: String = d.get(1..)?.iter().filter(|b| b.is_ascii_alphanumeric()).map(|&b| b as char).collect();
    (s.len() >= 17).then(|| s[s.len() - 17..].to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_powertrain_dtc() {
        assert_eq!(decode_dtc(0x01, 0x33), Some("P0133".into()));
    }

    #[test]
    fn decodes_each_system_letter() {
        assert_eq!(decode_dtc(0x41, 0x00), Some("C0100".into()));
        assert_eq!(decode_dtc(0x80, 0x01), Some("B0001".into()));
        assert_eq!(decode_dtc(0xC1, 0x55), Some("U0155".into()));
        assert_eq!(decode_dtc(0x34, 0x20), Some("P3420".into()));
    }

    #[test]
    fn zero_pair_is_padding_not_a_code() {
        assert_eq!(decode_dtc(0, 0), None);
    }

    #[test]
    fn dtc_list_skips_padding() {
        assert_eq!(decode_dtc_list(&[0x01, 0x33, 0x00, 0x00, 0x04, 0x20]), vec!["P0133", "P0420"]);
    }

    #[test]
    fn supported_pid_bitmap_maps_msb_to_lowest_pid() {
        // 0xBE1FA813 from the Wikipedia example: PIDs 01,03,04,05,06,07,0C,0D,0E,0F,10,11,13,15,1C,1F,20
        let pids = supported_pids(0x00, &[0xBE, 0x1F, 0xA8, 0x13]);
        assert_eq!(pids, vec![0x01, 0x03, 0x04, 0x05, 0x06, 0x07, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x13, 0x15, 0x1C, 0x1F, 0x20]);
    }

    #[test]
    fn last_bitmap_does_not_overflow_past_ff() {
        assert_eq!(supported_pids(0xE0, &[0, 0, 0, 0x03]), vec![0xFF]);
    }

    #[test]
    fn supported_pid_bitmap_is_offset_by_base() {
        assert_eq!(supported_pids(0x20, &[0x80, 0, 0, 0x01]), vec![0x21, 0x40]);
    }

    fn val(pid: u8, data: &[u8]) -> f64 {
        decode_pid(pid, data).unwrap().value
    }

    #[test]
    fn decodes_common_pids() {
        assert_eq!(val(0x0C, &[0x1A, 0xF8]), 1726.0); // rpm
        assert_eq!(val(0x05, &[0x7B]), 83.0); // coolant °C
        assert_eq!(val(0x0D, &[0x3C]), 60.0); // km/h
        assert_eq!(val(0x04, &[0xFF]), 100.0); // load %
        assert_eq!(val(0x06, &[0x80]), 0.0); // STFT %
        assert_eq!(val(0x07, &[0x00]), -100.0); // LTFT %
        assert_eq!(val(0x10, &[0x01, 0x90]), 4.0); // MAF g/s
        assert_eq!(val(0x42, &[0x36, 0xB0]), 14.0); // module voltage
        assert_eq!(val(0x1F, &[0x01, 0x00]), 256.0); // run time s
        assert_eq!(val(0xA6, &[0x00, 0x01, 0x86, 0xA0]), 10000.0); // odometer km (0.1 km units)
    }

    #[test]
    fn decodes_remaining_table_formulas() {
        assert_eq!(val(0x0A, &[100]), 300.0);
        assert_eq!(val(0x43, &[0x00, 0xFF]), 100.0);
        assert_eq!(val(0x44, &[0x80, 0x00]), 1.0);
        assert_eq!(val(0x4D, &[0x01, 0x02]), 258.0);
        assert_eq!(val(0x4E, &[0x00, 0x0A]), 10.0);
        assert_eq!(val(0x5C, &[130]), 90.0);
        assert_eq!(val(0x5E, &[0x00, 0x14]), 1.0);
    }

    fn all(pid: u8, data: &[u8]) -> Vec<(String, f64)> {
        decode_pid_all(pid, data).into_iter().map(|r| (r.key, r.value)).collect()
    }

    #[test]
    fn narrowband_o2_reports_voltage_and_trim_unless_unused() {
        assert_eq!(all(0x14, &[0x5A, 0x80]), vec![("o2_b1s1_v".to_string(), 0.45), ("o2_b1s1_trim".to_string(), 0.0)]);
        assert_eq!(all(0x19, &[0x8C, 0xFF]), vec![("o2_b2s2_v".to_string(), 0.7)]);
    }

    #[test]
    fn wideband_sensors_report_lambda_and_voltage_or_current() {
        assert_eq!(all(0x24, &[0x80, 0x00, 0x80, 0x00]), vec![("wb_b1s1_lambda".to_string(), 1.0), ("wb_b1s1_v".to_string(), 4.0)]);
        assert_eq!(all(0x38, &[0x80, 0x00, 0x7F, 0x00]), vec![("wb_b2s1_lambda".to_string(), 1.0), ("wb_b2s1_ma".to_string(), -1.0)]);
    }

    #[test]
    fn enumerated_pids_carry_their_meaning() {
        let r = &decode_pid_all(0x03, &[0x02, 0x00])[0];
        assert_eq!((r.key.as_str(), r.value), ("fuel_system", 2.0));
        assert!(r.text.unwrap().contains("closed loop"));
        assert_eq!(decode_pid_all(0x51, &[0x08])[0].text, Some("electric"));
    }

    #[test]
    fn signed_and_offset_pids() {
        assert_eq!(all(0x32, &[0xFF, 0xFC]), vec![("evap_vapor_pressure".to_string(), -1.0)]);
        assert_eq!(all(0x3C, &[0x1F, 0x40]), vec![("cat_temp_b1s1".to_string(), 760.0)]);
        assert_eq!(all(0x5B, &[0xFF]), vec![("hybrid_battery_charge".to_string(), 100.0)]);
        assert_eq!(decode_pid_all(0x51, &[17])[0].text, Some("hybrid gasoline"));
        assert_eq!(decode_pid_all(0x51, &[11])[0].text, Some("bi-fuel"));
    }

    #[test]
    fn keys_map_back_to_their_pids() {
        assert_eq!(pid_for_key("rpm"), Some(0x0C));
        assert_eq!(pids_for_key("wb_b1s1_lambda"), vec![0x24, 0x34]);
        assert_eq!(pid_for_key("o2_b1s2_v"), Some(0x15));
        assert!(pid_keys().any(|k| k == "fuel_system"));
    }

    #[test]
    fn pid_reports_name_and_unit() {
        let r = decode_pid(0x05, &[40]).unwrap();
        assert_eq!((r.key.as_str(), r.unit), ("coolant_temp", "°C"));
    }

    #[test]
    fn unknown_or_short_pid_is_none() {
        assert!(decode_pid(0xEE, &[1, 2]).is_none());
        assert!(decode_pid(0x0C, &[1]).is_none());
    }

    #[test]
    fn monitor_status_spark_ignition() {
        // MIL on, 3 DTCs; spark; misfire+fuel supported, fuel incomplete; catalyst+O2 supported, O2 incomplete
        let m = decode_monitor_status(&[0x83, 0x23, 0x21, 0x20]).unwrap();
        assert!(m.mil_on);
        assert_eq!(m.dtc_count, 3);
        assert_eq!(m.ignition, "spark");
        let get = |n: &str| m.monitors.iter().find(|x| x.name == n).unwrap().clone();
        assert!(!get("misfire").incomplete);
        assert!(get("fuel_system").incomplete);
        assert!(!get("catalyst").incomplete);
        assert!(get("oxygen_sensor").incomplete);
        assert!(m.monitors.iter().all(|x| x.name != "evap"));
    }

    #[test]
    fn monitor_status_compression_uses_diesel_names() {
        let m = decode_monitor_status(&[0x00, 0x08, 0x01, 0x00]).unwrap();
        assert_eq!(m.ignition, "compression");
        assert_eq!(m.monitors.iter().map(|x| x.name).collect::<Vec<_>>(), vec!["nmhc_catalyst"]);
    }

    #[test]
    fn reserved_monitor_bits_are_not_reported() {
        let m = decode_monitor_status(&[0x00, 0x08, 0x14, 0x14]).unwrap();
        assert!(m.monitors.is_empty());
    }

    #[test]
    fn vin_validation_rejects_placeholders_and_illegal_letters() {
        assert!(valid_vin("5FNRL5H60GB000001"));
        assert!(!valid_vin("00000000000000000"));
        assert!(!valid_vin("5FNRL5H6OGB000001")); // O is never used in a VIN
        assert!(!valid_vin("5FNRL5H6XGB00000"));
    }

    #[test]
    fn check_digit_is_verified() {
        assert!(vin_check_digit_ok("1HGCM82633A004352"));
        assert!(!vin_check_digit_ok("1HGCM82634A004352"));
    }

    #[test]
    fn vin_from_service09_payload() {
        let mut d = vec![0x01];
        d.extend_from_slice(b"5J6RD18294L123456");
        assert_eq!(decode_vin(&d), Some("5J6RD18294L123456".into()));
    }

    #[test]
    fn vin_strips_leading_padding_and_rejects_short() {
        let mut d = vec![0x01, 0x00, 0x00];
        d.extend_from_slice(b"KM8K53AG0LU000001");
        assert_eq!(decode_vin(&d), Some("KM8K53AG0LU000001".into()));
        assert_eq!(decode_vin(b"\x01ABC"), None);
    }
}
