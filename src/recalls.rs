//! Recalls NHTSA lists for a car's make, model and year, merged with the profile's own list.
//! What the API does and does not say: wiki `nhtsa-recalls-api`.

use crate::vehicles::Profile;
use serde::Serialize;

pub const NHTSA: &str = "https://api.nhtsa.gov";

pub const OPEN_STATUS: &str = "NHTSA lists the recalls for the model and year, not whether one was done on this car: check the VIN at nhtsa.gov/recalls or ask a dealer.";

#[derive(Debug, Serialize, PartialEq)]
pub struct Recall {
    /// Short form, e.g. 21V-127.
    pub campaign: String,
    pub nhtsa_campaign: String,
    /// yyyy-mm-dd
    pub report_date: Option<String>,
    pub component: String,
    pub summary: String,
    pub consequence: String,
    pub remedy: String,
    pub park_it: bool,
    pub park_outside: bool,
    pub over_the_air: bool,
    pub in_carlink_list: bool,
}

#[derive(Debug, Serialize)]
pub struct Query {
    pub make: &'static str,
    pub model: &'static str,
    pub model_year: u16,
}

#[derive(Debug, Serialize)]
pub struct Lookup {
    pub vehicle: &'static str,
    pub query: Query,
    /// `nhtsa` (live list) or `carlink` (NHTSA unreachable or its reply unreadable: only the built-in list).
    pub source: &'static str,
    pub fetch_error: Option<String>,
    /// NHTSA's live list, newest first; empty when `source` is `carlink`.
    pub recalls: Vec<Recall>,
    pub carlink_list: &'static [&'static str],
    /// Campaigns in carlink's list that NHTSA's reply did not include.
    pub carlink_only: Vec<String>,
    pub warnings: Vec<String>,
    pub open_status: &'static str,
}

/// Campaign ids (`21V127`, `07E104`) mentioned in free text, in order.
pub fn campaign_ids(text: &str) -> Vec<String> {
    let b = text.as_bytes();
    let digits = |r: std::ops::Range<usize>| b.get(r).is_some_and(|s| s.iter().all(u8::is_ascii_digit));
    let mut out = Vec::new();
    for i in 0..b.len() {
        if i > 0 && b[i - 1].is_ascii_alphanumeric() || !digits(i..i + 2) || !b.get(i + 2).is_some_and(|c| b"VETC".contains(c)) {
            continue;
        }
        let n = i + 3 + usize::from(b.get(i + 3) == Some(&b'-'));
        if digits(n..n + 3) && !b.get(n + 3).is_some_and(u8::is_ascii_alphanumeric) {
            out.push(format!("{}{}", &text[i..i + 3], &text[n..n + 3]));
        }
    }
    out
}

fn short(nhtsa: &str) -> String {
    match nhtsa.get(..6) {
        Some(s) if nhtsa.is_ascii() => format!("{}-{}", &s[..3], &s[3..]),
        _ => nhtsa.to_string(),
    }
}

pub fn parse(body: &[u8]) -> Result<Vec<Recall>, String> {
    let v: serde_json::Value = serde_json::from_slice(body).map_err(|e| format!("NHTSA's reply is not JSON: {e}"))?;
    let results = v["results"].as_array().ok_or_else(|| format!("NHTSA's reply has no results: {}", v["message"].as_str().or(v["Message"].as_str()).unwrap_or("no message")))?;
    let text = |r: &serde_json::Value, k: &str| r[k].as_str().unwrap_or_default().trim().to_string();
    Ok(results
        .iter()
        .map(|r| {
            let nhtsa_campaign = text(r, "NHTSACampaignNumber");
            // NHTSA writes dd/mm/yyyy.
            let date = text(r, "ReportReceivedDate");
            let d: Vec<&str> = date.split('/').collect();
            let num = |s: &str, len: usize, max: u32| s.len() == len && s.parse::<u32>().is_ok_and(|n| (1..=max).contains(&n));
            let dated = d.len() == 3 && num(d[0], 2, 31) && num(d[1], 2, 12) && num(d[2], 4, 9999);
            Recall {
                campaign: short(&nhtsa_campaign),
                nhtsa_campaign,
                report_date: dated.then(|| format!("{}-{}-{}", d[2], d[1], d[0])),
                component: text(r, "Component"),
                summary: text(r, "Summary"),
                consequence: text(r, "Consequence"),
                remedy: text(r, "Remedy"),
                park_it: r["parkIt"].as_bool().unwrap_or(false),
                park_outside: r["parkOutSide"].as_bool().unwrap_or(false),
                over_the_air: r["overTheAirUpdate"].as_bool().unwrap_or(false),
                in_carlink_list: false,
            }
        })
        .collect())
}

pub fn url(base: &str, p: &Profile) -> String {
    let (make, model, year) = p.nhtsa;
    format!("{base}/recalls/recallsByVehicle?make={make}&model={}&modelYear={year}", model.replace(' ', "%20"))
}

pub fn lookup(p: &'static Profile, fetched: Result<Vec<u8>, String>) -> Lookup {
    let (make, model, model_year) = p.nhtsa;
    let known: Vec<String> = p.recalls.iter().flat_map(|r| campaign_ids(r)).collect();
    let mut l = Lookup {
        vehicle: p.name,
        query: Query { make, model, model_year },
        source: "nhtsa",
        fetch_error: None,
        recalls: Vec::new(),
        carlink_list: p.recalls,
        carlink_only: Vec::new(),
        warnings: Vec::new(),
        open_status: OPEN_STATUS,
    };
    match fetched.and_then(|body| parse(&body)) {
        Err(e) => {
            l.source = "carlink";
            l.fetch_error = Some(e);
            l.warnings.push("NHTSA could not be reached, or its reply could not be read: this is carlink's built-in list only. It may miss newer recalls, and NHTSA's current safety advice for each recall could not be checked. Check the VIN at nhtsa.gov/recalls.".into());
        }
        Ok(mut recalls) => {
            let mut matched = vec![false; known.len()];
            for r in &mut recalls {
                for (i, k) in known.iter().enumerate() {
                    if r.nhtsa_campaign.starts_with(k.as_str()) {
                        matched[i] = true;
                        r.in_carlink_list = true;
                    }
                }
            }
            // yyyy-mm-dd sorts as text; undated last.
            recalls.sort_by(|a, b| b.report_date.cmp(&a.report_date));
            l.carlink_only = known.iter().zip(&matched).filter(|(_, m)| !**m).map(|(k, _)| short(k)).collect();
            if !l.carlink_only.is_empty() && recalls.is_empty() {
                l.warnings.push(format!("NHTSA's reply listed nothing for this query, which does not mean there is nothing: carlink's list has {}. Treat each as a recall to check by VIN.", l.carlink_only.join(", ")));
            } else if !l.carlink_only.is_empty() {
                l.warnings.push(format!("NHTSA did not list {} for this make, model and year, but carlink's list has it: treat it as a recall to check by VIN as well.", l.carlink_only.join(", ")));
            }
            for r in recalls.iter().filter(|r| r.park_it || r.park_outside) {
                let what = if r.park_it { "do not drive the car" } else { "park outside and away from buildings" };
                l.warnings.push(format!("{}: NHTSA says {what} until this recall's repair is done. Check by VIN whether it was.", r.campaign));
            }
            l.recalls = recalls;
        }
    }
    l
}

/// The reply body from `url`, via the system's curl.
pub fn fetch(url: &str) -> Result<Vec<u8>, String> {
    let out = std::process::Command::new("curl").args(["-sS", "--fail", "--proto", "=https", "-m", "20", url]).output().map_err(|e| format!("could not run curl: {e}"))?;
    if out.status.success() {
        Ok(out.stdout)
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vehicles::{PROFILES, by_id};

    const ODYSSEY: &[u8] = include_bytes!("../kb/raw/nhtsa/recalls-honda-odyssey-2016.json");
    const KONA: &[u8] = include_bytes!("../kb/raw/nhtsa/recalls-hyundai-kona-electric-2020-2026-11-15.json");
    const CRV: &[u8] = include_bytes!("../kb/raw/nhtsa/recalls-honda-crv-2004.json");
    const CLARITY: &[u8] = include_bytes!("../kb/raw/nhtsa/recalls-honda-clarity-phev-2018.json");

    fn one(campaign: &str, date: &str, park_outside: bool) -> String {
        format!(r#"{{"NHTSACampaignNumber":"{campaign}","ReportReceivedDate":"{date}","Component":"C","Summary":"S","Consequence":"Q","Remedy":"R","parkIt":false,"parkOutSide":{park_outside},"overTheAirUpdate":false}}"#)
    }

    fn reply(items: &[String]) -> Vec<u8> {
        format!(r#"{{"Count":{},"Message":"Results returned successfully","results":[{}]}}"#, items.len(), items.join(",")).into_bytes()
    }

    #[test]
    fn campaign_ids_are_found_in_carlink_recall_text() {
        assert_eq!(campaign_ids("18V170, 17V725, 16V932, 16V933: 2nd-row seat latches."), ["18V170", "17V725", "16V932", "16V933"]);
        assert_eq!(campaign_ids("21V-127 (Hyundai 200): pack replacement"), ["21V127"]);
        assert_eq!(campaign_ids("12V486 / 20V768: switch; 07E104 tires"), ["12V486", "20V768", "07E104"]);
    }

    #[test]
    fn text_that_only_resembles_a_campaign_is_ignored() {
        assert!(campaign_ids("TSB 22-EV-001H-1: motor rumble. S/B 17-093. Hyundai 200. 121V1234").is_empty());
    }

    #[test]
    fn nhtsa_reply_is_parsed_with_short_campaign_and_iso_date() {
        let r = parse(ODYSSEY).unwrap();
        assert_eq!(r.len(), 6);
        let first = &r[0];
        assert_eq!((first.campaign.as_str(), first.nhtsa_campaign.as_str(), first.report_date.as_deref()), ("18V-170", "18V170000", Some("2018-03-13")));
        assert!(first.component.starts_with("SEATS"));
        assert!(!first.summary.is_empty() && !first.remedy.is_empty() && !first.consequence.is_empty());
    }

    #[test]
    fn a_campaign_number_too_short_to_shorten_is_kept_as_is() {
        assert_eq!(parse(br#"{"results":[{"NHTSACampaignNumber":"21V"}]}"#).unwrap()[0].campaign, "21V");
    }

    #[test]
    fn a_reply_without_results_is_an_error() {
        assert!(parse(b"{\"message\":\"Missing Authentication Token\"}").is_err());
        assert!(parse(b"<html>").is_err());
    }

    #[test]
    fn a_live_list_matching_carlink_has_no_warnings() {
        let l = lookup(by_id("odyssey-2016").unwrap(), Ok(ODYSSEY.to_vec()));
        assert_eq!(l.source, "nhtsa");
        assert!(l.recalls.iter().all(|r| r.in_carlink_list));
        assert!(l.carlink_only.is_empty() && l.warnings.is_empty() && l.fetch_error.is_none());
    }

    #[test]
    fn live_recalls_come_newest_first() {
        let l = lookup(by_id("odyssey-2016").unwrap(), Ok(ODYSSEY.to_vec()));
        assert_eq!(l.recalls[0].campaign, "23V-858");
        assert_eq!(l.recalls[5].campaign, "16V-417");
    }

    #[test]
    fn a_recall_carlink_does_not_list_is_marked_new() {
        let l = lookup(by_id("clarity-phev-2018").unwrap(), Ok(reply(&[one("23V858000", "19/12/2023", false), one("26V900000", "01/11/2026", false)])));
        let new: Vec<_> = l.recalls.iter().filter(|r| !r.in_carlink_list).map(|r| r.campaign.as_str()).collect();
        assert_eq!(new, ["26V-900"]);
    }

    #[test]
    fn carlink_recalls_missing_from_the_live_list_are_named_and_warned() {
        let l = lookup(by_id("clarity-phev-2018").unwrap(), Ok(reply(&[])));
        assert_eq!(l.source, "nhtsa");
        assert_eq!(l.carlink_only, ["23V-858"]);
        assert!(l.warnings.iter().any(|w| w.contains("23V-858")), "{:?}", l.warnings);
    }

    #[test]
    fn an_empty_live_list_with_carlink_recalls_is_not_called_no_recalls() {
        let l = lookup(by_id("clarity-phev-2018").unwrap(), Ok(reply(&[])));
        assert!(l.warnings.iter().any(|w| w.contains("does not mean there is nothing")), "{:?}", l.warnings);
    }

    #[test]
    fn unreachable_nhtsa_falls_back_to_carlink_list_with_a_warning() {
        let p = by_id("kona-ev-2020").unwrap();
        let l = lookup(p, Err("curl: (6) Could not resolve host".into()));
        assert_eq!(l.source, "carlink");
        assert!(l.recalls.is_empty() && l.carlink_only.is_empty());
        assert_eq!(l.carlink_list, p.recalls);
        assert!(l.fetch_error.as_deref().unwrap().contains("resolve host"));
        assert!(l.warnings.iter().any(|w| w.contains("newer")), "{:?}", l.warnings);
    }

    #[test]
    fn an_unreadable_reply_is_treated_like_an_unreachable_nhtsa() {
        let l = lookup(by_id("kona-ev-2020").unwrap(), Ok(b"<html>".to_vec()));
        assert_eq!(l.source, "carlink");
        assert!(l.fetch_error.is_some());
    }

    #[test]
    fn a_park_outside_recall_is_warned() {
        let l = lookup(by_id("kona-ev-2020").unwrap(), Ok(reply(&[one("21V127000", "03/01/2021", true)])));
        assert!(l.warnings.iter().any(|w| w.starts_with("21V-127") && w.contains("outside")), "{:?}", l.warnings);
    }

    #[test]
    fn the_real_kona_reply_warns_to_park_outside_for_both_battery_recalls() {
        let l = lookup(by_id("kona-ev-2020").unwrap(), Ok(KONA.to_vec()));
        let parked: Vec<_> = l.recalls.iter().filter(|r| r.park_outside).map(|r| r.campaign.as_str()).collect();
        assert_eq!(parked, ["21V-127", "20V-630"]);
        assert_eq!(l.warnings.len(), 2);
    }

    #[test]
    fn offline_the_warning_says_park_and_drive_advice_was_not_checked() {
        let l = lookup(by_id("kona-ev-2020").unwrap(), Err("offline".into()));
        assert!(l.warnings[0].contains("safety advice"), "{:?}", l.warnings);
    }

    #[test]
    fn every_campaign_nhtsa_lists_for_each_saved_reply_is_in_carlinks_list() {
        for (id, body) in [("odyssey-2016", ODYSSEY), ("kona-ev-2020", KONA), ("crv-2004", CRV), ("clarity-phev-2018", CLARITY)] {
            let l = lookup(by_id(id).unwrap(), Ok(body.to_vec()));
            let new: Vec<_> = l.recalls.iter().filter(|r| !r.in_carlink_list).map(|r| r.campaign.clone()).collect();
            assert!(new.is_empty() && l.carlink_only.is_empty(), "{id}: new {new:?}, carlink_only {:?}", l.carlink_only);
        }
    }

    #[test]
    fn a_malformed_report_date_is_dropped() {
        for bad in ["1/3/2021", "March/01/2021", "01/13/2021"] {
            assert_eq!(parse(&reply(&[one("21V127000", bad, false)])).unwrap()[0].report_date, None, "{bad}");
        }
    }

    #[test]
    fn a_park_it_recall_says_not_to_drive() {
        let body = String::from_utf8(reply(&[one("21V127000", "03/01/2021", false)])).unwrap().replace(r#""parkIt":false"#, r#""parkIt":true"#);
        let l = lookup(by_id("kona-ev-2020").unwrap(), Ok(body.into_bytes()));
        assert!(l.warnings.iter().any(|w| w.contains("do not drive")), "{:?}", l.warnings);
    }

    #[test]
    fn an_undated_recall_sorts_last() {
        let l = lookup(by_id("odyssey-2016").unwrap(), Ok(reply(&[one("16V417000", "", false), one("23V858000", "19/12/2023", false)])));
        assert_eq!((l.recalls[0].report_date.as_deref(), l.recalls[1].report_date.as_deref()), (Some("2023-12-19"), None));
    }

    #[test]
    fn the_query_url_encodes_the_model() {
        assert_eq!(url("https://x", by_id("kona-ev-2020").unwrap()), "https://x/recalls/recallsByVehicle?make=hyundai&model=kona%20electric&modelYear=2020");
    }

    #[test]
    fn every_profile_has_a_url_safe_nhtsa_query() {
        for p in PROFILES {
            let (make, model, year) = p.nhtsa;
            assert!(format!("{make}{model}").chars().all(|c| c.is_ascii_lowercase() || c == ' ' || c == '-'), "{}", p.id);
            assert!(p.name.starts_with(&year.to_string()), "{}", p.id);
        }
    }

    #[test]
    fn fetch_reports_a_failure() {
        let dir = tempfile::tempdir().unwrap();
        assert!(fetch(&format!("file://{}/missing", dir.path().display())).is_err());
    }
}
