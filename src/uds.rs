//! Trouble codes in non-emissions modules: UDS (ISO 14229) service 19/14 and KWP2000 (ISO 14230) 18/14.

use crate::adapter::{Adapter, Transport};
use crate::obd::decode_dtc;
use anyhow::{Result, bail};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ModuleDtc {
    /// `C1611-00` (UDS: code + failure-type byte) or `C1101` (KWP: no failure type).
    pub code: String,
    /// One of: active (failing now), stored (confirmed), pending, history (failed since the last clear), not_failed.
    pub state: &'static str,
    pub status: Vec<&'static str>,
}

const UDS_STATUS: [&str; 8] = [
    "test_failed", "failed_this_cycle", "pending", "confirmed",
    "not_completed_since_clear", "failed_since_clear", "not_completed_this_cycle", "warning_lamp",
];

/// UDS status byte → the single word a technician needs.
pub fn state(status: u8) -> &'static str {
    if status & 0x01 != 0 {
        "active"
    } else if status & 0x08 != 0 {
        "stored"
    } else if status & 0x04 != 0 {
        "pending"
    } else if status & 0x20 != 0 {
        "history"
    } else {
        "not_failed"
    }
}

/// `59 02 <availability mask> [hi mid lo status]…`
pub fn parse_uds_dtcs(data: &[u8]) -> Result<Vec<ModuleDtc>> {
    let Some(records) = data.strip_prefix(&[0x59, 0x02][..]).and_then(|r| r.get(1..)) else {
        bail!("not a UDS DTC list (expected 59 02)");
    };
    Ok(records
        .as_chunks::<4>().0.iter()
        .filter_map(|r| {
            let code = decode_dtc(r[0], r[1])?;
            Some(ModuleDtc {
                code: format!("{code}-{:02X}", r[2]),
                state: state(r[3]),
                status: (0..8).filter(|b| r[3] & (1 << b) != 0).map(|b| UDS_STATUS[b]).collect(),
            })
        })
        .collect())
}

/// `58 <count> [hi lo status]…`; status bits 6–5 are the storage state, bit 7 the warning lamp.
pub fn parse_kwp_dtcs(data: &[u8]) -> Result<Vec<ModuleDtc>> {
    let Some(records) = data.strip_prefix(&[0x58][..]).and_then(|r| r.get(1..)) else {
        bail!("not a KWP2000 DTC list (expected 58)");
    };
    Ok(records
        .as_chunks::<3>().0.iter()
        .filter_map(|r| {
            let code = decode_dtc(r[0], r[1])?;
            let state = match (r[2] >> 5) & 0x03 {
                3 => "active",
                2 => "pending",
                1 => "stored",
                _ => "not_failed",
            };
            let status = if r[2] & 0x80 != 0 { vec!["warning_lamp"] } else { vec![] };
            Some(ModuleDtc { code, state, status })
        })
        .collect())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Uds,
    Kwp,
}

/// A diagnosable module reached by 11-bit physical addressing (reply ID = request + 8).
#[derive(Debug, Serialize)]
pub struct Module {
    pub name: &'static str,
    pub description: &'static str,
    pub request: &'static str,
    pub protocol: Protocol,
    /// "verified" (documented for this car), "sibling" (seen on a closely related model) — see the wiki.
    pub confidence: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ModuleReport {
    pub name: &'static str,
    pub description: &'static str,
    pub request: &'static str,
    /// False when nothing answered: not fitted, asleep, or a wrong address. Not an error by itself.
    pub responded: bool,
    pub dtcs: Vec<ModuleDtc>,
    /// The protocol the module answered on (UDS, or KWP2000 after a fallback); None if it did not answer.
    pub protocol: Option<Protocol>,
    /// The module answered but refused the request (with the reason). Not a failed read.
    pub refused: Option<String>,
    /// A read that failed (as opposed to a module that did not answer or refused).
    pub error: Option<String>,
}

fn dtc_request(protocol: Protocol) -> &'static str {
    match protocol {
        Protocol::Uds => "1902FF",
        Protocol::Kwp => "1802FF00",
    }
}

fn parse_reply(protocol: Protocol, data: &[u8]) -> Result<Vec<ModuleDtc>> {
    match protocol {
        Protocol::Uds => parse_uds_dtcs(data),
        Protocol::Kwp => parse_kwp_dtcs(data),
    }
}

/// Read one module's trouble codes. A UDS module that says "service not supported" is retried as KWP2000.
pub fn read_module<T: Transport>(a: &mut Adapter<T>, m: &Module) -> ModuleReport {
    let mut report = ModuleReport { name: m.name, description: m.description, request: m.request, responded: false, protocol: None, dtcs: vec![], refused: None, error: None };
    let result = a.with_header(m.request, |a| {
        let mut protocol = m.protocol;
        let mut msgs = a.query(dtc_request(protocol))?;
        if protocol == Protocol::Uds && msgs.iter().any(|x| negative(&x.data) == Some((0x19, 0x11))) {
            protocol = Protocol::Kwp;
            msgs = a.query(dtc_request(protocol))?;
        }
        Ok((protocol, msgs))
    });
    match result {
        Ok((protocol, msgs)) => {
            report.responded = !msgs.is_empty();
            report.protocol = report.responded.then_some(protocol);
            let positive = msgs.iter().find(|x| negative(&x.data).is_none());
            match (positive, msgs.iter().find_map(|x| negative(&x.data))) {
                (Some(p), _) => match parse_reply(protocol, &p.data) {
                    Ok(d) => report.dtcs = d,
                    Err(e) => report.error = Some(e.to_string()),
                },
                (None, Some((_, nrc))) => report.refused = Some(crate::actions::nrc_meaning(nrc).to_string()),
                (None, None) => {}
            }
        }
        Err(e) => report.error = Some(format!("{e:#}")),
    }
    report
}

/// `7F <service> <code>` → (service, negative response code).
pub fn negative(data: &[u8]) -> Option<(u8, u8)> {
    match data {
        [0x7F, sid, nrc, ..] => Some((*sid, *nrc)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uds_dtc_list_decodes_code_failure_type_and_status() {
        // 59 02 <availability> then C1611-00 confirmed+testFailed, B1620-17 failed-since-clear only
        let d = parse_uds_dtcs(&[0x59, 0x02, 0xFF, 0x56, 0x11, 0x00, 0x09, 0x96, 0x20, 0x17, 0x20]).unwrap();
        assert_eq!(d[0].code, "C1611-00");
        assert_eq!(d[0].state, "active");
        assert_eq!(d[1].code, "B1620-17");
        assert_eq!(d[1].state, "history");
        assert!(d[0].status.contains(&"confirmed"));
    }

    #[test]
    fn uds_status_states() {
        assert_eq!(state(0x08), "stored");
        assert_eq!(state(0x04), "pending");
        assert_eq!(state(0x01), "active");
        assert_eq!(state(0x20), "history");
        assert_eq!(state(0x50), "not_failed");
    }

    #[test]
    fn empty_uds_list_and_wrong_service_are_handled() {
        assert!(parse_uds_dtcs(&[0x59, 0x02, 0xFF]).unwrap().is_empty());
        assert!(parse_uds_dtcs(&[0x62, 0x01]).is_err());
    }

    #[test]
    fn kwp_dtc_list_decodes_two_byte_codes() {
        // 58 <count> then C1101 status 0xA0: storage state 01 (not present now), warning lamp on
        let d = parse_kwp_dtcs(&[0x58, 0x01, 0x51, 0x01, 0xA0]).unwrap();
        assert_eq!(d[0].code, "C1101");
        assert_eq!(d[0].state, "stored");
        assert!(d[0].status.contains(&"warning_lamp"));
        assert_eq!(parse_kwp_dtcs(&[0x58, 0x01, 0x51, 0x01, 0x60]).unwrap()[0].state, "active");
    }

    const TEST_UDS: Module = Module { name: "IEB", description: "brakes", request: "7D1", protocol: Protocol::Uds, confidence: "verified" };
    const TEST_KWP: Module = Module { name: "AIRBAG", description: "airbags", request: "7D2", protocol: Protocol::Uds, confidence: "verified" };
    const TEST_ABSENT: Module = Module { name: "NONE", description: "not fitted", request: "7D6", protocol: Protocol::Uds, confidence: "verified" };

    pub(crate) fn adapter_with_modules() -> Adapter<crate::sim::Sim> {
        let mut sim = crate::sim::Sim::new("kona-ev-2020").unwrap();
        sim.add_module(0x7D9, false, &[([0x56, 0x11, 0x00], 0x09)]);
        sim.add_module(0x7DA, true, &[([0x96, 0x20, 0x00], 0xA0)]);
        Adapter::connect(sim).unwrap()
    }

    #[test]
    fn reads_a_uds_module() {
        let mut a = adapter_with_modules();
        let r = read_module(&mut a, &TEST_UDS);
        assert!(r.responded && r.error.is_none());
        assert_eq!(r.dtcs[0].code, "C1611-00");
    }

    #[test]
    fn falls_back_to_kwp_when_uds_is_not_supported() {
        let mut a = adapter_with_modules();
        let r = read_module(&mut a, &TEST_KWP);
        assert_eq!(r.dtcs[0].code, "B1620");
        assert!(a.transport().log.join(" ").contains("1902FF 1802FF00"));
    }

    #[test]
    fn absent_module_is_not_an_error() {
        let mut a = adapter_with_modules();
        let r = read_module(&mut a, &TEST_ABSENT);
        assert!(!r.responded && r.error.is_none() && r.dtcs.is_empty());
    }

    #[test]
    fn negative_response_is_described() {
        assert_eq!(negative(&[0x7F, 0x19, 0x11]), Some((0x19, 0x11)));
        assert_eq!(negative(&[0x59, 0x02]), None);
    }
}
