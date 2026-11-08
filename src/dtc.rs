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

fn system(code: &str) -> &'static str {
    let b = code.as_bytes();
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

pub fn lookup(code: &str) -> DtcInfo {
    let code = code.trim().to_uppercase();
    let description = table().get(code.as_str()).cloned();
    DtcInfo {
        system: system(&code),
        // J2012: second character 0 or 2 is SAE-generic; any code in the generic table counts as generic.
        manufacturer_specific: description.is_none() && !matches!(code.as_bytes().get(1), Some(b'0' | b'2')),
        description,
        code,
    }
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
    fn lowercase_input_is_normalized() {
        assert_eq!(lookup("p0303").code, "P0303");
    }
}
