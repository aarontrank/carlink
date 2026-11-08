//! Turns ELM327/STN response text (headers on, spaces on) into per-ECU messages.

use anyhow::{Result, bail};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Bus {
    Can11,
    Can29,
    /// SAE J1850, ISO 9141-2, ISO 14230-4 (KWP2000): 3 header bytes + data + checksum.
    Legacy,
}

impl Bus {
    /// From the last character of `ATDPN` (which may be prefixed by `A` for auto).
    pub fn from_protocol(p: char) -> Bus {
        match p {
            '6' | '8' => Bus::Can11,
            '7' | '9' => Bus::Can29,
            _ => Bus::Legacy,
        }
    }

    pub fn is_can(self) -> bool {
        self != Bus::Legacy
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Message {
    pub ecu: String,
    pub data: Vec<u8>,
}

const ERRORS: &[&str] = &[
    "UNABLE TO CONNECT", "CAN ERROR", "BUS INIT: ...ERROR", "BUFFER FULL", "STOPPED", "BUS ERROR",
    "FB ERROR", "LV RESET", "ACT ALERT", "LP ALERT", "BUS BUSY", "FC RX TIMEOUT", "OUT OF MEMORY",
    "UART RX OVERFLOW", "ERR",
];

fn hex_bytes(tokens: &[&str]) -> Option<Vec<u8>> {
    tokens.iter().map(|t| u8::from_str_radix(t, 16).ok()).collect()
}

struct Pending {
    ecu: String,
    len: usize,
    data: Vec<u8>,
}

pub fn parse_response(raw: &str, bus: Bus) -> Result<Vec<Message>> {
    let mut out: Vec<Message> = Vec::new();
    let mut pending: Vec<Pending> = Vec::new();
    for line in raw.split(['\r', '\n']).map(|l| l.trim().trim_start_matches('>').trim()) {
        if line.is_empty() || line == "SEARCHING..." || line == "NO DATA" || line.contains('<') {
            continue;
        }
        let line = line.strip_prefix("BUS INIT: ...OK").unwrap_or(line).trim();
        if line == "?" || ERRORS.iter().any(|e| line.starts_with(e)) {
            bail!("adapter error: {line}");
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let header_len = match bus {
            Bus::Can11 => 1,
            Bus::Can29 => 4,
            Bus::Legacy => 3,
        };
        if tokens.len() <= header_len {
            continue;
        }
        let Some(bytes) = hex_bytes(&tokens[header_len..]) else { continue };
        if bus == Bus::Legacy {
            let ecu = tokens[2].to_string();
            out.push(Message { ecu, data: bytes[..bytes.len() - 1].to_vec() });
            continue;
        }
        let ecu = tokens[..header_len].concat();
        let pci = bytes[0];
        match pci >> 4 {
            0 => {
                let n = (pci & 0x0F) as usize;
                out.push(Message { ecu, data: bytes[1..].iter().take(n).copied().collect() });
            }
            1 if bytes.len() >= 2 => {
                let len = (((pci & 0x0F) as usize) << 8) | bytes[1] as usize;
                out.push(Message { ecu: ecu.clone(), data: Vec::new() });
                pending.retain(|p| p.ecu != ecu);
                pending.push(Pending { ecu, len, data: bytes[2..].to_vec() });
            }
            2 => {
                if let Some(p) = pending.iter_mut().find(|p| p.ecu == ecu) {
                    p.data.extend_from_slice(&bytes[1..]);
                }
            }
            _ => {}
        }
    }
    // Fill each multi-frame placeholder (kept in arrival order) with its reassembled payload.
    for p in pending {
        if let Some(m) = out.iter_mut().find(|m| m.ecu == p.ecu && m.data.is_empty()) {
            m.data = p.data.into_iter().take(p.len).collect();
        }
    }
    // 7F <sid> 78 = "response pending"; the adapter keeps listening and the real answer follows.
    out.retain(|m| !m.data.is_empty() && !matches!(m.data[..], [0x7F, _, 0x78]));
    Ok(out)
}
#[cfg(test)]
mod tests {
    use super::*;

    fn msg(ecu: &str, data: &[u8]) -> Message {
        Message { ecu: ecu.into(), data: data.to_vec() }
    }

    #[test]
    fn can_single_frame_strips_pci() {
        let r = parse_response("7E8 06 41 00 BE 3F A8 13\r\r>", Bus::Can11).unwrap();
        assert_eq!(r, vec![msg("7E8", &[0x41, 0x00, 0xBE, 0x3F, 0xA8, 0x13])]);
    }

    #[test]
    fn can_multiple_ecus_are_separated() {
        let r = parse_response("7E8 03 41 05 7B\r7E9 03 41 05 7C\r", Bus::Can11).unwrap();
        assert_eq!(r, vec![msg("7E8", &[0x41, 0x05, 0x7B]), msg("7E9", &[0x41, 0x05, 0x7C])]);
    }

    #[test]
    fn can_multi_frame_is_reassembled_to_declared_length() {
        let raw = "7E8 10 14 49 02 01 35 4A 36\r7E8 21 52 44 31 38 32 39\r7E8 22 34 4C 31 32 33 34\r7E8 23 35 36 00 00 00 00\r";
        let r = parse_response(raw, Bus::Can11).unwrap();
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].data.len(), 0x14);
        assert_eq!(&r[0].data[..3], &[0x49, 0x02, 0x01]);
        assert_eq!(&r[0].data[3..], b"5J6RD18294L123456");
    }

    #[test]
    fn can_interleaved_multi_frame_from_two_ecus() {
        let raw = "7EC 10 08 62 01 01 AA BB CC\r7EA 03 62 01 02\r7EC 21 DD EE 00 00 00 00 00\r";
        let r = parse_response(raw, Bus::Can11).unwrap();
        assert_eq!(r, vec![msg("7EC", &[0x62, 0x01, 0x01, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE]), msg("7EA", &[0x62, 0x01, 0x02])]);
    }

    #[test]
    fn can_29_bit_header_uses_last_byte_as_ecu() {
        let r = parse_response("18 DA F1 10 03 41 05 7B\r", Bus::Can29).unwrap();
        assert_eq!(r, vec![msg("18DAF110", &[0x41, 0x05, 0x7B])]);
    }

    #[test]
    fn legacy_strips_three_byte_header_and_checksum() {
        let r = parse_response("BUS INIT: ...OK\r48 6B 10 41 05 7B C4\r", Bus::Legacy).unwrap();
        assert_eq!(r, vec![msg("10", &[0x41, 0x05, 0x7B])]);
    }

    #[test]
    fn legacy_lines_stay_separate_messages() {
        let r = parse_response("48 6B 10 43 01 33 04 20 00 00 AA\r48 6B 10 43 00 00 00 00 00 00 BB\r", Bus::Legacy).unwrap();
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn flow_control_frames_are_ignored() {
        assert_eq!(parse_response("7E8 30 00 00\r7E8 03 41 05 7B\r", Bus::Can11).unwrap().len(), 1);
    }

    #[test]
    fn searching_and_echo_noise_is_ignored() {
        let r = parse_response("SEARCHING...\r7E8 03 41 05 7B\r\r>", Bus::Can11).unwrap();
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn no_data_is_empty_not_error() {
        assert_eq!(parse_response("NO DATA\r\r>", Bus::Can11).unwrap(), vec![]);
    }

    #[test]
    fn adapter_errors_are_reported() {
        for e in ["UNABLE TO CONNECT", "CAN ERROR", "BUS INIT: ...ERROR", "?", "BUFFER FULL", "STOPPED", "BUS ERROR", "FB ERROR", "LV RESET", "ACT ALERT"] {
            let err = parse_response(&format!("{e}\r>"), Bus::Can11).unwrap_err();
            assert!(err.to_string().contains(e), "{e}: {err}");
        }
    }

    #[test]
    fn corrupt_frames_are_skipped() {
        let r = parse_response("7E8 03 41 05 7B\r<DATA ERROR\r7E8 03 41 0D 00 <RX ERROR\r", Bus::Can11).unwrap();
        assert_eq!(r, vec![msg("7E8", &[0x41, 0x05, 0x7B])]);
    }

    #[test]
    fn more_adapter_errors_from_stn_manual() {
        for e in ["FC RX TIMEOUT", "OUT OF MEMORY", "UART RX OVERFLOW", "LP ALERT", "ERR94"] {
            assert!(parse_response(&format!("{e}\r>"), Bus::Can11).is_err(), "{e}");
        }
    }

    #[test]
    fn data_error_line_is_skipped_not_fatal() {
        assert_eq!(parse_response("DATA ERROR\r7E8 03 41 05 7B\r", Bus::Can11).unwrap().len(), 1);
    }

    #[test]
    fn response_pending_is_dropped_in_favor_of_final_answer() {
        let r = parse_response("7EC 03 7F 22 78\r7EC 04 62 01 01 AA\r", Bus::Can11).unwrap();
        assert_eq!(r, vec![msg("7EC", &[0x62, 0x01, 0x01, 0xAA])]);
    }

    #[test]
    fn negative_response_is_kept_as_data() {
        let r = parse_response("7EC 03 7F 22 31\r", Bus::Can11).unwrap();
        assert_eq!(r[0].data, vec![0x7F, 0x22, 0x31]);
    }

    #[test]
    fn bus_from_protocol_number() {
        assert_eq!(Bus::from_protocol('6'), Bus::Can11);
        assert_eq!(Bus::from_protocol('9'), Bus::Can29);
        assert_eq!(Bus::from_protocol('A'), Bus::Legacy); // J1939 not supported as CAN OBD
        assert_eq!(Bus::from_protocol('3'), Bus::Legacy);
    }
}
