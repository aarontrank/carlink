---
title: OBD-II service 01 — extended PIDs (O2/λ, EGR/EVAP, throttle/pedal, enums, torque) with a three-source cross-check
type: concept
created: 2026-11-08
updated: 2026-11-08
sources:
  - raw/wikipedia-obd2-pids-2026-11-07.wikitext
  - raw/obdb-saej1979-service01-excerpt-2026-11-08.md
  - raw/python-obd-mode06-tables-2026-11-08.md
  - raw/alflash-wide-range-afs-vs-scan-tool.md
  - raw/mode06-and-load-community-notes-2026-11-08.md
  - raw/obdb-hyundai-kona-electric.md
  - url:https://github.com/COVESA/vehicle_signal_specification/issues/304
  - url:https://www.obdautodoctor.com/help/articles/supported-obd-parameters/
  - raw/androbd-evap-pressure-pids-2026-11-08.md
tags: [obd2, j1979, pid, decoding, o2, wideband, lambda, evap, egr, throttle, enum, torque]
confidence: community
---

# Service 01 — extended PIDs

This extends [[obd2-pids]], which has the core table and the supported-PID bitmap rules. Notation:
A, B, C, D are the data bytes after `41 <PID>`. `i16(A,B)` means the 16-bit two's complement:
`(A − 256 if A ≥ 128 else A) × 256 + B`. With carlink's `fn(&[f64]) -> f64` formulas, that is
`(d[0] as i32 as i8) as f64 * 256.0 + d[1]`.

Every formula was checked against three independent sources:

- **W**: Wikipedia, which transcribes J1979 / HS-3000.
- **O**: OBDb's SAEJ1979 signalset (CC BY-SA). Its descriptions read like J1979-DA text.
- **P**: python-OBD (GPL; read for facts only).

The alflash page quotes J1979's own table for PIDs 24–2B and 34–3B, which makes those `verified`-grade.
Keys follow the names carlink already uses where one exists.

## Formulas

| PID | Bytes | Key | Name | Unit | Formula | Sources |
|---|---|---|---|---|---|---|
| 03 | 2 | `fuel_system`, `fuel_system_2` | Fuel system 1 (A) and 2 (B) status | code | A, B: see the enum below | W = P = O for 0–16; O adds bank-2 bits |
| 12 | 1 | `secondary_air` | Commanded secondary air status | code | A ∈ {1, 2, 4, 8} | W = P; O decodes only the low 3 bits, so it cannot show 8 |
| 13 | 1 | `o2_present` | O2 sensors present, 2 banks × 4 | bitmap | A0–A3 = B1 S1–S4, A4–A7 = B2 S1–S4 | W = P = O |
| 1D | 1 | `o2_present_4bank` | O2 sensors present, 4 banks × 2 | bitmap | A0 B1S1, A1 B1S2, A2 B2S1, A3 B2S2, A4 B3S1, A5 B3S2, A6 B4S1, A7 B4S2 | W = P = O |
| 14–1B | 2 | `o2_bXsY_v`, `o2_bXsY_trim` | Narrowband O2 voltage + the STFT it drives | V, % | V = A/200 (0–1.275); trim = B×100/128 − 100; **B = `FF` → sensor not used for trim, omit** | W = P = O |
| 1C | 1 | `obd_standard` | OBD standard the ECU was built to | code | A: enum below | W = O except 14–16; O extends to 59 |
| 24–2B | 4 | `wb_bXsY_lambda`, `wb_bXsY_v` | Wideband λ + voltage | λ, V | λ = (256A+B)×2/65536 (0–1.999); V = (256C+D)×8/65536 (0–7.999) | W = O = J1979 table; P uses /65535 and drops λ |
| 2C | 1 | `egr_commanded` | Commanded EGR | % | 100A/255 | W = P = O |
| 2D | 1 | `egr_error` | EGR error | % | 100A/128 − 100 (−100…99.2) | W = P = O |
| 2E | 1 | `evap_purge` | Commanded evaporative purge | % | 100A/255 | W = P = O |
| **32** | 2 | `evap_vapor_pressure` | EVAP system vapor pressure | Pa | **i16(A,B)/4** (−8192…8191.75) | W (range) = O (`sign: true`); **P and AndrOBD differ** |
| 34–3B | 4 | `wb_bXsY_lambda`, `wb_bXsY_ma` | Wideband λ + current | λ, mA | λ as above; I = (256C+D)/256 − 128 (`8000` = 0 mA, 0.00390625 mA/bit) | W = P = O = J1979 table |
| 3C / 3D / 3E / 3F | 2 | `cat_temp_b1s1` / `_b2s1` / `_b1s2` / `_b2s2` | Catalyst temperature | °C | (256A+B)/10 − 40 | W = P = O. **Note the order: 3D is bank 2, 3E is bank 1 sensor 2** |
| 45 | 1 | `relative_throttle` | Relative ("learned") throttle position | % | 100A/255 | W = P = O |
| 47 / 48 | 1 | `throttle_b` / `throttle_c` | Absolute throttle position B / C | % | 100A/255 | W = P = O |
| 49 / 4A / 4B | 1 | `pedal_d` / `pedal_e` / `pedal_f` | Accelerator pedal position D / E / F | % | 100A/255 | W = P = O |
| 4C | 1 | `throttle_commanded` | Commanded throttle actuator | % | 100A/255 | W = P = O |
| 51 | 1 | `fuel_type` | Fuel type | code | A: enum below | W = P for 0–23; O extends to 32 |
| 52 | 1 | `ethanol_pct` | Ethanol (alcohol) fuel % | % | 100A/255 | W = P = O |
| 53 | 2 | `evap_vapor_pressure_abs` | Absolute EVAP vapor pressure | kPa | (256A+B)/200 (0–327.675) | W = P = O |
| **54** | 2 | `evap_vapor_pressure_wide` | EVAP vapor pressure, wide range | Pa | **i16(A,B)** (−32768…32767) | W ("AB is two's complement") = O (`sign: true`); **P and AndrOBD differ** |
| 5A | 1 | `pedal_relative` | Relative accelerator pedal position | % | 100A/255 | W = P = O |
| **5B** | 1 | `hybrid_battery_soc` | Hybrid/EV battery pack **remaining charge (SOC)** | % | 100A/255 | Formula agrees; **the name does not** (see below) |
| 61 | 1 | `torque_demand_pct` | Driver's demand engine percent torque | % | A − 125 (−125…130) | W = O (P lacks it) |
| 62 | 1 | `torque_actual_pct` | Actual engine percent torque | % | A − 125 | W = O |
| 63 | 2 | `torque_reference` | Engine reference torque | N·m | 256A + B | W = O |

### Where the sources disagree

- **PID 32: three different decodes in the wild.**
  - python-OBD signs **A and B separately** and then computes (A·256 + B)/4. Whenever B ≥ `80` that
    differs from the 16-bit two's complement by 256/4 = **64 Pa**: `00 80` gives −32 Pa instead of +32.
  - AndrOBD reads the word **unsigned**: (256A+B)/4, 0…16383.75 Pa. Any vacuum then shows as roughly
    +16 kPa.
  - **Use i16(A,B)/4.** Wikipedia's −8192…8191.75 range and OBDb's `sign: true` both imply it.
    `community`
- **PID 54.** python-OBD and AndrOBD both use offset binary, (256A+B) − 32767 (AndrOBD's `pids.csv`
  maps 54 to that conversion). Wikipedia ("AB is two's complement signed") and OBDb (signed) use i16.
  The two decodes differ by about 32.8 kPa near zero, so a wrong choice is obvious: a sealed tank at
  ambient should read close to 0 Pa. Prefer i16 and record what a real car shows.
- **PIDs 24–2B.** python-OBD divides the voltage by 65535 instead of 65536, which is negligible, and does
  not decode λ at all.
- **PID 5B.** Wikipedia calls it "Hybrid battery pack remaining life". OBDb's J1979-style description:
  "The percent remaining level of charge for a battery pack used for propulsion, expressed as a percentage
  of total useable battery energy, commonly referred to as State Of Charge (SOC)." **Treat it as SOC, not
  battery health.** A label like "remaining life 45 %" would wrongly suggest a degraded pack. On the Kona
  only the BMS (`7E4`) answers it ([[kona-electric-2020]]); battery health is in [[kona-ev-battery-health]].
- **PID 1C, values 14–16.** Wikipedia gives 14 = OBD/EOBD/KOBD, 15 = OBD/OBD II/EOBD/KOBD,
  16 = reserved. OBDb gives 14 = EURO IV B1, 15 = EURO V B2, 16 = EURO C. These are probably different
  J1979-DA revisions. Show the number plus both names. The two sources match on 1–13 and 17–35; for 33,
  both say Euro Stage VI and only the abbreviation differs.
- **PID 03, newer bits.** Older J1979, as Wikipedia has it, says any value other than 0/1/2/4/8/16 is
  invalid. OBDb's newer table adds 32 = open loop not yet ready (bank 2), 64 = open loop for driving
  conditions (bank 2, "e.g. … cylinder deactivation"), 128 = open loop for a fault (bank 2), and the
  combination 68 = 4 + 64 (both banks). **Decode the byte as bit flags, not a single enum**, so a
  newer ECU's 68 does not show as "invalid". Whether the 2016 Odyssey's PCM uses these bits during VCM
  is `unverified`.

## Enumerations

**PID 03 fuel system status**, each byte (A = system 1, B = system 2):

| Value | Meaning |
|---|---|
| 0 | off / not reported (engine off, or no system 2) |
| 1 | open loop, not yet ready for closed loop (cold) |
| 2 | **closed loop on O2 feedback (normal when warm)** |
| 4 | open loop for driving conditions: power enrichment, decel fuel cut |
| 8 | open loop, system fault |
| 16 | closed loop, but a fault in at least one O2 sensor |
| 32 / 64 / 128 | (newer) the same as 1 / 4 / 8, for bank 2 |

These sources do not settle whether "fuel system 2" (byte B) means bank 2 on a V engine or a second,
independent fuel system. Wikipedia says only "fuel system #2 (if it exists)", and OBDb's newer bank-2
*bits in byte A* suggest it is not simply bank 2. Decode both bytes, label them "system 1" and
"system 2", and record what the Odyssey reports (`unverified`).

**PID 12 commanded secondary air:**

| Value | Meaning |
|---|---|
| 1 | upstream of the first catalyst |
| 2 | downstream of the first catalyst inlet |
| 4 | from atmosphere / off |
| 8 | pump commanded on for diagnostics |

Neither Honda has secondary air: the CR-V's monitor list has none, and the Odyssey's is unlisted.
Expect it to be unsupported.

**PID 1C OBD standard:**

| Value | Meaning |
|---|---|
| 1 | OBD-II (CARB) |
| 2 | OBD (EPA) |
| 3 | OBD and OBD-II |
| 4 | OBD-I |
| 5 | not OBD compliant |
| 6 | EOBD |
| 7 | EOBD and OBD-II |
| 8 | EOBD and OBD |
| 9 | EOBD, OBD and OBD-II |
| 10 | JOBD |
| 11 | JOBD and OBD-II |
| 12 | JOBD and EOBD |
| 13 | JOBD, EOBD and OBD-II |
| 14–16 | see the disagreement above |
| 17 | EMD |
| 18 | EMD+ |
| 19 | HD OBD-C |
| 20 | HD OBD |
| 21 | WWH-OBD |
| 23–26 | HD EOBD stages |
| 27 | HD ZEV |
| 28 / 29 | Brazil OBD 1 / 2 |
| 30 | KOBD |
| 31 / 32 | India OBD I / II |
| 33 | heavy-duty Euro OBD Stage VI |
| 34 | OBD, OBD-II and HD OBD |
| 35 | Brazil OBD 3 |
| 36–59 | OBDb only: motorcycles, China 6, India BS6, Light Duty ZEV ACC-II (51), EPA Tier 4 (57/58), and others |
| 251–255 | not available (J1939 meaning) |

**PID 51 fuel type:**

| Value | Meaning |
|---|---|
| 0 | not available |
| 1 | gasoline |
| 2 | methanol |
| 3 | ethanol |
| 4 | diesel |
| 5 | LPG |
| 6 | CNG |
| 7 | propane |
| 8 | electric |
| 9 | bifuel gasoline |
| 10 | bifuel methanol |
| 11 | bifuel ethanol |
| 12 | bifuel LPG |
| 13 | bifuel CNG |
| 14 | bifuel propane |
| 15 | bifuel electricity |
| 16 | bifuel electric + combustion |
| 17 | hybrid gasoline |
| 18 | hybrid ethanol |
| 19 | hybrid diesel |
| 20 | hybrid electric |
| 21 | hybrid electric + combustion |
| 22 | hybrid regenerative |
| 23 | bifuel diesel |
| 24–32 | OBDb only: bifuel natural gas (24), bifuel diesel again (25), natural gas (26), dual-fuel diesel+CNG (27) / +LNG (28), hydrogen fuel cell (29), hydrogen ICE (30), kerosene (31), heavy fuel oil (32) |

Hybrid codes are **17–22**, not 11/15/16; those three are bifuel codes.

## Which sensor is which (PIDs 14–1B, 24–2B, 34–3B)

A car supports **PID 13 or PID 1D, never both**. That is the J1979 rule as quoted in search excerpts
(`community`; COVESA VSS issue 304 and OBD Auto Doctor describe the same). The eight PIDs in each family
map to sensors in the order of whichever one is supported:

| PID (14… / 24… / 34…) | +0 | +1 | +2 | +3 | +4 | +5 | +6 | +7 |
|---|---|---|---|---|---|---|---|---|
| layout per **PID 13** | B1S1 | B1S2 | B1S3 | B1S4 | B2S1 | B2S2 | B2S3 | B2S4 |
| layout per **PID 1D** | B1S1 | B1S2 | B2S1 | B2S2 | B3S1 | B3S2 | B4S1 | B4S2 |

The 1D row is derived bit-for-bit from PID 1D's layout (`community`). Both Hondas have at most two
banks, so expect PID 13: B2S1 is then `18` / `28` / `38`, not `16`. A sensor appears in **one** family
only:

- narrowband → `14`–`1B`
- wideband → either `24`–`2B` (voltage) or `34`–`3B` (current)

Read the bitmap before choosing.

**Trust λ, not the second field.** On a fault-free Toyota with λ = 1.000 and good trims, a generic tool
read the PID-24 "voltage" field as more than 7 V (alflash). Makers fill C, D inconsistently. Judge
wideband sensors by λ (A, B) and use the voltage or current only for trend.

## What to expect on Aaron's cars (probe the bitmaps; all `unverified`)

**2004 CR-V** (ISO 9141-2, K24A1)

- **Speed-density: MAP sensor, no MAF** (`community`). Expect no PID `10`; use `0B` (MAP) and `04`.
- Upstream A/F sensor B1S1: expect `24` or `34`. One 2004 CR-V on a scan tool showed "AF Sensor (mA)" and
  "AF LAMBDA", which fits `34`, but it may have been Honda-enhanced data. Downstream HO2S B1S2 on `15`.
- No EGR or secondary-air monitor ([[crv-2004]]), so expect no `2C`/`2D`/`12`.
- It has an idle air control valve (Honda's 2005 mode 06 test TID `48` refers to it), so it is a cable
  throttle. Pedal and actuator PIDs `47`–`4C` are therefore unlikely. `51`/`52`/`5A`/`5B`/`61`–`63` are
  probably too new for a 2004 PCM.

**2016 Odyssey** (CAN, J35Z8)

- MAF **and** drive-by-wire, two banks.
- Expect `13` = B1S1, B1S2, B2S1, B2S2: A/F sensors on `24`/`28` or `34`/`38`, HO2S on `15`/`19`.
- Also likely: `44` (commanded λ), throttle and pedal `45`/`47`/`49`/`4A`/`4C`, `2E`, `51` = 1.

**2020 Kona Electric**

- Only `01`, `21`, `30`, `31`, and `5B` from the BMS (OBDb real-car lists, [[kona-electric-2020]]).
  `5B` is SOC.

## Decoding cautions

- A PID that the bitmap claims can still answer `NO DATA`. Record it as "unavailable", not as an error
  ([[obd2-pids]]).
- Round λ to 3 decimals. The 2-decimal rounding used for other PIDs hides a 0.005 lean or rich bias.
- For PID 44, systems with conventional O2 sensors show 1.000 in closed loop and the commanded value
  only in open loop. Wideband systems show the command in both (OBDb's J1979-style description). So on
  the Hondas, PID 44 below 1 under load is real enrichment.

## Related
[[obd2-pids]] · [[obd2-services]] · [[live-data-interpretation]] · [[diagnostic-thresholds]] ·
[[obd2-mode06]] · [[kona-electric-2020]]
