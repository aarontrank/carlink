//! An initialised ELM327/STN session over some byte transport.

use crate::elm::{AdapterError, Bus, Message, parse_response};
use crate::problem::{CHECK_IGNITION, problem};
use anyhow::{Result, bail};
use serde::Serialize;

pub trait Transport {
    /// Send one command (no terminator) and return everything up to and including the `>` prompt.
    fn exchange(&mut self, cmd: &str) -> Result<String>;

    /// Abort anything the adapter may still be doing from an earlier session and discard its output.
    fn interrupt(&mut self) {}
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
    /// Header of the module currently addressed physically (inside `with_header`).
    physical: Option<String>,
    /// Set while the adapter may not be in functional addressing; cleared only by a verified restore.
    dirty: bool,
}

const SETUP: &[&str] = &["ATE0", "ATL0", "ATS1", "ATH1", "ATCAF1", "ATAT1", "ATSP0"];

/// Services that only read. Requests for anything else (e.g. 04/14 clears) are never retried
/// automatically, and `raw` refuses them.
pub const READ_SERVICES: &[u8] = &[0x01, 0x02, 0x03, 0x06, 0x07, 0x09, 0x0A, 0x18, 0x19, 0x21, 0x22];

fn clean(s: &str) -> String {
    s.replace('>', "").split(['\r', '\n']).map(str::trim).filter(|l| !l.is_empty()).collect::<Vec<_>>().join(" ")
}

fn expect_ok<T: Transport>(t: &mut T, cmd: &str) -> Result<()> {
    let r = clean(&t.exchange(cmd)?);
    if !r.ends_with("OK") {
        bail!("adapter rejected {cmd} (answered {r:?})");
    }
    Ok(())
}

fn is_read(cmd: &str) -> bool {
    u8::from_str_radix(cmd.get(..2).unwrap_or(""), 16).is_ok_and(|sid| READ_SERVICES.contains(&sid))
}

fn adapter_line(e: &anyhow::Error) -> Option<&str> {
    e.downcast_ref::<AdapterError>().map(|a| a.line.as_str())
}

impl<T: Transport> Adapter<T> {
    /// Identify and configure the adapter without talking to the car.
    pub fn open(mut t: T) -> Result<Self> {
        t.interrupt();
        let mut id = clean(&t.exchange("ATI")?);
        if id.contains("STOPPED") {
            id = clean(&t.exchange("ATI")?);
        }
        if !["ELM327", "STN", "OBDLink"].iter().any(|k| id.contains(k)) {
            return Err(problem(
                "not_an_obd_adapter",
                format!("the device on this port does not answer like an ELM327/OBDLink adapter (ATI answered {id:?})"),
                "Run `carlink ports`, find the OBDLink EX (usb 0403:6015) and pass it with --port.",
            ));
        }
        let banner = clean(&t.exchange("ATZ")?);
        if !["ELM327", "STN", "OBDLink"].iter().any(|k| banner.contains(k)) {
            return Err(problem(
                "adapter_rejected_command",
                format!("the adapter did not restart cleanly (ATZ answered {banner:?})"),
                "Unplug the adapter from USB for 5 seconds, plug it back in, and run the command again.",
            ));
        }
        for cmd in SETUP {
            expect_ok(&mut t, cmd)?;
        }
        let device = clean(&t.exchange("ATI")?);
        let stn = Some(clean(&t.exchange("STI")?)).filter(|s| s != "?");
        let stn_device = Some(clean(&t.exchange("STDI")?)).filter(|s| s != "?");
        let info = AdapterInfo { device, stn, stn_device, protocol: String::new(), protocol_number: String::new() };
        Ok(Adapter { t, bus: Bus::Legacy, info, physical: None, dirty: false })
    }

    /// Find the car's OBD protocol (the first request triggers the adapter's protocol search).
    pub fn detect_vehicle(&mut self) -> Result<()> {
        let probe = clean(&self.t.exchange("0100")?);
        if !probe.contains("41 00") {
            return Err(problem("vehicle_not_responding", format!("the adapter is working but the car did not answer ({probe})"), CHECK_IGNITION));
        }
        let number = clean(&self.t.exchange("ATDPN")?);
        let digit = number.strip_prefix('A').unwrap_or(&number);
        if digit.len() != 1 || !"123456789ABC".contains(digit) {
            return Err(problem(
                "adapter_rejected_command",
                format!("unexpected protocol number from the adapter ({number:?})"),
                "Unplug the adapter from USB for 5 seconds, plug it back in, and run the command again.",
            ));
        }
        self.info.protocol_number = number.clone();
        self.info.protocol = clean(&self.t.exchange("ATDP")?).trim_start_matches("AUTO, ").to_string();
        self.bus = Bus::from_protocol(digit.chars().next().unwrap_or('0'));
        Ok(())
    }

    pub fn connect(t: T) -> Result<Self> {
        let mut a = Self::open(t)?;
        a.detect_vehicle()?;
        Ok(a)
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

    /// Send one OBD/UDS request and parse the replies. Reads are retried once after a transient
    /// adapter error or an adapter reboot; anything else (a clear) is sent exactly once.
    pub fn query(&mut self, cmd: &str) -> Result<Vec<Message>> {
        if self.dirty && self.physical.is_none() {
            self.reset_addressing().map_err(|e| {
                problem(
                    "adapter_addressing",
                    format!("could not return the adapter to normal addressing ({e})"),
                    "Unplug the adapter from USB for 5 seconds, plug it back in, and run the command again.",
                )
            })?;
        }
        let first = self.t.exchange(cmd).and_then(|r| parse_response(&r, self.bus));
        let Err(e) = first else { return first };
        let line = adapter_line(&e).map(str::to_string);
        let rebooted = line.as_deref().is_some_and(|l| l == "LV RESET" || l.starts_with("UNEXPECTED REPLY FORMAT") || l.contains("ELM327"));
        if rebooted {
            self.reinit()?;
        }
        let transient = rebooted
            || matches!(line.as_deref(), Some("STOPPED" | "BUS BUSY" | "BUFFER FULL" | "CAN ERROR" | "FC RX TIMEOUT" | "BUS ERROR"))
            || e.to_string().contains("timed out")
            || e.to_string().contains("incomplete multi-frame");
        if !transient || !is_read(cmd) {
            return Err(e);
        }
        self.t.exchange(cmd).and_then(|r| parse_response(&r, self.bus))
    }

    /// The adapter rebooted (low-voltage reset): it is back to echo on / headers off / functional addressing.
    fn reinit(&mut self) -> Result<()> {
        for cmd in SETUP {
            expect_ok(&mut self.t, cmd)?;
        }
        // ATSP0 left the protocol to be searched again: let a functional request do that, never a
        // physical one (and never `ATSP <n>`, which would be saved in the adapter's memory).
        self.t.exchange("0100")?;
        self.dirty = self.physical.is_some();
        if let Some(h) = self.physical.clone() {
            self.address(&h)?;
        }
        Ok(())
    }

    /// Battery voltage at the OBD port, as measured by the adapter. A reboot noticed here re-initialises.
    pub fn voltage(&mut self) -> Result<Option<f64>> {
        let mut reply = clean(&self.t.exchange("ATRV")?);
        if reply.contains("LV RESET") || reply.contains("ELM327") {
            self.reinit()?;
            reply = clean(&self.t.exchange("ATRV")?);
        }
        Ok(reply.trim_end_matches('V').parse().ok())
    }

    /// Address one module: its request ID, a receive filter for its reply ID (request + 8), and explicit
    /// flow control back to it so multi-frame replies continue (as OBDb does for the Kona's modules).
    fn address(&mut self, header: &str) -> Result<()> {
        let id = u16::from_str_radix(header, 16).ok().filter(|id| *id <= 0x7F7).ok_or_else(|| anyhow::anyhow!("bad 11-bit CAN header '{header}'"))?;
        for cmd in [format!("ATSH{header}"), format!("ATCRA{:03X}", id + 8), format!("ATFCSH{header}"), "ATFCSD300000".into(), "ATFCSM1".into()] {
            expect_ok(&mut self.t, &cmd)?;
        }
        Ok(())
    }

    /// Return to functional addressing: automatic flow control, default receive filters (`ATAR`, which
    /// both the ELM327 and OBDLink document as undoing `ATCRA`), header 7DF. Every reply is verified; all
    /// three are always attempted. A no-op on non-CAN buses, where carlink never changes the header.
    pub fn reset_addressing(&mut self) -> Result<()> {
        if self.bus != Bus::Can11 {
            return Ok(());
        }
        let results = ["ATFCSM0", "ATAR", "ATSH7DF"].map(|c| expect_ok(&mut self.t, c));
        for r in results {
            r?;
        }
        self.dirty = false;
        Ok(())
    }

    /// Run `f` with requests addressed physically to 11-bit `header`, listening only to its reply ID
    /// (header + 8), then return to functional addressing. If that return fails, the result of `f`
    /// is still returned, and every later `query` refuses to run until addressing is repaired.
    pub fn with_header<R>(&mut self, header: &str, f: impl FnOnce(&mut Self) -> Result<R>) -> Result<R> {
        if self.bus != Bus::Can11 {
            bail!("physical addressing is only supported on 11-bit CAN; this vehicle uses {:?}", self.bus);
        }
        self.dirty = true;
        self.physical = Some(header.to_string());
        let r = self.address(header).and_then(|_| f(self));
        self.physical = None;
        let restore = self.reset_addressing();
        match (r, restore) {
            (Err(e), Err(re)) => Err(e.context(format!("and the adapter could not be returned to normal addressing: {re}"))),
            (r, _) => r,
        }
    }
}

impl Transport for Box<dyn Transport> {
    fn exchange(&mut self, cmd: &str) -> Result<String> {
        (**self).exchange(cmd)
    }

    fn interrupt(&mut self) {
        (**self).interrupt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::Sim;

    /// Records what reaches the transport, independent of the simulator's own log.
    struct LogTap<'a> {
        inner: Sim,
        log: &'a mut Vec<String>,
    }

    impl Transport for LogTap<'_> {
        fn exchange(&mut self, cmd: &str) -> Result<String> {
            self.log.push(cmd.to_string());
            self.inner.exchange(cmd)
        }
    }

    #[test]
    fn init_detects_can_bus_and_device_info() {
        let mut a = Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap();
        assert_eq!(a.bus(), Bus::Can11);
        assert!(a.info().device.contains("ELM327"));
        assert!(a.info().stn.as_deref().unwrap().starts_with("STN"));
        assert_eq!(a.info().protocol, "ISO 15765-4 (CAN 11/500)");
        let log = a.transport().log.join(" ");
        assert!(log.starts_with("ATI ATZ ATE0 ATL0 ATS1 ATH1 ATCAF1 ATAT1 ATSP0"), "{log}");
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
        let err = Adapter::connect(sim).err().unwrap();
        let p = crate::problem::find(&err).unwrap();
        assert_eq!(p.kind, "vehicle_not_responding");
        assert!(p.what_to_do.contains("ignition"));
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
        sim.inject("ATCRA7EC", "?");
        let mut a = Adapter::connect(sim).unwrap();
        let err = a.with_header("7E4", |a| a.query("220101")).unwrap_err().to_string();
        assert!(err.contains("rejected ATCRA7EC"), "{err}");
        let log = a.transport().log.join(" ");
        assert!(log.ends_with("ATSH7E4 ATCRA7EC ATFCSM0 ATAR ATSH7DF"), "{log}");
    }

    #[test]
    fn identity_is_checked_before_anything_is_reset() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.identity = "Arduino Uno ready";
        let err = Adapter::connect(sim).err().unwrap().to_string();
        assert!(err.contains("ELM327"), "{err}");
    }

    #[test]
    fn identity_check_sends_nothing_but_ati() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.identity = "Arduino Uno ready";
        let mut log = Vec::new();
        let r = Adapter::connect(LogTap { inner: sim, log: &mut log });
        assert!(r.is_err());
        assert_eq!(log, vec!["ATI"]);
    }

    #[test]
    fn rejected_setup_command_fails_connect() {
        let mut sim = Sim::new("odyssey-2016").unwrap();
        sim.inject("ATH1", "?");
        let err = Adapter::connect(sim).err().unwrap().to_string();
        assert!(err.contains("ATH1"), "{err}");
    }

    #[test]
    fn adapter_reboot_is_reinitialised_and_the_read_retried() {
        let mut a = Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap();
        a.transport_mut().inject("0105", "LV RESET");
        let r = a.query("0105").unwrap();
        assert_eq!(r[0].data, vec![0x41, 0x05, 0x5A]);
        assert_eq!(a.transport().log.iter().filter(|c| *c == "ATH1").count(), 2);
    }

    #[test]
    fn reboot_inside_a_module_read_keeps_the_repair_flag() {
        let mut a = Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap();
        a.transport_mut().inject("220101", "LV RESET");
        a.transport_mut().inject("ATAR", "?");
        a.with_header("7E4", |a| a.query("220101")).unwrap();
        a.query("0100").unwrap();
        let log = a.transport().log.join(" ");
        assert!(log.ends_with("ATFCSM0 ATAR ATSH7DF 0100"), "{log}");
    }

    #[test]
    fn headerless_reply_after_a_silent_reboot_reinitialises_and_retries() {
        let mut a = Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap();
        a.transport_mut().inject("0105", "0105\r41 05 5A");
        assert_eq!(a.query("0105").unwrap()[0].data, vec![0x41, 0x05, 0x5A]);
        assert_eq!(a.transport().log.iter().filter(|c| *c == "ATH1").count(), 2);
    }

    #[test]
    fn reboot_noticed_while_reading_voltage_reinitialises() {
        let mut a = Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap();
        a.transport_mut().inject("ATRV", "LV RESET");
        assert_eq!(a.voltage().unwrap(), Some(12.6));
        assert_eq!(a.transport().log.iter().filter(|c| *c == "ATH1").count(), 2);
    }

    #[test]
    fn transient_error_on_a_read_is_retried_once() {
        let mut a = Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap();
        a.transport_mut().inject("0105", "BUS BUSY");
        assert!(a.query("0105").is_ok());
        a.transport_mut().inject("0105", "BUS BUSY");
        a.transport_mut().inject("0105", "BUS BUSY");
        assert!(a.query("0105").is_err());
    }

    #[test]
    fn clear_requests_are_never_retried() {
        let mut a = Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap();
        a.transport_mut().inject("04", "BUS BUSY");
        assert!(a.query("04").is_err());
        assert_eq!(a.transport().log.iter().filter(|c| *c == "04").count(), 1);
    }

    #[test]
    fn failed_restore_keeps_data_and_is_repaired_before_the_next_request() {
        let mut a = Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap();
        a.transport_mut().inject("ATAR", "?");
        let r = a.with_header("7E4", |a| a.query("220101")).unwrap();
        assert_eq!(r[0].ecu, "7EC");
        a.query("0100").unwrap();
        let log = a.transport().log.join(" ");
        assert!(log.ends_with("ATFCSM0 ATAR ATSH7DF ATFCSM0 ATAR ATSH7DF 0100"), "{log}");
    }

    #[test]
    fn unrepairable_addressing_blocks_every_later_request() {
        let mut a = Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap();
        a.transport_mut().inject("ATAR", "?");
        a.with_header("7E4", |a| a.query("220101")).unwrap();
        a.transport_mut().inject("ATAR", "?");
        let err = a.query("04").unwrap_err().to_string();
        assert!(err.contains("could not return the adapter to normal addressing"), "{err}");
        assert!(!a.transport().log.contains(&"04".to_string()));
    }

    #[test]
    fn request_and_restore_failures_are_both_reported() {
        let mut a = Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap();
        a.transport_mut().inject("ATAR", "?");
        let err = a.with_header("7E4", |a| a.query("22FFFF").and_then(|m| if m.is_empty() { anyhow::bail!("no reply") } else { Ok(m) })).unwrap_err();
        let text = format!("{err:#}");
        assert!(text.contains("no reply") && text.contains("normal addressing"), "{text}");
    }

    #[test]
    fn reset_addressing_always_sends_and_verifies() {
        let mut a = Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap();
        a.reset_addressing().unwrap();
        assert!(a.transport().log.join(" ").ends_with("ATFCSM0 ATAR ATSH7DF"));
        a.transport_mut().inject("ATSH7DF", "?");
        assert!(a.reset_addressing().is_err());
    }

    #[test]
    fn with_header_sets_and_restores_header() {
        let mut a = Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap();
        let r = a.with_header("7E4", |a| a.query("220101")).unwrap();
        assert_eq!(r[0].ecu, "7EC");
        let log = a.transport().log.join(" ");
        assert!(log.contains("ATSH7E4 ATCRA7EC ATFCSH7E4 ATFCSD300000 ATFCSM1 220101 ATFCSM0 ATAR ATSH7DF"), "{log}");
    }
}
