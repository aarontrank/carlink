//! Live data over time: sample a few readings repeatedly and summarise each (min / max / average).

use crate::adapter::{Adapter, Transport};
use crate::obd;
use crate::problem::problem;
use crate::scan;
use crate::vehicles::{Profile, kona};
use anyhow::Result;
use serde::Serialize;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

pub const MAX_SAMPLES: usize = 600;

const DEFAULT_ENGINE: &[&str] = &[
    "rpm", "speed", "coolant_temp", "engine_load", "stft_b1", "ltft_b1", "stft_b2", "ltft_b2", "maf", "map", "throttle",
    "timing_advance", "module_voltage",
];
const DEFAULT_KONA: &[&str] = &["soc_bms", "pack_voltage", "pack_current", "batt_temp_max", "cell_v_max", "cell_v_min", "aux_12v_bms"];

#[derive(Debug, Serialize)]
pub struct Stats {
    pub name: &'static str,
    pub unit: &'static str,
    pub samples: usize,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub last: f64,
}

#[derive(Debug, Serialize)]
pub struct Sample {
    pub t_ms: u64,
    pub values: BTreeMap<String, f64>,
}

#[derive(Debug, Serialize)]
pub struct Watch {
    pub stats: BTreeMap<String, Stats>,
    pub series: Vec<Sample>,
    pub errors: Vec<String>,
}

/// Errors after which further samples cannot succeed.
fn fatal(e: &anyhow::Error) -> bool {
    crate::problem::find(e).is_some_and(|p| matches!(p.kind, "adapter_disconnected" | "adapter_addressing" | "adapter_not_found" | "adapter_timeout"))
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

pub fn watch<T: Transport>(a: &mut Adapter<T>, profile: Option<&Profile>, keys: &[String], samples: usize, interval: Duration) -> Result<Watch> {
    if samples == 0 || samples > MAX_SAMPLES {
        return Err(problem("bad_input", format!("--samples must be 1..={MAX_SAMPLES}"), "Use fewer samples, e.g. --samples 60 --interval-ms 500 for 30 seconds."));
    }
    let supported = scan::supported(a)?;
    let is_supported = |p: &u8| supported.values().any(|s| s.contains(p));
    let generic: Vec<&'static str> = obd::pid_keys().filter(|k| obd::pids_for_key(k).iter().any(is_supported)).collect();
    let ev_keys: &[&str] = if profile.is_some_and(|p| p.id == "kona-ev-2020") { kona::LIVE_KEYS } else { &[] };
    let wanted: Vec<String> = if keys.is_empty() {
        let defaults = if ev_keys.is_empty() { DEFAULT_ENGINE } else { DEFAULT_KONA };
        defaults.iter().filter(|k| generic.contains(k) || ev_keys.contains(k)).map(|k| k.to_string()).collect()
    } else {
        keys.to_vec()
    };
    let mut available: Vec<&str> = generic.iter().chain(ev_keys).copied().collect();
    available.sort();
    available.dedup();
    if let Some(bad) = wanted.iter().find(|k| !available.contains(&k.as_str())) {
        return Err(problem(
            "bad_input",
            format!("'{bad}' cannot be watched on this car; available: {}", available.join(", ")),
            "Pick keys from the available list (or leave --keys out for a sensible default).",
        ));
    }
    let start = Instant::now();
    let mut series = Vec::new();
    let mut errors = Vec::new();
    let mut meta: BTreeMap<String, (&'static str, &'static str)> = BTreeMap::new();
    for i in 0..samples {
        if i > 0 {
            std::thread::sleep(interval);
        }
        let mut values = BTreeMap::new();
        let mut pids: Vec<u8> = Vec::new();
        for pid in wanted.iter().filter(|k| !ev_keys.contains(&k.as_str())).filter_map(|k| obd::pids_for_key(k).into_iter().find(is_supported)) {
            if !pids.contains(&pid) {
                pids.push(pid);
            }
        }
        for pid in pids {
            match a.query(&format!("01{pid:02X}")) {
                Ok(msgs) => {
                    let mine = msgs.iter().filter(|m| m.data.get(..2) == Some(&[0x41, pid][..]));
                    let pick = mine.clone().find(|m| matches!(m.ecu.as_str(), "7E8" | "10")).or_else(|| mine.clone().next());
                    for r in pick.map(|m| obd::decode_pid_all(pid, &m.data[2..])).unwrap_or_default() {
                        if wanted.contains(&r.key) {
                            meta.insert(r.key.clone(), (r.name, r.unit));
                            values.insert(r.key, r.value);
                        }
                    }
                }
                Err(e) if fatal(&e) => return Err(e),
                Err(e) => errors.push(format!("sample {i} pid {pid:02X}: {e}")),
            }
        }
        if wanted.iter().any(|k| ev_keys.contains(&k.as_str())) {
            match kona::bms_live(a) {
                Ok(rs) => {
                    for r in rs.into_iter().filter(|r| wanted.contains(&r.key)) {
                        meta.insert(r.key.clone(), (r.name, r.unit));
                        values.insert(r.key, r.value);
                    }
                }
                Err(e) if fatal(&e) => return Err(e),
                Err(e) => errors.push(format!("sample {i} battery: {e}")),
            }
        }
        series.push(Sample { t_ms: start.elapsed().as_millis() as u64, values });
    }
    let mut stats = BTreeMap::new();
    for key in &wanted {
        let vals: Vec<f64> = series.iter().filter_map(|s| s.values.get(key).copied()).collect();
        let (Some(&last), Some((name, unit))) = (vals.last(), meta.get(key)) else { continue };
        stats.insert(key.clone(), Stats {
            name,
            unit,
            samples: vals.len(),
            min: vals.iter().copied().fold(f64::MAX, f64::min),
            max: vals.iter().copied().fold(f64::MIN, f64::max),
            avg: round2(vals.iter().sum::<f64>() / vals.len() as f64),
            last,
        });
    }
    Ok(Watch { stats, series, errors })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::Adapter;
    use crate::sim::Sim;

    fn keys(k: &[&str]) -> Vec<String> {
        k.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn samples_generic_pids_and_summarises_them() {
        let mut a = Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap();
        let w = watch(&mut a, None, &keys(&["rpm", "coolant_temp"]), 3, Duration::ZERO).unwrap();
        let rpm = &w.stats["rpm"];
        assert_eq!((rpm.samples, rpm.min, rpm.max, rpm.avg, rpm.unit), (3, 720.0, 720.0, 720.0, "rpm"));
        assert_eq!(w.series.len(), 3);
        assert_eq!(w.series[0].values["coolant_temp"], 50.0);
    }

    #[test]
    fn default_keys_are_the_supported_engine_pids() {
        let mut a = Adapter::connect(Sim::new("crv-2004").unwrap()).unwrap();
        let w = watch(&mut a, None, &[], 1, Duration::ZERO).unwrap();
        assert!(w.stats.contains_key("rpm") && w.stats.contains_key("stft_b1"), "{:?}", w.stats.keys());
        assert!(!w.stats.contains_key("maf"));
    }

    #[test]
    fn kona_battery_keys_come_from_the_bms() {
        let mut a = Adapter::connect(Sim::new("kona-ev-2020").unwrap()).unwrap();
        let p = crate::vehicles::by_id("kona-ev-2020");
        let w = watch(&mut a, p, &keys(&["pack_current", "soc_bms"]), 2, Duration::ZERO).unwrap();
        assert_eq!(w.stats["soc_bms"].last, 36.5);
        let w = watch(&mut a, p, &[], 1, Duration::ZERO).unwrap();
        assert!(w.stats.contains_key("pack_voltage"));
    }

    #[test]
    fn unknown_or_unsupported_key_is_refused_with_the_valid_list() {
        let mut a = Adapter::connect(Sim::new("crv-2004").unwrap()).unwrap();
        let err = watch(&mut a, None, &keys(&["warp_drive"]), 1, Duration::ZERO).unwrap_err();
        let p = crate::problem::find(&err).unwrap();
        assert_eq!(p.kind, "bad_input");
        assert!(err.to_string().contains("rpm"), "{err}");
        assert!(watch(&mut a, None, &keys(&["maf"]), 1, Duration::ZERO).is_err());
    }

    #[test]
    fn a_key_sharing_a_pid_with_another_is_requested_once_per_sample() {
        let mut a = Adapter::connect(Sim::new("odyssey-2016").unwrap()).unwrap();
        let before = a.transport().log.len();
        watch(&mut a, None, &keys(&["wb_b1s1_lambda", "rpm", "wb_b1s1_ma"]), 1, Duration::ZERO).unwrap();
        let sent: Vec<&String> = a.transport().log[before..].iter().filter(|c| *c == "0134").collect();
        assert_eq!(sent.len(), 1);
    }

    #[test]
    fn too_many_samples_is_refused() {
        let mut a = Adapter::connect(Sim::new("crv-2004").unwrap()).unwrap();
        assert!(watch(&mut a, None, &keys(&["rpm"]), 10_000, Duration::ZERO).is_err());
    }
}
