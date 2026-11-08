//! Wiki: odyssey-2016, honda-common-dtcs, honda-maintenance-minder, odyssey-2016-tpms-calibration, honda-idle-relearn.

use super::{KnownDtc, Powertrain, Profile, Reset, clear_reset};

const VCM_MISFIRE: &str = "VCM (cylinder deactivation) oil burning fouls plugs, mostly cylinders 1–3. Check oil consumption (dipstick between changes) and the plugs/coils on the flagged cylinder. Honda warranty extension 25-061 covers this for 10 years from original sale / 150k mi — check by VIN with a Honda dealer before paying. Avoid hard driving while it misfires: it can overheat the catalytic converter.";

pub const PROFILE: Profile = Profile {
    id: "odyssey-2016",
    name: "2016 Honda Odyssey",
    powertrain: Powertrain::Gasoline,
    wiki: "odyssey-2016",
    recalls: &[
        "23V858: fuel pump.",
        "16V417: fuel tank welds.",
        "18V170, 17V725, 16V932, 16V933: 2nd-row seat latches.",
        "Warranty extension 25-061: VCM-related oil consumption / misfire (10 yr from original sale / 150k mi).",
    ],
    notes: &[
        "CAN 11-bit 500k; expect the PCM at 7E8 (it also runs the transmission). Other Honda modules use 29-bit 18DAxxF1 and are unverified on a 2016.",
        "Readiness monitor set not verified; read from 0101.",
        "Clearing codes likely also wipes the crank pattern learn: do idle learn, then crank learn, then a drive cycle.",
        "Maintenance: timing belt (with water pump, plugs, valve check) is Maintenance Minder code 4, usually ~100–110k mi; ATF Honda DW-1 only; Type 2 coolant; 0W-20 oil.",
        "Judder at 20–60 mph: bulletin 17-043 (transmission software + triple DW-1 flush).",
    ],
    known_dtcs: &[
        KnownDtc { code: "P0300", note: VCM_MISFIRE },
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
            "Choose the item to reset (or all items), then press and hold about 5 seconds; oil life returns to 100%.",
        ], notes: "Reset only the sub-codes actually serviced." },
        Reset { name: "TPMS low-pressure light", carlink_can_do: false, steps: &[
            "Set all tires to the door-placard pressure (cold).",
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
    vin: &[("5FNRL5", 'G')],
};
