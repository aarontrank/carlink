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
    let (code, v) = run_code(e, args);
    (code == 0, v)
}

fn run_code(e: &Env, args: &[&str]) -> (i32, Value) {
    let out = Command::new(env!("CARGO_BIN_EXE_carlink")).args(["--db", &e.db]).args(args).env_remove("CARLINK_PORT").output().unwrap();
    let v = serde_json::from_slice(&out.stdout).unwrap_or_else(|_| panic!("not JSON: {}", String::from_utf8_lossy(&out.stdout)));
    (out.status.code().unwrap_or(-1), v)
}

const CRV_VIN: &str = "JHLRD78834C000001";
const KONA_VIN: &str = "KM8K53AG0LU000001";

fn kind(v: &Value) -> &str {
    v["error_kind"].as_str().unwrap_or_else(|| panic!("no error_kind in {v}"))
}

#[test]
fn scan_saves_session_keyed_by_vin_and_history_reads_it_back() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:odyssey-2016", "scan", "--odometer-mi", "98000"]);
    assert!(ok, "{r}");
    assert_eq!(r["vehicle"]["id"], "odyssey-2016");
    assert_eq!(r["vehicle_key"], "5FNRL5H60GB000001");
    let id = r["session_id"].as_i64().unwrap();
    let (_, s) = run(&e, &["history", "sessions"]);
    assert_eq!(s[0]["id"].as_i64(), Some(id));
    assert_eq!(s[0]["odometer_mi"].as_f64(), Some(98000.0));
    let (_, t) = run(&e, &["history", "trend", "5FNRL5H60GB000001", "battery_voltage"]);
    assert_eq!(t["points"][0]["value"].as_f64(), Some(14.1));
    let (_, d) = run(&e, &["history", "dtcs", "5FNRL5H60GB000001"]);
    assert_eq!(d.as_array().unwrap().len(), 2);
    let (_, v) = run(&e, &["history", "vehicles"]);
    assert_eq!((v[0]["key"].as_str(), v[0]["profile"].as_str()), (Some("5FNRL5H60GB000001"), Some("odyssey-2016")));
}

#[test]
fn scan_output_starts_with_the_summary() {
    let e = env();
    let out = Command::new(env!("CARGO_BIN_EXE_carlink")).args(["--db", &e.db, "--port", "sim:odyssey-2016", "scan"]).output().unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.trim_start().starts_with("{\n  \"summary\""), "{}", &text[..80]);
}

#[test]
fn vin_flag_keys_a_car_that_reports_no_vin() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:crv-2004", "scan", "--vehicle", "crv-2004", "--vin", "jhlrd78834c000001"]);
    assert!(ok, "{r}");
    assert_eq!(r["vehicle_key"], "JHLRD78834C000001");
}

#[test]
fn malformed_vin_flag_is_rejected_before_connecting() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "/dev/does-not-exist", "scan", "--vin", "12345"]);
    assert!(!ok);
    assert_eq!(kind(&r), "bad_input");
}

#[test]
fn odometer_reported_by_car_fills_session_when_not_given() {
    let e = env();
    run(&e, &["--port", "sim:kona-ev-2020", "scan", "--vehicle", "kona-ev-2020", "--vin", KONA_VIN]);
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
    assert_eq!(kind(&r), "confirmation_required");
    assert!(r["error"].as_str().unwrap().contains("--confirm-clear-codes"));
}

#[test]
fn clear_on_running_engine_is_refused_and_records_nothing() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:crv-2004", "clear", "--vehicle", "crv-2004", "--vin", CRV_VIN, "--confirm-clear-codes"]);
    assert!(!ok);
    assert_eq!(kind(&r), "engine_running");
    assert!(r["what_to_do"].as_str().unwrap().contains("engine"));
    assert!(run(&e, &["history", "sessions"]).1.as_array().unwrap().is_empty());
}

#[test]
fn clear_with_engine_off_is_verified_and_gives_next_steps() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:crv-2004:engine-off", "clear", "--vehicle", "crv-2004", "--vin", CRV_VIN, "--confirm-clear-codes"]);
    assert!(ok, "{r}");
    assert_eq!(r["clear_status"], "cleared");
    assert!(r["next_steps"].as_array().unwrap().iter().any(|s| s.as_str().unwrap().contains("idle learn")), "{r}");
}

#[test]
fn clear_without_a_vin_is_refused() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:crv-2004:engine-off", "clear", "--vehicle", "crv-2004", "--confirm-clear-codes"]);
    assert!(!ok);
    assert_eq!(kind(&r), "vin_required");
}

#[test]
fn ev_clear_needs_the_vehicle_and_names_modules_explicitly() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:kona-ev-2020", "clear", "--vin", KONA_VIN, "--confirm-clear-codes"]);
    assert!(ok, "{r}");
    assert_eq!(r["vehicle"]["profile"], "kona-ev-2020", "the VIN identifies the Kona Electric: {r}");
    assert_eq!(r["clear_status"], "nothing_to_clear");
    let (ok, r) = run(&e, &["--port", "sim:kona-ev-2020", "clear", "--vehicle", "kona-ev-2020", "--vin", KONA_VIN, "--only", "IEB", "--confirm-clear-codes"]);
    assert!(ok, "{r}");
    assert_eq!(r["clear_status"], "cleared");
    assert_eq!(r["targets"][0]["target"], "IEB");
    assert_eq!(r["targets"][0]["acknowledged_by"], serde_json::json!(["7D9"]));
    let kinds: Vec<_> = run(&e, &["history", "sessions"]).1.as_array().unwrap().iter().map(|s| s["kind"].as_str().unwrap().to_string()).collect();
    assert_eq!(kinds, vec!["clear", "pre-clear"]);
}

#[test]
fn mistyped_vin_fails_the_check_digit() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:crv-2004", "scan", "--vehicle", "crv-2004", "--vin", "JHLRD78B34C000001"]);
    assert!(!ok);
    assert!(r["error"].as_str().unwrap().contains("check digit"), "{r}");
    let (ok, _) = run(&e, &["--port", "sim:crv-2004", "scan", "--vehicle", "crv-2004", "--vin", "JHLRD78B34C000001", "--vin-no-check"]);
    assert!(ok);
}

#[test]
fn scan_without_a_vin_is_not_saved() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:crv-2004", "scan", "--vehicle", "crv-2004"]);
    assert!(ok);
    assert!(r["session_id"].is_null());
    assert!(run(&e, &["history", "sessions"]).1.as_array().unwrap().is_empty());
}

#[test]
fn ignition_off_is_reported_with_what_to_do() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:odyssey-2016:ignition-off", "scan"]);
    assert!(!ok);
    assert_eq!(kind(&r), "vehicle_not_responding");
    assert!(r["what_to_do"].as_str().unwrap().contains("ignition"));
}

#[test]
fn missing_adapter_is_reported_with_what_to_do() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "/dev/cu.does-not-exist", "scan"]);
    assert!(!ok);
    assert_eq!(kind(&r), "adapter_not_found", "{r}");
}

#[test]
fn usage_errors_are_json_too() {
    let e = env();
    let (code, r) = run_code(&e, &["scna"]);
    assert_eq!(code, 2);
    assert_eq!(kind(&r), "usage");
    assert!(r["what_to_do"].as_str().unwrap().contains("carlink --help"));
}

#[test]
fn help_is_not_an_error() {
    let out = Command::new(env!("CARGO_BIN_EXE_carlink")).arg("--help").output().unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("scan"));
}

#[test]
fn raw_refuses_non_read_services_without_connecting() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "/dev/does-not-exist", "raw", "1101"]);
    assert!(!ok);
    assert_eq!(kind(&r), "request_not_allowed");
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
    assert_eq!(kind(&r), "unknown_vehicle");
}

#[test]
fn dtc_typo_with_letter_o_is_explained() {
    let e = env();
    let (ok, r) = run(&e, &["dtc", "PO420"]);
    assert!(!ok);
    assert_eq!(kind(&r), "bad_input");
    assert!(r["error"].as_str().unwrap().contains("P0420"));
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
fn probe_failure_names_the_broken_link_and_exits_non_zero() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:odyssey-2016:not-in-car", "probe"]);
    assert!(!ok);
    assert_eq!(kind(&r), "probe_power_at_obd_port");
    assert_eq!(r["checks"][0]["ok"], true);
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
    let id = run(&e, &["--port", "sim:crv-2004", "scan", "--vehicle", "crv-2004", "--vin", CRV_VIN]).1["session_id"].as_i64().unwrap();
    let (_, s) = run(&e, &["history", "show", &id.to_string()]);
    assert_eq!(s["report"]["vehicle"]["id"], "crv-2004");
    assert!(run(&e, &["history", "keys", CRV_VIN]).1.as_array().unwrap().iter().any(|k| k == "coolant_temp"));
    let (ok, m) = run(&e, &["history", "show", "999"]);
    assert!(!ok);
    assert_eq!(kind(&m), "not_found");
}

#[test]
fn raw_without_header_and_bad_header() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:crv-2004", "raw", "0105"]);
    assert!(ok);
    assert_eq!(r["messages"][0]["hex"], "41 05 7B");
    let (ok, r) = run(&e, &["--port", "sim:crv-2004", "raw", "0105", "--header", "18DAF110"]);
    assert!(!ok);
    assert_eq!(kind(&r), "bad_input");
}

#[test]
fn history_compare_shows_what_changed() {
    let e = env();
    let a = run(&e, &["--port", "sim:odyssey-2016", "scan"]).1["session_id"].as_i64().unwrap();
    let (ok, cleared) = run(&e, &["--port", "sim:odyssey-2016:engine-off", "clear", "--confirm-clear-codes"]);
    assert!(ok, "{cleared}");
    let b = cleared["post_clear_session_id"].as_i64().unwrap();
    let (ok, c) = run(&e, &["history", "compare", &a.to_string(), &b.to_string()]);
    assert!(ok, "{c}");
    assert_eq!(c["gone_codes"], serde_json::json!(["P0303", "P3400"]));
}

#[test]
fn tests_command_reads_misfire_counters() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:odyssey-2016", "tests"]);
    assert!(ok, "{r}");
    assert_eq!(r["misfire"][3]["cylinder"], 3);
    assert!(r["findings"].as_array().unwrap().iter().any(|f| f["title"] == "Cylinder 3 misfiring"));
    let (ok, r) = run(&e, &["--port", "sim:crv-2004", "tests", "--vehicle", "crv-2004"]);
    assert!(ok, "{r}");
    assert!(r["results"].as_array().unwrap().iter().any(|x| x["test"].as_str().unwrap().contains("thermostat")));
}

#[test]
fn ports_lists_simulators() {
    let e = env();
    let (ok, r) = run(&e, &["ports"]);
    assert!(ok);
    assert!(r["simulators"].as_array().unwrap().iter().any(|s| s == "sim:kona-ev-2020"));
}

#[test]
fn watch_samples_and_summarises() {
    let e = env();
    let (ok, r) = run(&e, &["--port", "sim:odyssey-2016", "watch", "--keys", "rpm,coolant_temp", "--samples", "2", "--interval-ms", "0"]);
    assert!(ok, "{r}");
    assert_eq!(r["stats"]["rpm"]["samples"], 2);
    let (ok, r) = run(&e, &["--port", "sim:kona-ev-2020", "watch", "--vehicle", "kona-ev-2020", "--samples", "1", "--interval-ms", "0"]);
    assert!(ok, "{r}");
    assert!(r["stats"]["pack_voltage"]["last"].as_f64().is_some(), "{r}");
}
