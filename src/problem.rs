//! Errors an agent can act on: a stable `kind` plus what to do about it.

use std::fmt;

#[derive(Debug)]
pub struct Problem {
    pub kind: &'static str,
    pub message: String,
    pub what_to_do: &'static str,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Problem {}

pub fn problem(kind: &'static str, message: impl Into<String>, what_to_do: &'static str) -> anyhow::Error {
    Problem { kind, message: message.into(), what_to_do }.into()
}

/// The first `Problem` in the error chain, if any.
pub fn find(e: &anyhow::Error) -> Option<&Problem> {
    e.chain().find_map(|c| c.downcast_ref::<Problem>())
}

pub const CHECK_IGNITION: &str = "Turn the ignition ON (engine running, or the Kona in READY) and run the command again. If it still fails, push the adapter firmly into the car's OBD port (under the dashboard, driver side).";
pub const GIVE_VIN: &str = "Read the VIN from the windshield (driver's side, bottom corner) or the driver's door jamb and pass it with --vin.";
pub const CHECK_USB: &str = "Plug the OBDLink EX into this computer's USB port and into the car's OBD port (under the dashboard, driver side), then run `carlink ports`.";
