//! 2020 Kona Electric: BMS (7E4), VMCU (7E2), TPMS (7A0), cluster odometer (7C6).
//! Byte map and thresholds: wiki `kona-ev-bms-pids`, `kona-ev-other-ecus`, `kona-ev-battery-health`.
//! `p` offsets index the payload after the 3-byte `62 xx xx` (or 2-byte `61 xx`) response header.

use crate::adapter::{Adapter, Transport};
use crate::findings::{Finding, Severity};
use crate::obd::Reading;
use anyhow::Result;
use std::collections::BTreeMap;

use super::{KnownDtc, Powertrain, Profile, Reset, payloads};
use crate::uds::{Module, Protocol};

/// Modules observed answering on a 1st-generation Kona Electric (wiki `kona-ev-modules`); reply ID =
/// request + 8. Read-only: the BMS (Hyundai battery campaign 9A4 relies on its fault memory), the airbag
/// unit, and the smart key (wiki `uds-module-dtcs` §8).
const MODULES: &[Module] = &[
    Module { name: "BMS", description: "high-voltage battery management", request: "7E4", protocol: Protocol::Uds, confidence: "verified", clearable: false },
    Module { name: "MCU", description: "drive motor and inverter", request: "7E3", protocol: Protocol::Kwp, confidence: "verified", clearable: true },
    Module { name: "VCU", description: "vehicle control unit", request: "7E2", protocol: Protocol::Kwp, confidence: "verified", clearable: true },
    Module { name: "OBC", description: "on-board charger and 12 V converter (LDC)", request: "7E5", protocol: Protocol::Kwp, confidence: "verified", clearable: true },
    Module { name: "IEB", description: "brakes, ABS and stability control", request: "7D1", protocol: Protocol::Uds, confidence: "verified", clearable: true },
    Module { name: "MDPS", description: "electric power steering", request: "7D4", protocol: Protocol::Uds, confidence: "verified", clearable: true },
    Module { name: "SRS", description: "airbags (module present; carlink cannot read its codes)", request: "7D2", protocol: Protocol::Uds, confidence: "present, name assumed", clearable: false },
    Module { name: "LKAS", description: "front camera (lane keeping)", request: "7C4", protocol: Protocol::Uds, confidence: "verified", clearable: true },
    Module { name: "BCM", description: "body control and tire-pressure monitoring (TPMS)", request: "7A0", protocol: Protocol::Uds, confidence: "verified", clearable: true },
    Module { name: "IGPM", description: "body electrics: lights, doors, wipers", request: "770", protocol: Protocol::Uds, confidence: "verified", clearable: true },
    Module { name: "SMK", description: "smart key", request: "7A5", protocol: Protocol::Uds, confidence: "present, name from a sibling", clearable: false },
    Module { name: "HVAC", description: "heating, air conditioning and heat pump", request: "7B3", protocol: Protocol::Uds, confidence: "verified", clearable: true },
    Module { name: "SHIFTER", description: "electronic shifter (shift-by-wire)", request: "7B6", protocol: Protocol::Uds, confidence: "verified", clearable: true },
    Module { name: "CLUSTER", description: "instrument cluster", request: "7C6", protocol: Protocol::Uds, confidence: "verified", clearable: true },
];

pub const PROFILE: Profile = Profile {
    id: "kona-ev-2020",
    name: "2020 Hyundai Kona Electric",
    powertrain: Powertrain::Ev,
    wiki: "kona-electric-2020",
    recalls: &[
        "21V-127 (Hyundai 200): LG Nanjing cells with folded anode tabs — HV battery pack replacement for cars built 2018-08-28 to 2020-03-02. Confirm it was done.",
        "20V-630 (Hyundai 196): BMS software update and battery inspection.",
        "20V-748 (Hyundai 199): integrated electronic brake (IEB) software.",
        "26V-728 (Hyundai 311): airbag control unit software — filed 2026-11-03, likely still open.",
        "TSB 22-EV-001H-1: motor/reduction-gear rumble.",
    ],
    notes: &[
        "Generic OBD-II answers only from the MCU and BMS; no RPM/speed PIDs. The car does not report its VIN (the BMS returns a placeholder): always pass --vehicle kona-ev-2020 --vin <VIN>.",
        "Module codes are read from 14 modules (modules[]). The airbag unit (SRS) answers but serves its codes only in a session carlink does not open: an airbag light needs the dealer tool.",
        "carlink never sends the OBD clear (service 04) on this car, and never clears the BMS, SRS or SMK modules (battery campaign evidence, airbags, smart key). Other modules are cleared by name.",
        "Battery data comes from BMS DIDs 220101–220105; SOH stays at 100% for a long time, min deterioration moves first.",
        "Cell spread is meaningful only at ≥80% SOC after resting; a 100% AC charge rebalances.",
        "12 V battery is charged by the LDC only while READY/charging (plus Aux Battery Saver+); a dead 12 V prevents READY.",
    ],
    after_clear: &[],
    modules: MODULES,
    obd_clear_refused: Some("on the Kona Electric, OBD service 04 is answered by the battery management module (BMS), and Hyundai's battery campaign 9A4 (TSB 23-01-073H-1) decides on pack replacement from the codes it stores; clearing them destroys that evidence"),
    kept_codes: (&[], ""),
    known_dtcs: &[
        KnownDtc { code: "P0AA6", note: "HV isolation fault: high-voltage leakage to chassis (often coolant intrusion or a damaged HV connector/cable; on this model also check the battery recall 21V-127 status). Do not attempt repair; have a Hyundai EV-certified dealer inspect. Compare isolation_kohm in carlink history." },
    ],
    resets: &[
        Reset {
            name: "Clear module trouble codes (by name)",
            carlink_can_do: true,
            steps: &[
                "Only after the repair: explain what clearing does, name the system (e.g. brakes), and get an explicit yes from the person (carlink skill, step 6).",
                "Ignition ON (READY not needed).",
                "Run `carlink clear --vehicle kona-ev-2020 --vin <VIN> --only <MODULE> --confirm-clear-codes` with the module name from the scan; read clear_status and follow next_steps.",
            ],
            notes: "carlink never sends the OBD clear (service 04) on this car and never clears the BMS, SRS or SMK modules: battery-campaign evidence, airbags, smart key.",
        },
        Reset { name: "TPMS after tire rotation", carlink_can_do: false, steps: &["Inflate all tires to the door-placard pressure (cold).", "Drive above 25 mph for 10–20 minutes; the display relearns positions by itself."], notes: "Direct TPMS. New sensors or a second wheel set need a TPMS tool or dealer to register IDs." },
        Reset { name: "TPMS low-pressure light", carlink_can_do: false, steps: &["Inflate to placard pressure and drive; the light clears when sensors report the new pressure."], notes: "There is no reset button." },
        Reset { name: "Service interval reminder", carlink_can_do: false, steps: &["Cluster: User Settings → Service Interval → set/reset distance and months, confirm.", "If not in the cluster menu: head unit Setup → Vehicle → Cluster → Service Interval."], notes: "From Hyundai TSB text, not Kona-specific; confirm on the car." },
        Reset { name: "12 V battery replacement", carlink_can_do: false, steps: &["Nothing to reset. After replacement, check aux_12v in a carlink scan with the car READY (expect ~14 V while the LDC charges)."], notes: "" },
    ],
    // Position 8 'G' = battery-electric (NHTSA vPIC); a gasoline 2020 Kona must not get this profile.
    vin: &[("KM8K", 'L', Some('G'))],
};

fn r(key: impl Into<String>, name: &'static str, value: f64, unit: &'static str) -> Reading {
    Reading::new(key, name, (value * 100.0).round() / 100.0, unit)
}

fn u16be(p: &[u8], i: usize) -> f64 {
    u16::from_be_bytes([p[i], p[i + 1]]) as f64
}

fn s16be(p: &[u8], i: usize) -> f64 {
    i16::from_be_bytes([p[i], p[i + 1]]) as f64
}

fn s16le(p: &[u8], i: usize) -> f64 {
    i16::from_le_bytes([p[i], p[i + 1]]) as f64
}

fn u32be(p: &[u8], i: usize) -> f64 {
    u32::from_be_bytes([p[i], p[i + 1], p[i + 2], p[i + 3]]) as f64
}

fn s8(b: u8) -> f64 {
    b as i8 as f64
}

pub fn decode_0101(p: &[u8]) -> Vec<Reading> {
    if p.len() < 59 {
        return vec![];
    }
    let mut v = vec![
        r("soc_bms", "Battery SOC (BMS)", p[4] as f64 / 2.0, "%"),
        r("avail_charge_kw", "Available charge power", u16be(p, 5) / 100.0, "kW"),
        r("avail_discharge_kw", "Available discharge power", u16be(p, 7) / 100.0, "kW"),
        r("main_relay", "HV main relay closed (car READY or charging)", (p[9] & 0x01) as f64, "0/1"),
        r("pack_current", "Pack current (+ = discharge)", s16be(p, 10) / 10.0, "A"),
        r("pack_voltage", "Pack voltage", u16be(p, 12) / 10.0, "V"),
        r("batt_temp_max", "Battery max temperature", s8(p[14]), "°C"),
        r("batt_temp_min", "Battery min temperature", s8(p[15]), "°C"),
        r("inlet_temp", "Battery inlet temperature", s8(p[22]), "°C"),
        r("cell_v_max", "Max cell voltage", p[23] as f64 / 50.0, "V"),
        r("cell_v_max_no", "Max cell number", p[24] as f64, "#"),
        r("cell_v_min", "Min cell voltage", p[25] as f64 / 50.0, "V"),
        r("cell_v_min_no", "Min cell number", p[26] as f64, "#"),
        r("aux_12v_bms", "12 V aux battery (BMS)", p[29] as f64 / 10.0, "V"),
        r("ccc_ah", "Cumulative charge current", u32be(p, 30) / 10.0, "Ah"),
        r("cdc_ah", "Cumulative discharge current", u32be(p, 34) / 10.0, "Ah"),
        r("cec_kwh", "Cumulative energy charged", u32be(p, 38) / 10.0, "kWh"),
        r("ced_kwh", "Cumulative energy discharged", u32be(p, 42) / 10.0, "kWh"),
        r("operating_hours", "BMS operating time", u32be(p, 46) / 3600.0, "h"),
        r("isolation_kohm", "Isolation resistance", u16be(p, 57), "kΩ"),
    ];
    const MODULES: [&str; 4] = ["Module 1 temperature", "Module 2 temperature", "Module 3 temperature", "Module 4 temperature"];
    for (i, name) in MODULES.into_iter().enumerate() {
        v.push(r(format!("module_temp_{}", i + 1), name, s8(p[16 + i]), "°C"));
    }
    v
}

/// (cell number, volts) for DIDs 0102–0105.
pub fn decode_cells(did: u8, p: &[u8]) -> Vec<(u32, f64)> {
    let (first, start, count) = match did {
        0x02 => (1, 4, 32),
        0x03 => (33, 4, 32),
        0x04 => (65, 4, 32),
        0x05 => (97, 34, 2),
        _ => return vec![],
    };
    // 0102–0104 begin with FF FF FF FF; anything else is not a cell-voltage reply.
    if did != 0x05 && p.get(..4) != Some(&[0xFF; 4][..]) {
        return vec![];
    }
    p.iter().skip(start).take(count).enumerate().map(|(i, &b)| (first + i as u32, b as f64 / 50.0)).collect()
}

pub fn decode_0105(p: &[u8]) -> Vec<Reading> {
    if p.len() < 36 {
        return vec![];
    }
    vec![
        r("cell_deviation_v", "Cell voltage deviation (BMS)", p[20] as f64 / 50.0, "V"),
        r("soh", "Battery state of health", u16be(p, 25) / 10.0, "%"),
        r("min_deterioration", "Min cell deterioration", u16be(p, 28) / 10.0, "%"),
        r("min_deterioration_cell", "Min deterioration cell number", p[30] as f64, "#"),
        r("soc_display", "Displayed SOC", p[31] as f64 / 2.0, "%"),
    ]
}

pub fn decode_odometer(p: &[u8]) -> Vec<Reading> {
    if p.len() < 12 {
        return vec![];
    }
    vec![r("odometer_mi", "Odometer", ((p[9] as u32) << 16 | (p[10] as u32) << 8 | p[11] as u32) as f64, "mi")]
}

/// Rear left/right order is disputed between sources, hence `rear_a`/`rear_b`.
pub fn decode_tpms(p: &[u8]) -> Vec<Reading> {
    if p.len() < 18 {
        return vec![];
    }
    let mut v = Vec::new();
    for (tire, i) in [("fl", 4), ("fr", 8), ("rear_a", 12), ("rear_b", 16)] {
        v.push(r(format!("tire_{tire}_psi"), "Tire pressure", p[i] as f64 / 5.0, "psi"));
        v.push(r(format!("tire_{tire}_temp"), "Tire temperature", p[i + 1] as f64 - 50.0, "°C"));
    }
    v
}

/// Service 21 response `61 02`, so `p` starts after 2 header bytes.
pub fn decode_vmcu_2102(p: &[u8]) -> Vec<Reading> {
    if p.len() < 24 {
        return vec![];
    }
    vec![
        r("aux_12v", "12 V aux battery voltage", s16le(p, 19) / 1000.0, "V"),
        r("aux_12v_current", "12 V aux battery current", s16le(p, 21) / 1000.0, "A"),
        r("aux_12v_soc", "12 V aux battery SOC", p[23] as f64, "%"),
    ]
}

const REQUESTS: &[(&str, &[&str])] = &[
    ("7E4", &["220101", "220102", "220103", "220104", "220105"]),
    ("7E2", &["2102"]),
    ("7A0", &["22C00B"]),
    ("7C6", &["22B002"]),
];

fn decode(req: &str, p: &[u8], out: &mut Vec<Reading>, cells: &mut Vec<(u32, f64)>) {
    match req {
        "220101" => out.extend(decode_0101(p)),
        "220105" => {
            out.extend(decode_0105(p));
            cells.extend(decode_cells(0x05, p));
        }
        "22B002" => out.extend(decode_odometer(p)),
        "22C00B" => out.extend(decode_tpms(p)),
        "2102" => out.extend(decode_vmcu_2102(p)),
        _ => cells.extend(decode_cells(u8::from_str_radix(&req[4..], 16).unwrap_or(0), p)),
    }
}

/// Keys `bms_live` can produce, i.e. what `watch` may sample on this car.
pub const LIVE_KEYS: &[&str] = &[
    "soc_bms", "pack_voltage", "pack_current", "batt_temp_max", "batt_temp_min", "inlet_temp", "cell_v_max",
    "cell_v_min", "aux_12v_bms", "avail_charge_kw", "avail_discharge_kw", "isolation_kohm",
];

/// One BMS 220101 read, for live sampling.
pub fn bms_live<T: Transport>(a: &mut Adapter<T>) -> Result<Vec<Reading>> {
    match payloads(a, "7E4", &["220101"])?.pop() {
        Some(Ok(Some(p))) => Ok(decode_0101(&p)),
        Some(Err(e)) => Err(e),
        _ => anyhow::bail!("the battery management module did not answer"),
    }
}

/// All Kona-specific reads. Each failure is reported, the rest continue.
pub fn read<T: Transport>(a: &mut Adapter<T>) -> (Vec<Reading>, Vec<String>) {
    let mut out = Vec::new();
    let mut errors = Vec::new();
    let mut cells = Vec::new();
    for (header, reqs) in REQUESTS {
        match payloads(a, header, reqs) {
            Ok(results) => {
                for (req, res) in reqs.iter().zip(results) {
                    match res {
                        Ok(Some(p)) => {
                            let (readings, cell_count) = (out.len(), cells.len());
                            decode(req, &p, &mut out, &mut cells);
                            if out.len() == readings && cells.len() == cell_count {
                                errors.push(format!("{header} {req}: reply too short or malformed ({} bytes)", p.len()));
                            }
                        }
                        Ok(None) => errors.push(format!("{header} {req}: no response")),
                        Err(e) => errors.push(format!("{header} {req}: {e}")),
                    }
                }
            }
            Err(e) => errors.push(format!("{header}: {e}")),
        }
    }
    match cell_spread(&cells, &out) {
        Ok(spread) => {
            out.push(r("cell_delta_v", "Cell voltage spread (max − min, all 98)", spread, "V"));
            out.extend(cells.into_iter().map(|(n, v)| r(format!("cell_v_{n:02}"), "Cell voltage", v, "V")));
        }
        Err(e) if !cells.is_empty() => errors.push(format!("cell voltages not used: {e}")),
        Err(_) => {}
    }
    (out, errors)
}

/// Max − min over all 98 cells, only when every cell is plausible and the extremes agree (±0.02 V,
/// one count) with the BMS's own max/min from 220101. Otherwise the cell data is not trusted at all.
pub fn cell_spread(cells: &[(u32, f64)], summary: &[Reading]) -> std::result::Result<f64, String> {
    if cells.len() != 98 {
        return Err(format!("{} of 98 cells read", cells.len()));
    }
    if let Some((n, v)) = cells.iter().find(|c| !(2.5..=4.3).contains(&c.1)) {
        return Err(format!("implausible voltage {v:.2} V for cell {n}"));
    }
    let max = cells.iter().map(|c| c.1).fold(f64::MIN, f64::max);
    let min = cells.iter().map(|c| c.1).fold(f64::MAX, f64::min);
    let close = |key: &str, v: f64| get(summary, key).is_none_or(|s| (s - v).abs() <= 0.021);
    if !close("cell_v_max", max) || !close("cell_v_min", min) {
        return Err(format!("cell readings ({min:.2}–{max:.2} V) disagree with the BMS summary"));
    }
    Ok(((max - min) * 100.0).round() / 100.0)
}

fn get(rs: &[Reading], key: &str) -> Option<f64> {
    rs.iter().find(|r| r.key == key).map(|r| r.value)
}

fn finding(severity: Severity, title: &str, detail: String, recommendation: &str) -> Finding {
    Finding { severity, code: None, title: title.into(), detail, recommendation: recommendation.into() }
}

pub fn findings(rs: &[Reading]) -> Vec<Finding> {
    let mut out = Vec::new();
    if let Some(d) = get(rs, "cell_delta_v") {
        let soc = get(rs, "soc_display").map(|s| format!(" at {s:.0}% SOC")).unwrap_or_default();
        let detail = format!("Max−min cell voltage {:.0} mV{soc}; healthy is ≤20 mV (0.02 V resolution).", d * 1000.0);
        let compare = "Re-measure at ≥80% SOC after resting; a 100% AC charge rebalances the pack.";
        if get(rs, "soc_display").is_some_and(|s| s < 80.0) {
            if d >= 0.04 {
                out.push(finding(Severity::Info, "Battery cell spread measured at low SOC", detail, "Spread is only meaningful at ≥80% SOC after resting; re-scan after charging."));
            }
        } else if d >= 0.10 {
            out.push(finding(Severity::Critical, "Battery cell imbalance severe", detail, "Large imbalance can mean a failing cell. Have a Hyundai EV dealer inspect the pack and check recall 21V-127 status."));
        } else if d >= 0.05 {
            out.push(finding(Severity::Warning, "Battery cell imbalance", detail, compare));
        } else if d >= 0.04 {
            out.push(finding(Severity::Advice, "Battery cell spread worth watching", detail, compare));
        }
    }
    if let Some(soh) = get(rs, "soh") {
        let detail = format!("State of health {soh:.1}%. Warranty covers capacity loss below 70%.");
        if soh < 80.0 {
            out.push(finding(Severity::Warning, "EV battery degraded", detail, "Track the trend; below 70% is a warranty claim. Ask a dealer for a capacity test."));
        } else if soh < 90.0 {
            out.push(finding(Severity::Advice, "EV battery showing wear", detail, "Normal ageing; compare with history to see the rate."));
        }
    }
    let ready = get(rs, "main_relay") == Some(1.0);
    // With the HV system on, the converter (LDC) charges the 12 V battery (wiki kona-ev-resets); a voltage
    // below that of a rested, healthy battery means it is not doing so. Uses the mV-resolution VCU reading.
    match (get(rs, "aux_12v"), get(rs, "aux_12v").or_else(|| get(rs, "aux_12v_bms"))) {
        (Some(v), _) if ready && v < 12.4 => out.push(finding(Severity::Warning, "12 V battery is not being charged", format!("{v:.2} V with the high-voltage system on — lower than a rested, healthy battery."), "Have the 12 V converter (LDC, in the integrated charger/converter unit) and its fuse checked; without it the 12 V battery runs flat and the car will not go READY.")),
        (_, Some(v)) if v < 12.2 => out.push(finding(Severity::Warning, "12 V aux battery low", format!("{v:.2} V."), "A weak 12 V battery stops the car from going READY. Test/replace it; it is charged only while the car is on or charging.")),
        _ => {}
    }
    let tires: Vec<(&str, f64)> = ["fl", "fr", "rear_a", "rear_b"].iter().filter_map(|t| get(rs, &format!("tire_{t}_psi")).map(|p| (*t, p))).collect();
    if let Some(max) = tires.iter().map(|t| t.1).reduce(f64::max) {
        for (tire, psi) in &tires {
            if *psi < 28.0 || *psi < max * 0.8 {
                let which = match *tire {
                    "fl" => "front-left",
                    "fr" => "front-right",
                    _ => "one rear",
                };
                out.push(finding(Severity::Warning, "Tire pressure low", format!("{which} tire at {psi:.1} psi (highest is {max:.1} psi)."), "Check it for a puncture and inflate all tires to the door-placard pressure with the tires cold. (Which rear tire is which is not certain from the data.)"));
            }
        }
    }
    out
}

/// Simulator data: the real 0101 capture plus synthetic other DIDs.
pub fn sim_data() -> Vec<(u16, &'static str, Vec<u8>)> {
    let hex = |s: &str| (0..s.len() / 2).map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap()).collect::<Vec<u8>>();
    let d0101 = hex("620101FFF7E7FF4900000000800000 0DD9090908080908000007B402B40100007E0000DB540000D91500005259 00004F57007B0213000000000000000BB8".replace(' ', "").as_str());
    let cells = |did: u8, pre: usize, n: usize| [vec![0x62, 0x01, did], vec![0xFF; pre], vec![180; n]].concat();
    let mut d0105 = vec![0u8; 46];
    d0105[..3].copy_from_slice(&[0x62, 0x01, 0x05]);
    d0105[3 + 20] = 1;
    d0105[3 + 25..3 + 27].copy_from_slice(&992u16.to_be_bytes());
    d0105[3 + 31] = 75;
    d0105[3 + 34] = 180;
    d0105[3 + 35] = 181;
    let mut odo = vec![0x62, 0xB0, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    odo.extend_from_slice(&[0x00, 0x9C, 0x40]);
    let mut tpms = vec![0x62, 0xC0, 0x0B];
    tpms.extend_from_slice(&[0, 0, 0, 0, 190, 70, 0, 0, 191, 70, 0, 0, 185, 70, 0, 0, 186, 70]);
    let mut vmcu = vec![0x61, 0x02];
    vmcu.extend_from_slice(&[0; 19]);
    vmcu.extend_from_slice(&[0x10, 0x31, 0x00, 0x00, 88]);
    vec![
        (0x7EC, "220101", d0101),
        (0x7EC, "220102", cells(0x02, 4, 32)),
        (0x7EC, "220103", cells(0x03, 4, 32)),
        (0x7EC, "220104", cells(0x04, 4, 32)),
        (0x7EC, "220105", d0105),
        (0x7EA, "2102", vmcu),
        (0x7A8, "22C00B", tpms),
        (0x7CE, "22B002", odo),
    ]
}

pub fn sim_extra(ecu: u16) -> BTreeMap<String, Vec<u8>> {
    sim_data().into_iter().filter(|(e, ..)| *e == ecu).map(|(_, k, v)| (k.to_string(), v)).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::elm::{Bus, parse_response};

    /// Real 2020 Kona Electric capture from OBDb (wiki: kona-ev-bms-pids), spaces added.
    const CAPTURE_0101: &str = "7EC 10 3E 62 01 01 FF F7 E7\r7EC 21 FF 49 00 00 00 00 80\r7EC 22 00 00 0D D9 09 09 08\r\
7EC 23 08 09 08 00 00 07 B4\r7EC 24 02 B4 01 00 00 7E 00\r7EC 25 00 DB 54 00 00 D9 15\r7EC 26 00 00 52 59 00 00 4F\r\
7EC 27 57 00 7B 02 13 00 00\r7EC 28 00 00 00 00 00 0B B8\r";

    fn payload(raw: &str) -> Vec<u8> {
        parse_response(raw, Bus::Can11).unwrap().remove(0).data
    }

    fn get(rs: &[Reading], key: &str) -> f64 {
        rs.iter().find(|r| r.key == key).unwrap_or_else(|| panic!("missing {key}")).value
    }

    #[test]
    fn decodes_real_0101_capture() {
        let r = decode_0101(&payload(CAPTURE_0101)[3..]);
        assert_eq!(get(&r, "soc_bms"), 36.5);
        assert_eq!(get(&r, "pack_voltage"), 354.5);
        assert_eq!(get(&r, "pack_current"), 0.0);
        assert_eq!(get(&r, "batt_temp_max"), 9.0);
        assert_eq!(get(&r, "module_temp_3"), 9.0);
        assert_eq!(get(&r, "inlet_temp"), 7.0);
        assert_eq!((get(&r, "cell_v_max"), get(&r, "cell_v_max_no")), (3.6, 2.0));
        assert_eq!(get(&r, "aux_12v_bms"), 12.6);
        assert_eq!(get(&r, "ccc_ah"), 5614.8);
        assert_eq!(get(&r, "cdc_ah"), 5557.3);
        assert_eq!(get(&r, "cec_kwh"), 2108.1);
        assert_eq!(get(&r, "ced_kwh"), 2031.1);
        assert_eq!(get(&r, "operating_hours"), 2239.29);
        assert_eq!(get(&r, "isolation_kohm"), 3000.0);
    }

    #[test]
    fn signed_fields_decode_negative() {
        let mut p = payload(CAPTURE_0101)[3..].to_vec();
        p[10] = 0xFF; p[11] = 0x9C; // -100 -> -10.0 A (charging)
        p[16] = 0xFB; // -5 °C
        let r = decode_0101(&p);
        assert_eq!(get(&r, "pack_current"), -10.0);
        assert_eq!(get(&r, "module_temp_1"), -5.0);
    }

    #[test]
    fn short_payload_yields_nothing() {
        assert!(decode_0101(&[0; 10]).is_empty());
        assert!(decode_0105(&[0; 10]).is_empty());
        assert!(decode_odometer(&[0; 5]).is_empty());
        assert!(decode_tpms(&[0; 5]).is_empty());
        assert!(decode_vmcu_2102(&[0; 5]).is_empty());
        assert!(decode_cells(0x06, &[0; 40]).is_empty());
    }

    #[test]
    fn cells_map_to_their_numbers_across_dids() {
        let mut p2 = vec![0xFF; 4];
        p2.extend((1..=32).map(|n| 150 + n as u8)); // cell n = (150+n)/50 V
        let c = decode_cells(0x02, &p2);
        assert_eq!(c.len(), 32);
        assert_eq!((c[0].0, c[0].1), (1, 3.02));
        assert_eq!((c[31].0, c[31].1), (32, 3.64));
    }

    fn hexs(s: &str) -> Vec<u8> {
        (0..s.len() / 2).map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap()).collect()
    }

    #[test]
    fn cells_33_to_96_start_after_the_ff_prefix_like_0102() {
        // evDash's real 220103 / 220104 samples (wiki kona-ev-bms-pids), header 62 01 0x stripped
        let p3 = hexs("FFFFFFFFCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCACBCACACFCCCBCBCBCBCBCBCBCBAAAA");
        let c = decode_cells(0x03, &p3);
        assert_eq!(c.len(), 32);
        assert_eq!((c[0].0, c[0].1), (33, 4.06));
        assert_eq!((c[18].0, c[18].1), (51, 4.04));
        assert_eq!((c[22].0, c[22].1), (55, 4.14));
        assert_eq!(c[31].0, 64);
        let p4 = hexs("FFFFFFFFCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBAAAA");
        assert_eq!(decode_cells(0x04, &p4).iter().map(|c| c.0).collect::<Vec<_>>(), (65..=96).collect::<Vec<_>>());
    }

    #[test]
    fn cell_reply_without_the_ff_prefix_is_rejected() {
        let p = hexs("00FFFFFFCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCBCB");
        assert!(decode_cells(0x02, &p).is_empty());
    }

    #[test]
    fn implausible_or_inconsistent_cells_give_no_spread() {
        let ok = |n: u32| (n, 3.60);
        let mut cells: Vec<(u32, f64)> = (1..=98).map(ok).collect();
        let summary = |max: f64, min: f64| vec![Reading::new("cell_v_max", "", max, "V"), Reading::new("cell_v_min", "", min, "V")];
        assert_eq!(cell_spread(&cells, &summary(3.60, 3.60)), Ok(0.0));
        cells[40].1 = 5.10;
        assert!(cell_spread(&cells, &summary(3.60, 3.60)).unwrap_err().contains("implausible"));
        cells[40].1 = 3.40;
        assert!(cell_spread(&cells, &summary(3.60, 3.60)).unwrap_err().contains("disagree"));
        assert_eq!(cell_spread(&cells, &summary(3.60, 3.40)), Ok(0.2));
        assert!(cell_spread(&cells[..97], &summary(3.60, 3.40)).is_err());
    }

    #[test]
    fn decodes_0105_soh_display_soc_and_last_cells() {
        let mut p = vec![0u8; 43];
        p[20] = 2; // 0.04 V deviation
        p[25] = 0x03; p[26] = 0xE0; // 992 -> 99.2 %
        p[28] = 0x00; p[29] = 0x31; p[30] = 55;
        p[31] = 75; // 37.5 %
        p[34] = 180; p[35] = 181;
        let r = decode_0105(&p);
        assert_eq!(get(&r, "soh"), 99.2);
        assert_eq!(get(&r, "soc_display"), 37.5);
        assert_eq!(get(&r, "min_deterioration"), 4.9);
        assert_eq!(get(&r, "cell_deviation_v"), 0.04);
        assert_eq!(decode_cells(0x05, &p), vec![(97, 3.6), (98, 3.62)]);
    }

    #[test]
    fn odometer_and_tpms_and_12v() {
        let mut odo = vec![0u8; 12];
        odo[9] = 0x00; odo[10] = 0x9C; odo[11] = 0x40; // 40000 mi
        assert_eq!(get(&decode_odometer(&odo), "odometer_mi"), 40000.0);
        let mut t = vec![0u8; 18];
        t[4] = 190; t[5] = 70; t[8] = 191; t[12] = 185; t[16] = 186;
        let r = decode_tpms(&t);
        assert_eq!((get(&r, "tire_fl_psi"), get(&r, "tire_fl_temp")), (38.0, 20.0));
        assert_eq!(get(&r, "tire_rear_b_psi"), 37.2);
        let mut v = vec![0u8; 24];
        v[19] = 0x10; v[20] = 0x31; // 0x3110 = 12560 mV LE
        v[23] = 88;
        let r = decode_vmcu_2102(&v);
        assert_eq!((get(&r, "aux_12v"), get(&r, "aux_12v_soc")), (12.56, 88.0));
    }

    #[test]
    fn reply_for_a_different_identifier_is_not_decoded() {
        let mut sim = crate::sim::Sim::new("kona-ev-2020").unwrap();
        // a late 220101 reply arriving as the answer to 220102
        sim.inject("220102", "7EC 10 3E 62 01 01 FF F7 E7\r7EC 21 FF 49 00 00 00 00 80\r7EC 22 00 00 0D D9 09 09 08\r7EC 23 08 09 08 00 00 07 B4\r7EC 24 02 B4 01 00 00 7E 00\r7EC 25 00 DB 54 00 00 D9 15\r7EC 26 00 00 52 59 00 00 4F\r7EC 27 57 00 7B 02 13 00 00\r7EC 28 00 00 00 00 00 0B B8");
        let mut a = crate::adapter::Adapter::connect(sim).unwrap();
        let (readings, errors) = read(&mut a);
        assert!(errors.iter().any(|e| e.contains("220102")), "{errors:?}");
        assert!(!readings.iter().any(|r| r.key == "cell_v_01"));
        assert!(!readings.iter().any(|r| r.key == "cell_delta_v"));
    }

    #[test]
    fn refused_and_short_replies_are_reported_as_such() {
        let mut sim = crate::sim::Sim::new("kona-ev-2020").unwrap();
        sim.inject("22C00B", "7A8 03 7F 22 31");
        sim.inject("22B002", "7CE 04 62 B0 02 00");
        let mut a = crate::adapter::Adapter::connect(sim).unwrap();
        let (_, errors) = read(&mut a);
        assert!(errors.iter().any(|e| e.contains("22C00B") && e.contains("out of range")), "{errors:?}");
        assert!(errors.iter().any(|e| e.contains("22B002") && e.contains("too short")), "{errors:?}");
    }

    #[test]
    fn findings_from_battery_health() {
        let cells = |spread: f64| vec![Reading::new("cell_delta_v", "", spread, "V"), Reading::new("soc_display", "", 85.0, "%")];
        let sev = |rs: Vec<Reading>| findings(&rs).into_iter().map(|f| f.severity).collect::<Vec<_>>();
        assert!(sev(cells(0.02)).is_empty());
        assert_eq!(sev(cells(0.04)), vec![Severity::Advice]);
        assert_eq!(sev(cells(0.06)), vec![Severity::Warning]);
        assert_eq!(sev(cells(0.10)), vec![Severity::Critical]);
        let low_soc = vec![Reading::new("cell_delta_v", "", 0.10, "V"), Reading::new("soc_display", "", 15.0, "%")];
        assert_eq!(sev(low_soc), vec![Severity::Info]);
        let soh = |v: f64| vec![Reading::new("soh", "", v, "%")];
        assert!(sev(soh(95.0)).is_empty());
        assert_eq!(sev(soh(85.0)), vec![Severity::Advice]);
        assert_eq!(sev(soh(78.0)), vec![Severity::Warning]);
        let aux = vec![Reading::new("aux_12v", "", 11.8, "V")];
        assert_eq!(sev(aux), vec![Severity::Warning]);
    }

    #[test]
    fn ready_car_whose_12v_is_not_charged_is_flagged() {
        let ready = |v: f64| vec![Reading::new("main_relay", "", 1.0, "0/1"), Reading::new("aux_12v", "", v, "V")];
        assert_eq!(findings(&ready(12.3))[0].title, "12 V battery is not being charged");
        assert!(findings(&ready(14.1)).is_empty());
        assert!(findings(&ready(12.8)).is_empty(), "a converter in a low-load mode is not a fault");
    }

    #[test]
    fn a_low_tire_is_flagged() {
        let tires = |fl: f64| ["fl", "fr", "rear_a", "rear_b"].iter().map(|t| Reading::new(format!("tire_{t}_psi"), "", if *t == "fl" { fl } else { 37.0 }, "psi")).collect::<Vec<_>>();
        assert_eq!(findings(&tires(18.0))[0].title, "Tire pressure low");
        assert!(findings(&tires(36.0)).is_empty());
    }
}
