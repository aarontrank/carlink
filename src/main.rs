use anyhow::{Context, Result};
use carlink::actions;
use carlink::adapter::{Adapter, Transport};
use carlink::problem::{self, problem};
use carlink::report::{self, Ident};
use carlink::store::Store;
use carlink::{dtc, obd, serial, sim, vehicles};
use clap::{Parser, Subcommand};
use serde_json::{Value, json};
use std::path::PathBuf;

/// OBDLink EX diagnostics for AI agents. Every command prints one JSON object on stdout; on failure
/// it has `error`, `error_kind` and `what_to_do`, and the exit code is non-zero.
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Serial device, or `sim:<vehicle>` for the built-in simulator (add `:engine-off`, `:ignition-off`, `:not-in-car`, or `:can29` for the Odyssey). Default: auto-detect the OBDLink EX.
    #[arg(long, global = true, env = "CARLINK_PORT")]
    port: Option<String>,
    /// History database. Default: ~/.local/share/carlink/carlink.db
    #[arg(long, global = true, env = "CARLINK_DB")]
    db: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// List serial ports and which one would be used.
    Ports,
    /// Setup checklist: adapter on USB, power at the OBD port, car answering, VIN. Run this first. Saves nothing.
    Probe,
    /// Full diagnostic: codes, readiness, freeze frame, live data, vehicle-specific data, findings. Saved to history.
    Scan {
        /// Vehicle profile (see `vehicles`); needed when the car reports no VIN.
        #[arg(long)]
        vehicle: Option<String>,
        /// The car's VIN, from the windshield or door jamb; files history under this car when it reports none.
        #[arg(long)]
        vin: Option<String>,
        /// Accept a --vin whose check digit does not match (only for cars built outside North America).
        #[arg(long)]
        vin_no_check: bool,
        /// Odometer reading in miles, recorded with the session.
        #[arg(long)]
        odometer_mi: Option<f64>,
        /// The complaint in a few words (e.g. "CEL on, rough idle"), saved with the session.
        #[arg(long)]
        note: Option<String>,
        /// Do not record this scan in history.
        #[arg(long)]
        no_save: bool,
    },
    /// Clear trouble codes (engine/emissions via OBD service 04, and modules via UDS), then re-scan to verify. Saves a scan first.
    Clear {
        /// Vehicle profile (see `vehicles`), e.g. kona-ev-2020. Refused if the VIN says it is a different car.
        #[arg(long)]
        vehicle: Option<String>,
        /// Required unless the car reports its own VIN.
        #[arg(long)]
        vin: Option<String>,
        /// Accept a --vin whose check digit does not match (only for cars built outside North America).
        #[arg(long)]
        vin_no_check: bool,
        /// Required. Clearing also resets readiness monitors and erases freeze-frame data.
        #[arg(long)]
        confirm_clear_codes: bool,
        /// What to clear, comma-separated: `engine` and/or module names from the scan's modules[].name, or `all`
        /// (engine + every module with codes). Default: engine/emissions codes only.
        #[arg(long, value_delimiter = ',')]
        only: Vec<String>,
        /// What was repaired, or why the person asked for the clear; saved with the sessions.
        #[arg(long)]
        note: Option<String>,
    },
    /// Sample live readings repeatedly (e.g. idle, then 2,500 rpm, or a test drive) and summarise min/max/average. Not saved.
    Watch {
        /// Vehicle profile (see `vehicles`); picks the default readings (the Kona's battery data needs it).
        #[arg(long)]
        vehicle: Option<String>,
        /// Comma-separated reading keys, e.g. rpm,stft_b1,ltft_b1. Default: a sensible set for the car.
        #[arg(long, value_delimiter = ',')]
        keys: Vec<String>,
        /// Number of samples (1–600).
        #[arg(long, default_value_t = 20)]
        samples: usize,
        /// Pause between samples, in milliseconds.
        #[arg(long, default_value_t = 500)]
        interval_ms: u64,
    },
    /// The car's own self-test results (OBD service 06): per-cylinder misfire counts on CAN cars, and each
    /// monitor's test values with pass/FAIL against its limits. Read it BEFORE clearing (a clear zeroes it).
    Tests {
        /// Vehicle profile (see `vehicles`); names the CR-V's Honda-specific tests.
        #[arg(long)]
        vehicle: Option<String>,
    },
    /// Describe a trouble code (generic SAE meaning plus vehicle notes).
    Dtc {
        /// The code, e.g. P0303 (the second character is a digit 0–3, never the letter O).
        code: String,
        /// Vehicle profile (see `vehicles`); adds what the code usually means on that car.
        #[arg(long)]
        vehicle: Option<String>,
    },
    /// Known vehicle profiles.
    Vehicles,
    /// Reset/relearn procedures for a vehicle and whether carlink can perform them.
    Resets {
        /// Vehicle profile (see `vehicles`), e.g. odyssey-2016.
        #[arg(long)]
        vehicle: String,
    },
    /// Send one read-only request (services 01 02 03 06 07 09 0A 18 19 21 22) and show the replies.
    Raw {
        /// 1–7 bytes of hex, e.g. 010C or 220101.
        request: String,
        /// CAN request header for physical addressing, e.g. 7E4.
        #[arg(long)]
        header: Option<String>,
    },
    /// Query saved history.
    History {
        #[command(subcommand)]
        cmd: HistoryCmd,
    },
}

#[derive(Subcommand)]
enum HistoryCmd {
    /// Every car in the history, most recently seen first, with its key.
    Vehicles,
    /// Sessions, newest first.
    Sessions {
        /// Vehicle key from `history vehicles`.
        #[arg(long)]
        vehicle: Option<String>,
        /// Most sessions to list.
        #[arg(long, default_value_t = 20)]
        limit: i64,
    },
    /// One session's full saved report.
    Show {
        /// Session id from `history sessions` (or a scan's `session_id`).
        id: i64,
    },
    /// A reading's values over time, oldest first.
    Trend {
        /// Vehicle key (the VIN) from `history vehicles`.
        vehicle: String,
        /// Reading key from `history keys`, e.g. ltft_b1 or soh.
        key: String,
    },
    /// Reading keys recorded for a vehicle.
    Keys {
        /// Vehicle key (the VIN) from `history vehicles`.
        vehicle: String,
    },
    /// Every trouble code ever seen on a vehicle, with first/last seen and whether it is set now.
    Dtcs {
        /// Vehicle key (the VIN) from `history vehicles`.
        vehicle: String,
    },
    /// What changed between two sessions (e.g. last visit → today): codes new/gone/still set, readings.
    Compare {
        /// The earlier session id.
        from: i64,
        /// The later session id.
        to: i64,
    },
}

fn transport(port: Option<&str>) -> Result<Box<dyn Transport>> {
    Ok(match port {
        Some(p) if p.starts_with("sim:") => Box::new(sim::Sim::from_spec(&p[4..]).map_err(|e| problem("bad_input", e.to_string(), "Use one of the simulators listed by `carlink ports`."))?),
        Some(p) => Box::new(serial::open(p)?),
        None => {
            let p = serial::pick_port(&serial::candidates()).ok_or_else(|| {
                problem("adapter_not_found", "no single OBDLink EX found on USB (none, or several look-alike USB-serial devices)", "Run `carlink ports`. If the OBDLink EX is listed, pass it with --port <name>; if it is not, plug it in (USB and the car's OBD port).")
            })?;
            Box::new(serial::open(&p)?)
        }
    })
}

fn connect(port: Option<&str>) -> Result<Adapter<Box<dyn Transport>>> {
    Adapter::connect(transport(port)?)
}

/// The real history, or — when the simulator is in use and no --db was given — a separate practice file,
/// so rehearsal scans never appear in a car's history.
fn store(db: Option<PathBuf>, port: Option<&str>) -> Result<Store> {
    let file = if port.is_some_and(|p| p.starts_with("sim:")) { "simulator.db" } else { "carlink.db" };
    let path = match db {
        Some(p) => p,
        None => PathBuf::from(std::env::var("HOME").context("HOME not set")?).join(".local/share/carlink").join(file),
    };
    Store::open(&path)
}

fn profile(id: Option<&str>) -> Result<Option<&'static vehicles::Profile>> {
    id.map(|id| {
        vehicles::by_id(id).ok_or_else(|| problem("unknown_vehicle", format!("unknown vehicle '{id}'"), "Run `carlink vehicles` for the valid ids, or leave --vehicle out."))
    })
    .transpose()
}

fn checked_vin(vin: Option<&str>, no_check: bool) -> Result<Option<String>> {
    vin.map(|v| {
        let v = v.trim().to_uppercase();
        if !obd::valid_vin(&v) {
            return Err(problem("bad_input", format!("'{v}' is not a valid VIN"), "A VIN is 17 letters and digits (never I, O or Q). Read it again from the windshield or the driver's door jamb."));
        }
        if !no_check && !obd::vin_check_digit_ok(&v) {
            return Err(problem(
                "bad_input",
                format!("'{v}' fails the VIN check digit (position 9), so it is probably mistyped"),
                "Read the VIN again carefully (common mix-ups: 0/D, 5/S, 8/B, 2/Z). Only if it is correct and the car was built for a market outside North America, add --vin-no-check.",
            ));
        }
        Ok(v)
    })
    .transpose()
}

fn run(cli: Cli) -> Result<Value> {
    let port = cli.port.as_deref();
    Ok(match cli.cmd {
        Cmd::Ports => {
            let c = serial::candidates();
            json!({
                "selected": serial::pick_port(&c),
                "ports": c.iter().map(|p| json!({"name": p.name, "usb": p.usb.map(|(v, p)| format!("{v:04x}:{p:04x}")), "product": p.product})).collect::<Vec<_>>(),
                "simulators": sim::VEHICLES.iter().map(|v| format!("sim:{v}")).collect::<Vec<_>>(),
            })
        }
        Cmd::Probe => {
            let p = carlink::probe::probe(transport(port)?);
            let mut v = json!(p);
            if let Some(first) = p.checks.iter().find(|c| !c.ok && c.name != "vin") {
                v["error"] = json!(format!("{}: {}", first.name, first.detail));
                v["error_kind"] = json!(format!("probe_{}", first.name));
                v["what_to_do"] = json!(first.what_to_do);
                println!("{}", serde_json::to_string_pretty(&v)?);
                std::process::exit(1);
            }
            v
        }
        Cmd::Scan { vehicle, vin, vin_no_check, odometer_mi, note, no_save } => {
            profile(vehicle.as_deref())?;
            let vin = checked_vin(vin.as_deref(), vin_no_check)?;
            let db = if no_save { None } else { Some(store(cli.db, port)?) };
            let mut a = connect(port)?;
            let r = report::build(&mut a, &Ident { vehicle: vehicle.as_deref(), vin: vin.as_deref() })?;
            let session_id = match (db, &r.vehicle_key) {
                (Some(db), Some(_)) => Some(report::save(&db, &r, "scan", odometer_mi, note)?),
                _ => None,
            };
            let mut v = serde_json::to_value(&r)?;
            v["session_id"] = json!(session_id);
            v
        }
        Cmd::Clear { vehicle, vin, vin_no_check, confirm_clear_codes, only, note } => {
            if !confirm_clear_codes {
                return Err(problem(
                    "confirmation_required",
                    "refusing to clear without --confirm-clear-codes",
                    "Clearing turns off the check-engine light, erases freeze-frame data and resets readiness monitors (the car fails an emissions inspection until they complete). Ask the person you are working for; only with their explicit yes, run the same command with --confirm-clear-codes.",
                ));
            }
            profile(vehicle.as_deref())?;
            let vin = checked_vin(vin.as_deref(), vin_no_check)?;
            let db = store(cli.db, port)?;
            let mut a = connect(port)?;
            actions::clear_with_history(&mut a, &db, &Ident { vehicle: vehicle.as_deref(), vin: vin.as_deref() }, &only, note)?
        }
        Cmd::Watch { vehicle, keys, samples, interval_ms } => {
            let flagged = profile(vehicle.as_deref())?;
            let mut a = connect(port)?;
            let p = flagged.or_else(|| carlink::scan::vin(&mut a).ok().flatten().as_deref().and_then(vehicles::detect));
            let mut v = json!(carlink::watch::watch(&mut a, p, &keys, samples, std::time::Duration::from_millis(interval_ms))?);
            v["simulated"] = json!(a.info().simulated);
            v
        }
        Cmd::Tests { vehicle } => {
            let flagged = profile(vehicle.as_deref())?;
            let mut a = connect(port)?;
            let p = flagged.or_else(|| carlink::scan::vin(&mut a).ok().flatten().as_deref().and_then(vehicles::detect));
            let t = carlink::mode06::read(&mut a, p.map(|p| p.id))?;
            let mut v = json!(t);
            v["findings"] = json!(carlink::findings::tests(&t));
            v["simulated"] = json!(a.info().simulated);
            v
        }
        Cmd::Dtc { code, vehicle } => {
            let p = profile(vehicle.as_deref())?;
            let code = dtc::parse(&code).map_err(|e| problem("bad_input", e, "Ask the person to read the code again, character by character (the second character is always a digit 0–3)."))?;
            let mut v = serde_json::to_value(dtc::lookup(&code))?;
            v["vehicle_notes"] = json!(p.and_then(|p| vehicles::dtc_note(p, &code)));
            v
        }
        Cmd::Vehicles => json!(vehicles::PROFILES),
        Cmd::Resets { vehicle } => {
            let p = profile(Some(&vehicle))?.expect("profile given");
            json!({"vehicle": p.name, "resets": vehicles::resets(p)})
        }
        Cmd::Raw { request, header } => {
            let req = actions::check_raw(&request)?;
            if let Some(h) = &header
                && !(h.len() == 3 && h.chars().all(|c| c.is_ascii_hexdigit()))
            {
                return Err(problem("bad_input", format!("header must be 3 hex digits (11-bit CAN), got '{h}'"), "Pass an 11-bit request ID such as 7E4, or leave --header out."));
            }
            let mut a = connect(port)?;
            let msgs = match header {
                Some(h) => a.with_header(&h, |a| a.query(&req))?,
                None => a.query(&req)?,
            };
            json!({"request": req, "bus": a.bus(), "messages": msgs.iter().map(|m| json!({
                "ecu": m.ecu, "hex": m.data.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ")})).collect::<Vec<_>>()})
        }
        Cmd::History { cmd } => {
            let db = store(cli.db, port)?;
            match cmd {
                HistoryCmd::Vehicles => json!(db.vehicles()?),
                HistoryCmd::Sessions { vehicle, limit } => json!(db.sessions(vehicle.as_deref(), limit)?),
                HistoryCmd::Show { id } => json!(db.session(id)?.ok_or_else(|| problem("not_found", format!("no session {id}"), "Run `carlink history sessions` to list session ids."))?),
                HistoryCmd::Trend { vehicle, key } => json!({"vehicle": vehicle, "key": key, "points": db.trend(&vehicle, &key)?}),
                HistoryCmd::Keys { vehicle } => json!(db.keys(&vehicle)?),
                HistoryCmd::Dtcs { vehicle } => json!(db.dtc_history(&vehicle)?),
                HistoryCmd::Compare { from, to } => json!(db.compare(from, to)?.ok_or_else(|| problem("not_found", format!("session {from} or {to} does not exist"), "Run `carlink history sessions --vehicle <key>` to list session ids."))?),
            }
        }
    })
}

fn fail(error: String, kind: &str, what_to_do: &str, code: i32) -> ! {
    println!("{}", json!({"error": error, "error_kind": kind, "what_to_do": what_to_do}));
    std::process::exit(code);
}

fn main() {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) if matches!(e.kind(), clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion) => e.exit(),
        Err(e) => fail(e.to_string().trim().to_string(), "usage", "That is not a valid carlink command line. Run `carlink --help` (or `carlink <command> --help`) and use only the commands and flags it lists.", 2),
    };
    match run(cli) {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).expect("serializable")),
        Err(e) => match problem::find(&e) {
            Some(p) => fail(format!("{e:#}"), p.kind, p.what_to_do, 1),
            None => fail(format!("{e:#}"), "error", "Read the error message. If it mentions the adapter or the car, check the connections and the ignition, then run the command once more; if it repeats, report the message to the person you are working for.", 1),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    fn undocumented(cmd: &clap::Command, path: &str, out: &mut Vec<String>) {
        for a in cmd.get_arguments() {
            if a.get_help().is_none() {
                out.push(format!("{path} {}", a.get_id()));
            }
        }
        for s in cmd.get_subcommands() {
            let sub = format!("{path} {}", s.get_name());
            if s.get_about().is_none() {
                out.push(sub.clone());
            }
            undocumented(s, &sub, out);
        }
    }

    #[test]
    fn every_command_and_argument_has_help_text() {
        let mut missing = Vec::new();
        undocumented(&Cli::command(), "carlink", &mut missing);
        assert!(missing.is_empty(), "without help text: {missing:?}");
    }
}
