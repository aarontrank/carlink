---
title: Common DTCs and failure patterns — 2004 CR-V and 2016 Odyssey
type: analysis
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/charm-crv-2004/tsb-09-076-ac-13-021-p2646-08-016-p1457.txt
  - raw/charm-crv-2004/dtc-p0011-p0420-p0700.txt
  - raw/nhtsa/complaints-honda-crv-2004.json
  - raw/nhtsa/complaints-honda-odyssey-2016.json
  - raw/honda/a25-061-vcm-misfire-warranty-extension-memo-2025-06-17.pdf
  - url:https://static.nhtsa.gov/odi/tsbs/2018/MC-10129797-9999.pdf
  - url:https://www.obd-codes.com/pdf/p3400-p3497-honda-13-031.pdf
  - url:https://www.go-parts.com/garage/obd-p0141-honda-cr-v-2002-2006-k24a1-2-4l
  - url:https://www.crvownersclub.com/threads/black-death-and-buying-a-used-cr-v.120225/
  - url:https://www.crvownersclub.com/threads/p0740-code-torque-transmission-does-not-appear-to-be-locking-up.222143/
  - url:https://honda-tech.com/forums/honda-cr-v-element-156/p1457-2003-cr-v-2960833/
tags: [dtc, honda, crv-2004, odyssey-2016, tsb]
confidence: community
---

# Common DTCs and failure patterns: 2004 CR-V and 2016 Odyssey

This is the lookup `carlink` uses to turn a code into a likely cause and a next step. Codes are
listed per car because the same number can mean different things in different Honda families.
"Verified" here means a Honda TSB or service-manual chart. Freeze frame and Mode 06 should be
recorded **before** any clear ([[clearing-codes]], [[honda-idle-relearn]]).

## 2004 CR-V (K24A1, ISO 9141-2): see [[crv-2004]]

| DTC | Meaning | Usual cause → recommendation | Conf. |
|---|---|---|---|
| — (no code) | A/C blows warm | **Compressor clutch** (worn plate, excess air gap, open coil, relay) or internal compressor failure. TSB 09-076 (2002–04): swab the discharge port; metal or grey/black residue means compressor + condenser + receiver/drier + expansion valve, otherwise fix the clutch only | verified + community |
| P1457 | EVAP leak, canister side | **Canister vent shut valve** (corroded, under the car by the fuel door), the EVAP solenoid kit per TSB 08-016, or a clogged purge line (Service News 2004). After that, smoke-test the filler neck and hoses. Gas cap rarely fixes it | verified |
| P1456 | EVAP leak, fuel-tank side | Fuel cap binding or not clicked (TSB); then the tank-side hoses | verified (TSB title) |
| P2646 / P2647 | VTEC (rocker arm) oil pressure switch low/high | Check oil level and grade first, then replace the switch, P/N 37250-PNE-G01 (TSB 13-021, 2002–09 CR-V) | verified |
| P0420 | Catalyst efficiency | First fix any P0137/P0138/P0141. Then the manual's drive test (2 min at 50–55 mph, 4 s closed-throttle decel, back to 55) and replace the TWC if it fails. A downstream sensor that mirrors the upstream one means the cat is gone | verified (manual) |
| P0141 | Secondary HO2S heater | Downstream O2 sensor heater open. Heater should measure 5.0–6.4 Ω at 68 °F. Replace the sensor | community |
| P0011 | VTC system malfunction | Low oil or oil pressure, a **clogged VTC strainer**, or a sticky VTC oil control solenoid. Clean or replace, change the oil and filter, update the PCM if the code persists | verified (manual) |
| P0700 | "A/T DTC present" | Not a fault by itself. Read the companion P07xx **from the same PCM** before clearing | verified (manual) |
| P0740 | Lock-up (TCC) control | Low or degraded ATF (refill with DW-1), lock-up solenoid or harness. Shudder at lock-up, high highway rpm | community |
| P0730 / P0780 | Wrong ratio / shift malfunction | Worn clutches; P0780 sends you through pressure-control solenoids A→B→C before replacing the transmission. Burnt ATF with P0730+P0740 points to a rebuild | community |
| P1298 | ELD circuit high | Bad solder joint in the electrical load detector inside the under-hood fuse box. Replace the ELD, P/N 38255-S5A-003 (TSB 05-006) | verified |
| P1077 | IMRC / intake manifold runner | IMRC valve or actuator (TSB title, superseded). One NHTSA complaint cites a high idle with oil/battery lights from an IMRC fault | verified (TSB title) |
| P0122 | TP sensor low | TSB "MIL ON/DTC P0122": TP sensor or throttle body circuit | verified (TSB title) |

## 2016 Odyssey (J35Z8 VCM, CAN): see [[odyssey-2016]]

| DTC | Meaning | Usual cause → recommendation | Conf. |
|---|---|---|---|
| P0301–P0304 (P0305/P0306 less often) | Cylinder misfire | **VCM oil consumption**: oil past worn or sludged rings fouls the plugs, mostly on cylinders 1–3. Check oil consumption and pull the plugs. Honda **warranty extension 25-061**: 10 yr from original purchase / 150k mi; the any-age window ran to 2026-01-31. The dealer does a PCM update, rings and plugs. Rule out a coil (swap test) | verified (Honda memo) |
| P0300 | Random misfire | Same mechanism, or a vacuum leak or fuel delivery fault. Check fuel trims (PIDs 06/07/08/09) | community |
| P3400 / P3497 | VCM valve-pause stuck off, bank 1 (rear) / bank 2 (front) | **Low oil first** (consumption). Then the rocker-arm oil pressure switch for that bank, P/N 37240-R70-A04 (TSB 13-031 family); then the VCM spool valve | community (TSB 13-031 years listed only to 2012) |
| P0740 / P0741, judder 20–60 mph | TCC | **TSB 17-043** (2014–17): PCM/A-T software update plus a triple ATF DW-1 flush. 17-044: flush again if it returns. Fluid breakdown, not converter damage | verified |
| P0848 (and other P08xx pressure switch codes) | 6AT clutch pressure switch | Cited in NHTSA complaints. ATF condition and level, then the pressure switch or valve body | unverified |
| P0420 | Catalyst efficiency | Generic; often follows prolonged misfire or oil burning. Fix the misfires first | community |
| P2646 / P2647 | — | **Not a known Odyssey pattern.** These are K-series (CR-V) VTEC switch codes; the J35 VCM switch faults appear as P3400/P3497 | community |

## How `carlink` should use this

- Match the vehicle by VIN pattern first (see the entity pages). The P1xxx meanings above are
  **Honda-family and year specific**.
- A P0700 on the CR-V means "read again". A P03xx on the Odyssey means "check oil level and the
  warranty status" before anyone buys coils.
- Clearing on the Odyssey wipes the CKP pattern (see [[honda-idle-relearn]]), so the misfire
  monitor needs the relearn before it can confirm a fix.
