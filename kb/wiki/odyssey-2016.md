---
title: 2016 Honda Odyssey (J35Z8, VCM, 6AT)
type: entity
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/nhtsa/recalls-honda-odyssey-2016.json
  - raw/nhtsa/complaints-honda-odyssey-2016.json
  - raw/honda/a25-061-vcm-misfire-warranty-extension-memo-2025-06-17.pdf
  - raw/honda/2016-odyssey-touring-tpms-quickguide.pdf
  - url:https://www.manualslib.com/manual/979848/Honda-2016-Odyssey.html?page=458
  - url:https://charm.li/Honda%20Truck/2013/Odyssey%20V6-3.5L%20%28J35Z8%29/Repair%20and%20Diagnosis/
  - url:https://static.nhtsa.gov/odi/tsbs/2018/MC-10129797-9999.pdf
  - url:https://github.com/commaai/opendbc/blob/master/opendbc/car/honda/fingerprints.py
  - url:https://www.odyclub.com/threads/transmission-temperature-with-torque-app-and-elm327.236777/
  - url:https://www.odyclub.com/threads/tcm-location-2014-ody-elite.373404/
  - url:https://vpic.nhtsa.dot.gov/api/
tags: [vehicle, honda, odyssey, j35z8, vcm, can, iso15765]
confidence: community
---

# 2016 Honda Odyssey (J35Z8, VCM, 6AT)

Aaron's minivan: 4th-generation Odyssey (RL5, 2011–2017; facelift 2014). It has a 3.5 L **J35Z8**
SOHC i-VTEC V6 with **VCM** (Variable Cylinder Management, which runs on 3 or 4 cylinders at
cruise) and a 6-speed automatic (all trims 2014+). Diagnosed with the [[obdlink-ex]]. Sister page:
[[crv-2004]].

## Recognising it from the VIN

Checked with NHTSA vPIC on 2026-11-07. confidence: verified.

| VIN position | Value | Meaning |
|---|---|---|
| 1–3 | `5FN` | Honda MPV built in the USA (Lincoln, AL) |
| 4–7 | `RL5H` | Odyssey RL5 |
| 8 | trim code (vPIC: `2` LX, `6` EX-L, `9` Touring; others not checked) | vPIC decodes the trim |
| 10 | `G` | model year 2016 |

`^5FNRL5H.{2}G` matches a 2016 Odyssey. vPIC returns `EngineModel=J35Z8`. Mode 09 PID 02 returns
the VIN on this CAN car.

## Diagnostic link

| Item | Value | Confidence |
|---|---|---|
| Protocol | **ISO 15765-4 CAN, 11-bit, 500 kbit/s** (`ATSP6`) for generic OBD. Honda's own diagnostics use **29-bit** physical addressing (`18DA<tgt>F1` / `18DAF1<tgt>`, `ATSP7`) on the same pins 6/14 | community (Honda moved to CAN in 2008+; 29-bit scheme documented by openpilot/opendbc and forum PIDs) |
| Expected generic responders | **`7E8`** (PCM). Probably **no `7E9`**: the 2011–2017 Odyssey has no separate TCM, because the PCM runs the 6AT | community — OdyClub "TCM location 2014 Ody Elite": "It is not separate, it is part of the PCM". Observe with `ATH1` + `0100` |
| Other ECUs (29-bit, HDS addressing) | From opendbc fingerprints for the **2018+** Odyssey: `18DA28F1` VSA, `18DA30F1` EPS, `18DA53F1` SRS, `18DA60F1` gauge cluster, `18DAEFF1` gateway, `18DA1EF1` transmission, `18DA10F1` PGM-FI. These answer UDS `22 F181` (application software ID). Not confirmed on a 2016 | unverified for 2016 |
| B-CAN / body network | Not reachable through a generic STN adapter (needs the OBDLink MX+/HDS path) | community |

### Honda-enhanced data (Service 22), only what is documented

- **ATF temperature:** request `22 2201`, header `18DA1DF1` (29-bit, `ATSP7`, `ATSH DA1DF1`).
  Formula `°F = A × 9/5 − 40` (`°C = A − 40`), where A is the first data byte after `62 22 01`.
  OdyClub users posted this for 2008+ Odysseys, confirmed it on a 2010 and a 2018 9AT, and Pilot
  owners report it too. One OBDLink MX user reported the same DID working with the functional header
  `7DF`. **No thread tests a 2016 6AT specifically**, and one reply claims the temperature sits at
  byte 27 of the response, which conflicts with the "first byte" formula. Treat it as **unverified on
  this van**: dump the full `62 22 01` response once, check a byte that tracks warm-up, and record
  the result here.
- **Software IDs:** UDS `22 F181` is the Honda application-software DID that opendbc uses to
  fingerprint ECUs (2018+ cars). It is probably harmless to read, but unconfirmed on the 2016.
- Everything else (VCM status, misfire counters per cylinder, idle-learn status, CKP-learn status,
  TPMS IDs) is **HDS territory**. No community-documented DIDs were found. Do not guess DIDs.
  `carlink` must stay read-only on Service 22 ([[adapter-safety]]).

## Readiness monitors

Not pulled from a 2016 manual. Expect the spark-ignition set: catalyst, EVAP, O2/A/F sensor,
O2/A/F heater, plus the continuous misfire, fuel and CCM monitors. The car may also report
"EGR/VVT". **Read `0101`/`0141` and record which bits it actually supports**; do not hardcode.
confidence: unverified. See [[readiness-monitors]].

## Known problems

Detail on [[honda-common-dtcs]]. NHTSA complaints for MY2016 (272 total, retrieved 2026-11-07):
"spark plug" appears in 48, "misfire" in 38, "check engine" in 41, "VCM" in 16 and
"shudder/judder" in 18. The DTCs cited most are P0301, P0303, P0304, P0302, P3400 and P0848.

- **VCM misfire, P0301–P0304** (sometimes up to P0306), mostly on cylinders 1–3. VCM deactivates
  the rear bank. Oil gets past worn or sludged piston rings, fouls the plugs, and sets misfire codes.
  Honda's **warranty extension 25-061** (announced 2025-06-17) covers **2013–2017 Odyssey**
  misfire repairs that trigger P0301–P0304 for **10 years from original purchase or 150,000 mi**.
  Any affected van was also covered regardless of age or mileage **until 2026-01-31** (that window
  has passed). The repair is a PCM software update, new plugs, piston cleaning and new rings.
  Whether a 2016 is still covered depends on its **original sale date**: a van first sold after
  2016-11-07 is still inside 10 years as of today. Verify by VIN at a dealer.
  confidence: verified (Honda dealer memo in raw).
- **P3400 / P3497** (VCM "valve pause" stuck off, bank 1 rear / bank 2 front). Check for low oil
  first (oil consumption above), then the rocker-arm oil pressure switch (TSB 13-031 lineage,
  P/N 37240-R70-A04).
- **Torque-converter judder, 20–60 mph light throttle.** TSB **17-043** (2014–17 Odyssey) is a
  PCM/A-T software update plus a triple ATF DW-1 flush. 17-044 calls for another flush if the
  judder returns. Honda says the converter is not damaged; heat breaks the fluid down.
  confidence: verified (NHTSA TSB index).
- **P0848** (2nd-clutch pressure switch) and other 6AT pressure-switch codes appear in complaints.
- **Spark plugs / ignition coils**: plug replacement is due at Maintenance Minder code **4**.
  Fouled plugs from VCM oil burning are the usual cause of misfires, not coils.
- **P2646/P2647** are **K-series** VTEC switch codes (see [[crv-2004]]). None of the MY2016
  Odyssey complaints cite them, and on the J35 VCM engine the switch faults show up as P3400/P3497.
  Not confirmed as a common Odyssey code.
- **P0420**: generic catalyst efficiency. No Odyssey-specific bulletin was found.

## Recalls (NHTSA, retrieved 2026-11-07)

| Campaign | Component | Summary |
|---|---|---|
| 23V858000 | Fuel pump (Denso low-pressure pump) | Pump can fail, so the engine stalls or won't start. Covers 2013–2023 multi-model; VIN-specific |
| 16V417000 | Fuel tank | 2015–16 Odyssey: insufficient welds, so fuel can leak |
| 18V170000 | 2nd-row outboard seat | Fold/walk-in lever can leave the seat unlatched (2014–16) |
| 17V725000 | 2nd-row outboard seat | Seat can be installed between positions and not latch (2011–17) |
| 16V932000 | 2nd-row center seat | Adjuster bar can stick unlocked (built 2016-09-23 to 2016-10-24 only) |
| 16V933000 | 2nd-row outboard seat | Release lever (built 2010-08-17 to 2015-10-01) |

There is no Takata inflator recall listed for MY2016. Check completion by VIN; OBD cannot show it.

## Maintenance highlights

The Maintenance Minder (owner's manual p. 457–458) is the schedule, not fixed miles. See
[[honda-maintenance-minder]]. confidence: verified.

- **Timing: belt.** Sub-code **4** = replace spark plugs + **timing belt** + inspect water pump +
  inspect valve clearance. The Minder usually calls it around **100–110k mi**. The manual footnote
  says to replace at 60,000 mi when regularly driven in temperatures above 110 °F or below −20 °F.
  The often-quoted "105k / 7 yr" is the fixed-schedule equivalent (charm.li's 2013 J35Z8 schedule
  has a 105,000 mi line). Overdue is common on 10-year-old vans: check the sticker.
- **Valve clearance:** J35 screw adjusters. Inspect at code 4, adjust if noisy.
- **ATF:** Honda **ATF DW-1** only (sub-code 3). Judder TSB 17-043 uses a triple drain-and-fill.
- **Coolant:** Honda Long-Life Type 2 (sub-code 5).
- **Oil:** 0W-20. VCM oil consumption means **check the dipstick between changes**.
- **Brake fluid** every 3 years regardless of the Minder.
- **TPMS:** direct (wheel sensors). See [[odyssey-2016-tpms-calibration]].
