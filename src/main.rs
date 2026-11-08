use anyhow::{Context, Result, bail};
use carlink::actions;
use carlink::adapter::{Adapter, Transport};
use carlink::report;
use carlink::store::Store;
use carlink::{dtc, serial, sim, vehicles};
use clap::{Parser, Subcommand};
use serde_json::{Value, json};
use std::path::PathBuf;

/// OBDLink EX diagnostics for AI agents. Every command prints JSON on stdout.
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Serial device, or `sim:<vehicle>` for the built-in simulator. Default: auto-detect the OBDLink EX.
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
    /// Connect and identify adapter, protocol, voltage and VIN. Saves nothing.
    Probe,
    /// Full diagnostic: codes, readiness, freeze frame, live data, vehicle-specific data, findings. Saved to history.
    Scan {
        /// Vehicle profile when the car reports no VIN (see `vehicles`).
        #[arg(long)]
        vehicle: Option<String>,
        /// Odometer reading in miles, recorded with the session.
        #[arg(long)]
        odometer_mi: Option<f64>,
        #[arg(long)]
        note: Option<String>,
        /// Do not record this scan in history.
        #[arg(long)]
        no_save: bool,
    },
    /// Clear trouble codes and the check-engine light (OBD service 04). Saves a full scan first.
    Clear {
        #[arg(long)]
        vehicle: Option<String>,
        /// Required. Clearing also resets readiness monitors and erases freeze-frame data.
        #[arg(long)]
        confirm_clear_codes: bool,
        #[arg(long)]
        note: Option<String>,
    },
    /// Describe a trouble code (generic SAE meaning plus vehicle notes).
    Dtc {
        code: String,
        #[arg(long)]
        vehicle: Option<String>,
    },
    /// Known vehicle profiles.
    Vehicles,
    /// Reset/relearn procedures for a vehicle and whether carlink can perform them.
    Resets {
        #[arg(long)]
        vehicle: String,
    },
    /// Send one read-only request (services 01 02 03 06 07 09 0A 22) and show the decoded frames.
    Raw {
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
    /// Sessions, newest first.
    Sessions {
        #[arg(long)]
        vehicle: Option<String>,
        #[arg(long, default_value_t = 20)]
        limit: i64,
    },
    /// One session's full saved report.
    Show { id: i64 },
    /// A reading's values over time, oldest first.
    Trend { vehicle: String, key: String },
    /// Reading keys recorded for a vehicle.
    Keys { vehicle: String },
    /// Every trouble code ever seen on a vehicle, with first/last seen.
    Dtcs { vehicle: String },
}

fn connect(port: Option<&str>) -> Result<Adapter<Box<dyn Transport>>> {
    let t: Box<dyn Transport> = match port {
        Some(p) if p.starts_with("sim:") => Box::new(sim::Sim::new(&p[4..])?),
        Some(p) => Box::new(serial::open(p)?),
        None => {
            let p = serial::pick_port(&serial::candidates()).context("no OBDLink EX found: is it plugged into USB? (`carlink ports` lists devices)")?;
            Box::new(serial::open(&p)?)
        }
    };
    Adapter::connect(t)
}

fn store(db: Option<PathBuf>) -> Result<Store> {
    let path = match db {
        Some(p) => p,
        None => PathBuf::from(std::env::var("HOME").context("HOME not set")?).join(".local/share/carlink/carlink.db"),
    };
    Store::open(&path)
}

fn profile(id: Option<&str>) -> Result<Option<&'static vehicles::Profile>> {
    id.map(|id| vehicles::by_id(id).with_context(|| format!("unknown vehicle '{id}' (see `carlink vehicles`)"))).transpose()
}


fn run(cli: Cli) -> Result<Value> {
    let port = cli.port.as_deref();
    Ok(match cli.cmd {
        Cmd::Ports => {
            let c = serial::candidates();
            json!({
                "selected": serial::pick_port(&c),
                "ports": c.iter().map(|p| json!({"name": p.name, "usb": p.usb.map(|(v, p)| format!("{v:04x}:{p:04x}"))})).collect::<Vec<_>>(),
                "simulators": sim::VEHICLES.iter().map(|v| format!("sim:{v}")).collect::<Vec<_>>(),
            })
        }
        Cmd::Probe => {
            let mut a = connect(port)?;
            let vin = carlink::scan::vin(&mut a).ok().flatten();
            json!({
                "adapter": a.info(), "bus": a.bus(), "battery_voltage": a.voltage()?, "vin": vin,
                "vehicle": vin.as_deref().and_then(vehicles::detect),
            })
        }
        Cmd::Scan { vehicle, odometer_mi, note, no_save } => {
            profile(vehicle.as_deref())?;
            let mut a = connect(port)?;
            let r = report::build(&mut a, vehicle.as_deref())?;
            let session_id = if no_save { None } else { Some(report::save(&store(cli.db)?, &r, "scan", odometer_mi, note)?) };
            let mut v = serde_json::to_value(&r)?;
            v["session_id"] = json!(session_id);
            v
        }
        Cmd::Clear { vehicle, confirm_clear_codes, note } => {
            if !confirm_clear_codes {
                bail!("refusing to clear without --confirm-clear-codes. Clearing turns off the MIL, erases freeze frames and resets readiness monitors (the car will fail an emissions inspection until they complete). Ask the owner first.");
            }
            profile(vehicle.as_deref())?;
            let db = store(cli.db)?;
            let mut a = connect(port)?;
            actions::clear_with_history(&mut a, &db, vehicle.as_deref(), note)?
        }
        Cmd::Dtc { code, vehicle } => {
            let p = profile(vehicle.as_deref())?;
            let mut v = serde_json::to_value(dtc::lookup(&code))?;
            v["vehicle_notes"] = json!(p.and_then(|p| vehicles::dtc_note(p, &code.to_uppercase())));
            v
        }
        Cmd::Vehicles => json!(vehicles::PROFILES),
        Cmd::Resets { vehicle } => {
            let p = profile(Some(&vehicle))?.expect("profile given");
            json!({"vehicle": p.name, "resets": vehicles::resets(p)})
        }
        Cmd::Raw { request, header } => {
            let req = actions::check_raw(&request)?;
            let mut a = connect(port)?;
            let msgs = match header {
                Some(h) if h.len() == 3 && h.chars().all(|c| c.is_ascii_hexdigit()) => a.with_header(&h, |a| a.query(&req))?,
                Some(h) => bail!("header must be 3 hex digits (11-bit CAN), got '{h}'"),
                None => a.query(&req)?,
            };
            json!({"request": req, "bus": a.bus(), "messages": msgs.iter().map(|m| json!({
                "ecu": m.ecu, "hex": m.data.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ")})).collect::<Vec<_>>()})
        }
        Cmd::History { cmd } => {
            let db = store(cli.db)?;
            match cmd {
                HistoryCmd::Sessions { vehicle, limit } => json!(db.sessions(vehicle.as_deref(), limit)?),
                HistoryCmd::Show { id } => json!(db.session(id)?.with_context(|| format!("no session {id}"))?),
                HistoryCmd::Trend { vehicle, key } => json!({"vehicle": vehicle, "key": key, "points": db.trend(&vehicle, &key)?}),
                HistoryCmd::Keys { vehicle } => json!(db.keys(&vehicle)?),
                HistoryCmd::Dtcs { vehicle } => json!(db.dtc_history(&vehicle)?),
            }
        }
    })
}

fn main() {
    match run(Cli::parse()) {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).expect("serializable")),
        Err(e) => {
            println!("{}", json!({"error": format!("{e:#}")}));
            std::process::exit(1);
        }
    }
}
