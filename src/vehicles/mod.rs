//! The three vehicles carlink knows in depth. Facts cite the wiki in `kb/wiki/`.

mod crv;
pub mod kona;
mod odyssey;

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Powertrain {
    Gasoline,
    Ev,
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
    #[serde(skip)]
    known_dtcs: &'static [KnownDtc],
    #[serde(skip)]
    resets: &'static [Reset],
    /// (VIN prefix, model-year character at VIN position 10)
    #[serde(skip)]
    vin: &'static [(&'static str, char)],
}

pub static PROFILES: &[Profile] = &[crv::PROFILE, odyssey::PROFILE, kona::PROFILE];

const CLEAR_STEPS: &[&str] = &[
    "Ignition ON, engine OFF (EV: not required to be READY).",
    "carlink clear --vehicle <id> --confirm-clear-codes --note \"<why>\"",
    "Readiness monitors restart; drive normally for several days before an emissions inspection.",
];

pub(crate) const fn clear_reset(notes: &'static str) -> Reset {
    Reset { name: "Clear trouble codes / check-engine light (OBD service 04)", carlink_can_do: true, steps: CLEAR_STEPS, notes }
}

pub fn by_id(id: &str) -> Option<&'static Profile> {
    PROFILES.iter().find(|p| p.id == id)
}

pub fn detect(vin: &str) -> Option<&'static Profile> {
    let year = vin.chars().nth(9)?;
    PROFILES.iter().find(|p| p.vin.iter().any(|(prefix, y)| vin.starts_with(prefix) && year == *y))
}

/// Vehicle-specific reads beyond generic OBD-II. Failures become info findings, never abort the scan.
pub fn extended<T: crate::adapter::Transport>(p: &Profile, a: &mut crate::adapter::Adapter<T>) -> (Vec<crate::obd::Reading>, Vec<crate::findings::Finding>) {
    let (readings, errors) = match p.id {
        "kona-ev-2020" => kona::read(a),
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
pub fn annotate(p: &Profile, _scan: &crate::scan::Scan, extended: &[crate::obd::Reading], findings: &mut Vec<crate::findings::Finding>) {
    for f in findings.iter_mut() {
        if let Some(note) = f.code.as_deref().and_then(|c| dtc_note(p, c)) {
            f.recommendation = note.into();
        }
    }
    if p.id == "kona-ev-2020" {
        findings.extend(kona::findings(extended));
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
        assert_eq!(detect("5FNRL5H6XGB000001").map(|p| p.id), Some("odyssey-2016"));
        assert_eq!(detect("JHLRD78874C000001").map(|p| p.id), Some("crv-2004"));
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
        annotate(p, &scan, &[], &mut f);
        assert_eq!(f[0].recommendation, dtc_note(p, "P0AA6").unwrap());
    }

    #[test]
    fn odyssey_misfire_gets_vcm_warranty_note_in_report() {
        let mut a = crate::adapter::Adapter::connect(crate::sim::Sim::new("odyssey-2016").unwrap()).unwrap();
        let r = crate::report::build(&mut a, None).unwrap();
        let f = r.findings.iter().find(|f| f.code.as_deref() == Some("P0303")).unwrap();
        assert!(f.recommendation.contains("25-061"), "{}", f.recommendation);
    }

    #[test]
    fn profiles_resolve_by_id() {
        assert_eq!(by_id("kona-ev-2020").unwrap().powertrain, Powertrain::Ev);
        assert!(by_id("nope").is_none());
        assert_eq!(PROFILES.len(), 3);
    }
}
