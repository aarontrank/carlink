//! USB serial link to the OBDLink EX (FTDI 0403:6015, 115200 8N1, CR-terminated commands, `>` prompt).

use crate::adapter::Transport;
use crate::problem::{CHECK_USB, problem};
use anyhow::{Result, bail};
use std::io::{ErrorKind, Read, Write};
use std::time::{Duration, Instant};

pub const BAUD: u32 = 115_200;
const OBDLINK_USB: (u16, u16) = (0x0403, 0x6015);

pub struct Candidate {
    pub name: String,
    pub usb: Option<(u16, u16)>,
    /// USB product string, e.g. "OBDLink EX".
    pub product: Option<String>,
}

pub fn candidates() -> Vec<Candidate> {
    serialport::available_ports()
        .unwrap_or_default()
        .into_iter()
        .map(|p| {
            let (usb, product) = match p.port_type {
                serialport::SerialPortType::UsbPort(u) => (Some((u.vid, u.pid)), u.product),
                _ => (None, None),
            };
            Candidate { name: p.port_name, usb, product }
        })
        .collect()
}

/// macOS: open the `cu.*` call-out device, never `tty.*` (which blocks waiting for carrier).
/// 0403:6015 is FTDI's generic ID, so an "OBDLink" product string wins; with several look-alikes and
/// no such string, nothing is picked and the caller must pass --port.
pub fn pick_port(ports: &[Candidate]) -> Option<String> {
    let callout: Vec<&Candidate> = ports.iter().filter(|c| !c.name.starts_with("/dev/tty.")).collect();
    if let Some(c) = callout.iter().find(|c| c.product.as_deref().is_some_and(|p| p.contains("OBDLink"))) {
        return Some(c.name.clone());
    }
    let ftdi: Vec<&&Candidate> = callout.iter().filter(|c| c.usb == Some(OBDLINK_USB)).collect();
    let usbserial: Vec<&&Candidate> = callout.iter().filter(|c| c.name.contains("usbserial")).collect();
    match (ftdi.as_slice(), usbserial.as_slice()) {
        ([one], _) => Some(one.name.clone()),
        ([], [one]) => Some(one.name.clone()),
        _ => None,
    }
}

/// Upper bound for one exchange however long the adapter keeps talking.
const MAX_WAIT: Duration = Duration::from_secs(60);

pub fn timeout_for(cmd: &str) -> Duration {
    match cmd {
        "ATZ" | "ATWS" => Duration::from_secs(5),
        // First request after ATSP0 runs the protocol search, incl. ISO 9141 slow init.
        "0100" => Duration::from_secs(30),
        c if c.starts_with("AT") || c.starts_with("ST") => Duration::from_secs(2),
        // The adapter itself waits up to 5 s after each "response pending"; allow a little more.
        _ => Duration::from_secs(7),
    }
}

pub struct Line<P: Read + Write> {
    pub(crate) io: P,
}

impl<P: Read + Write> Line<P> {
    /// Read up to the `>` prompt. `quiet` is how long the adapter may stay silent; every byte received
    /// (e.g. a `7F xx 78` response-pending line, or `BUS INIT: ...` dots) restarts it, up to `MAX_WAIT`.
    fn read_to_prompt(&mut self, quiet: Duration) -> std::result::Result<Vec<u8>, ReadFail> {
        let start = Instant::now();
        let mut deadline = start + quiet;
        let mut out = Vec::new();
        let mut buf = [0u8; 256];
        while !out.contains(&b'>') {
            let now = Instant::now();
            if now > deadline || now - start > MAX_WAIT {
                return Err(ReadFail::Timeout(out));
            }
            match self.io.read(&mut buf) {
                Ok(0) => std::thread::sleep(Duration::from_millis(5)),
                Ok(n) => {
                    out.extend_from_slice(&buf[..n]);
                    deadline = Instant::now() + quiet;
                }
                Err(e) if e.kind() == ErrorKind::TimedOut => std::thread::sleep(Duration::from_millis(5)),
                Err(e) => return Err(ReadFail::Io(e)),
            }
        }
        Ok(out)
    }

    pub fn exchange_within(&mut self, cmd: &str, timeout: Duration) -> Result<String> {
        if cmd.trim().is_empty() {
            bail!("refusing empty command: a bare CR makes the adapter repeat its previous command");
        }
        self.io.write_all(format!("{cmd}\r").as_bytes()).map_err(disconnected)?;
        self.io.flush().map_err(disconnected)?;
        match self.read_to_prompt(timeout) {
            Ok(out) => Ok(String::from_utf8_lossy(&out).into_owned()),
            Err(ReadFail::Io(e)) => Err(disconnected(e)),
            Err(ReadFail::Timeout(partial)) => {
                self.interrupt();
                Err(problem(
                    "adapter_timeout",
                    format!("'{cmd}' timed out after {timeout:?}; got {:?}", String::from_utf8_lossy(&partial)),
                    "The adapter stopped answering. Check both ends of the adapter are firmly connected, then run the command again. If it keeps happening, unplug the adapter from USB for 5 seconds.",
                ))
            }
        }
    }

    pub fn exchange(&mut self, cmd: &str) -> Result<String> {
        self.exchange_within(cmd, timeout_for(cmd))
    }

    /// Stop whatever the adapter is doing and discard its output: any byte interrupts a command in
    /// progress (it answers STOPPED), and a space is ignored by an idle adapter, so it cannot become
    /// part of the next command.
    pub fn interrupt(&mut self) {
        let _ = self.io.write_all(b" ");
        let _ = self.read_to_prompt(Duration::from_secs(1));
        drain_until_quiet(&mut self.io, Duration::from_millis(300), Duration::from_secs(2));
    }
}

/// Read and discard until nothing has arrived for `quiet`, giving up after `max`.
fn drain_until_quiet(io: &mut impl Read, quiet: Duration, max: Duration) {
    let start = Instant::now();
    let mut last = Instant::now();
    let mut junk = [0u8; 256];
    while start.elapsed() < max && last.elapsed() < quiet {
        match io.read(&mut junk) {
            Ok(n) if n > 0 => last = Instant::now(),
            _ => std::thread::sleep(Duration::from_millis(5)),
        }
    }
}

enum ReadFail {
    Timeout(Vec<u8>),
    Io(std::io::Error),
}

fn disconnected(e: std::io::Error) -> anyhow::Error {
    problem("adapter_disconnected", format!("lost the connection to the adapter ({e})"), CHECK_USB)
}

/// Discard whatever the adapter is still sending, for at most `max`.
pub fn drain(io: &mut impl Read, max: Duration) {
    let deadline = Instant::now() + max;
    let mut junk = [0u8; 256];
    while Instant::now() < deadline && matches!(io.read(&mut junk), Ok(n) if n > 0) {}
}

pub fn open_error(path: &str, e: serialport::Error) -> anyhow::Error {
    match e.kind() {
        serialport::ErrorKind::NoDevice | serialport::ErrorKind::Io(ErrorKind::NotFound) => {
            problem("adapter_not_found", format!("no adapter at {path}: unplugged or the wrong path ({e})"), CHECK_USB)
        }
        serialport::ErrorKind::Io(ErrorKind::WouldBlock | ErrorKind::PermissionDenied | ErrorKind::ResourceBusy) => problem(
            "adapter_busy",
            format!("the adapter at {path} is in use by another program ({e})"),
            "Another carlink command or a scan app is using the adapter. Wait for it to finish (or close the app), then run the command again.",
        ),
        _ => problem("adapter_open_failed", format!("could not open {path} ({e})"), CHECK_USB),
    }
}

pub type SerialLine = Line<Box<dyn serialport::SerialPort>>;

pub fn open(path: &str) -> Result<SerialLine> {
    let mut io = serialport::new(path, BAUD).timeout(Duration::from_millis(100)).open().map_err(|e| open_error(path, e))?;
    drain(&mut io, Duration::from_secs(1));
    Ok(Line { io })
}

impl<P: Read + Write> Transport for Line<P> {
    fn exchange(&mut self, cmd: &str) -> Result<String> {
        Line::exchange(self, cmd)
    }

    fn interrupt(&mut self) {
        Line::interrupt(self)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor, Read, Write};

    fn cand(name: &str, usb: Option<(u16, u16)>) -> Candidate {
        Candidate { name: name.into(), usb, product: None }
    }

    #[test]
    fn product_name_beats_a_generic_ftdi_device_and_ambiguity_picks_nothing() {
        let ftdi = Some((0x0403, 0x6015));
        let mut obd = cand("/dev/cu.usbserial-B", ftdi);
        obd.product = Some("OBDLink EX".into());
        assert_eq!(pick_port(&[cand("/dev/cu.usbserial-A", ftdi), obd]).as_deref(), Some("/dev/cu.usbserial-B"));
        assert_eq!(pick_port(&[cand("/dev/cu.usbserial-A", ftdi), cand("/dev/cu.usbserial-B", ftdi)]), None);
    }

    #[test]
    fn prefers_obdlink_ftdi_ids() {
        let ports = [cand("/dev/cu.usbserial-A", None), cand("/dev/cu.usbserial-223230293267", Some((0x0403, 0x6015))), cand("/dev/cu.Bluetooth", None)];
        assert_eq!(pick_port(&ports).as_deref(), Some("/dev/cu.usbserial-223230293267"));
    }

    #[test]
    fn falls_back_to_usbserial_callout_device() {
        let ports = [cand("/dev/tty.usbserial-A", None), cand("/dev/cu.usbserial-A", None)];
        assert_eq!(pick_port(&ports).as_deref(), Some("/dev/cu.usbserial-A"));
        assert_eq!(pick_port(&[cand("/dev/cu.Bluetooth-Incoming-Port", None)]), None);
    }

    /// Each write releases the next scripted reply into the read buffer.
    struct Fake {
        input: Cursor<Vec<u8>>,
        script: std::collections::VecDeque<&'static str>,
        written: Vec<u8>,
    }
    impl Read for Fake {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            match self.input.read(buf)? {
                0 => Err(io::ErrorKind::TimedOut.into()),
                n => Ok(n),
            }
        }
    }
    impl Write for Fake {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            self.written.extend_from_slice(b);
            if let Some(next) = self.script.pop_front() {
                let pos = self.input.position() as usize;
                let mut rest = self.input.get_ref()[pos..].to_vec();
                rest.extend_from_slice(next.as_bytes());
                self.input = Cursor::new(rest);
            }
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn scripted(replies: &[&'static str]) -> Line<Fake> {
        Line { io: Fake { input: Cursor::new(vec![]), script: replies.iter().copied().collect(), written: vec![] } }
    }

    fn line(input: &'static str) -> Line<Fake> {
        scripted(&[input])
    }

    #[test]
    fn timeout_interrupts_the_adapter_so_the_next_command_is_clean() {
        let mut l = scripted(&["41 05", "STOPPED\r\r>", "41 06 80\r\r>"]);
        assert!(l.exchange_within("0105", Duration::from_millis(50)).is_err());
        assert_eq!(l.exchange_within("0106", Duration::from_millis(200)).unwrap(), "41 06 80\r\r>");
        assert_eq!(l.io.written, b"0105\r 0106\r");
    }

    #[test]
    fn unplugged_adapter_is_reported_as_disconnected() {
        struct Gone;
        impl Read for Gone {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::ErrorKind::BrokenPipe.into())
            }
        }
        impl Write for Gone {
            fn write(&mut self, b: &[u8]) -> io::Result<usize> {
                Ok(b.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let err = Line { io: Gone }.exchange("0105").unwrap_err().to_string();
        assert!(err.contains("lost the connection"), "{err}");
    }

    #[test]
    fn bytes_still_arriving_keep_the_request_alive() {
        struct Slow {
            chunks: std::collections::VecDeque<&'static [u8]>,
        }
        impl Read for Slow {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                std::thread::sleep(Duration::from_millis(30));
                match self.chunks.pop_front() {
                    Some(c) => {
                        buf[..c.len()].copy_from_slice(c);
                        Ok(c.len())
                    }
                    None => Err(io::ErrorKind::TimedOut.into()),
                }
            }
        }
        impl Write for Slow {
            fn write(&mut self, b: &[u8]) -> io::Result<usize> {
                Ok(b.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let chunks = ["7EC 03 7F 22 78\r", "7EC 03 7F 22 78\r", "7EC 04 62 01 01 AA\r", "\r>"].map(str::as_bytes).into_iter().collect();
        let mut l = Line { io: Slow { chunks } };
        assert!(l.exchange_within("220101", Duration::from_millis(50)).unwrap().contains("62 01 01"));
    }

    #[test]
    fn drain_gives_up_on_an_endless_stream() {
        struct Endless;
        impl Read for Endless {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                buf[0] = b'x';
                Ok(1)
            }
        }
        let t = std::time::Instant::now();
        drain(&mut Endless, Duration::from_millis(50));
        assert!(t.elapsed() < Duration::from_secs(1));
    }

    #[test]
    fn busy_port_error_says_what_to_do() {
        let e = serialport::Error::new(serialport::ErrorKind::Io(io::ErrorKind::WouldBlock), "Resource temporarily unavailable");
        let msg = open_error("/dev/cu.usbserial-X", e).to_string();
        assert!(msg.contains("in use"), "{msg}");
        let e = serialport::Error::new(serialport::ErrorKind::NoDevice, "Device not configured");
        assert!(open_error("/dev/cu.usbserial-X", e).to_string().contains("unplugged"));
    }

    #[test]
    fn exchange_writes_cr_terminated_command_and_reads_to_prompt() {
        let mut l = line("41 05 7B\r\r>");
        assert_eq!(l.exchange("0105").unwrap(), "41 05 7B\r\r>");
        assert_eq!(l.io.written, b"0105\r");
    }

    #[test]
    fn exchange_times_out_without_prompt() {
        let mut l = line("41 05");
        let err = l.exchange_within("0105", std::time::Duration::from_millis(50)).unwrap_err().to_string();
        assert!(err.contains("0105") && err.contains("timed out"), "{err}");
    }

    #[test]
    fn empty_command_is_refused_because_bare_cr_repeats_the_last_one() {
        let mut l = line(">");
        assert!(l.exchange("").is_err());
        assert!(l.io.written.is_empty());
    }

    #[test]
    fn slow_commands_get_longer_timeouts() {
        assert!(timeout_for("ATZ") > timeout_for("ATE0"));
        assert!(timeout_for("0100") > timeout_for("0105"));
    }
}
