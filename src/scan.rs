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
    /// Non-fatal problems hit during the sweep.
    pub errors: Vec<String>,
}

fn by_service(msgs: Vec<Message>, sid: u8) -> impl Iterator<Item = Message> {
    msgs.into_iter().filter(move |m| m.data.first() == Some(&sid))
}

fn supported<T: Transport>(a: &mut Adapter<T>) -> Result<BTreeMap<String, Vec<u8>>> {
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

fn dtcs<T: Transport>(a: &mut Adapter<T>, service: &str, kind: &'static str) -> Result<Vec<Dtc>> {
    let sid = u8::from_str_radix(service, 16)? + 0x40;
    let skip = if a.bus().is_can() { 2 } else { 1 };
    Ok(by_service(a.query(service)?, sid)
        .flat_map(|m| {
            let ecu = m.ecu.clone();
            obd::decode_dtc_list(m.data.get(skip..).unwrap_or(&[])).into_iter().map(move |code| Dtc { code, kind, ecu: ecu.clone() })
        })
        .collect())
}

pub fn vin<T: Transport>(a: &mut Adapter<T>) -> Result<Option<String>> {
    let msgs: Vec<Message> = by_service(a.query("0902")?, 0x49).collect();
    // CAN: one reassembled message. K-line: one message per 4-byte chunk, each "49 02 <seq> d d d d".
    if let Some(v) = msgs.iter().find_map(|m| obd::decode_vin(m.data.get(2..)?)) {
        return Ok(Some(v));
    }
    let chunks: Vec<u8> = std::iter::once(1).chain(msgs.iter().flat_map(|m| m.data.get(3..).unwrap_or(&[]).to_vec())).collect();
    Ok(obd::decode_vin(&chunks))
}

pub fn scan<T: Transport>(a: &mut Adapter<T>) -> Result<Scan> {
    let mut errors = Vec::new();
    let mut soft = |r: Result<Vec<Dtc>>, what: &str| r.unwrap_or_else(|e| { errors.push(format!("{what}: {e}")); vec![] });
    let battery_voltage = a.voltage()?;
    let supported = supported(a)?;
    let mut dtcs_all = soft(dtcs(a, "03", "stored"), "stored codes");
    dtcs_all.extend(soft(dtcs(a, "07", "pending"), "pending codes"));
    dtcs_all.extend(soft(dtcs(a, "0A", "permanent"), "permanent codes"));
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
            } else if let Some(reading) = obd::decode_pid(pid, &m.data[2..]) {
                live.push(EcuReading { ecu: m.ecu, reading });
            }
        }
    }

    let mut freeze_frames: Vec<FreezeFrame> = Vec::new();
    if dtcs_all.iter().any(|d| d.kind == "stored") {
        for m in by_service(a.query("020200").unwrap_or_default(), 0x42) {
            let dtc = m.data.get(3..5).and_then(|p| obd::decode_dtc(p[0], p[1]));
            freeze_frames.push(FreezeFrame { ecu: m.ecu, dtc, readings: vec![] });
        }
        for pid in obd::snapshot_pids() {
            if freeze_frames.is_empty() || !supported.values().any(|p| p.contains(&pid)) {
                continue;
            }
            for m in by_service(a.query(&format!("02{pid:02X}00")).unwrap_or_default(), 0x42) {
                if let (Some(ff), Some(r)) = (freeze_frames.iter_mut().find(|f| f.ecu == m.ecu), obd::decode_pid(pid, m.data.get(3..).unwrap_or(&[]))) {
                    ff.readings.push(r);
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
        errors,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::Adapter;
    use crate::sim::Sim;

    fn run(vehicle: &str) -> Scan {
        let mut a = Adapter::connect(Sim::new(vehicle).unwrap()).unwrap();
        scan(&mut a).unwrap()
    }

    #[test]
    fn reads_vin_codes_and_readiness_on_can() {
        let s = run("odyssey-2016");
        assert_eq!(s.vin.as_deref(), Some("5FNRL5H6XGB000001"));
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
        assert_eq!(vin(&mut a).unwrap().as_deref(), Some("5FNRL5H6XGB000001"));
        assert_eq!(vin(&mut Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap()).unwrap(), None);
    }

    #[test]
    fn k_line_vin_chunks_are_joined() {
        let raw = "48 6B 10 49 02 01 00 00 00 4A 00\r48 6B 10 49 02 02 48 4C 52 44 00\r48 6B 10 49 02 03 37 38 38 37 00\r48 6B 10 49 02 04 34 43 30 30 00\r48 6B 10 49 02 05 30 30 30 31 00\r";
        let msgs = crate::elm::parse_response(raw, crate::elm::Bus::Legacy).unwrap();
        let chunks: Vec<u8> = std::iter::once(1).chain(msgs.iter().flat_map(|m| m.data[3..].to_vec())).collect();
        assert_eq!(obd::decode_vin(&chunks).as_deref(), Some("JHLRD78874C000001"));
    }

    #[test]
    fn ev_with_few_generic_pids_still_scans() {
        let s = run("kona-ev-2020");
        assert_eq!(s.vin, None); // no source documents Service 09 VIN on this car
        assert!(s.dtcs.is_empty());
        assert!(s.freeze_frames.is_empty());
    }
}
