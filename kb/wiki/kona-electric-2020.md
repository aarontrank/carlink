---
title: 2020 Hyundai Kona Electric
type: entity
created: 2026-11-07
updated: 2026-11-08
sources:
  - raw/obdb-hyundai-kona-electric.md
  - raw/evdash-carkiaeniro-src.md
  - raw/jejusoul-kona-niro-ev-pids.md
  - raw/nhtsa-recalls-hyundai-kona-electric.md
  - raw/hyundai-kona-electric-erg-power-button.md
tags: [vehicle, kona-electric, ev, hyundai]
confidence: community
---

# 2020 Hyundai Kona Electric

Aaron's EV. It is a 1st-generation Kona Electric (OS EV, 2018–2023): 64 kWh LG Chem/LG Energy
Solution pack with 98 series groups of 3 parallel cells, a 150 kW front motor, CCS DC charging up to
about 100 kW, and an AC OBC. Diagnosed with the [[obdlink-ex]] through `carlink`.

## Pages

| Topic | Page |
|---|---|
| BMS byte map (SOC, SOH, cells, temps, counters) | [[kona-ev-bms-pids]] |
| VMCU, MCU, TPMS, HVAC, odometer, OBC/LDC | [[kona-ev-other-ecus]] |
| Interpreting SOH and cell imbalance | [[kona-ev-battery-health]] |
| Recalls, TSBs, known issues | [[kona-ev-recalls]] |
| TPMS, 12 V, service reminder, clearing codes | [[kona-ev-resets]] |

## Bus

- OBD port: ISO 15765-4 CAN, 11-bit, 500 kbit/s (ELM `ATSP6`). All documented modules sit on the
  diagnostic CAN behind the gateway, and each answers its own physical request ID (request + 8). See
  the address map in [[kona-ev-other-ecus]].
- Nearly everything useful is manufacturer-specific: UDS service **0x22** (DIDs such as `220101`)
  and KWP-style service **0x21** (`2101`, `2102`). These are not generic OBD-II PIDs.

## Generic OBD-II behaviour (see [[obd2-services]])

| Question | Answer | Confidence |
|---|---|---|
| Does Service 01 answer? | **Yes, but only from two ECUs: MCU `7E3`→`7EB` and BMS `7E4`→`7EC`.** On 2019, 2020 and 2021 cars, both answer PIDs `01` (monitor status / MIL / DTC count), `21` (distance with MIL on), `30` (warm-ups since clear), `31` (distance since clear). The BMS also answers `5B` (hybrid/EV battery pack remaining life, `A×100/255` %). | community: OBDb `service01/modelyears.json` + 2019/2020 `command_support.yaml`, from real-car scans |
| Which ECUs answer `7DF`? | 7E3 and 7E4. No 7E8/7E0 "engine" ECU. A scanner expecting 7E8 can report "no ECU" or fail to read the VIN, which matches owner reports of generic scanners "not supporting" the car. | community / inference |
| Service 01 PID 0D speed, 0C rpm, 05 coolant, etc. | Not in OBDb's supported list for any year. Expect `NO DATA`. Read speed from 7E2 `2101` instead. | community |
| Readiness monitors | PID 0101 is answered, so the bitfield exists. No source lists which monitors this car marks as supported. On an EV the combustion monitors (catalyst, O2, EVAP, EGR, misfire, fuel) should read *not supported*; comprehensive components is the likely exception. **Unverified:** decode whatever comes back and record it on the car. | unverified |
| Service 03 | **Contradicted 2026-11-08 ([[kona-ev-modules]]):** in real captures 03/07 were answered by 7EA, 7EB, 7EC and 7ED. Original text: Expected to return emission-related **stored** DTCs from 7E3/7E4, typically P0Axx/P1xxx hybrid/EV codes. Faults in body, chassis and TPMS modules are not visible through generic 03. No source documents the Kona's actual 03 behaviour. | unverified (inference from Service 01 support) |
| Service 04 | **Contradicted 2026-11-08 ([[kona-ev-modules]]):** on a bench only the BMS (7EC) answered `44`; the others refused `7F 04 22`. carlink never sends 04 on this car (battery-campaign evidence, [[uds-module-dtcs]] §8). Original text: Clears emission DTCs in 7E3/7E4 only. See [[kona-ev-resets]] and [[clearing-codes]]. | verified (semantics) |
| Service 09 VIN | **Settled 2026-11-08 ([[kona-ev-modules]]):** only the BMS answers, with the placeholder `OSEV__ BMS `, not a VIN — ask the user for it. Original text: Not documented by any source for this car. One owner reported that a generic scan tool could not read the VIN. | unverified |
| Power states (for scanning vs clearing) | POWER button **without** the brake cycles OFF → ACC → ON → OFF; brake + button (in P) gives READY. "Ignition on, not READY" is therefore two presses from off, foot off the brake. carlink scans and clears in READY and retries a refused clear once in ON — its own choice (DECISIONS D45), not something a source says the modules require. | verified for the button cycle (raw/hyundai-kona-electric-erg-power-button.md: Hyundai's Kona Electric Emergency Response Guide, first-generation car; owners describe the same cycle); unverified convention for the clear state (DECISIONS O10) |
| PID `015B` (hybrid/EV battery pack remaining life) | Standard J1979 PID, `100/255 × A` % (raw/wikipedia-obd2-pids-2026-11-07.wikitext). The BMS answers it. No source says whether it tracks BMS SOC or displayed SOC, so compare it on the car. Prefer 220101 p4 / 220105 p31, which have 0.5 % resolution. | community (support) / unverified (meaning) |

## Key numbers (from [[kona-ev-bms-pids]])

- SOC (BMS) 220101 p4/2. Displayed SOC 220105 p31/2. SOH 220105 p25–26/10.
- Pack V 220101 p12–13/10. Current 220101 p10–11 as s16/10 (positive = discharge).
- Cells 1–32 in 220102, 33–64 in 220103, 65–96 in 220104, 97–98 in 220105 p34–35. All are `u8/50` V.
- Odometer (US car, miles): 7C6 22B002 p9–11.

## Status of knowledge

Nothing on these pages has yet been observed on Aaron's car. The first `carlink` scan should check:
which ECUs answer `0100` on 7DF; the readiness bits; the Service 03 result; 220101/220105 against the
cluster's SOC and odometer; the rear-tire left/right order; and the VMCU speed decode. Record any
contradiction on the relevant page.

## Related
[[obdlink-ex]] · [[obd2-services]] · [[clearing-codes]] · [[adapter-response-format]]
