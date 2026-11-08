//! Wiki: crv-2004, honda-common-dtcs, honda-maintenance-minder, honda-idle-relearn.

use super::{KnownDtc, Powertrain, Profile, Reset, clear_reset};

const IDLE_LEARN: &[&str] = &[
    "All electrical loads off (A/C, lights, rear defroster).",
    "Start the engine, hold 3,000 rpm in Park/Neutral until the radiator fan comes on or coolant reaches 90 °C (194 °F).",
    "Let it idle 5 minutes without touching anything (time with the fan running does not count).",
    "Optional: run `carlink scan --vehicle crv-2004 --no-save` to confirm idle rpm and coolant temperature.",
];

pub const PROFILE: Profile = Profile {
    id: "crv-2004",
    name: "2004 Honda CR-V",
    powertrain: Powertrain::Gasoline,
    wiki: "crv-2004",
    recalls: &[
        "Takata airbag inflators (driver and passenger; many campaigns) — confirm completion by VIN at nhtsa.gov/recalls.",
        "12V486 / 20V768: driver power-window master switch can melt.",
        "12V136: low-beam headlights.",
        "04V255: airbag wiring harness.",
    ],
    notes: &[
        "ISO 9141-2 K-line; only the PCM (header 48 6B 10) answers. Protocol search takes several seconds.",
        "No separate TCM: transmission faults appear as P0700 plus the specific P07xx from the PCM.",
        "ABS, SRS and body modules are not reachable with a generic adapter.",
        "VIN over OBD (0902) may not work; pass --vehicle crv-2004.",
        "Maintenance: timing chain (no belt); valve clearance check at 110k mi; plugs 110k; ATF Honda DW-1 only; Type 2 coolant; 5W-20 oil.",
        "A/C blowing warm is a known compressor failure pattern (bulletin 09-076) with no OBD code.",
    ],
    after_clear: &[
        "Clearing also reset the PCM's learned idle: do the idle learn now (`carlink resets --vehicle crv-2004`, \"Idle learn\") before the car leaves.",
    ],
    modules: &[],
    known_dtcs: &[
        KnownDtc { code: "P1457", note: "EVAP leak at the canister side: usually the canister vent shut valve or a clogged purge line (Honda bulletin 08-016). Replace the valve, then smoke-test the EVAP system. Not urgent to drive." },
        KnownDtc { code: "P2646", note: "VTEC oil pressure switch reports wrong state (bulletin 13-021). Check oil level and condition first; then replace the switch (37250-PNE-G01). Safe to drive short-term." },
        KnownDtc { code: "P2647", note: "VTEC oil pressure switch stuck on (bulletin 13-021). Check oil level/condition; replace switch 37250-PNE-G01." },
        KnownDtc { code: "P0420", note: "Catalyst efficiency low. Fix any O2 sensor codes (e.g. P0141) and misfires first; then run the service-manual catalyst drive test before replacing the converter." },
        KnownDtc { code: "P0141", note: "Downstream O2 sensor heater circuit: usually the sensor itself (heater open) or its connector. Replace the secondary O2 sensor; fix this before judging a P0420." },
        KnownDtc { code: "P0011", note: "VTC (cam timing) over-advanced: low or dirty oil, clogged VTC strainer, or a sticking VTC solenoid. Check oil level, change oil, clean/replace the strainer/solenoid." },
        KnownDtc { code: "P1456", note: "EVAP leak on the fuel-tank side: first check the fuel cap is the right one, seals, and clicks when tightened (Honda TSB); then the tank-side EVAP hoses. Not urgent to drive." },
        KnownDtc { code: "P0700", note: "Transmission fault flag from the PCM; look at the accompanying P07xx code. Check ATF level/condition (Honda DW-1 only)." },
        KnownDtc { code: "P0730", note: "Incorrect gear ratio: often internal clutch wear. Check ATF (DW-1) for burnt smell/debris; with P0740 as well it usually means a rebuild." },
        KnownDtc { code: "P0740", note: "Torque converter lock-up fault: check ATF condition and lock-up solenoids; with P0730 as well it usually means a rebuild." },
        KnownDtc { code: "P0780", note: "Shift malfunction: check ATF level/condition (DW-1) and shift solenoids; a drain-and-fill with DW-1 is the first step." },
        KnownDtc { code: "P1298", note: "Electrical load detector (in the under-hood fuse box) circuit high (bulletin 05-006); usually the ELD unit or fuse-box connection." },
    ],
    resets: &[
        clear_reset("Also resets PCM adaptive values: do the idle learn afterwards. Does not reset the MAINT REQ'D light. ABS/SRS codes are in modules carlink cannot reach."),
        Reset { name: "MAINT REQ'D / oil-change light", carlink_can_do: false, steps: &[
            "Engine off; headlights and parking lights off (it refuses otherwise).",
            "Press and hold the Select/Reset (trip) button on the cluster.",
            "Turn the key to ON (II) while holding; keep holding about 10 seconds until the indicator resets.",
        ], notes: "Owner procedure on the car." },
        Reset { name: "Idle learn (after battery disconnect, clearing codes, or throttle-body cleaning)", carlink_can_do: false, steps: IDLE_LEARN, notes: "Done by the driver; carlink can monitor it." },
        Reset { name: "TPMS", carlink_can_do: false, steps: &[], notes: "Not fitted on the 2004 CR-V." },
        Reset { name: "PCM replacement / immobilizer", carlink_can_do: false, steps: &[], notes: "Requires Honda HDS at a dealer." },
    ],
    vin: &[("JHLRD", '4', None), ("SHSRD", '4', None)],
};
