//! The vehicles carlink knows in depth. Facts cite the wiki in `kb/wiki/`.

mod crv;
pub mod clarity;
pub mod kona;
pub mod odyssey;

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Powertrain {
    Gasoline,
    Ev,
    /// Engine plus a plug-in battery: the engine can start by itself whenever the car is READY.
    #[serde(rename = "plug_in_hybrid")]
    PluginHybrid,
}

#[derive(Debug, Serialize)]
pub struct KnownDtc {
    pub code: &'static str,
    /// Likely causes on this vehicle and what to do, in order.
    pub note: &'static str,
}

#[derive(Debug, Serialize)]
pub struct Reset {
    pub name: &'static str,
    pub carlink_can_do: bool,
    pub steps: &'static [&'static str],
    pub notes: &'static str,
}

#[derive(Debug, Serialize)]
pub struct Profile {
    pub id: &'static str,
    pub name: &'static str,
    pub powertrain: Powertrain,
    /// Wiki page (`kb/wiki/<name>.md` in the carlink repo) with everything known about this car.
    pub wiki: &'static str,
    pub recalls: &'static [&'static str],
    pub notes: &'static [&'static str],
    /// Extra steps after a successful clear on this car.
    pub after_clear: &'static [&'static str],
    /// Modules whose trouble codes carlink reads (and can clear) beyond the emissions codes.
    pub modules: &'static [crate::uds::Module],
    /// Why OBD service 04 must not be used on this car, if it must not.
    pub obd_clear_refused: Option<&'static str>,
    /// Codes a dealer needs as evidence (and why): while any is present, carlink will not send service 04.
    #[serde(skip)]
    pub kept_codes: (&'static [&'static str], &'static str),
    #[serde(skip)]
    known_dtcs: &'static [KnownDtc],
    #[serde(skip)]
    resets: &'static [Reset],
    /// (VIN prefix, model-year character at VIN position 10, required character at VIN position 8 if any)
    #[serde(skip)]
    vin: &'static [(&'static str, char, Option<char>)],
}

pub static PROFILES: &[Profile] = &[crv::PROFILE, odyssey::PROFILE, kona::PROFILE, clarity::PROFILE];

const CLEAR_STEPS: &[&str] = &[
    "Only after the repair: explain what clearing does and get an explicit yes from the person (carlink skill, step 6).",
    "Ignition ON, engine OFF (EV: not required to be READY).",
    "Then run `carlink clear` as the skill's step 6 shows; read clear_status and follow next_steps.",
];

pub(crate) const fn clear_reset(notes: &'static str) -> Reset {
    Reset { name: "Clear trouble codes / check-engine light (OBD service 04)", carlink_can_do: true, steps: CLEAR_STEPS, notes }
}

pub(crate) fn hex(s: &str) -> Vec<u8> {
    (0..s.len() / 2).filter_map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).ok()).collect()
}

/// Positive-response payloads (after the 62 xx xx / 61 xx header) for each request to one ECU.
pub(crate) fn payloads<T: crate::adapter::Transport>(a: &mut crate::adapter::Adapter<T>, header: &str, reqs: &[&str]) -> anyhow::Result<Vec<anyhow::Result<Option<Vec<u8>>>>> {
    a.with_header(header, |a| {
        Ok(reqs
            .iter()
            .map(|req| {
                // The positive reply echoes the service (+0x40) and the identifier, e.g. 62 01 01 for 220101.
                let mut echo = hex(req);
                echo[0] += 0x40;
                let msgs = a.query(req)?;
                if let Some((_, nrc)) = msgs.iter().find_map(|m| crate::uds::negative(&m.data)) {
                    anyhow::bail!("refused: {}", crate::actions::nrc_meaning(nrc));
                }
                Ok(msgs.into_iter().find(|m| m.data.starts_with(&echo)).map(|m| m.data[echo.len()..].to_vec()))
            })
            .collect())
    })
}

pub fn by_id(id: &str) -> Option<&'static Profile> {
    PROFILES.iter().find(|p| p.id == id)
}

pub fn detect(vin: &str) -> Option<&'static Profile> {
    let year = vin.chars().nth(9)?;
    let eighth = vin.chars().nth(7)?;
    PROFILES.iter().find(|p| p.vin.iter().any(|(prefix, y, e)| vin.starts_with(prefix) && year == *y && e.is_none_or(|e| e == eighth)))
}

/// World manufacturer identifiers (VIN positions 1–3) for the makes a Honda/Hyundai shop sees.
const WMI: &[(&str, &str)] = &[
    ("1HG", "Honda"), ("2HG", "Honda"), ("3HG", "Honda"), ("5FN", "Honda"), ("5J6", "Honda"), ("7FA", "Honda"),
    ("19X", "Honda"), ("2HK", "Honda"), ("JHL", "Honda"), ("JHM", "Honda"), ("SHH", "Honda"), ("SHS", "Honda"),
    ("19U", "Acura"), ("5J8", "Acura"), ("JH4", "Acura"),
    ("KMH", "Hyundai"), ("KM8", "Hyundai"), ("5NP", "Hyundai"), ("5NM", "Hyundai"),
    ("KNA", "Kia"), ("KND", "Kia"), ("5XX", "Kia"), ("5XY", "Kia"), ("3KP", "Kia"),
    ("KMT", "Genesis"),
];

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct VinFacts {
    pub make: Option<&'static str>,
    pub model_year: Option<u16>,
    pub check_digit_ok: bool,
}

/// Make (from a short WMI table) and model year (position 10; position 7 being a letter means 2010+).
pub fn vin_info(vin: &str) -> VinFacts {
    let make = WMI.iter().find(|(w, _)| vin.starts_with(w)).map(|(_, m)| *m);
    let b = vin.as_bytes();
    let model_year = (b.len() == 17).then(|| {
        let base: u16 = if b[6].is_ascii_alphabetic() { 2010 } else { 1980 };
        let pos = b"ABCDEFGHJKLMNPRSTVWXY123456789".iter().position(|c| *c == b[9])? as u16;
        Some(base + pos)
    }).flatten();
    VinFacts { make, model_year, check_digit_ok: crate::obd::vin_check_digit_ok(vin) }
}

/// Vehicle-specific reads beyond generic OBD-II. Failures become info findings, never abort the scan.
pub fn extended<T: crate::adapter::Transport>(p: &Profile, a: &mut crate::adapter::Adapter<T>) -> (Vec<crate::obd::Reading>, Vec<crate::findings::Finding>) {
    let (readings, errors) = match p.id {
        "kona-ev-2020" => kona::read(a),
        "clarity-phev-2018" => clarity::read(a),
        _ => (vec![], vec![]),
    };
    let findings = if errors.is_empty() {
        vec![]
    } else {
        vec![crate::findings::Finding {
            severity: crate::findings::Severity::Info,
            code: None,
            title: "Vehicle-specific data partly unavailable".into(),
            detail: errors.join("; "),
            recommendation: "Some modules only answer with the car fully on (READY). If it persists, the request map in the wiki may be wrong for this car; record what answered.".into(),
        }]
    };
    (readings, findings)
}

/// Add vehicle knowledge to generic findings (known causes for codes, model-specific checks).
pub fn annotate(p: &Profile, _scan: &crate::scan::Scan, extended: &[crate::obd::Reading], tests: Option<&crate::mode06::Tests>, findings: &mut Vec<crate::findings::Finding>) {
    if p.id == "odyssey-2016" {
        findings.extend(odyssey::vcm_pattern(tests));
    }
    for f in findings.iter_mut() {
        if let Some(note) = f.code.as_deref().and_then(|c| dtc_note(p, c)) {
            // Safety advice (stop driving, flashing light) stays first; the car-specific note follows it.
            f.recommendation = if f.severity == crate::findings::Severity::Critical || f.code.as_deref().is_some_and(crate::findings::is_misfire) {
                format!("{} On this car: {note}", f.recommendation)
            } else {
                note.into()
            };
        }
    }
    if p.id == "kona-ev-2020" {
        findings.extend(kona::findings(extended));
    }
    if p.id == "clarity-phev-2018" {
        clarity::adapt_engine_findings(findings);
        findings.extend(clarity::findings(extended));
    }
}

pub fn dtc_note(p: &Profile, code: &str) -> Option<&'static str> {
    p.known_dtcs.iter().find(|d| d.code == code).map(|d| d.note)
}

pub fn resets(p: &Profile) -> &'static [Reset] {
    p.resets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_profiles_from_vin() {
        assert_eq!(detect("5FNRL5H60GB000001").map(|p| p.id), Some("odyssey-2016"));
        assert_eq!(detect("JHLRD78834C000001").map(|p| p.id), Some("crv-2004"));
        assert_eq!(detect("SHSRD78874U000001").map(|p| p.id), Some("crv-2004"));
        assert_eq!(detect("KM8K53AG0LU000001").map(|p| p.id), Some("kona-ev-2020"));
    }

    #[test]
    fn other_years_and_makes_are_not_matched() {
        assert!(detect("5FNRL5H6XHB000001").is_none()); // 2017 Odyssey
        assert!(detect("1HGCM82633A004352").is_none());
        assert!(detect("SHORT").is_none());
    }

    #[test]
    fn every_profile_offers_code_clearing_via_carlink() {
        for p in PROFILES {
            let r = resets(p);
            assert!(r.iter().any(|x| x.carlink_can_do && x.steps.iter().any(|s| s.contains("carlink clear"))), "{}", p.id);
            assert!(r.iter().filter(|x| x.carlink_can_do).count() == 1, "only clearing is done by carlink: {}", p.id);
        }
    }

    #[test]
    fn vehicle_dtc_note_found_only_for_known_codes() {
        let p = by_id("kona-ev-2020").unwrap();
        assert!(dtc_note(p, "P0AA6").is_some());
        assert!(dtc_note(p, "P0420").is_none());
    }

    #[test]
    fn annotate_replaces_recommendation_for_known_code() {
        let p = by_id("kona-ev-2020").unwrap();
        let mut f = vec![crate::findings::Finding { severity: crate::findings::Severity::Warning, code: Some("P0AA6".into()), title: "t".into(), detail: "d".into(), recommendation: "generic".into() }];
        let scan = crate::scan::scan(&mut crate::adapter::Adapter::connect(crate::sim::Sim::new("kona-ev-2020").unwrap()).unwrap()).unwrap();
        annotate(p, &scan, &[], None, &mut f);
        assert_eq!(f[0].recommendation, dtc_note(p, "P0AA6").unwrap());
    }

    #[test]
    fn safety_advice_survives_the_vehicle_note() {
        let p = by_id("kona-ev-2020").unwrap();
        let mut f = vec![crate::findings::Finding { severity: crate::findings::Severity::Critical, code: Some("P0AA6".into()), title: "t".into(), detail: "d".into(), recommendation: "Do not drive or charge.".into() }];
        let scan = crate::scan::scan(&mut crate::adapter::Adapter::connect(crate::sim::Sim::new("kona-ev-2020").unwrap()).unwrap()).unwrap();
        annotate(p, &scan, &[], None, &mut f);
        assert!(f[0].recommendation.starts_with("Do not drive or charge."));
        assert!(f[0].recommendation.contains("On this car:"));
    }

    #[test]
    fn kona_profile_needs_the_electric_vin_pattern() {
        assert_eq!(detect("KM8K53AG0LU000001").map(|p| p.id), Some("kona-ev-2020"));
        assert!(detect("KM8K2CAA3LU123456").is_none(), "gasoline 2020 Kona must not match the EV profile");
    }

    #[test]
    fn odyssey_misfire_gets_vcm_warranty_note_in_report() {
        let mut a = crate::adapter::Adapter::connect(crate::sim::Sim::new("odyssey-2016").unwrap()).unwrap();
        let r = crate::report::build(&mut a, &crate::report::Ident::default()).unwrap();
        let f = r.findings.iter().find(|f| f.code.as_deref() == Some("P0303")).unwrap();
        assert!(f.recommendation.contains("25-061"), "{}", f.recommendation);
        assert!(f.recommendation.to_lowercase().contains("flashing"), "{}", f.recommendation);
        assert!(dtc_note(by_id("odyssey-2016").unwrap(), "P0300").is_some_and(|n| !n.contains("25-061")));
    }

    #[test]
    fn vin_info_decodes_make_and_model_year() {
        let i = vin_info("5FNRL5H60GB000001");
        assert_eq!((i.make, i.model_year, i.check_digit_ok), (Some("Honda"), Some(2016), true));
        assert_eq!(vin_info("KM8K53AG0LU000001").make, Some("Hyundai"));
        assert_eq!(vin_info("KM8K53AG0LU000001").model_year, Some(2020));
        assert_eq!(vin_info("JHLRD78834C000001").model_year, Some(2004));
        assert_eq!(vin_info("WVWZZZ1JZXW000001").make, None);
    }

    #[test]
    fn clarity_plug_in_hybrid_is_detected_from_its_vin_and_not_the_electric() {
        assert_eq!(detect(clarity::SIM_VIN).map(|p| p.id), Some("clarity-phev-2018"));
        assert_eq!(detect("JHMZC5F18JC000001").map(|p| p.id), Some("clarity-phev-2018"), "base trim");
        assert!(detect("JHMZC5F36KC000001").is_none(), "2019");
        assert_eq!(vin_info(clarity::SIM_VIN).make, Some("Honda"));
        assert_eq!(by_id("clarity-phev-2018").unwrap().powertrain, Powertrain::PluginHybrid);
        assert_eq!(serde_json::to_value(Powertrain::PluginHybrid).unwrap(), "plug_in_hybrid");
    }

    #[test]
    fn clarity_report_carries_battery_data_and_the_engine_codes() {
        let mut a = crate::adapter::Adapter::connect(crate::sim::Sim::new("clarity-phev-2018").unwrap()).unwrap();
        let r = crate::report::build(&mut a, &crate::report::Ident::default()).unwrap();
        assert_eq!(r.vehicle.unwrap().id, "clarity-phev-2018");
        assert!(r.extended.iter().any(|x| x.key == "hv_capacity_ah"));
        assert!(r.codes.iter().any(|c| c.code == "P0456"));
    }

    #[test]
    fn profiles_resolve_by_id() {
        assert_eq!(by_id("kona-ev-2020").unwrap().powertrain, Powertrain::Ev);
        assert!(by_id("nope").is_none());
        assert_eq!(PROFILES.len(), 4);
    }
}
