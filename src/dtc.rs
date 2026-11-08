//! Trouble-code descriptions: SAE J2012 generic table (data/dtc-generic.csv, CC0) plus code structure.

use serde::Serialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize)]
pub struct DtcInfo {
    pub code: String,
    /// SAE generic meaning; `None` for manufacturer-specific or unknown codes.
    pub description: Option<String>,
    pub system: &'static str,
    pub manufacturer_specific: bool,
}

fn table() -> &'static HashMap<&'static str, String> {
    static T: OnceLock<HashMap<&'static str, String>> = OnceLock::new();
    T.get_or_init(|| {
        include_str!("../data/dtc-generic.csv")
            .lines()
            .skip(1)
            .filter_map(|l| l.split_once(','))
            .map(|(code, d)| {
                let d = match d.strip_prefix('"').and_then(|d| d.strip_suffix('"')) {
                    Some(q) => q.replace("\"\"", "\""),
                    None => d.to_string(),
                };
                (code, d)
            })
            .collect()
    })
}

/// A code as the person read it → normalised `P0420`, or why it is not a code.
pub fn parse(code: &str) -> std::result::Result<String, String> {
    let c = code.trim().to_uppercase().replace(['-', ' '], "");
    if !c.is_ascii() {
        return Err(format!("'{code}' is not a trouble code: expected a letter P, C, B or U followed by four digits/letters, e.g. P0420"));
    }
    let c = c.get(..5).filter(|_| c.len() == 5 || c.len() == 7).unwrap_or(&c).to_string();
    let b = c.as_bytes();
    let ok = b.len() == 5 && matches!(b[0], b'P' | b'C' | b'B' | b'U') && (b'0'..=b'3').contains(&b[1]) && b[2..].iter().all(u8::is_ascii_hexdigit);
    if ok {
        Ok(c)
    } else if c.len() == 5 && c[1..].contains('O') {
        Err(format!("'{code}' is not a trouble code: it has the letter O where a zero belongs (try {}{})", &c[..1], c[1..].replace('O', "0")))
    } else {
        Err(format!("'{code}' is not a trouble code: expected a letter P, C, B or U followed by four digits/letters, e.g. P0420"))
    }
}

/// SAE J2012: P1, P30–P33, and B/C/U 1 and 2 are manufacturer-controlled ranges.
fn manufacturer_range(code: &str) -> bool {
    let b = code.as_bytes();
    match (b[0], b[1], b[2]) {
        (b'P', b'1', _) => true,
        (b'P', b'3', d) => (b'0'..=b'3').contains(&d),
        (b'B' | b'C' | b'U', d, _) => matches!(d, b'1' | b'2'),
        _ => false,
    }
}

fn system(code: &str) -> &'static str {
    let b = code.as_bytes();
    if b.first() == Some(&b'P') && b.get(2) == Some(&b'0') {
        return "Fuel and air metering and auxiliary emission controls";
    }
    match (b.first(), b.get(2)) {
        (Some(b'B'), _) => "Body",
        (Some(b'C'), _) => "Chassis",
        (Some(b'U'), _) => "Network communication",
        (_, Some(b'1' | b'2')) => "Fuel and air metering",
        (_, Some(b'3')) => "Ignition system or misfire",
        (_, Some(b'4')) => "Auxiliary emission controls",
        (_, Some(b'5')) => "Vehicle speed, idle control and auxiliary inputs",
        (_, Some(b'6')) => "Computer and output circuits",
        (_, Some(b'7' | b'8' | b'9')) => "Transmission",
        (_, Some(b'A' | b'B' | b'C')) => "Hybrid/EV propulsion",
        _ => "Unknown",
    }
}

/// `code` must already be a valid 5-character code (see `parse`); a UDS suffix like `-00` is ignored.
pub fn lookup(code: &str) -> DtcInfo {
    let code = code.trim().to_uppercase();
    let base = code.get(..5).unwrap_or(&code);
    let manufacturer_specific = base.len() == 5 && manufacturer_range(base);
    let description = if manufacturer_specific { None } else { table().get(base).cloned() };
    DtcInfo { system: system(base), manufacturer_specific, description, code }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_code_has_sae_description() {
        let d = lookup("P0420");
        assert_eq!(d.description.as_deref(), Some("Catalyst System Efficiency Below Threshold Bank 1"));
        assert!(!d.manufacturer_specific);
        assert_eq!(d.system, "Auxiliary emission controls");
    }

    #[test]
    fn quoted_csv_description_is_unescaped() {
        assert_eq!(lookup("P2646").description.as_deref(), Some("\"A\" Rocker Arm Actuator System Performance/Stuck Off (Bank 1)"));
    }

    #[test]
    fn manufacturer_code_falls_back_to_structure() {
        let d = lookup("P1457");
        assert_eq!(d.description, None);
        assert!(d.manufacturer_specific);
        assert_eq!(d.system, "Auxiliary emission controls");
    }

    #[test]
    fn network_and_body_systems() {
        assert_eq!(lookup("U0100").system, "Network communication");
        assert_eq!(lookup("B1234").system, "Body");
        assert_eq!(lookup("C1234").system, "Chassis");
        assert_eq!(lookup("P0A80").system, "Hybrid/EV propulsion");
        assert_eq!(lookup("P0700").system, "Transmission");
        assert_eq!(lookup("P0505").system, "Vehicle speed, idle control and auxiliary inputs");
        assert_eq!(lookup("P0D00").system, "Unknown");
    }

    #[test]
    fn parse_accepts_codes_and_explains_typos() {
        assert_eq!(parse(" p0420 ").unwrap(), "P0420");
        assert_eq!(parse("C1611-00").unwrap(), "C1611");
        assert!(parse("PO420").unwrap_err().contains("letter O"));
        assert!(parse("0420").is_err());
        assert!(parse("P5420").is_err());
        assert!(parse("ÉO42").is_err());
    }

    #[test]
    fn j2012_manufacturer_ranges() {
        assert!(lookup("P1457").manufacturer_specific);
        assert!(lookup("P3000").manufacturer_specific);
        assert!(!lookup("P3400").manufacturer_specific);
        assert!(lookup("U2100").manufacturer_specific);
        assert!(lookup("C1611").manufacturer_specific);
        assert!(!lookup("U0100").manufacturer_specific);
        assert_eq!(lookup("P0011").system, "Fuel and air metering and auxiliary emission controls");
    }

    #[test]
    fn lowercase_input_is_normalized() {
        assert_eq!(lookup("p0303").code, "P0303");
    }
}
