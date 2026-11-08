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
    /// A non-emissions module answering trouble-code requests (UDS 19/14, or KWP 18/14).
    module: Option<ModuleSim>,
    /// Service 06: monitor/test ID → reply payload after the `46`.
    mode06: BTreeMap<u8, Vec<u8>>,
}

pub struct Sim {
    bus: Bus,
    protocol: (&'static str, &'static str),
    ecus: Vec<Ecu>,
    header: u16,
    /// Receive filter set by `ATCRA <id>`; cleared by `ATAR` or a bare `ATCRA`.
    rx_filter: Option<u16>,
    pub ignition_off: bool,
    /// ECUs stay silent on service 04.
    pub ignore_clear: bool,
    pub ignition_off_after_clear: bool,
    pub forget_vin_after_clear: bool,
    /// `ATRV` reply override (e.g. "0.1V" when the adapter is on USB but not in the car).
    pub port_voltage: Option<&'static str>,
    /// Reply to the adapter-identity request (`ATI`).
    pub identity: &'static str,
    /// Replies answered instead of the simulated one, each used once, in order.
    injected: Vec<Injection>,
    /// (command, extra line) appended to the simulated reply, each used once: the command still takes effect.
    appended: Vec<(String, String)>,
    failure: Option<(String, &'static str, usize)>,
    pub log: Vec<String>,
}

/// How a simulated module answers trouble-code requests (wiki `uds-module-dtcs`, `kona-ev-modules`).
#[derive(Clone)]
pub struct ModuleSim {
    /// KWP2000 (`18 00 FF 00` / `14 FF 00`) instead of UDS (`19 02 mask` / `14 FF FF FF`).
    pub kwp: bool,
    /// What a KWP module answers to UDS 0x19 (Kona MCU/OBC: 0x11, VCU: 0x12).
    pub kwp_nrc: u8,
    /// UDS DTC status availability mask.
    pub avail: u8,
    /// Refuses (7F 19 31) a mask containing bits outside `avail`, like the Kona MDPS.
    pub strict: bool,
    /// Serves 0x19 only in a non-default session (7F 19 7F), like the Kona's 7D2.
    pub session_only: bool,
    /// (3-byte DTC, status byte); KWP modules use the first two bytes.
    pub dtcs: Vec<([u8; 3], u8)>,
}

impl Default for ModuleSim {
    fn default() -> Self {
        ModuleSim { kwp: false, kwp_nrc: 0x11, avail: 0xFF, strict: false, session_only: false, dtcs: vec![] }
    }
}

fn uds(avail: u8, dtcs: Vec<([u8; 3], u8)>) -> Option<ModuleSim> {
    Some(ModuleSim { avail, dtcs, ..Default::default() })
}

fn kwp(nrc: u8, dtcs: Vec<([u8; 3], u8)>) -> Option<ModuleSim> {
    Some(ModuleSim { kwp: true, kwp_nrc: nrc, dtcs, ..Default::default() })
}

struct Injection {
    cmd: String,
    reply: String,
    /// Only while this request header is set (11-bit, e.g. 0x7D1).
    header: Option<u16>,
    /// Only once this command has been sent at least once.
    after: Option<String>,
}

pub const VEHICLES: &[&str] = &["crv-2004", "odyssey-2016", "kona-ev-2020"];
// Must name every fault `apply_fault` accepts: tests hold the --port help and SKILL.md to this list.
pub const FAULTS: &[&str] = &["engine-off", "ignition-off", "not-in-car", "can29"];

fn pids(list: &[(u8, &[u8])]) -> BTreeMap<u8, Vec<u8>> {
    list.iter().map(|(p, d)| (*p, d.to_vec())).collect()
}

fn misfire(mid: u8, average: u16, current: u16) -> (u8, Vec<u8>) {
    let rec = |tid: u8, n: u16| [vec![mid, tid, 0x24], n.to_be_bytes().to_vec(), vec![0, 0, 0xFF, 0xFF]].concat();
    (mid, [rec(0x0B, average), rec(0x0C, current)].concat())
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
                    (0x0F, &[0x3C]), (0x11, &[0x26]), (0x03, &[0x02, 0x00]), (0x15, &[0x8C, 0xFF]),
                    (0x24, &[0x80, 0x10, 0x66, 0x00]),
                ]),
                stored: vec![0x0420, 0x1457],
                mode06: [(0x4D, vec![0x4D, 0x81, 0x00, 0x82, 0x00, 0x46]), (0x62, vec![0x62, 0x04, 0x00, 0x20, 0x00, 0x40])].into(),
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
                        (0x03, &[0x02, 0x00]), (0x15, &[0x82, 0xFF]), (0x19, &[0x7A, 0xFF]),
                        (0x34, &[0x80, 0x00, 0x80, 0x00]), (0x38, &[0x7F, 0xC0, 0x80, 0x10]), (0x3C, &[0x1C, 0x52]),
                    ]),
                    stored: vec![0x0303, 0x3400],
                    permanent: vec![0x0303],
                    vin: Some("5FNRL5H60GB000001"),
                    // VCM pattern: misfires on cylinders 1–3 (wiki honda-mode06).
                    mode06: [
                        (0x21, vec![0x21, 0xA9, 0x86, 0x02, 0xE5, 0x02, 0xD0, 0x7F, 0xFF]),
                        (0xA1, vec![0xA1, 0x0C, 0x24, 0x01, 0x86, 0, 0, 0xFF, 0xFF]),
                        misfire(0xA2, 30, 120),
                        misfire(0xA3, 20, 90),
                        misfire(0xA4, 45, 180),
                        misfire(0xA5, 0, 0),
                        misfire(0xA6, 0, 0),
                        misfire(0xA7, 0, 0),
                    ]
                    .into(),
                    ..Default::default()
                },
                Ecu { id: 0x7E9, pids: pids(&[(0x01, &[0x00, 0x04, 0x00, 0x00]), (0x0D, &[0x00])]), ..Default::default() },
            ]),
            // Modules and their protocols as observed on a 1st-generation Kona (wiki kona-ev-modules).
            "kona-ev-2020" => (Bus::Can11, ("6", "ISO 15765-4 (CAN 11/500)"), vec![
                Ecu { id: 0x7EB, pids: pids(&[(0x01, &[0x00, 0x00, 0x00, 0x00]), (0x21, &[0, 0]), (0x30, &[0x10]), (0x31, &[0x01, 0x00])]), module: kwp(0x11, vec![]), ..Default::default() },
                Ecu { id: 0x7EC, pids: pids(&[(0x01, &[0x00, 0x00, 0x00, 0x00]), (0x21, &[0, 0]), (0x30, &[0x10]), (0x31, &[0x01, 0x00]), (0x5B, &[0x5D])]), extra: kona::sim_extra(0x7EC), module: uds(0xFF, vec![]), ..Default::default() },
                Ecu { id: 0x7EA, extra: kona::sim_extra(0x7EA), module: kwp(0x12, vec![]), ..Default::default() },
                Ecu { id: 0x7ED, module: kwp(0x11, vec![]), ..Default::default() },
                Ecu { id: 0x7A8, extra: kona::sim_extra(0x7A8), module: uds(0x09, vec![]), ..Default::default() },
                Ecu { id: 0x7CE, extra: kona::sim_extra(0x7CE), module: uds(0x09, vec![]), ..Default::default() },
                Ecu { id: 0x7D9, module: uds(0x89, vec![([0x56, 0x11, 0x00], 0x08)]), ..Default::default() },
                Ecu { id: 0x7DC, module: Some(ModuleSim { avail: 0x89, strict: true, ..Default::default() }), ..Default::default() },
                Ecu { id: 0x7DA, module: Some(ModuleSim { session_only: true, ..Default::default() }), ..Default::default() },
                Ecu { id: 0x7CC, module: uds(0x89, vec![]), ..Default::default() },
                Ecu { id: 0x778, module: uds(0x09, vec![]), ..Default::default() },
                Ecu { id: 0x7AD, module: uds(0x08, vec![]), ..Default::default() },
                Ecu { id: 0x7BB, module: uds(0x09, vec![]), ..Default::default() },
                Ecu { id: 0x7BE, module: uds(0x7F, vec![]), ..Default::default() },
            ]),
            other => bail!("unknown sim vehicle '{other}' (known: {})", VEHICLES.join(", ")),
        };
        Ok(Sim { bus, protocol, ecus, header: 0x7DF, rx_filter: None, ignition_off: false, ignore_clear: false, ignition_off_after_clear: false, forget_vin_after_clear: false, port_voltage: None, identity: "ELM327 v1.4b", injected: Vec::new(), appended: Vec::new(), failure: None, log: Vec::new() })
    }

    pub fn from_spec(spec: &str) -> Result<Sim> {
        let mut parts = spec.split(':');
        let vehicle = parts.next().unwrap_or_default();
        let mut sim = Sim::new(vehicle)?;
        for fault in parts {
            sim.apply_fault(vehicle, fault)?;
        }
        Ok(sim)
    }

    fn apply_fault(&mut self, vehicle: &str, fault: &str) -> Result<()> {
        match fault {
            "" => {}
            "engine-off" => self.ecus.iter_mut().filter(|e| e.pids.contains_key(&0x0C)).for_each(|e| {
                e.pids.insert(0x0C, vec![0, 0]);
            }),
            "ignition-off" => self.ignition_off = true,
            "can29" if vehicle == "odyssey-2016" => {
                // 2011–2015 Odysseys answer generic OBD on 29-bit CAN: engine 0x11, transmission 0x1D.
                self.bus = Bus::Can29;
                self.protocol = ("7", "ISO 15765-4 (CAN 29/500)");
                for (e, id) in self.ecus.iter_mut().zip([0x11, 0x1D]) {
                    e.id = id;
                }
            }
            "can29" => bail!("the can29 fault only applies to odyssey-2016"),
            "not-in-car" => {
                self.ignition_off = true;
                self.port_voltage = Some("0.1V");
            }
            other => bail!("unknown simulator fault '{other}' (known: {}; can29 only for odyssey-2016)", FAULTS.join(", ")),
        }
        Ok(())
    }

    /// Fail the next `cmd` and the `count - 1` exchanges after it, whatever they are, with error `kind`
    /// (`adapter_timeout`, `adapter_disconnected`).
    pub fn fail(&mut self, cmd: &str, kind: &'static str, count: usize) {
        self.failure = Some((cmd.to_string(), kind, count));
    }

    /// Answer the next `cmd` with `reply` (e.g. `"?"`, `"LV RESET"`, a garbled frame) instead of simulating it.
    pub fn inject(&mut self, cmd: &str, reply: &str) {
        self.injected.push(Injection { cmd: cmd.to_string(), reply: reply.to_string(), header: None, after: None });
    }

    /// Like `inject`, but only for requests to `header`, and only after `after` has been sent.
    pub fn inject_where(&mut self, header: u16, after: Option<&str>, cmd: &str, reply: &str) {
        self.injected.push(Injection { cmd: cmd.to_string(), reply: reply.to_string(), header: Some(header), after: after.map(str::to_string) });
    }

    /// A module at reply ID `id` holding `dtcs`; `kwp` makes it answer KWP2000 instead of UDS.
    pub fn add_module(&mut self, id: u16, kwp: bool, dtcs: &[([u8; 3], u8)]) {
        self.add_module_with(id, ModuleSim { kwp, dtcs: dtcs.to_vec(), ..Default::default() });
    }

    /// Install (or replace) the module behaviour of the ECU answering on `id`.
    pub fn add_module_with(&mut self, id: u16, m: ModuleSim) {
        match self.ecus.iter_mut().find(|e| e.id == id) {
            Some(e) => e.module = Some(m),
            None => self.ecus.push(Ecu { id, module: Some(m), ..Default::default() }),
        }
    }

    /// Process the next `cmd` normally but add `line` to its reply (e.g. a `CAN ERROR` after the real answer).
    pub fn append(&mut self, cmd: &str, line: &str) {
        self.appended.push((cmd.to_string(), line.to_string()));
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

    /// Replace one ECU's stored (confirmed) emissions codes.
    pub fn set_stored(&mut self, ecu: u16, codes: &[u16]) {
        if let Some(e) = self.ecus.iter_mut().find(|e| e.id == ecu) {
            e.stored = codes.to_vec();
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
            bus => {
                let id = if bus == Bus::Can29 { format!("18 DA F1 {:02X}", ecu & 0xFF) } else { format!("{ecu:03X}") };
                if payload.len() <= 7 {
                    let mut f = vec![payload.len() as u8];
                    f.extend_from_slice(payload);
                    return vec![format!("{id} {}", hex(&f))];
                }
                let mut out = vec![format!("{id} 1{:X} {:02X} {}", payload.len() >> 8, payload.len() & 0xFF, hex(&payload[..6]))];
                for (i, chunk) in payload[6..].chunks(7).enumerate() {
                    let mut c = chunk.to_vec();
                    c.resize(7, 0);
                    out.push(format!("{id} 2{:X} {}", (i + 1) % 16, hex(&c)));
                }
                out
            }
        }
    }

    fn answer(&self, ecu: &Ecu, req: &[u8], cmd: &str) -> Vec<Vec<u8>> {
        if let ([0x09, 0x02], Bus::Legacy, Some(v)) = (req, self.bus, ecu.vin) {
            // J1979 non-CAN: 3 zero pad bytes + 17 characters, 4 per message, numbered 1..=5.
            let bytes: Vec<u8> = [0, 0, 0].into_iter().chain(v.bytes()).collect();
            return bytes.chunks(4).enumerate().map(|(i, c)| [vec![0x49, 0x02, i as u8 + 1], c.to_vec()].concat()).collect();
        }
        self.answer_one(ecu, req, cmd).into_iter().collect()
    }

    fn answer_one(&self, ecu: &Ecu, req: &[u8], cmd: &str) -> Option<Vec<u8>> {
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
            [0x06, id] if id % 0x20 == 0 && !ecu.mode06.is_empty() => {
                let mut bitmap = 0u32;
                for m in ecu.mode06.keys().filter(|m| **m > *id && **m <= id.saturating_add(0x20)) {
                    bitmap |= 1 << (0x20 - (m - id));
                }
                if ecu.mode06.keys().any(|m| *m as u16 > *id as u16 + 0x20) {
                    bitmap |= 1;
                }
                let filler = if self.bus.is_can() { vec![] } else { vec![0xFF] };
                Some([vec![0x46, *id], filler, bitmap.to_be_bytes().to_vec()].concat())
            }
            [0x06, id] => ecu.mode06.get(id).map(|p| [vec![0x46], p.clone()].concat()),
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
            // Monitor status: MIL and confirmed-code count follow the stored codes, like a real ECU.
            [0x01, 0x01] => ecu.pids.get(&0x01).map(|d| {
                let a = (ecu.stored.len() as u8 & 0x7F) | if ecu.stored.is_empty() { 0 } else { 0x80 };
                vec![0x41, 0x01, a, d[1], d[2], d[3]]
            }),
            [0x01, pid] => ecu.pids.get(pid).map(|d| [vec![0x41, *pid], d.clone()].concat()),
            [0x02, 0x02, 0x00] => ecu.stored.first().map(|c| [vec![0x42, 0x02, 0x00], c.to_be_bytes().to_vec()].concat()),
            [0x02, pid, 0x00] if !ecu.stored.is_empty() => ecu.pids.get(pid).map(|d| [vec![0x42, *pid, 0x00], d.clone()].concat()),
            [0x03] if !ecu.pids.is_empty() => Some(dtcs(0x43, &ecu.stored)),
            [0x07] if !ecu.pids.is_empty() => Some(dtcs(0x47, &ecu.pending)),
            [0x0A] if !ecu.pids.is_empty() => Some(dtcs(0x4A, &ecu.permanent)),
            [0x04] if !ecu.pids.is_empty() && !self.ignore_clear => Some(vec![0x44]),
            [0x09, 0x02] => ecu.vin.map(|v| [vec![0x49, 0x02, 0x01], v.as_bytes().to_vec()].concat()),
            [0x19, ..] | [0x18, ..] | [0x14, ..] if ecu.module.is_some() => {
                let m = ecu.module.as_ref().expect("checked");
                let list = |svc: Vec<u8>, keep: &dyn Fn(u8) -> bool, three: bool| {
                    let mut v = svc;
                    for (dtc, status) in m.dtcs.iter().filter(|(_, st)| keep(*st)) {
                        v.extend_from_slice(if three { &dtc[..] } else { &dtc[..2] });
                        v.push(*status);
                    }
                    v
                };
                Some(match (req, m.kwp) {
                    ([0x19, ..], true) => vec![0x7F, 0x19, m.kwp_nrc],
                    ([0x19, ..], false) if m.session_only => vec![0x7F, 0x19, 0x7F],
                    ([0x19, 0x02, mask], false) if m.strict && mask & !m.avail != 0 => vec![0x7F, 0x19, 0x31],
                    ([0x19, 0x02, mask], false) => list(vec![0x59, 0x02, m.avail], &|st| st & mask != 0, true),
                    ([0x18, 0x00, 0xFF, 0x00], true) => list(vec![0x58, m.dtcs.len() as u8], &|_| true, false),
                    ([0x18, ..], true) => vec![0x7F, 0x18, 0x12],
                    ([0x18, ..], false) => vec![0x7F, 0x18, 0x11],
                    ([0x14, 0xFF, 0xFF, 0xFF], false) => vec![0x54],
                    ([0x14, 0xFF, 0x00], true) => vec![0x54, 0xFF, 0x00],
                    ([0x14, ..], false) => vec![0x7F, 0x14, 0x13],
                    ([0x14, ..], true) => vec![0x7F, 0x14, 0x79],
                    _ => vec![0x7F, req[0], 0x12],
                })
            }
            _ => ecu.extra.get(cmd).cloned(),
        }
    }
}

impl Transport for Sim {
    fn simulated(&self) -> bool {
        true
    }

    fn exchange(&mut self, cmd: &str) -> Result<String> {
        let cmd = cmd.replace(' ', "").to_uppercase();
        self.log.push(cmd.clone());
        if let Some((from, kind, left)) = &mut self.failure
            && *left > 0
            && (from.is_empty() || *from == cmd)
        {
            from.clear();
            *left -= 1;
            return Err(crate::problem::problem(kind, format!("simulated {kind} on '{cmd}'"), crate::problem::CHECK_USB));
        }
        let log = &self.log;
        let header = self.header;
        if let Some(i) = self.injected.iter().position(|j| {
            j.cmd == cmd && j.header.is_none_or(|h| h == header) && j.after.as_ref().is_none_or(|a| log[..log.len() - 1].contains(a))
        }) {
            let reply = self.injected.remove(i).reply;
            return Ok(format!("{reply}\r\r>"));
        }
        let body = match cmd.as_str() {
            "ATI" => self.identity.to_string(),
            "ATZ" => "ELM327 v1.4b".to_string(),
            "STI" => "STN2232 v5.10.3".to_string(),
            "STDI" => "OBDLink EX r1.0".to_string(),
            "ATRV" => self.port_voltage.unwrap_or(if self.engine_running() { "14.1V" } else { "12.6V" }).to_string(),
            "ATDPN" => format!("A{}", self.protocol.0),
            "ATDP" => format!("AUTO, {}", self.protocol.1),
            "ATAR" | "ATCRA" => {
                self.rx_filter = None;
                "OK".to_string()
            }
            c if c.starts_with("ATCRA") => {
                self.rx_filter = u16::from_str_radix(&c[5..], 16).ok();
                "OK".to_string()
            }
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
                if req == [0x04] && self.ignition_off_after_clear {
                    self.ignition_off = true;
                    return Ok("7EB 01 44\r7EC 01 44\r\r>".into());
                }
                // A module clears only on the request form of its own protocol.
                let target = self.header + 8;
                if let Some(m) = self.ecus.iter_mut().find(|e| e.id == target).and_then(|e| e.module.as_mut())
                    && ((!m.kwp && req == [0x14, 0xFF, 0xFF, 0xFF]) || (m.kwp && req == [0x14, 0xFF, 0x00]))
                {
                    m.dtcs.clear();
                }
                if req == [0x04] && !self.ignore_clear {
                    let first_value_byte = if self.bus.is_can() { 3 } else { 2 };
                    for e in &mut self.ecus {
                        e.stored.clear();
                        e.pending.clear();
                        // A service 04 clear zeroes the service 06 results: value and limits (wiki obd2-mode06).
                        for record in e.mode06.values_mut() {
                            if self.bus.is_can() {
                                record.chunks_mut(9).for_each(|r| r.iter_mut().skip(first_value_byte).for_each(|b| *b = 0));
                            } else {
                                record.iter_mut().skip(first_value_byte).for_each(|b| *b = 0);
                            }
                        }
                        // Clearing restarts the non-continuous self-tests: every supported one becomes incomplete.
                        if let Some(m) = e.pids.get_mut(&0x01) {
                            m[3] = m[2];
                        }
                    }
                }
                let lines: Vec<String> = self
                    .ecus
                    .iter()
                    .filter(|e| self.header == 0x7DF || !self.bus.is_can() || e.id == self.header + 8)
                    .filter(|e| self.rx_filter.is_none_or(|f| e.id == f))
                    .flat_map(|e| self.answer(e, &req, c).into_iter().flat_map(|p| self.frames(e.id, &p)).collect::<Vec<_>>())
                    .collect();
                if lines.is_empty() { "NO DATA".to_string() } else { lines.join("\r") }
            }
        };
        let extra = match self.appended.iter().position(|(c, _)| *c == cmd) {
            Some(i) => format!("\r{}", self.appended.remove(i).1),
            None => String::new(),
        };
        Ok(format!("{body}{extra}\r\r>"))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::Transport;

    #[test]
    fn faults_combine_so_a_29_bit_clear_can_be_rehearsed() {
        let mut a = crate::adapter::Adapter::connect(Sim::from_spec("odyssey-2016:can29:engine-off").unwrap()).unwrap();
        assert_eq!(a.bus(), Bus::Can29);
        let rpm: Vec<f64> = a.query("010C").unwrap().iter().filter_map(|m| crate::obd::decode_pid(0x0C, &m.data[2..]).map(|r| r.value)).collect();
        assert_eq!(rpm, vec![0.0]);
    }

    #[test]
    fn a_clear_resets_the_self_test_results_as_a_real_ecu_does() {
        let mut a = crate::adapter::Adapter::connect(Sim::from_spec("odyssey-2016:engine-off").unwrap()).unwrap();
        a.query("04").unwrap();
        let t = crate::mode06::read(&mut a, Some("odyssey-2016")).unwrap();
        assert!(!t.misfire.is_empty() && t.misfire.iter().all(|m| m.current == Some(0.0) && (m.cylinder == 0 || m.average == Some(0.0))), "{:?}", t.misfire);
        assert!(!t.results.is_empty() && t.results.iter().all(|r| r.verdict == "not_run"), "{:?}", t.results.iter().map(|r| r.verdict).collect::<Vec<_>>());
    }

    #[test]
    fn a_k_line_clear_resets_the_self_test_results_too() {
        let mut a = crate::adapter::Adapter::connect(Sim::from_spec("crv-2004:engine-off").unwrap()).unwrap();
        a.query("04").unwrap();
        let t = crate::mode06::read(&mut a, Some("crv-2004")).unwrap();
        assert!(!t.results.is_empty() && t.results.iter().all(|r| r.verdict == "not_run"), "{:?}", t.results.iter().map(|r| r.verdict).collect::<Vec<_>>());
    }

    #[test]
    fn every_listed_fault_is_accepted_and_named_in_the_skill() {
        let skill = include_str!("../skills/carlink/SKILL.md");
        for fault in FAULTS {
            assert!(Sim::from_spec(&format!("odyssey-2016:{fault}")).is_ok(), "{fault}");
            assert!(skill.contains(&format!(":{fault}")), "SKILL.md does not name :{fault}");
        }
    }

    #[test]
    fn can29_on_another_car_is_refused_as_odyssey_only() {
        let e = Sim::from_spec("kona-ev-2020:can29").err().unwrap().to_string();
        assert!(e.contains("only") && !e.contains("unknown"), "{e}");
    }

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
    fn spec_selects_vehicle_and_fault() {
        let mut s = Sim::from_spec("crv-2004:engine-off").unwrap();
        assert_eq!(s.exchange("010C").unwrap(), "48 6B 10 41 0C 00 00 10\r\r>");
        assert!(Sim::from_spec("crv-2004:flat-tire").is_err());
        assert!(Sim::from_spec("odyssey-2016:ignition-off").unwrap().ignition_off);
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
