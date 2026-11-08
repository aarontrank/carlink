//! The generic OBD-II sweep: what every vehicle is asked, regardless of make.

use crate::adapter::{Adapter, AdapterInfo, Transport};
use crate::elm::Message;
use crate::obd::{self, MonitorStatus, Reading};
use anyhow::Result;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct EcuReading {
    pub ecu: String,
    #[serde(flatten)]
    pub reading: Reading,
}

#[derive(Debug, Clone, Serialize)]
pub struct Dtc {
    pub code: String,
    pub kind: &'static str,
    pub ecu: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct EcuMonitors {
    pub ecu: String,
    #[serde(flatten)]
    pub status: MonitorStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct FreezeFrame {
    pub ecu: String,
    pub dtc: Option<String>,
    pub readings: Vec<Reading>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Scan {
    pub adapter: AdapterInfo,
    pub battery_voltage: Option<f64>,
    pub vin: Option<String>,
    pub ecus: Vec<String>,
    pub monitors: Vec<EcuMonitors>,
    pub dtcs: Vec<Dtc>,
    pub freeze_frames: Vec<FreezeFrame>,
    pub live: Vec<EcuReading>,
    /// False when any of the stored/pending/permanent code reads failed, so `dtcs` may be missing codes.
    pub codes_complete: bool,
    /// Non-fatal problems hit during the sweep.
    pub errors: Vec<String>,
}

/// Engine computer reply addresses, most likely first: 7E8 (11-bit CAN), 18DAF110 (29-bit), Honda's
/// 18DAF111 (29-bit), 10 (K-line).
const ENGINE_ECUS: [&str; 4] = ["7E8", "18DAF110", "18DAF111", "10"];

/// The one ECU among `ecus` treated as the engine computer: its readings get bare history keys and
/// drive the engine findings. Exactly one, so a key never belongs to two modules.
pub fn engine_ecu<'a>(ecus: impl IntoIterator<Item = &'a str> + Clone) -> Option<&'a str> {
    ENGINE_ECUS.iter().find_map(|e| ecus.clone().into_iter().find(|x| x == e))
}

fn by_service(msgs: Vec<Message>, sid: u8) -> impl Iterator<Item = Message> {
    msgs.into_iter().filter(move |m| m.data.first() == Some(&sid))
}

pub fn supported<T: Transport>(a: &mut Adapter<T>) -> Result<BTreeMap<String, Vec<u8>>> {
    let mut out: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    let mut base = 0u8;
    loop {
        let mut more = false;
        for m in by_service(a.query(&format!("01{base:02X}"))?, 0x41) {
            if m.data.len() >= 6 && m.data[1] == base {
                let pids = obd::supported_pids(base, &m.data[2..6]);
                more |= base < 0xE0 && pids.contains(&(base + 0x20));
                out.entry(m.ecu).or_default().extend(pids);
            }
        }
        if !more || base == 0xE0 {
            return Ok(out);
        }
        base += 0x20;
    }
}

/// The codes, and which ECUs answered at all.
fn dtcs<T: Transport>(a: &mut Adapter<T>, service: &str, kind: &'static str) -> Result<(Vec<Dtc>, Vec<String>)> {
    let sid = u8::from_str_radix(service, 16)? + 0x40;
    let skip = if a.bus().is_can() { 2 } else { 1 };
    let msgs: Vec<Message> = by_service(a.query(service)?, sid).collect();
    let answered = msgs.iter().map(|m| m.ecu.clone()).collect();
    let codes = msgs
        .into_iter()
        .flat_map(|m| {
            let ecu = m.ecu.clone();
            obd::decode_dtc_list(m.data.get(skip..).unwrap_or(&[])).into_iter().map(move |code| Dtc { code, kind, ecu: ecu.clone() })
        })
        .collect();
    Ok((codes, answered))
}

/// The engine ECU's VIN when it has a valid one, else any module's valid VIN.
pub fn vin<T: Transport>(a: &mut Adapter<T>) -> Result<Option<String>> {
    let msgs: Vec<Message> = by_service(a.query("0902")?, 0x49).filter(|m| m.data.get(1) == Some(&0x02)).collect();
    let mut ecus: Vec<&str> = msgs.iter().map(|m| m.ecu.as_str()).collect();
    ecus.dedup();
    let engine = engine_ecu(ecus.iter().copied());
    ecus.sort_by_key(|e| Some(*e) != engine);
    let candidates = ecus.iter().filter_map(|ecu| {
        let mine: Vec<&Message> = msgs.iter().filter(|m| m.ecu == *ecu).collect();
        let payload: Vec<u8> = if a.bus().is_can() {
            mine.first()?.data.get(2..)?.to_vec()
        } else {
            // K-line: one message per 4 bytes, "49 02 <seq> d d d d", possibly out of order.
            let mut chunks: Vec<(u8, &[u8])> = mine.iter().filter_map(|m| Some((*m.data.get(2)?, m.data.get(3..)?))).collect();
            chunks.sort_by_key(|c| c.0);
            std::iter::once(1).chain(chunks.iter().flat_map(|c| c.1.iter().copied())).collect()
        };
        obd::decode_vin(&payload)
    });
    Ok(candidates.into_iter().find(|v| obd::valid_vin(v)))
}

pub fn scan<T: Transport>(a: &mut Adapter<T>) -> Result<Scan> {
    let mut errors = Vec::new();
    let battery_voltage = a.voltage().unwrap_or_else(|e| {
        errors.push(format!("battery voltage: {e}"));
        None
    });
    let supported = supported(a)?;
    let mut codes_complete = true;
    let mut dtcs_all = Vec::new();
    for (service, kind, what) in [("03", "stored", "stored codes"), ("07", "pending", "pending codes"), ("0A", "permanent", "permanent codes")] {
        match dtcs(a, service, kind) {
            Ok((d, answered)) => {
                // On CAN every OBD ECU answers 03 ("43 00" when it has none); silence is a lost reply.
                if service == "03" && a.bus().is_can() {
                    for ecu in supported.keys().filter(|e| !answered.contains(e)) {
                        errors.push(format!("{what}: ECU {ecu} did not answer"));
                        codes_complete = false;
                    }
                }
                dtcs_all.extend(d);
            }
            Err(e) => {
                errors.push(format!("{what}: {e}"));
                codes_complete = false;
            }
        }
    }
    let vin = vin(a).unwrap_or_else(|e| { errors.push(format!("vin: {e}")); None });

    let mut monitors = Vec::new();
    let mut live = Vec::new();
    for pid in obd::snapshot_pids().chain([0x01]) {
        if !supported.values().any(|p| p.contains(&pid)) {
            continue;
        }
        let msgs = match a.query(&format!("01{pid:02X}")) {
            Ok(m) => m,
            Err(e) => { errors.push(format!("pid {pid:02X}: {e}")); continue }
        };
        for m in by_service(msgs, 0x41).filter(|m| m.data.get(1) == Some(&pid)) {
            if pid == 0x01 {
                if let Some(status) = obd::decode_monitor_status(&m.data[2..]) {
                    monitors.push(EcuMonitors { ecu: m.ecu, status });
                }
            } else {
                for reading in obd::decode_pid_all(pid, &m.data[2..]) {
                    live.push(EcuReading { ecu: m.ecu.clone(), reading });
                }
            }
        }
    }

    // The monitor status counts each ECU's confirmed codes; fewer read than counted means some were lost.
    for m in &monitors {
        let read = dtcs_all.iter().filter(|d| d.ecu == m.ecu && d.kind == "stored").count();
        if (m.status.dtc_count as usize) > read {
            errors.push(format!("ECU {} reports {} confirmed code(s) but {read} were read", m.ecu, m.status.dtc_count));
            codes_complete = false;
        }
    }

    let mut freeze_frames: Vec<FreezeFrame> = Vec::new();
    if dtcs_all.iter().any(|d| d.kind == "stored") {
        let first = a.query("020200").unwrap_or_else(|e| {
            errors.push(format!("freeze frame: {e}"));
            vec![]
        });
        // A module that answers DTC 0000 has no freeze frame; its other service 02 data means nothing.
        for m in by_service(first, 0x42).filter(|m| m.data.get(1..3) == Some(&[0x02, 0x00][..])) {
            if let Some(dtc) = m.data.get(3..5).and_then(|p| obd::decode_dtc(p[0], p[1])) {
                freeze_frames.push(FreezeFrame { ecu: m.ecu, dtc: Some(dtc), readings: vec![] });
            }
        }
        for pid in obd::snapshot_pids() {
            if freeze_frames.is_empty() || !supported.values().any(|p| p.contains(&pid)) {
                continue;
            }
            let msgs = a.query(&format!("02{pid:02X}00")).unwrap_or_else(|e| {
                errors.push(format!("freeze frame pid {pid:02X}: {e}"));
                vec![]
            });
            for m in by_service(msgs, 0x42).filter(|m| m.data.get(1..3) == Some(&[pid, 0x00][..])) {
                if let Some(ff) = freeze_frames.iter_mut().find(|f| f.ecu == m.ecu) {
                    ff.readings.extend(obd::decode_pid_all(pid, m.data.get(3..).unwrap_or(&[])));
                }
            }
        }
    }

    Ok(Scan {
        adapter: a.info().clone(),
        battery_voltage,
        vin,
        ecus: supported.keys().cloned().collect(),
        monitors,
        dtcs: dtcs_all,
        freeze_frames,
        live,
        codes_complete,
        errors,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::Adapter;
    use crate::sim::Sim;

    fn run(vehicle: &str) -> Scan {
        let mut a = Adapter::connect(Sim::from_spec(vehicle).unwrap()).unwrap();
        scan(&mut a).unwrap()
    }

    #[test]
    fn reads_vin_codes_and_readiness_on_can() {
        let s = run("odyssey-2016");
        assert_eq!(s.vin.as_deref(), Some("5FNRL5H60GB000001"));
        assert_eq!(s.battery_voltage, Some(14.1));
        let codes: Vec<_> = s.dtcs.iter().map(|d| (d.code.as_str(), d.kind)).collect();
        assert_eq!(codes, vec![("P0303", "stored"), ("P3400", "stored"), ("P0303", "permanent")]);
        let m = &s.monitors[0];
        assert_eq!(m.ecu, "7E8");
        assert!(m.status.mil_on);
        assert!(m.status.monitors.iter().any(|x| x.name == "evap" && x.incomplete));
        assert_eq!(s.ecus, vec!["7E8", "7E9"]);
    }

    #[test]
    fn live_readings_come_from_supported_pids_per_ecu() {
        let s = run("odyssey-2016");
        let rpm = s.live.iter().find(|r| r.reading.key == "rpm").unwrap();
        assert_eq!((rpm.ecu.as_str(), rpm.reading.value), ("7E8", 720.0));
        assert_eq!(s.live.iter().filter(|r| r.reading.key == "speed").count(), 2);
        assert!(s.live.iter().all(|r| r.reading.key != "fuel_rate"));
    }

    #[test]
    fn freeze_frame_records_triggering_code() {
        let s = run("odyssey-2016");
        let ff = &s.freeze_frames[0];
        assert_eq!(ff.dtc.as_deref(), Some("P0303"));
        assert!(ff.readings.iter().any(|r| r.key == "rpm"));
    }

    #[test]
    fn legacy_bus_without_vin_or_freeze_codes() {
        let s = run("crv-2004");
        assert_eq!(s.vin, None);
        assert_eq!(s.dtcs.iter().map(|d| d.code.as_str()).collect::<Vec<_>>(), vec!["P0420", "P1457"]);
        assert!(s.dtcs.iter().all(|d| d.ecu == "10"));
        assert!(s.live.iter().any(|r| r.reading.key == "coolant_temp" && r.reading.value == 83.0));
    }

    #[test]
    fn vin_survives_malformed_and_multi_ecu_replies() {
        let mut a = Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap();
        a.transport_mut().set_vin(0x7E9, "00000000000000000");
        assert_eq!(vin(&mut a).unwrap().as_deref(), Some("5FNRL5H60GB000001"));
        assert_eq!(vin(&mut Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap()).unwrap(), None);
    }

    #[test]
    fn k_line_vin_chunks_are_ordered_and_joined() {
        let mut sim = Sim::new("crv-2004").unwrap();
        sim.set_vin(0x10, "JHLRD78834C000001");
        let mut a = Adapter::connect(sim).unwrap();
        assert_eq!(vin(&mut a).unwrap().as_deref(), Some("JHLRD78834C000001"));
        let mut sim = Sim::new("crv-2004").unwrap();
        sim.inject("0902", "48 6B 10 49 02 02 48 4C 52 44 00\r48 6B 10 49 02 01 00 00 00 4A 00\r48 6B 10 49 02 03 37 38 38 33 00\r48 6B 10 49 02 04 34 43 30 30 00\r48 6B 10 49 02 05 30 30 30 31 00");
        let mut a = Adapter::connect(sim).unwrap();
        assert_eq!(vin(&mut a).unwrap().as_deref(), Some("JHLRD78834C000001"));
    }

    #[test]
    fn engine_ecu_vin_wins_over_a_placeholder_answering_first() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.inject("0902", "7E9 10 14 49 02 01 30 30 30\r7E9 21 30 30 30 30 30 30 30\r7E9 22 30 30 30 30 30 30 30\r7E8 10 14 49 02 01 35 46 4E\r7E8 21 52 4C 35 48 36 30 47\r7E8 22 42 30 30 30 30 30 31");
        let mut a = Adapter::connect(sim).unwrap();
        assert_eq!(vin(&mut a).unwrap().as_deref(), Some("5FNRL5H60GB000001"));
    }

    #[test]
    fn module_without_a_freeze_frame_is_not_reported_as_one() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.inject("020200", "7E8 05 42 02 00 03 03\r7E9 05 42 02 00 00 00");
        let s = scan(&mut Adapter::connect(sim).unwrap()).unwrap();
        assert_eq!(s.freeze_frames.iter().map(|f| f.ecu.as_str()).collect::<Vec<_>>(), vec!["7E8"]);
    }

    #[test]
    fn an_ecu_that_stays_silent_to_03_makes_the_codes_incomplete() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.inject("03", "7E9 02 43 00");
        let s = scan(&mut Adapter::connect(sim).unwrap()).unwrap();
        assert!(!s.codes_complete);
        assert!(s.errors.iter().any(|e| e.contains("7E8 did not answer")), "{:?}", s.errors);
    }

    #[test]
    fn fewer_codes_than_the_ecu_counts_makes_the_codes_incomplete() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.inject("03", "7E8 04 43 01 03 03\r7E9 02 43 00");
        let s = scan(&mut Adapter::connect(sim).unwrap()).unwrap();
        assert!(!s.codes_complete, "PCM counts 2 confirmed codes, only 1 was read");
        assert!(scan(&mut Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap()).unwrap().codes_complete);
    }

    #[test]
    fn a_lit_mil_with_only_a_pending_code_is_a_complete_read() {
        // a severe misfire lights (flashes) the MIL while its code is still pending
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.inject("0101", "7E8 06 41 01 80 07 E5 04\r7E9 06 41 01 00 04 00 00");
        sim.inject("03", "7E8 02 43 00\r7E9 02 43 00");
        sim.inject("07", "7E8 04 47 01 03 01\r7E9 02 47 00");
        let s = scan(&mut Adapter::connect(sim).unwrap()).unwrap();
        assert!(s.codes_complete, "{:?}", s.errors);
    }

    #[test]
    fn freeze_frame_failure_is_reported_not_hidden() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.inject("020200", "CAN ERROR");
        sim.inject("020200", "CAN ERROR");
        let s = scan(&mut Adapter::connect(sim).unwrap()).unwrap();
        assert!(s.errors.iter().any(|e| e.starts_with("freeze frame")), "{:?}", s.errors);
    }

    #[test]
    fn voltage_failure_does_not_abort_the_scan() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.inject("ATRV", "?");
        let s = scan(&mut Adapter::connect(sim).unwrap()).unwrap();
        assert_eq!(s.battery_voltage, None);
        assert!(!s.dtcs.is_empty());
    }

    #[test]
    fn a_29_bit_honda_is_scanned_with_its_engine_computer_recognised() {
        let s = run("odyssey-2016:can29");
        assert_eq!(s.ecus, vec!["18DAF111", "18DAF11D"]);
        assert_eq!(s.vin.as_deref(), Some("5FNRL5H60GB000001"));
        assert!(s.dtcs.iter().any(|d| d.code == "P0303" && d.ecu == "18DAF111"));
        assert!(s.codes_complete, "{:?}", s.errors);
    }

    #[test]
    fn exactly_one_engine_ecu_is_chosen() {
        assert_eq!(engine_ecu(["18DAF111", "18DAF110"]), Some("18DAF110"));
        assert_eq!(engine_ecu(["18DAF11D", "18DAF111"]), Some("18DAF111"));
        assert_eq!(engine_ecu(["7EB", "7EC"]), None);
    }

    #[test]
    fn ev_with_few_generic_pids_still_scans() {
        let s = run("kona-ev-2020");
        assert_eq!(s.vin, None); // no source documents Service 09 VIN on this car
        assert!(s.dtcs.is_empty());
        assert!(s.freeze_frames.is_empty());
    }
}
