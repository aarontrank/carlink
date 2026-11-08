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

/// `58 <count> [hi lo status]…`. The status byte is manufacturer-specific; Hyundai's (wiki
/// `uds-module-dtcs` §3.1): 0x40 failing now, 0x20 stored, 0x80 present during this drive cycle.
pub fn parse_kwp_dtcs(data: &[u8]) -> Result<Vec<ModuleDtc>> {
    let Some(records) = data.strip_prefix(&[0x58][..]).and_then(|r| r.get(1..)) else {
        bail!("not a KWP2000 DTC list (expected 58)");
    };
    Ok(records
        .as_chunks::<3>().0.iter()
        .filter_map(|r| {
            let code = decode_dtc(r[0], r[1])?;
            let st = r[2];
            let state = if st & 0x40 != 0 {
                "active"
            } else if st & 0x20 != 0 {
                "stored"
            } else if st & 0x80 != 0 {
                "pending"
            } else {
                "not_failed"
            };
            let names = [(0x80, "present_this_drive"), (0x40, "failing_now"), (0x20, "stored"), (0x08, "test_inhibited"), (0x04, "test_conditions_met")];
            let status = names.iter().filter(|(b, _)| st & b != 0).map(|(_, n)| *n).collect();
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
    /// False for modules carlink only reads (battery evidence, airbag, smart key: wiki `uds-module-dtcs` §8).
    pub clearable: bool,
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
    /// Set when only part of the list could be read (codes in `dtcs` are real; some may be missing).
    pub partial: Option<String>,
    /// The module answered but refused the request (with the reason). Not a failed read.
    pub refused: Option<String>,
    /// A read that failed (as opposed to a module that did not answer or refused).
    pub error: Option<String>,
    /// Whether carlink may clear this module (see `Module::clearable`).
    pub clearable: bool,
}

/// What one protocol's read produced.
enum Outcome {
    /// Codes, and a note when only part of the list could be read.
    Codes(Vec<ModuleDtc>, Option<String>),
    /// The module does not speak this protocol (`7F <service> 11/12`): try the other one.
    NotThisProtocol,
    Refused(String),
    /// Nothing answered this request.
    Silent,
}

fn refusal(nrc: u8) -> String {
    match nrc {
        0x7F => "this module serves its codes only in a diagnostic session carlink does not open; use the factory tool".into(),
        n => crate::actions::nrc_meaning(n).into(),
    }
}

/// Classify the replies to a request for `service`: the positive reply (first byte `service + 0x40`),
/// or what stands in for it.
fn classify(msgs: &[crate::elm::Message], service: u8) -> std::result::Result<Vec<u8>, Outcome> {
    if let Some(p) = msgs.iter().find(|m| m.data.first() == Some(&(service + 0x40))) {
        return Ok(p.data.clone());
    }
    Err(match msgs.iter().find_map(|m| negative(&m.data).filter(|(sid, _)| *sid == service)) {
        Some((_, 0x11 | 0x12)) => Outcome::NotThisProtocol,
        Some((_, nrc)) => Outcome::Refused(refusal(nrc)),
        None if msgs.is_empty() => Outcome::Silent,
        None => Outcome::Refused("unexpected reply".into()),
    })
}

/// UDS (wiki `uds-module-dtcs` §2): `19 02 08` (confirmed codes, accepted by every Kona module), then
/// `19 02 <supported failing/pending bits>` — never 0x10/0x40 ("test not run"). If that second request
/// fails, the confirmed codes are kept and the gap is noted.
fn read_uds<T: Transport>(a: &mut Adapter<T>) -> Result<Outcome> {
    let data = match classify(&a.query("190208")?, 0x19) {
        Ok(d) => d,
        Err(o) => return Ok(o),
    };
    let mut dtcs = parse_uds_dtcs(&data)?;
    let extra = data.get(2).map(|avail| avail & 0xAF & !0x08).unwrap_or(0);
    if extra == 0 {
        return Ok(Outcome::Codes(dtcs, None));
    }
    let more = a.query(&format!("1902{:02X}", extra | 0x08)).ok().and_then(|m| classify(&m, 0x19).ok()).and_then(|d| parse_uds_dtcs(&d).ok());
    let Some(more) = more else {
        return Ok(Outcome::Codes(dtcs, Some("only confirmed codes could be read; the request for failing and pending codes was not answered".into())));
    };
    for x in more {
        if !dtcs.iter().any(|y| y.code == x.code) {
            dtcs.push(x);
        }
    }
    Ok(Outcome::Codes(dtcs, None))
}

/// KWP2000 as Hyundai uses it (wiki `uds-module-dtcs` §3): `18 00 FF 00`.
fn read_kwp<T: Transport>(a: &mut Adapter<T>) -> Result<Outcome> {
    Ok(match classify(&a.query("1800FF00")?, 0x18) {
        Ok(d) => Outcome::Codes(parse_kwp_dtcs(&d)?, None),
        Err(o) => o,
    })
}

/// Read one module's trouble codes, trying its declared protocol first and the other one when the
/// module says it does not support the first, or stays silent (wiki `uds-module-dtcs` §1: decide from
/// the reply, not the address).
pub fn read_module<T: Transport>(a: &mut Adapter<T>, m: &Module) -> ModuleReport {
    let mut report = ModuleReport { name: m.name, description: m.description, request: m.request, responded: false, protocol: None, dtcs: vec![], partial: None, refused: None, error: None, clearable: m.clearable };
    let order = match m.protocol {
        Protocol::Uds => [Protocol::Uds, Protocol::Kwp],
        Protocol::Kwp => [Protocol::Kwp, Protocol::Uds],
    };
    let result = a.with_header(m.request, |a| -> Result<()> {
        for (i, protocol) in order.into_iter().enumerate() {
            let outcome = match protocol {
                Protocol::Uds => read_uds(a)?,
                Protocol::Kwp => read_kwp(a)?,
            };
            match outcome {
                Outcome::Codes(d, note) => {
                    report.responded = true;
                    report.protocol = Some(protocol);
                    report.dtcs = d;
                    report.partial = note;
                    return Ok(());
                }
                Outcome::Refused(why) => {
                    report.responded = true;
                    report.protocol = Some(protocol);
                    report.refused = Some(why);
                    return Ok(());
                }
                Outcome::NotThisProtocol if i == 0 => report.responded = true,
                Outcome::NotThisProtocol => {
                    report.refused = Some("the module refused both trouble-code services (UDS and KWP2000)".into());
                    return Ok(());
                }
                Outcome::Silent if i == 0 => {}
                Outcome::Silent if report.responded => {
                    report.error = Some(format!("the module answered but gave no reply to the {protocol:?} trouble-code request"));
                    return Ok(());
                }
                Outcome::Silent => return Ok(()),
            }
        }
        Ok(())
    });
    if let Err(e) = result {
        report.error = Some(format!("{e:#}"));
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
    fn kwp_dtc_list_uses_hyundais_status_bits() {
        // bench VCU (wiki uds-module-dtcs): 58 04 1A 78 E8 C1 29 E4 … → P1A78 (E8: failing now + stored)
        let d = parse_kwp_dtcs(&[0x58, 0x02, 0x1A, 0x78, 0xE8, 0xC1, 0x29, 0x20]).unwrap();
        assert_eq!((d[0].code.as_str(), d[0].state), ("P1A78", "active"));
        assert_eq!((d[1].code.as_str(), d[1].state), ("U0129", "stored"));
        assert!(!d[0].status.contains(&"warning_lamp"), "bit 7 is not a warning lamp on Hyundai");
        assert_eq!(parse_kwp_dtcs(&[0x58, 0x01, 0x51, 0x01, 0x80]).unwrap()[0].state, "pending");
        assert_eq!(parse_kwp_dtcs(&[0x58, 0x01, 0x51, 0x01, 0x08]).unwrap()[0].state, "not_failed");
    }

    const TEST_UDS: Module = Module { name: "IEB", description: "brakes", request: "7D1", protocol: Protocol::Uds, confidence: "verified", clearable: true };
    const TEST_KWP: Module = Module { name: "KWP", description: "kwp module", request: "7D3", protocol: Protocol::Uds, confidence: "verified", clearable: true };
    const TEST_ABSENT: Module = Module { name: "NONE", description: "not fitted", request: "7D6", protocol: Protocol::Uds, confidence: "verified", clearable: true };

    pub(crate) fn adapter_with_modules() -> Adapter<crate::sim::Sim> {
        let mut sim = crate::sim::Sim::new("kona-ev-2020").unwrap();
        sim.add_module(0x7D9, false, &[([0x56, 0x11, 0x00], 0x09)]);
        sim.add_module(0x7DB, true, &[([0x96, 0x20, 0x00], 0x20)]);
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
        assert_eq!(r.protocol, Some(Protocol::Kwp));
        assert!(a.transport().log.join(" ").contains("190208 1800FF00"));
    }

    #[test]
    fn confirmed_codes_first_then_only_the_supported_failing_and_pending_bits() {
        let mut sim = crate::sim::Sim::new("kona-ev-2020").unwrap();
        // strict like the Kona MDPS: availability 89, refuses masks with other bits
        sim.add_module_with(0x7DE, crate::sim::ModuleSim { avail: 0x89, strict: true, dtcs: vec![([0x58, 0x12, 0x87], 0x08), ([0x58, 0x13, 0x00], 0x01)], ..Default::default() });
        let mut a = Adapter::connect(sim).unwrap();
        let m = Module { name: "X", description: "x", request: "7D6", protocol: Protocol::Uds, confidence: "verified", clearable: true };
        let r = read_module(&mut a, &m);
        assert!(r.refused.is_none() && r.error.is_none(), "{r:?}");
        assert_eq!(r.dtcs.iter().map(|d| d.code.as_str()).collect::<Vec<_>>(), vec!["C1812-87", "C1813-00"]);
        assert!(a.transport().log.join(" ").contains("190208 190289"), "{}", a.transport().log.join(" "));
    }

    #[test]
    fn kwp_fallback_also_on_sub_function_not_supported() {
        let mut sim = crate::sim::Sim::new("kona-ev-2020").unwrap();
        sim.add_module_with(0x7DE, crate::sim::ModuleSim { kwp: true, kwp_nrc: 0x12, dtcs: vec![([0x1A, 0x78, 0x00], 0x60)], ..Default::default() });
        let mut a = Adapter::connect(sim).unwrap();
        let m = Module { name: "X", description: "x", request: "7D6", protocol: Protocol::Uds, confidence: "verified", clearable: true };
        assert_eq!(read_module(&mut a, &m).dtcs[0].code, "P1A78");
    }

    #[test]
    fn a_module_declared_kwp_that_speaks_uds_is_read_as_uds() {
        let mut sim = crate::sim::Sim::new("kona-ev-2020").unwrap();
        sim.add_module_with(0x7DE, crate::sim::ModuleSim { avail: 0x09, dtcs: vec![([0x56, 0x11, 0x00], 0x08)], ..Default::default() });
        let mut a = Adapter::connect(sim).unwrap();
        let m = Module { name: "X", description: "x", request: "7D6", protocol: Protocol::Kwp, confidence: "verified", clearable: true };
        let r = read_module(&mut a, &m);
        assert_eq!((r.protocol, r.dtcs.len()), (Some(Protocol::Uds), 1), "{r:?}");
    }

    #[test]
    fn a_kwp_module_that_goes_silent_is_an_error_not_clean() {
        let mut sim = crate::sim::Sim::new("kona-ev-2020").unwrap();
        sim.add_module_with(0x7DE, crate::sim::ModuleSim { kwp: true, kwp_nrc: 0x12, ..Default::default() });
        sim.inject_where(0x7D6, None, "1800FF00", "NO DATA");
        let mut a = Adapter::connect(sim).unwrap();
        let m = Module { name: "X", description: "x", request: "7D6", protocol: Protocol::Uds, confidence: "verified", clearable: true };
        let r = read_module(&mut a, &m);
        assert!(r.error.is_some() && r.dtcs.is_empty(), "{r:?}");
    }

    #[test]
    fn a_failed_second_read_keeps_the_confirmed_codes_and_notes_the_gap() {
        for bad in ["7DE 03 7F 19 31", "NO DATA", "7DE 02 59 07"] {
            let mut sim = crate::sim::Sim::new("kona-ev-2020").unwrap();
            sim.add_module_with(0x7DE, crate::sim::ModuleSim { avail: 0x09, dtcs: vec![([0x56, 0x11, 0x00], 0x08)], ..Default::default() });
            sim.inject_where(0x7D6, None, "190209", bad);
            let mut a = Adapter::connect(sim).unwrap();
            let m = Module { name: "X", description: "x", request: "7D6", protocol: Protocol::Uds, confidence: "verified", clearable: true };
            let r = read_module(&mut a, &m);
            assert_eq!(r.dtcs.len(), 1, "{bad}: {r:?}");
            assert!(r.partial.as_deref().unwrap().contains("only confirmed codes"), "{bad}: {r:?}");
            assert!(r.refused.is_none() && r.error.is_none());
        }
    }

    #[test]
    fn a_module_silent_on_its_declared_protocol_is_tried_on_the_other() {
        let mut sim = crate::sim::Sim::new("kona-ev-2020").unwrap();
        sim.add_module_with(0x7DE, crate::sim::ModuleSim { avail: 0x08, dtcs: vec![([0x56, 0x11, 0x00], 0x08)], ..Default::default() });
        sim.inject_where(0x7D6, None, "1800FF00", "NO DATA");
        let mut a = Adapter::connect(sim).unwrap();
        let m = Module { name: "X", description: "x", request: "7D6", protocol: Protocol::Kwp, confidence: "verified", clearable: true };
        assert_eq!(read_module(&mut a, &m).dtcs.len(), 1);
    }

    #[test]
    fn a_negative_reply_for_another_service_does_not_switch_protocol() {
        let mut sim = crate::sim::Sim::new("kona-ev-2020").unwrap();
        sim.add_module_with(0x7DE, crate::sim::ModuleSim { avail: 0x08, dtcs: vec![], ..Default::default() });
        sim.inject_where(0x7D6, None, "190208", "7DE 03 7F 3E 11\r7DE 03 59 02 08");
        let mut a = Adapter::connect(sim).unwrap();
        let m = Module { name: "X", description: "x", request: "7D6", protocol: Protocol::Uds, confidence: "verified", clearable: true };
        assert_eq!(read_module(&mut a, &m).protocol, Some(Protocol::Uds));
    }

    #[test]
    fn a_module_that_serves_codes_only_in_another_session_says_so() {
        let mut sim = crate::sim::Sim::new("kona-ev-2020").unwrap();
        sim.add_module_with(0x7DE, crate::sim::ModuleSim { session_only: true, ..Default::default() });
        let mut a = Adapter::connect(sim).unwrap();
        let m = Module { name: "X", description: "x", request: "7D6", protocol: Protocol::Uds, confidence: "verified", clearable: false };
        let r = read_module(&mut a, &m);
        assert!(r.refused.as_deref().unwrap().contains("diagnostic session"), "{r:?}");
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
