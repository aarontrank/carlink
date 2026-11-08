use serde_json::Value;
use std::process::Command;

struct Env {
    _dir: tempfile::TempDir,
    db: String,
}

fn env() -> Env {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("h.db").to_string_lossy().into_owned();
    Env { _dir: dir, db }
}

fn run(e: &Env, args: &[&str]) -> (bool, Value) {
    let out = Command::new(env!("CARGO_BIN_EXE_carlink")).args(["--db", &e.db]).args(args).env_remove("CARLINK_PORT").output().unwrap();
    (out.status.success(), serde_json::from_slice(&out.stdout).unwrap_or_else(|_| panic!("not JSON: {}", String::from_utf8_lossy(&out.stdout))))
}

#[test]
fn scan_saves_session_and_history_reads_it_back() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:odyssey-2016", "scan", "--odometer-mi", "98000"]);
    assert!(ok, "{r}");
    assert_eq!(r["vehicle"]["id"], "odyssey-2016");
    let id = r["session_id"].as_i64().unwrap();
    let (_, s) = run(&e, &["history", "sessions"]);
    assert_eq!(s[0]["id"].as_i64(), Some(id));
    assert_eq!(s[0]["odometer_mi"].as_f64(), Some(98000.0));
    let (_, t) = run(&e, &["history", "trend", "odyssey-2016", "battery_voltage"]);
    assert_eq!(t["points"][0]["value"].as_f64(), Some(14.1));
    let (_, d) = run(&e, &["history", "dtcs", "odyssey-2016"]);
    assert_eq!(d.as_array().unwrap().len(), 2);
}

#[test]
fn odometer_reported_by_car_fills_session_when_not_given() {
    let e = env();
    run(&e, &["--port", "sim:kona-ev-2020", "scan", "--vehicle", "kona-ev-2020"]);
    assert_eq!(run(&e, &["history", "sessions"]).1[0]["odometer_mi"].as_f64(), Some(40000.0));
}

#[test]
fn no_save_scan_leaves_history_empty() {
    let e = env();
    let (_, r) = run(&e, &["--port", "sim:kona-ev-2020", "scan", "--no-save"]);
    assert!(r["session_id"].is_null());
    assert_eq!(run(&e, &["history", "sessions"]).1.as_array().unwrap().len(), 0);
}

#[test]
fn clear_requires_confirmation_flag() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:crv-2004", "clear", "--vehicle", "crv-2004"]);
    assert!(!ok);
    assert!(r["error"].as_str().unwrap().contains("--confirm-clear-codes"));
}

#[test]
fn clear_on_running_engine_is_refused_and_records_nothing() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:crv-2004", "clear", "--vehicle", "crv-2004", "--confirm-clear-codes"]);
    assert!(!ok);
    assert!(r["error"].as_str().unwrap().contains("engine is running"), "{r}");
    assert!(run(&e, &["history", "sessions"]).1.as_array().unwrap().is_empty());
}

#[test]
fn ev_clear_succeeds_and_records_both_sessions() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:kona-ev-2020", "clear", "--confirm-clear-codes"]);
    assert!(!ok && r["error"].as_str().unwrap().contains("could not confirm"), "unknown vehicle must not be assumed electric: {r}");
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:kona-ev-2020", "clear", "--vehicle", "kona-ev-2020", "--confirm-clear-codes"]);
    assert!(ok, "{r}");
    assert_eq!(r["acknowledged_by"], serde_json::json!(["7EB", "7EC"]));
    let kinds: Vec<_> = run(&e, &["history", "sessions"]).1.as_array().unwrap().iter().map(|s| s["kind"].as_str().unwrap().to_string()).collect();
    assert_eq!(kinds, vec!["clear", "pre-clear"]);
}

#[test]
fn raw_refuses_non_read_services_without_connecting() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "/dev/does-not-exist", "raw", "1101"]);
    assert!(!ok);
    assert!(r["error"].as_str().unwrap().contains("not a read-only"));
}

#[test]
fn raw_with_header_reads_physical_ecu() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:kona-ev-2020", "raw", "220101", "--header", "7E4"]);
    assert!(ok, "{r}");
    assert_eq!(r["messages"][0]["ecu"], "7EC");
}

#[test]
fn unknown_vehicle_profile_is_an_error() {
    let e = env();
    let (ok, r) = run(&e, &["dtc", "P0420", "--vehicle", "delorean"]);
    assert!(!ok);
    assert!(r["error"].as_str().unwrap().contains("unknown vehicle"));
}

#[test]
fn dtc_lookup_without_hardware() {
    let e = env();
    let (ok, r) = run(&e, &["dtc", "p0420"]);
    assert!(ok);
    assert_eq!(r["code"], "P0420");
}

#[test]
fn probe_identifies_adapter_and_vehicle() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:odyssey-2016", "probe"]);
    assert!(ok, "{r}");
    assert_eq!(r["adapter"]["stn"], "STN2232 v5.10.3");
    assert_eq!(r["vehicle"]["id"], "odyssey-2016");
}

#[test]
fn vehicles_and_resets_list_profiles() {
    let e = env();
    let (_, v) = run(&e, &["vehicles"]);
    assert_eq!(v.as_array().unwrap().len(), 3);
    let (ok, r) = run(&e, &["resets", "--vehicle", "kona-ev-2020"]);
    assert!(ok);
    assert!(r["resets"].as_array().unwrap().iter().any(|x| x["carlink_can_do"] == true));
}

#[test]
fn history_show_and_keys() {
    let e = env();
    let id = run(&e, &["--port", "sim:crv-2004", "scan", "--vehicle", "crv-2004"]).1["session_id"].as_i64().unwrap();
    let (_, s) = run(&e, &["history", "show", &id.to_string()]);
    assert_eq!(s["report"]["vehicle"]["id"], "crv-2004");
    assert!(run(&e, &["history", "keys", "crv-2004"]).1.as_array().unwrap().iter().any(|k| k == "coolant_temp"));
    let (ok, m) = run(&e, &["history", "show", "999"]);
    assert!(!ok && m["error"].as_str().unwrap().contains("no session"));
}

#[test]
fn raw_without_header_and_bad_header() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:crv-2004", "raw", "0105"]);
    assert!(ok);
    assert_eq!(r["messages"][0]["hex"], "41 05 7B");
    let (ok, r) = run(&e, &["--port", "sim:crv-2004", "raw", "0105", "--header", "18DAF110"]);
    assert!(!ok && r["error"].as_str().unwrap().contains("3 hex digits"));
}

#[test]
fn ports_lists_simulators() {
    let e = env();
    let (ok, r) = run(&e, &["ports"]);
    assert!(ok);
    assert!(r["simulators"].as_array().unwrap().iter().any(|s| s == "sim:kona-ev-2020"));
}
