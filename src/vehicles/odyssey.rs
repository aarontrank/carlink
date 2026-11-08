//! Wiki: odyssey-2016, honda-common-dtcs, honda-maintenance-minder, odyssey-2016-tpms-calibration, honda-idle-relearn.

use super::{KnownDtc, Powertrain, Profile, Reset, clear_reset};

const VCM_MISFIRE: &str = "VCM (cylinder deactivation) oil burning fouls plugs, mostly cylinders 1–3. Check oil consumption (dipstick between changes), pull the plug, and swap the coil to another cylinder to rule it out. Honda warranty extension 25-061 covers misfire repairs that set P0301–P0304 for 10 years from the van's ORIGINAL purchase date or 150,000 mi (the any-age window ended 2026-01-31): a 2016 first sold before 2016-11-08 is out of term — ask a Honda dealer to check the VIN before paying.";
const RANDOM_MISFIRE: &str = "Random misfire (no single cylinder): look for a shared cause first — vacuum leak, low fuel pressure, dirty MAF (check fuel trims with `carlink watch`), then plugs and coils. If cylinder codes P0301–P0304 also appear, see their note (VCM and the Honda warranty extension).";

pub const PROFILE: Profile = Profile {
    id: "odyssey-2016",
    name: "2016 Honda Odyssey",
    powertrain: Powertrain::Gasoline,
    wiki: "odyssey-2016",
    recalls: &[
        "23V858: fuel pump.",
        "16V417: fuel tank welds.",
        "18V170, 17V725, 16V932, 16V933: 2nd-row seat latches.",
    ],
    notes: &[
        "CAN 11-bit 500k; expect the PCM at 7E8 (it also runs the transmission). Other Honda modules use 29-bit 18DAxxF1 and are unverified on a 2016.",
        "Readiness monitor set not verified; read from 0101.",
        "Clearing codes likely also wipes the crank pattern learn: do idle learn, then crank learn, then a drive cycle.",
        "Maintenance: timing belt (with water pump, plugs, valve check) is Maintenance Minder code 4, usually ~100–110k mi; ATF Honda DW-1 only; Type 2 coolant; 0W-20 oil.",
        "Judder at 20–60 mph: bulletin 17-043 (transmission software + triple DW-1 flush).",
        "Warranty extension 25-061 (not a recall): misfire repairs that set P0301–P0304, 10 years from original purchase or 150,000 mi; vans first sold before 2016-11-08 are out of term.",
    ],
    after_clear: &[
        "Clearing also reset the PCM's learned values: do the idle learn, then the crank pattern learn (`carlink resets --vehicle odyssey-2016`) before the van leaves.",
    ],
    modules: &[],
    known_dtcs: &[
        KnownDtc { code: "P0300", note: RANDOM_MISFIRE },
        KnownDtc { code: "P0301", note: VCM_MISFIRE },
        KnownDtc { code: "P0302", note: VCM_MISFIRE },
        KnownDtc { code: "P0303", note: VCM_MISFIRE },
        KnownDtc { code: "P0304", note: VCM_MISFIRE },
        KnownDtc { code: "P3400", note: "VCM bank 1 fault: check oil level first (low oil is the common cause), then the rocker-arm oil pressure switch 37240-R70-A04. Can cause rough running." },
        KnownDtc { code: "P3497", note: "VCM bank 2 fault: check oil level first, then the rocker-arm oil pressure switch 37240-R70-A04." },
        KnownDtc { code: "P0740", note: "Torque converter clutch: matches the 20–60 mph judder pattern. Honda bulletin 17-043: transmission software update plus a triple Honda DW-1 flush (17-044 if it returns)." },
        KnownDtc { code: "P0741", note: "Torque converter clutch performance: bulletin 17-043 (software update + triple DW-1 flush)." },
        KnownDtc { code: "P0848", note: "Transmission clutch pressure switch circuit high (seen in owner complaints; unverified pattern). Check ATF level/condition, then the switch and connector." },
    ],
    resets: &[
        clear_reset("Also resets PCM learned values (likely including crank pattern learn): do idle learn, then crank learn, then a drive cycle. Does not reset the Maintenance Minder or TPMS."),
        Reset { name: "Maintenance Minder / oil life", carlink_can_do: false, steps: &[
            "Ignition ON (engine off).",
            "Press the Select/Reset knob (or SEL/RESET) until the oil-life display shows, then press and hold about 10 seconds.",
            "Choose ONLY the items that were actually serviced, then press and hold about 5 seconds; oil life returns to 100%.",
        ], notes: "Never reset code 4 (timing belt, water pump, plugs) unless that work was done: resetting all items after an oil change hides the timing-belt reminder, and a missed belt on this engine is very expensive." },
        Reset { name: "TPMS low-pressure light", carlink_can_do: false, steps: &[
            "Look at the light: if it BLINKS for about a minute and then stays on, that is a TPMS system fault (often a dead sensor battery) — inflating will not fix it; it needs a TPMS tool or a dealer.",
            "If it is steady: set all tires to the door-placard pressure (cold).",
            "Drive above 28 mph for at least a minute; the light goes out when the sensors report correct pressure.",
        ], notes: "Wheel sensors; no calibration menu on 2016 (that came in 2018 — confirm in owner's manual p. 428). New sensors need HDS or a TPMS tool." },
        Reset { name: "Idle learn", carlink_can_do: false, steps: &[
            "All electrical loads off.",
            "Hold 3,000 rpm in Park until the radiator fan comes on or coolant reaches 90 °C (194 °F).",
            "Idle 5 minutes without touching anything (fan-on time does not count).",
        ], notes: "Driver-performed; carlink can monitor rpm and coolant." },
        Reset { name: "Crank pattern learn (CKP)", carlink_can_do: false, steps: &[
            "After idle learn, with the engine warm: in gear '2', close the throttle and coast from 2,500 to 1,000 rpm; repeat several times.",
            "Key off, then ON and wait 30 seconds.",
        ], notes: "Unverified for the 2016 Odyssey (Honda text from other models). Do it on a safe, empty road." },
    ],
    vin: &[("5FNRL5", 'G', None)],
};

/// Misfire counts concentrated on cylinders 1–3 (the bank VCM deactivates) match the oil-fouling pattern
/// behind warranty extension 25-061 (wiki `honda-mode06`).
pub fn vcm_pattern(tests: Option<&crate::mode06::Tests>) -> Option<crate::findings::Finding> {
    let t = tests?;
    let count = |c: u8| t.misfire.iter().find(|m| m.cylinder == c).map(|m| m.current.unwrap_or(0.0).max(m.average.unwrap_or(0.0))).unwrap_or(0.0);
    let rear: f64 = (1..=3).map(count).sum();
    let front: f64 = (4..=6).map(count).sum();
    (rear >= 20.0 && rear > 3.0 * front).then(|| crate::findings::Finding {
        severity: crate::findings::Severity::Advice,
        code: None,
        title: "Misfires match the VCM pattern (cylinders 1–3)".into(),
        detail: format!("Cylinders 1–3 (the bank VCM shuts off) account for most misfire counts: {rear:.0} vs {front:.0} on cylinders 4–6."),
        recommendation: VCM_MISFIRE.into(),
    })
}
