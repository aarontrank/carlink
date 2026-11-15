//! 2018 Honda Clarity Plug-in Hybrid: generic OBD on 29-bit CAN from seven ECUs (engine 18DAF10E); HV
//! battery from the two BMS halves (18DAF101, 18DAF116) and odometer from the meter (18DAF160).
//! Byte map: wiki `clarity-phev-hv-battery`. `p` offsets index the payload after the `62 xx xx` header.

use super::{KnownDtc, Powertrain, Profile, Reset, clear_reset, hex, payloads};
use crate::adapter::{Adapter, Transport};
use crate::findings::{Finding, Severity};
use crate::obd::Reading;
use std::collections::BTreeMap;

pub const SIM_VIN: &str = "JHMZC5F36JC000001";

const PCM_UPDATE: &str = "Honda product update 18-090 (an emissions recall since 2023): a PCM software update (coolant 3-way valve and electric VTC). Ask a Honda dealer whether it was done; it is free.";
const CHARGER_UPDATE: &str = "Charger refuses or stops charging on poor or fluctuating mains power. Honda service bulletin 18-097: charger software update (charging may then take up to 1.5 times longer). Try another outlet or charger to confirm.";
const EVAP_AFTER_RECALL: &str = "Fuel vapor (EVAP) leak. Check the fuel cap first. Owners report the \"Emission System Problem\" message with these codes after the fuel-pump recall 23V858 repair; the dealer that did that repair should check the filler and tank seals.";
const CHARGER_FAULT: &str = "On-board charger or charge inlet fault, reported by owners with \"Plug In Charging System Problem. Range Limited\". Rule out a weak 12 V battery and the charge-port pins first; owners report on-board charger replacements, often under the emissions warranty.";

const BATTERY_DETERIORATION: &str = "High-voltage battery deterioration; it comes with the check-engine and POWER SYSTEM lights and often \"Plug In Charging System Problem. Range Limited\". Owners report Honda approving warranty battery replacements on P0A7F with P1DC7. Read hv_capacity_ah, do not clear (carlink refuses: the code and the capacity reading are the warranty evidence), and book a Honda dealer battery diagnosis while the battery warranty runs (8 years/100,000 miles; 10 years/150,000 miles in CARB states).";

pub const PROFILE: Profile = Profile {
    id: "clarity-phev-2018",
    name: "2018 Honda Clarity Plug-in Hybrid",
    powertrain: Powertrain::PluginHybrid,
    wiki: "clarity-phev-2018",
    recalls: &["23V858: fuel pump."],
    notes: &[
        "Plug-in hybrid: in READY the gasoline engine starts and stops by itself (system check, low battery charge, heavy climate use), so rpm 0 in READY is normal. Scan in READY; clear with the car ON but not READY (POWER pressed twice without the brake).",
        "Generic OBD answers on 29-bit CAN from seven computers; 18DAF10E is the engine computer. The others (hybrid, battery, charger and brake units) report their own emissions-related codes in codes[].",
        "High-voltage battery: extended.hv_capacity_ah is the capacity Honda's dealer tool shows (55 Ah new). Honda service bulletin 17-093: below 36.6 Ah the pack is eligible for warranty replacement, and the car sets no code or light at that level. Warranty 8 years/100,000 miles; 10 years/150,000 miles in California and other CARB states. A 2018 first sold in 2018 reaches 8 years during 2026.",
        "Clearing codes (or disconnecting the 12 V battery) is reported to reset the capacity reading to 55 Ah, which then relearns over weeks: read and record hv_capacity_ah BEFORE any clear.",
        "The 12 V battery is charged only by the DC-DC converter from the high-voltage battery (no alternator): about 14 V with rpm 0 in READY is normal. The '12-volt battery charging system' light means that converter is not charging.",
        "Airbag, ABS/VSA, steering and body codes are not read by carlink: they need a Honda-capable scan tool (i-HDS).",
        "TPMS is indirect (no tire sensors): calibrate it after any pressure change, rotation or tire change (`carlink resets`).",
        "At power-on the check-engine light blinks five times when the readiness self-tests are not complete (owner's manual p. 90). The engine-related self-tests only run while the engine runs: a car driven mostly on battery may need trips in HV mode before an inspection.",
        "Maintenance: 0W-20 oil (3.5 US qt with filter), at least once a year even with no Maintenance Minder message; ATF Honda DW-1; three coolant circuits (engine, inverter, high-voltage battery), all Honda Type 2 — only a Honda dealer refills the battery and inverter circuits.",
    ],
    after_clear: &[
        "The battery capacity reading (hv_capacity_ah) may now read 55.0 Ah until the battery computer relearns it over weeks of driving and charging: use the value saved in the pre-clear scan, not a new one, when talking about battery health.",
        "Several days of normal driving including trips with the engine running (HV mode) are needed before the self-tests complete.",
    ],
    modules: &[],
    obd_clear_refused: None,
    kept_codes: (&["P0A7F", "P1DC7", "P144A"], "Honda's high-voltage battery deterioration codes are the evidence for a warranty battery replacement, and clearing them also resets the battery's capacity estimate to 55 Ah (wiki clarity-phev-2018)"),
    known_dtcs: &[
        KnownDtc { code: "P0010", note: PCM_UPDATE },
        KnownDtc { code: "P100C", note: PCM_UPDATE },
        KnownDtc { code: "P26A3", note: PCM_UPDATE },
        KnownDtc { code: "P0D28", note: CHARGER_UPDATE },
        KnownDtc { code: "P0D2A", note: CHARGER_UPDATE },
        KnownDtc { code: "P0D3D", note: CHARGER_UPDATE },
        KnownDtc { code: "P1D23", note: CHARGER_UPDATE },
        KnownDtc { code: "P0D27", note: CHARGER_FAULT },
        KnownDtc { code: "P0D5C", note: CHARGER_FAULT },
        KnownDtc { code: "P1D8D", note: "Generator motor inverter short (\"Hybrid System Failure\" message). Honda service bulletin 21-008: replace the power control unit (PCU); it covers all 2018 Claritys. Have a Honda dealer handle it." },
        KnownDtc { code: "P0087", note: "Low fuel pressure: the failing fuel pump of recall 23V858 (the engine can stall). Check by VIN at nhtsa.gov/recalls; the recall repair is free at a Honda dealer." },
        KnownDtc { code: "P0454", note: EVAP_AFTER_RECALL },
        KnownDtc { code: "P0455", note: EVAP_AFTER_RECALL },
        KnownDtc { code: "P0456", note: EVAP_AFTER_RECALL },
        KnownDtc { code: "P0A7F", note: BATTERY_DETERIORATION },
        KnownDtc { code: "P1DC7", note: BATTERY_DETERIORATION },
        KnownDtc { code: "P144A", note: "High-voltage battery pack deterioration (Honda service information: usable capacity 24.9% or less). The POWER SYSTEM light comes on with it. A Honda dealer must diagnose the battery; carlink will not clear this code." },
    ],
    resets: &[
        clear_reset("Do it with the car ON but NOT READY: press POWER twice with your foot off the brake (the dashboard lights up, READY does not show) — in READY the engine can start by itself. Clearing resets the battery capacity estimate to 55 Ah (reported by owners), so read hv_capacity_ah first; carlink refuses while P0A7F, P1DC7 or P144A is stored. It does not reset the Maintenance Minder or the TPMS."),
        Reset { name: "Maintenance Minder / oil life", carlink_can_do: false, steps: &[
            "Car in ON mode (POWER pressed twice without the brake).",
            "Press the display/information button on the steering wheel, select the maintenance (wrench) screen with the arrow buttons, and press ENTER.",
            "Press and hold ENTER about 10 seconds to enter reset mode.",
            "With the up/down buttons choose ONLY the item that was done (or All Due Items if everything due was done), then press ENTER. Repeat for each other item that was done.",
        ], notes: "Owner's manual pp. 477–479 (also Settings > Vehicle > Maintenance Info on the touchscreen). Codes: A oil and filter; 1 rotate tires; 2 cabin filter; 3 transmission fluid; 4 spark plugs and valve clearance; 5 engine coolant; 7 brake fluid; 8 air cleaner. Never reset an item that was not done. Change the oil at least once a year even if no message appears: the engine runs little, so the minder rarely asks." },
        Reset { name: "TPMS calibration (low tire pressure light)", carlink_can_do: false, steps: &[
            "Set all four tires to the door-jamb pressure with the tires cold (36 psi on the standard 235/45R18 tires).",
            "Car stopped, in Park, power mode ON.",
            "On the touchscreen: HOME, then Settings, then Vehicle, then TPMS Calibration, then Calibrate. If it says the calibration failed to start, repeat those last steps.",
            "Drive normally: the calibration finishes by itself after about 30 minutes of driving at 30–65 mph.",
        ], notes: "Indirect TPMS (no tire sensors; it compares wheel speeds), so recalibrate after any pressure change, tire rotation or tire replacement. Owner's manual pp. 425–426." },
        Reset { name: "After the 12 V battery was disconnected or replaced", carlink_can_do: false, steps: &[
            "Turn the power system on (brake + POWER until READY), then off again.",
            "Plug in the charger and confirm the car charges.",
            "If the low tire pressure and collision-braking (CMBS) lights are on, drive a short distance above 12 mph: they should go off.",
        ], notes: "Owner's manual pp. 426 and 508. The battery capacity reading may restart at 55 Ah and relearn over weeks." },
    ],
    // vPIC: JHMZC5F + trim (1 base, 3 Touring) + check digit + J (2018); the Clarity Electric is JHMZC6F.
    vin: &[("JHMZC5F", 'J', None)],
};

/// Real 2018 captures from OBDb Honda-Clarity, chosen so the pack voltages agree (raw/obdb-honda-clarity-2026-11-13.md):
/// (ECU, request, reply).
const SIM_REPLIES: &[(u16, &str, &str)] = &[
    (0x01, "22202A", "62202AFFFF3FFFFFFFFFFFFFE7FFFC03FFF2B33FFF33333333FFE1000000000000000044002D0000415B00A8000000008101C10003412056003318DE005B1E1E18DC012C159CF5FC2F70E0161A771A364A1349EA0000445B0000436F01200405AA012C0082FF0003020000181A377838FE3101F30082005B19190383000000000000000018FD1100271037FA7F007F007F000000010001002710000001F40000F640F00C630007F900200FB0012D00000123000018DD0000005700004A10000049F000001A7400001A390000012E0123019001F40457044C01042E042E00000000000000000000000000000000000000000000000000"),
    (0x01, "222028", "622028FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0000000000004A0F4A164A0F4A0B4A104A134A194A184A164A1F4A1C4A194A1C4A1B4A164A194A154A164A164A134A184A1B4A124A124A1E4A0A4A084A0F4A194A194A1C4A184A134A154A0A4A244A0249F24A024A0449FC4A0249FC49FF49F84A0749FB4A014A0D4A134A164A124A134A134A194A134A054A0F4A104A184A0849FF4A044A0A49FB49F649FF4A0249FC49FF4A044A124A084A0B4A0F4A134A074A0B4A134A0D4A074A134A1B4A15000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
    (0x16, "222028", "622028FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0000000000004A6D4A6C4A694A694A6A4A6C4A694A674A6F4A624A734A624A694A6A4A6D4A674A674A6A4A6C4A694A6A4A694A674A6D4A5E4A674A614A614A614A5E4A694A614A5E4A6C4A644A6F4A594A624A704A6D4A5F4A644A5C4A644A6C4A644A704A694A6A4A6D4A6C4A694A704A694A6A4A5E4A6D4A624A6D4A674A6F4A664A594A584A554A614A564A564A5E4A674A704A624A6A4A664A694A644A6F4A694A6C4A694A6D4A6C4A6F4A70000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
    (0x60, "227022", "627022EC0000000000015FB1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
];

fn r(key: impl Into<String>, name: &'static str, value: f64, unit: &'static str) -> Reading {
    Reading::new(key, name, value, unit)
}

fn u16be(p: &[u8], i: usize) -> f64 {
    u16::from_be_bytes([p[i], p[i + 1]]) as f64
}

/// 22 202A from the main BMS (18DAF101), OBDb `CLARITY_*` signals.
pub fn decode_summary(p: &[u8]) -> Vec<Reading> {
    if p.len() < 169 {
        return vec![];
    }
    vec![
        r("hv_soc", "High-voltage battery state of charge (BMS)", p[50] as f64, "%"),
        r("pack_voltage", "High-voltage battery voltage", u16be(p, 61) / 20.0, "V"),
        r("pack_current", "High-voltage battery current (+ = discharging)", i16::from_be_bytes([p[57], p[58]]) as f64 / 50.0, "A"),
        r("cell_soc_max", "Highest cell state of charge", u16be(p, 73) / 100.0, "%"),
        r("cell_soc_min", "Lowest cell state of charge", u16be(p, 75) / 100.0, "%"),
    ]
    .into_iter()
    .chain(Some(u16be(p, 167) / 100.0).filter(|ah| *ah > 0.0 && *ah <= 60.0).map(|ah| r("hv_capacity_ah", "High-voltage battery capacity (BMS estimate)", ah, "Ah")))
    .collect()
}

/// 22 2028: 84 cell voltages in volts, from byte 27, 2 bytes each in units of 0.2 mV.
pub fn decode_cells(p: &[u8]) -> Vec<f64> {
    if p.len() < 27 + 2 * 84 {
        return vec![];
    }
    (0..84).map(|i| u16be(p, 27 + 2 * i) / 5000.0).collect()
}

/// 22 7022 from the meter (18DAF160): odometer in miles, 3 bytes from byte 6.
pub fn decode_odometer(p: &[u8]) -> Vec<Reading> {
    if p.len() < 9 {
        return vec![];
    }
    vec![r("odometer_mi", "Odometer", u32::from_be_bytes([0, p[6], p[7], p[8]]) as f64, "mi")]
}

const REQUESTS: &[(&str, &[&str])] = &[("18DA01F1", &["22202A", "222028"]), ("18DA16F1", &["222028"]), ("18DA60F1", &["227022"])];

/// All Clarity-specific reads. Each failure is reported, the rest continue.
pub fn read<T: Transport>(a: &mut Adapter<T>) -> (Vec<Reading>, Vec<String>) {
    let mut out = Vec::new();
    let mut errors = Vec::new();
    let mut strings: Vec<(char, Vec<f64>)> = Vec::new();
    for (header, reqs) in REQUESTS {
        let results = match payloads(a, header, reqs) {
            Ok(r) => r,
            Err(e) => {
                errors.push(format!("{header}: {e}"));
                continue;
            }
        };
        for (req, res) in reqs.iter().zip(results) {
            let p = match res {
                Ok(Some(p)) => p,
                Ok(None) => {
                    errors.push(format!("{header} {req}: no response"));
                    continue;
                }
                Err(e) => {
                    errors.push(format!("{header} {req}: {e}"));
                    continue;
                }
            };
            let before = out.len();
            match (*header, *req) {
                ("18DA01F1", "22202A") => out.extend(decode_summary(&p)),
                (_, "222028") => {
                    let cells = decode_cells(&p);
                    if !cells.is_empty() {
                        strings.push((if *header == "18DA01F1" { 'a' } else { 'b' }, cells));
                        continue;
                    }
                }
                _ => out.extend(decode_odometer(&p)),
            }
            if out.len() == before {
                errors.push(format!("{header} {req}: reply too short or malformed ({} bytes)", p.len()));
            }
        }
    }
    let cells = |s: char| strings.iter().find(|x| x.0 == s).map(|x| x.1.as_slice()).unwrap_or(&[]);
    match cell_spread(cells('a'), cells('b')) {
        Ok(spread) => {
            out.push(r("cell_delta_v", "Cell voltage spread (max − min within a string, worse of the two)", spread, "V"));
            for (s, cs) in &strings {
                out.extend(cs.iter().enumerate().map(|(i, v)| r(format!("cell_v_{s}{:02}", i + 1), "Cell voltage", *v, "V")));
            }
        }
        Err(e) => errors.push(format!("cell voltages not used: {e}")),
    }
    (out, errors)
}

/// What a new pack reads, and Honda's warranty-replacement line (S/B 17-093; wiki clarity-phev-2018).
const NEW_AH: f64 = 55.0;
const WARRANTY_AH: f64 = 36.6;
const WARRANTY_TERMS: &str = "8 years/100,000 miles from the original sale; 10 years/150,000 miles in California and other CARB states";

fn get(rs: &[Reading], key: &str) -> Option<f64> {
    rs.iter().find(|r| r.key == key).map(|r| r.value)
}

fn finding(severity: Severity, title: &str, detail: String, recommendation: String) -> Finding {
    Finding { severity, code: None, title: title.into(), detail, recommendation }
}

/// The generic findings assume an engine that runs whenever the car is on and an alternator; this car
/// has neither (its 12 V battery is charged by the DC-DC converter).
pub fn adapt_engine_findings(fs: &mut Vec<Finding>) {
    fs.retain(|f| !f.title.starts_with("Engine not warming up"));
    for f in fs.iter_mut() {
        let (title, rec) = match f.title.as_str() {
            "Alternator may not be charging" => ("12 V battery may not be charging", "This car has no alternator: the DC-DC converter charges the 12 V battery from the high-voltage battery. Re-check in READY; if it stays low, have a Honda dealer check the DC-DC converter and test the 12 V battery."),
            "Charging voltage on the low side" => ("12 V charging voltage on the low side", "The DC-DC converter charges the 12 V battery in READY. Re-check in READY with the headlights and blower on; if it stays low, have the DC-DC converter and the 12 V battery checked."),
            "Charging voltage high" => ("12 V charging voltage high", "Over-charging damages the battery and electronics; have a Honda dealer check the DC-DC converter."),
            _ => continue,
        };
        f.title = title.into();
        f.recommendation = rec.into();
    }
}

pub fn findings(rs: &[Reading]) -> Vec<Finding> {
    let mut out = Vec::new();
    if let Some(ah) = get(rs, "hv_capacity_ah") {
        let pct = ah / NEW_AH * 100.0;
        let trend = "`carlink history trend <VIN> hv_capacity_ah`";
        if ah == NEW_AH {
            out.push(finding(Severity::Info, "Battery capacity reads exactly 55.0 Ah",
                "55.0 Ah is what a new pack reads, and also the value the battery computer starts from after codes are cleared or the 12 V battery is disconnected; it then relearns over weeks of driving and charging.".into(),
                format!("Do not report it as a health result. Scan again in a few weeks and compare with {trend}.")));
        } else if ah < WARRANTY_AH {
            out.push(finding(Severity::Warning, "High-voltage battery below Honda's warranty-replacement capacity",
                format!("Battery capacity {ah:.1} Ah, {pct:.0}% of the 55 Ah a new pack reads. Honda (service bulletin 17-093) treats below {WARRANTY_AH} Ah as eligible for warranty replacement, and the car turns on no warning light at that level."),
                format!("If the battery warranty still runs ({WARRANTY_TERMS}), ask a Honda dealer to check \"Battery Pack Capacity\" with i-HDS for a warranty replacement. Do not clear codes first.")));
        } else if pct < 80.0 {
            out.push(finding(Severity::Advice, "High-voltage battery has lost capacity",
                format!("Battery capacity {ah:.1} Ah, {pct:.0}% of the 55 Ah a new pack reads. Honda's warranty-replacement line is {WARRANTY_AH} Ah."),
                format!("Expected ageing on an older pack, but track it with {trend}. Below {WARRANTY_AH} Ah while the battery warranty runs ({WARRANTY_TERMS}), a Honda dealer can confirm it with i-HDS for a warranty replacement.")));
        }
    }
    let resting = get(rs, "pack_current").is_some_and(|i| i.abs() <= 5.0);
    if let (Some(d), true) = (get(rs, "cell_delta_v"), resting) {
        let detail = format!("Largest difference between cells in one string: {:.0} mV (real Claritys showed up to about 46 mV).", d * 1000.0);
        if d >= 0.10 {
            out.push(finding(Severity::Warning, "Battery cell imbalance", detail, "A large spread can mean a weak cell or module. Charge fully, let the car rest an hour, and scan again; if it stays, have a Honda dealer check the high-voltage battery.".into()));
        } else if d >= 0.05 {
            out.push(finding(Severity::Advice, "Battery cell spread worth watching", detail, "Charge fully, let the car rest an hour, and scan again to see whether it stays.".into()));
        }
    }
    out
}

/// The larger max − min of the two 84-cell strings, only when every cell of both reads plausibly.
pub fn cell_spread(a: &[f64], b: &[f64]) -> std::result::Result<f64, String> {
    let mut worst: f64 = 0.0;
    for (name, cells) in [("A", a), ("B", b)] {
        if cells.len() != 84 {
            return Err(format!("{} of 84 cells read in string {name}", cells.len()));
        }
        if let Some((n, v)) = cells.iter().enumerate().find(|c| !(2.5..=4.3).contains(c.1)) {
            return Err(format!("implausible voltage {v:.2} V for cell {name}{}", n + 1));
        }
        let max = cells.iter().copied().fold(f64::MIN, f64::max);
        let min = cells.iter().copied().fold(f64::MAX, f64::min);
        worst = worst.max(max - min);
    }
    Ok((worst * 1000.0).round() / 1000.0)
}

pub fn sim_extra(ecu: u16) -> BTreeMap<String, Vec<u8>> {
    SIM_REPLIES.iter().filter(|(e, ..)| *e == ecu).map(|(_, k, v)| (k.to_string(), hex(v))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reply(ecu: u16, req: &str) -> Vec<u8> {
        sim_extra(ecu)[req][3..].to_vec()
    }

    fn get(rs: &[Reading], key: &str) -> f64 {
        rs.iter().find(|r| r.key == key).unwrap_or_else(|| panic!("missing {key}: {rs:?}")).value
    }

    #[test]
    fn decodes_the_pack_summary_as_obdb_does() {
        let rs = decode_summary(&reply(0x01, "22202A"));
        assert_eq!(get(&rs, "hv_soc"), 65.0);
        assert_eq!(get(&rs, "pack_voltage"), 318.2);
        assert_eq!(get(&rs, "pack_current"), 1.82);
        assert_eq!((get(&rs, "cell_soc_max"), get(&rs, "cell_soc_min")), (67.75, 67.1));
        assert_eq!(get(&rs, "hv_capacity_ah"), 40.16);
    }

    #[test]
    fn a_negative_current_decodes_as_charging() {
        let mut p = reply(0x01, "22202A");
        p[57..59].copy_from_slice(&(-250i16).to_be_bytes());
        assert_eq!(get(&decode_summary(&p), "pack_current"), -5.0);
    }

    #[test]
    fn decodes_84_cells_per_string() {
        let a = decode_cells(&reply(0x01, "222028"));
        let b = decode_cells(&reply(0x16, "222028"));
        assert_eq!((a.len(), b.len()), (84, 84));
        assert_eq!((a[0], b[0]), (3.7918, 3.8106));
    }

    #[test]
    fn decodes_the_odometer_in_miles() {
        assert_eq!(get(&decode_odometer(&reply(0x60, "227022")), "odometer_mi"), 90033.0);
    }

    fn sim() -> crate::adapter::Adapter<crate::sim::Sim> {
        crate::adapter::Adapter::connect(crate::sim::Sim::new("clarity-phev-2018").unwrap()).unwrap()
    }

    #[test]
    fn reads_battery_cells_and_odometer_from_the_car() {
        let (rs, errors) = read(&mut sim());
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(get(&rs, "hv_capacity_ah"), 40.16);
        assert_eq!(rs.iter().filter(|r| r.unit == "Ah").count(), 1, "one capacity, the pack's: a per-string figure misleads");
        assert_eq!(get(&rs, "odometer_mi"), 90033.0);
        assert_eq!(get(&rs, "cell_v_a01"), 3.7918);
        assert_eq!(get(&rs, "cell_v_b84"), 3.8112);
        assert_eq!(rs.iter().filter(|r| r.key.starts_with("cell_v_")).count(), 168);
        assert_eq!(get(&rs, "cell_delta_v"), 0.01);
    }

    #[test]
    fn spread_is_the_larger_of_the_two_strings_and_needs_every_cell() {
        let a: Vec<f64> = (0..84).map(|i| 3.70 + if i == 5 { 0.03 } else { 0.0 }).collect();
        let b = vec![3.80; 84];
        assert_eq!(cell_spread(&a, &b), Ok(0.03));
        assert!(cell_spread(&a[..83], &b).unwrap_err().contains("83 of 84"));
        let mut bad = b.clone();
        bad[10] = 0.0;
        assert!(cell_spread(&a, &bad).unwrap_err().contains("implausible"));
    }

    #[test]
    fn a_module_that_does_not_answer_is_reported_and_the_rest_is_still_read() {
        let mut a = sim();
        a.transport_mut().silence(0x16);
        let (rs, errors) = read(&mut a);
        assert_eq!(get(&rs, "hv_capacity_ah"), 40.16);
        assert!(rs.iter().all(|r| r.key != "cell_delta_v"));
        assert!(errors.iter().any(|e| e.starts_with("18DA16F1 222028: no response")), "{errors:?}");
        assert!(errors.iter().any(|e| e.contains("cell voltages not used")), "{errors:?}");
    }

    fn titles(rs: &[(&str, f64)]) -> Vec<(crate::findings::Severity, String)> {
        let rs: Vec<Reading> = rs.iter().map(|(k, v)| r(*k, "t", *v, "")).collect();
        findings(&rs).into_iter().map(|f| (f.severity, f.title)).collect()
    }

    #[test]
    fn capacity_is_judged_against_hondas_warranty_line() {
        use crate::findings::Severity::*;
        assert_eq!(titles(&[("hv_capacity_ah", 36.5)]), vec![(Warning, "High-voltage battery below Honda's warranty-replacement capacity".to_string())]);
        assert_eq!(titles(&[("hv_capacity_ah", 40.16)]), vec![(Advice, "High-voltage battery has lost capacity".to_string())]);
        assert!(titles(&[("hv_capacity_ah", 44.0)]).is_empty());
        assert_eq!(titles(&[("hv_capacity_ah", 55.0)]), vec![(Info, "Battery capacity reads exactly 55.0 Ah".to_string())]);
    }

    #[test]
    fn the_warranty_finding_names_the_threshold_and_the_dealer_check() {
        let f = findings(&[r("hv_capacity_ah", "t", 36.0, "Ah")]);
        assert!(f[0].detail.contains("36.6 Ah") && f[0].detail.contains("65%"), "{}", f[0].detail);
        assert!(f[0].recommendation.contains("i-HDS"), "{}", f[0].recommendation);
    }

    #[test]
    fn cell_spread_is_judged_only_near_rest() {
        use crate::findings::Severity::*;
        assert_eq!(titles(&[("cell_delta_v", 0.06), ("pack_current", 1.0)]), vec![(Advice, "Battery cell spread worth watching".to_string())]);
        assert_eq!(titles(&[("cell_delta_v", 0.12), ("pack_current", -2.0)]), vec![(Warning, "Battery cell imbalance".to_string())]);
        assert!(titles(&[("cell_delta_v", 0.12), ("pack_current", 30.0)]).is_empty(), "under load the spread says little");
        assert!(titles(&[("cell_delta_v", 0.12)]).is_empty(), "current unknown");
        assert!(titles(&[("cell_delta_v", 0.046), ("pack_current", 0.0)]).is_empty(), "seen on real cars");
    }

    #[test]
    fn refusals_short_replies_and_addressing_faults_are_each_reported() {
        let mut sim = crate::sim::Sim::new("clarity-phev-2018").unwrap();
        sim.inject("ATCRA18DAF160", "?");
        sim.inject_where(0x18DA01F1, None, "222028", "18 DA F1 01 03 7F 22 31");
        sim.inject_where(0x18DA16F1, None, "222028", "18 DA F1 16 04 62 20 28 FF");
        let (rs, errors) = read(&mut crate::adapter::Adapter::connect(sim).unwrap());
        assert!(errors.iter().any(|e| e.starts_with("18DA60F1: adapter rejected ATCRA18DAF160")), "{errors:?}");
        assert!(errors.iter().any(|e| e.starts_with("18DA01F1 222028: refused")), "{errors:?}");
        assert!(errors.iter().any(|e| e.starts_with("18DA16F1 222028: reply too short")), "{errors:?}");
        assert_eq!(get(&rs, "hv_capacity_ah"), 40.16, "the rest is still read");
    }

    #[test]
    fn the_battery_worn_rehearsal_has_low_capacity_and_the_deterioration_codes() {
        let mut a = crate::adapter::Adapter::connect(crate::sim::Sim::from_spec("clarity-phev-2018:battery-worn").unwrap()).unwrap();
        let r = crate::report::build(&mut a, &crate::report::Ident::default()).unwrap();
        assert_eq!(get(&r.extended, "hv_capacity_ah"), 35.2);
        assert!(["P0A7F", "P1DC7"].iter().all(|c| r.codes.iter().any(|x| x.code == *c)), "{:?}", r.codes);
        assert!(r.findings.iter().any(|f| f.title.contains("warranty-replacement")));
        assert!(crate::sim::Sim::from_spec("kona-ev-2020:battery-worn").err().unwrap().to_string().contains("only"));
    }

    #[test]
    fn an_unset_or_implausible_capacity_is_not_reported() {
        let mut p = reply(0x01, "22202A");
        for raw in [0xFFFFu16, 0, 7000] {
            p[167..169].copy_from_slice(&raw.to_be_bytes());
            let rs = decode_summary(&p);
            assert!(rs.iter().all(|r| r.key != "hv_capacity_ah"), "{raw:04X}");
            assert!(rs.iter().any(|r| r.key == "hv_soc"), "the rest of the summary stays");
        }
    }

    #[test]
    fn engine_advice_that_assumes_an_always_running_engine_is_reworded_or_dropped() {
        use crate::findings::{Finding, Severity};
        let f = |t: &str| Finding { severity: Severity::Advice, code: None, title: t.into(), detail: "d".into(), recommendation: "test the alternator".into() };
        let mut fs = vec![f("Engine not warming up — possible stuck-open thermostat"), f("Alternator may not be charging"), f("Charging voltage on the low side"), f("Charging voltage high"), f("Other")];
        adapt_engine_findings(&mut fs);
        let titles: Vec<&str> = fs.iter().map(|f| f.title.as_str()).collect();
        assert_eq!(titles, vec!["12 V battery may not be charging", "12 V charging voltage on the low side", "12 V charging voltage high", "Other"]);
        assert!(fs[..3].iter().all(|f| f.recommendation.contains("DC-DC") && !f.recommendation.contains("test the alternator")));
    }

    #[test]
    fn a_report_drops_the_thermostat_advice_and_names_the_dc_dc_converter() {
        let mut sim = crate::sim::Sim::new("clarity-phev-2018").unwrap();
        sim.set_pid(0x0E, 0x05, &[0x50]);
        sim.set_pid(0x0E, 0x1F, &[0x05, 0x00]);
        sim.port_voltage = Some("12.3V");
        let r = crate::report::build(&mut crate::adapter::Adapter::connect(sim).unwrap(), &crate::report::Ident::default()).unwrap();
        let titles: Vec<&str> = r.findings.iter().map(|f| f.title.as_str()).collect();
        assert!(!titles.iter().any(|t| t.contains("warming up")), "{titles:?}");
        let f = r.findings.iter().find(|f| f.title == "12 V battery may not be charging").unwrap_or_else(|| panic!("{titles:?}"));
        assert!(f.recommendation.contains("DC-DC") && !f.recommendation.contains("test the alternator"));
    }

    #[test]
    fn short_replies_decode_to_nothing() {
        assert!(decode_summary(&[0; 160]).is_empty());
        assert!(decode_cells(&[0; 100]).is_empty());
        assert!(decode_odometer(&[0; 8]).is_empty());
    }
}
