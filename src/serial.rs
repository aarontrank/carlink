//! USB serial link to the OBDLink EX (FTDI 0403:6015, 115200 8N1, CR-terminated commands, `>` prompt).

use crate::adapter::Transport;
use anyhow::{Context, Result, bail};
use std::io::{ErrorKind, Read, Write};
use std::time::{Duration, Instant};

pub const BAUD: u32 = 115_200;
const OBDLINK_USB: (u16, u16) = (0x0403, 0x6015);

pub struct Candidate {
    pub name: String,
    pub usb: Option<(u16, u16)>,
}

pub fn candidates() -> Vec<Candidate> {
    serialport::available_ports()
        .unwrap_or_default()
        .into_iter()
        .map(|p| Candidate {
            usb: match p.port_type {
                serialport::SerialPortType::UsbPort(u) => Some((u.vid, u.pid)),
                _ => None,
            },
            name: p.port_name,
        })
        .collect()
}

/// macOS: open the `cu.*` call-out device, never `tty.*` (which blocks waiting for carrier).
pub fn pick_port(ports: &[Candidate]) -> Option<String> {
    let callout = |c: &&Candidate| !c.name.starts_with("/dev/tty.");
    ports
        .iter()
        .filter(callout)
        .find(|c| c.usb == Some(OBDLINK_USB))
        .or_else(|| ports.iter().filter(callout).find(|c| c.name.contains("usbserial")))
        .map(|c| c.name.clone())
}

pub fn timeout_for(cmd: &str) -> Duration {
    match cmd {
        "ATZ" | "ATWS" => Duration::from_secs(5),
        // First request after ATSP0 runs the protocol search, incl. ISO 9141 slow init.
        "0100" => Duration::from_secs(30),
        c if c.starts_with("AT") || c.starts_with("ST") => Duration::from_secs(2),
        _ => Duration::from_secs(5),
    }
}

pub struct Line<P: Read + Write> {
    pub(crate) io: P,
}

impl<P: Read + Write> Line<P> {
    pub fn exchange_within(&mut self, cmd: &str, timeout: Duration) -> Result<String> {
        if cmd.trim().is_empty() {
            bail!("refusing empty command: a bare CR makes the adapter repeat its previous command");
        }
        self.io.write_all(format!("{cmd}\r").as_bytes())?;
        self.io.flush()?;
        let deadline = Instant::now() + timeout;
        let mut out = Vec::new();
        let mut buf = [0u8; 256];
        while !out.contains(&b'>') {
            if Instant::now() > deadline {
                bail!("'{cmd}' timed out after {timeout:?}; got {:?}", String::from_utf8_lossy(&out));
            }
            match self.io.read(&mut buf) {
                Ok(n) => out.extend_from_slice(&buf[..n]),
                Err(e) if e.kind() == ErrorKind::TimedOut => std::thread::sleep(Duration::from_millis(5)),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(String::from_utf8_lossy(&out).into_owned())
    }

    pub fn exchange(&mut self, cmd: &str) -> Result<String> {
        self.exchange_within(cmd, timeout_for(cmd))
    }
}

pub type SerialLine = Line<Box<dyn serialport::SerialPort>>;

pub fn open(path: &str) -> Result<SerialLine> {
    let io = serialport::new(path, BAUD).timeout(Duration::from_millis(100)).open().with_context(|| format!("opening {path}"))?;
    let mut io = io;
    let mut junk = [0u8; 256];
    // Drain stale output; the ATZ that starts every session resets the adapter state.
    while matches!(io.read(&mut junk), Ok(n) if n > 0) {}
    Ok(Line { io })
}

impl<P: Read + Write> Transport for Line<P> {
    fn exchange(&mut self, cmd: &str) -> Result<String> {
        Line::exchange(self, cmd)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor, Read, Write};

    fn cand(name: &str, usb: Option<(u16, u16)>) -> Candidate {
        Candidate { name: name.into(), usb }
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

    struct Fake {
        input: Cursor<Vec<u8>>,
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
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn line(input: &str) -> Line<Fake> {
        Line { io: Fake { input: Cursor::new(input.as_bytes().to_vec()), written: vec![] } }
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
