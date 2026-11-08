//! An initialised ELM327/STN session over some byte transport.

use crate::elm::{Bus, Message, parse_response};
use anyhow::{Result, bail};
use serde::Serialize;

pub trait Transport {
    /// Send one command (no terminator) and return everything up to and including the `>` prompt.
    fn exchange(&mut self, cmd: &str) -> Result<String>;
}

#[derive(Debug, Clone, Serialize)]
pub struct AdapterInfo {
    pub device: String,
    pub stn: Option<String>,
    pub stn_device: Option<String>,
    pub protocol: String,
    pub protocol_number: String,
}

pub struct Adapter<T: Transport> {
    t: T,
    bus: Bus,
    info: AdapterInfo,
}

const INIT: &[&str] = &["ATZ", "ATE0", "ATL0", "ATS1", "ATH1", "ATCAF1", "ATAT1", "ATSP0"];

fn clean(s: &str) -> String {
    s.replace('>', "").split(['\r', '\n']).map(str::trim).filter(|l| !l.is_empty()).collect::<Vec<_>>().join(" ")
}

impl<T: Transport> Adapter<T> {
    pub fn connect(mut t: T) -> Result<Self> {
        for cmd in INIT {
            t.exchange(cmd)?;
        }
        let device = clean(&t.exchange("ATI")?);
        let stn = Some(clean(&t.exchange("STI")?)).filter(|s| s != "?");
        let stn_device = Some(clean(&t.exchange("STDI")?)).filter(|s| s != "?");
        let probe = t.exchange("0100")?;
        if probe.contains("UNABLE TO CONNECT") || probe.contains("NO DATA") || probe.contains("ERROR") {
            bail!("adapter found but the vehicle did not answer ({}); is the ignition ON (engine running, or EV in READY)?", clean(&probe));
        }
        let protocol_number = clean(&t.exchange("ATDPN")?);
        let protocol = clean(&t.exchange("ATDP")?).trim_start_matches("AUTO, ").to_string();
        let bus = Bus::from_protocol(protocol_number.chars().last().unwrap_or('0'));
        Ok(Adapter { t, bus, info: AdapterInfo { device, stn, stn_device, protocol, protocol_number } })
    }

    pub fn bus(&self) -> Bus {
        self.bus
    }

    pub fn info(&self) -> &AdapterInfo {
        &self.info
    }

    pub fn transport(&self) -> &T {
        &self.t
    }

    pub fn transport_mut(&mut self) -> &mut T {
        &mut self.t
    }

    pub fn query(&mut self, cmd: &str) -> Result<Vec<Message>> {
        parse_response(&self.t.exchange(cmd)?, self.bus)
    }

    /// Battery voltage at the OBD port, as measured by the adapter.
    pub fn voltage(&mut self) -> Result<Option<f64>> {
        Ok(clean(&self.t.exchange("ATRV")?).trim_end_matches('V').parse().ok())
    }

    /// Run `f` with requests addressed physically to 11-bit `header`, listening only to its reply ID
    /// (header + 8), then restore functional addressing and the default receive filter.
    pub fn with_header<R>(&mut self, header: &str, f: impl FnOnce(&mut Self) -> Result<R>) -> Result<R> {
        if self.bus != Bus::Can11 {
            bail!("physical addressing is only supported on 11-bit CAN; this vehicle uses {:?}", self.bus);
        }
        let id = u16::from_str_radix(header, 16).ok().filter(|id| *id <= 0x7F7).ok_or_else(|| anyhow::anyhow!("bad 11-bit CAN header '{header}'"))?;
        let mut setup = Ok(());
        for cmd in [format!("ATSH{header}"), format!("ATCRA{:03X}", id + 8)] {
            match self.t.exchange(&cmd) {
                Ok(r) if clean(&r) == "?" => setup = Err(anyhow::anyhow!("adapter rejected {cmd}")),
                Err(e) => setup = Err(e),
                Ok(_) => continue,
            }
            break;
        }
        let r = setup.and_then(|_| f(self));
        // Both restores are always attempted: a stale header would silently misroute every later request.
        let restore = [self.t.exchange("ATCRA"), self.t.exchange("ATSH7DF")];
        let r = r?;
        for x in restore {
            x.map_err(|e| anyhow::anyhow!("could not restore adapter addressing: {e}"))?;
        }
        Ok(r)
    }
}

impl Transport for Box<dyn Transport> {
    fn exchange(&mut self, cmd: &str) -> Result<String> {
        (**self).exchange(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::Sim;

    #[test]
    fn init_detects_can_bus_and_device_info() {
        let mut a = Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap();
        assert_eq!(a.bus(), Bus::Can11);
        assert!(a.info().device.contains("ELM327"));
        assert!(a.info().stn.as_deref().unwrap().starts_with("STN"));
        assert_eq!(a.info().protocol, "ISO 15765-4 (CAN 11/500)");
        let log = a.transport().log.join(" ");
        assert!(log.starts_with("ATZ ATE0 ATL0 ATS1 ATH1 ATCAF1 ATAT1 ATSP0"), "{log}");
        assert!(a.query("0105").unwrap()[0].data.starts_with(&[0x41, 0x05]));
    }

    #[test]
    fn init_detects_legacy_bus() {
        let a = Adapter::connect(Sim::new("crv-2004").unwrap()).unwrap();
        assert_eq!(a.bus(), Bus::Legacy);
    }

    #[test]
    fn voltage_is_parsed() {
        let mut a = Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap();
        assert_eq!(a.voltage().unwrap(), Some(12.6));
    }

    #[test]
    fn unable_to_connect_fails_init_with_hint() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.ignition_off = true;
        let err = Adapter::connect(sim).err().unwrap().to_string();
        assert!(err.contains("ignition"), "{err}");
    }

    #[test]
    fn with_header_refuses_non_11_bit_bus() {
        let mut a = Adapter::connect(Sim::new("crv-2004").unwrap()).unwrap();
        let err = a.with_header("7E4", |a| a.query("220101")).unwrap_err().to_string();
        assert!(err.contains("11-bit CAN"), "{err}");
        assert!(!a.transport().log.iter().any(|c| c.starts_with("ATSH")));
    }

    #[test]
    fn rejected_receive_filter_restores_addressing_and_errors() {
        let mut sim = Sim::new("kona-ev-2020").unwrap();
        sim.reject_atcra = true;
        let mut a = Adapter::connect(sim).unwrap();
        let err = a.with_header("7E4", |a| a.query("220101")).unwrap_err().to_string();
        assert!(err.contains("rejected ATCRA7EC"), "{err}");
        let log = a.transport().log.join(" ");
        assert!(log.ends_with("ATSH7E4 ATCRA7EC ATCRA ATSH7DF"), "{log}");
    }

    #[test]
    fn with_header_sets_and_restores_header() {
        let mut a = Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap();
        let r = a.with_header("7E4", |a| a.query("220101")).unwrap();
        assert_eq!(r[0].ecu, "7EC");
        let log = a.transport().log.join(" ");
        assert!(log.contains("ATSH7E4 ATCRA7EC 220101 ATCRA ATSH7DF"), "{log}");
    }
}
