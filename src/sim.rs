//! A scripted fake vehicle + adapter, so the whole stack runs without a car (`--port sim:<vehicle>`).

use crate::adapter::Transport;
use crate::elm::Bus;
use crate::vehicles::kona;
use anyhow::{Result, bail};
use std::collections::BTreeMap;

#[derive(Default)]
struct Ecu {
    id: u16,
    pids: BTreeMap<u8, Vec<u8>>,
    stored: Vec<u16>,
    pending: Vec<u16>,
    permanent: Vec<u16>,
    vin: Option<&'static str>,
    /// Service 22 etc: request hex (no spaces) -> response payload.
    extra: BTreeMap<String, Vec<u8>>,
}

pub struct Sim {
    bus: Bus,
    protocol: (&'static str, &'static str),
    ecus: Vec<Ecu>,
    header: u16,
    pub ignition_off: bool,
    /// ECUs stay silent on service 04.
    pub ignore_clear: bool,
    pub ignition_off_after_clear: bool,
    pub forget_vin_after_clear: bool,
    pub garble_clear_reply: bool,
    pub reject_atcra: bool,
    pub log: Vec<String>,
}

pub const VEHICLES: &[&str] = &["crv-2004", "odyssey-2016", "kona-ev-2020"];

fn pids(list: &[(u8, &[u8])]) -> BTreeMap<u8, Vec<u8>> {
    list.iter().map(|(p, d)| (*p, d.to_vec())).collect()
}

impl Sim {
    pub fn new(vehicle: &str) -> Result<Sim> {
        let (bus, protocol, ecus) = match vehicle {
            "crv-2004" => (Bus::Legacy, ("3", "ISO 9141-2"), vec![Ecu {
                id: 0x10,
                pids: pids(&[
                    (0x01, &[0x82, 0x07, 0x65, 0x00]),
                    (0x04, &[0x40]), (0x05, &[0x7B]), (0x06, &[0x80]), (0x07, &[0x8C]),
                    (0x0B, &[0x21]), (0x0C, &[0x0C, 0x80]), (0x0D, &[0x00]), (0x0E, &[0x8C]),
                    (0x0F, &[0x3C]), (0x11, &[0x26]),
                ]),
                stored: vec![0x0420, 0x1457],
                ..Default::default()
            }]),
            "odyssey-2016" => (Bus::Can11, ("6", "ISO 15765-4 (CAN 11/500)"), vec![
                Ecu {
                    id: 0x7E8,
                    pids: pids(&[
                        (0x01, &[0x82, 0x07, 0xE5, 0x04]),
                        (0x04, &[0x50]), (0x05, &[0x5A]), (0x06, &[0x82]), (0x07, &[0x84]),
                        (0x08, &[0x7E]), (0x09, &[0x81]), (0x0C, &[0x0B, 0x40]), (0x0D, &[0x00]),
                        (0x0E, &[0x90]), (0x0F, &[0x40]), (0x10, &[0x01, 0x90]), (0x11, &[0x28]),
                        (0x1F, &[0x02, 0x58]), (0x21, &[0x00, 0x2A]), (0x2F, &[0x80]), (0x30, &[0x05]),
                        (0x31, &[0x01, 0x10]), (0x33, &[0x63]), (0x42, &[0x37, 0x96]), (0x46, &[0x3A]),
                    ]),
                    stored: vec![0x0303, 0x3400],
                    permanent: vec![0x0303],
                    vin: Some("5FNRL5H6XGB000001"),
                    ..Default::default()
                },
                Ecu { id: 0x7E9, pids: pids(&[(0x01, &[0x00, 0x04, 0x00, 0x00]), (0x0D, &[0x00])]), ..Default::default() },
            ]),
            "kona-ev-2020" => (Bus::Can11, ("6", "ISO 15765-4 (CAN 11/500)"), vec![
                Ecu { id: 0x7EB, pids: pids(&[(0x01, &[0x00, 0x00, 0x00, 0x00]), (0x21, &[0, 0]), (0x30, &[0x10]), (0x31, &[0x01, 0x00])]), ..Default::default() },
                Ecu { id: 0x7EC, pids: pids(&[(0x01, &[0x00, 0x00, 0x00, 0x00]), (0x21, &[0, 0]), (0x30, &[0x10]), (0x31, &[0x01, 0x00]), (0x5B, &[0x5D])]), extra: kona::sim_extra(0x7EC), ..Default::default() },
                Ecu { id: 0x7EA, extra: kona::sim_extra(0x7EA), ..Default::default() },
                Ecu { id: 0x7A8, extra: kona::sim_extra(0x7A8), ..Default::default() },
                Ecu { id: 0x7CE, extra: kona::sim_extra(0x7CE), ..Default::default() },
            ]),
            other => bail!("unknown sim vehicle '{other}' (known: {})", VEHICLES.join(", ")),
        };
        Ok(Sim { bus, protocol, ecus, header: 0x7DF, ignition_off: false, ignore_clear: false, ignition_off_after_clear: false, forget_vin_after_clear: false, garble_clear_reply: false, reject_atcra: false, log: Vec::new() })
    }

    /// Make one ECU stop answering everything.
    pub fn silence(&mut self, ecu: u16) {
        self.ecus.retain(|e| e.id != ecu);
    }

    fn engine_running(&self) -> bool {
        self.ecus.iter().any(|e| e.pids.get(&0x0C).is_some_and(|d| d.iter().any(|b| *b != 0)))
    }

    pub fn set_vin(&mut self, ecu: u16, vin: &'static str) {
        if let Some(e) = self.ecus.iter_mut().find(|e| e.id == ecu) {
            e.vin = Some(vin);
        }
    }

    pub fn remove_pid(&mut self, ecu: u16, pid: u8) {
        if let Some(e) = self.ecus.iter_mut().find(|e| e.id == ecu) {
            e.pids.remove(&pid);
        }
    }

    pub fn set_pid(&mut self, ecu: u16, pid: u8, data: &[u8]) {
        if let Some(e) = self.ecus.iter_mut().find(|e| e.id == ecu) {
            e.pids.insert(pid, data.to_vec());
        }
    }

    fn frames(&self, ecu: u16, payload: &[u8]) -> Vec<String> {
        let hex = |b: &[u8]| b.iter().map(|x| format!("{x:02X}")).collect::<Vec<_>>().join(" ");
        match self.bus {
            Bus::Legacy => {
                let mut f = vec![0x48, 0x6B, ecu as u8];
                f.extend_from_slice(payload);
                f.push(f.iter().map(|&b| b as u32).sum::<u32>() as u8);
                vec![hex(&f)]
            }
            _ if payload.len() <= 7 => {
                let mut f = vec![payload.len() as u8];
                f.extend_from_slice(payload);
                vec![format!("{ecu:03X} {}", hex(&f))]
            }
            _ => {
                let mut out = vec![format!("{ecu:03X} 1{:X} {:02X} {}", payload.len() >> 8, payload.len() & 0xFF, hex(&payload[..6]))];
                for (i, chunk) in payload[6..].chunks(7).enumerate() {
                    let mut c = chunk.to_vec();
                    c.resize(7, 0);
                    out.push(format!("{ecu:03X} 2{:X} {}", (i + 1) % 16, hex(&c)));
                }
                out
            }
        }
    }

    fn answer(&self, ecu: &Ecu, req: &[u8], cmd: &str) -> Option<Vec<u8>> {
        let dtcs = |svc: u8, list: &[u16]| {
            let mut v = vec![svc];
            if self.bus.is_can() {
                v.push(list.len() as u8);
            }
            for c in list {
                v.extend_from_slice(&c.to_be_bytes());
            }
            if !self.bus.is_can() {
                v.resize(7, 0);
            }
            v
        };
        match req {
            [0x01, pid] if pid % 0x20 == 0 && !ecu.pids.is_empty() => {
                let mut bitmap = 0u32;
                for p in ecu.pids.keys().filter(|p| **p > *pid && **p <= pid + 0x20) {
                    bitmap |= 1 << (0x20 - (p - pid));
                }
                if ecu.pids.keys().any(|p| *p > pid + 0x20) {
                    bitmap |= 1;
                }
                (bitmap != 0 || *pid == 0).then(|| [vec![0x41, *pid], bitmap.to_be_bytes().to_vec()].concat())
            }
            [0x01, pid] => ecu.pids.get(pid).map(|d| [vec![0x41, *pid], d.clone()].concat()),
            [0x02, 0x02, 0x00] => ecu.stored.first().map(|c| [vec![0x42, 0x02, 0x00], c.to_be_bytes().to_vec()].concat()),
            [0x02, pid, 0x00] if !ecu.stored.is_empty() => ecu.pids.get(pid).map(|d| [vec![0x42, *pid, 0x00], d.clone()].concat()),
            [0x03] if !ecu.pids.is_empty() => Some(dtcs(0x43, &ecu.stored)),
            [0x07] if !ecu.pids.is_empty() => Some(dtcs(0x47, &ecu.pending)),
            [0x0A] if !ecu.pids.is_empty() => Some(dtcs(0x4A, &ecu.permanent)),
            [0x04] if !ecu.pids.is_empty() && !self.ignore_clear => Some(vec![0x44]),
            [0x09, 0x02] => ecu.vin.map(|v| [vec![0x49, 0x02, 0x01], v.as_bytes().to_vec()].concat()),
            _ => ecu.extra.get(cmd).cloned(),
        }
    }
}

impl Transport for Sim {
    fn exchange(&mut self, cmd: &str) -> Result<String> {
        let cmd = cmd.replace(' ', "").to_uppercase();
        self.log.push(cmd.clone());
        let body = match cmd.as_str() {
            "ATZ" | "ATI" => "ELM327 v1.4b".to_string(),
            "STI" => "STN2232 v5.10.3".to_string(),
            "STDI" => "OBDLink EX r1.0".to_string(),
            "ATRV" => if self.engine_running() { "14.1V" } else { "12.6V" }.to_string(),
            "ATDPN" => format!("A{}", self.protocol.0),
            "ATDP" => format!("AUTO, {}", self.protocol.1),
            c if self.reject_atcra && c.starts_with("ATCRA") && c.len() > 5 => "?".to_string(),
            c if c.starts_with("ATSH") => {
                self.header = u16::from_str_radix(&c[4..], 16).unwrap_or(0x7DF);
                "OK".to_string()
            }
            c if c.starts_with("AT") || c.starts_with("ST") => "OK".to_string(),
            _ if self.ignition_off => "SEARCHING...\rUNABLE TO CONNECT".to_string(),
            c => {
                let req: Vec<u8> = (0..c.len() / 2).filter_map(|i| u8::from_str_radix(&c[2 * i..2 * i + 2], 16).ok()).collect();
                if req == [0x04] && self.forget_vin_after_clear {
                    self.ecus.iter_mut().for_each(|e| e.vin = None);
                }
                if req == [0x04] && self.garble_clear_reply {
                    return Ok("7EB 01 44\rCAN ERROR\r\r>".into());
                }
                if req == [0x04] && self.ignition_off_after_clear {
                    self.ignition_off = true;
                    return Ok("7EB 01 44\r7EC 01 44\r\r>".into());
                }
                if req == [0x04] && !self.ignore_clear {
                    for e in &mut self.ecus {
                        e.stored.clear();
                        e.pending.clear();
                    }
                }
                let lines: Vec<String> = self
                    .ecus
                    .iter()
                    .filter(|e| self.header == 0x7DF || !self.bus.is_can() || e.id == self.header + 8)
                    .filter_map(|e| self.answer(e, &req, c).map(|p| self.frames(e.id, &p)))
                    .flatten()
                    .collect();
                if lines.is_empty() { "NO DATA".to_string() } else { lines.join("\r") }
            }
        };
        Ok(format!("{body}\r\r>"))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::Transport;

    #[test]
    fn unknown_command_is_no_data_and_at_is_ok() {
        let mut s = Sim::new("odyssey-2016").unwrap();
        assert_eq!(s.exchange("ATE0").unwrap(), "OK\r\r>");
        assert_eq!(s.exchange("01FF").unwrap(), "NO DATA\r\r>");
        assert_eq!(s.log, vec!["ATE0", "01FF"]);
    }

    #[test]
    fn clear_removes_stored_codes() {
        let mut s = Sim::new("odyssey-2016").unwrap();
        assert!(s.exchange("03").unwrap().contains("43 02"));
        s.exchange("04").unwrap();
        assert!(s.exchange("03").unwrap().contains("43 00"));
    }

    #[test]
    fn unknown_vehicle_is_an_error() {
        assert!(Sim::new("delorean").is_err());
    }

    #[test]
    fn legacy_frames_carry_header_and_checksum() {
        let mut s = Sim::new("crv-2004").unwrap();
        // 48+6B+10+41+05+7B = 0x184 -> checksum 84
        assert_eq!(s.exchange("0105").unwrap(), "48 6B 10 41 05 7B 84\r\r>");
    }

    #[test]
    fn long_can_payload_is_split_into_iso_tp_frames() {
        let mut s = Sim::new("odyssey-2016").unwrap();
        let r = s.exchange("0902").unwrap();
        assert!(r.starts_with("7E8 10 14 49 02 01 "), "{r}");
        assert!(r.contains("\r7E8 21 "), "{r}");
    }
}
