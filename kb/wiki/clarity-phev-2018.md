---
title: 2018 Honda Clarity Plug-in Hybrid (Touring)
type: entity
created: 2026-11-13
updated: 2026-11-13
sources:
  - raw/obdb-honda-clarity-2026-11-13.md
  - raw/nhtsa/recalls-honda-clarity-phev-2018.json
  - raw/nhtsa/complaints-honda-clarity-phev-2018.json
  - raw/honda/clarity-phev-2018-sb-17-093-pdi.md
  - raw/honda/clarity-phev-2018-owners-manual-excerpts.md
  - raw/honda/clarity-phev-2018-erg-excerpts.md
  - raw/honda/clarity-phev-sm-p144a-battery-system.md
  - raw/clarity-phev-battery-capacity-community-2026-11-13.md
  - raw/nhtsa/vpic-clarity-vin-decodes-2026-11-13.md
  - raw/honda/clarity-phev-2018-product-updates.md
  - raw/honda/clarity-phev-sb-21-008-21-017-18-090v5.md
  - raw/honda/clarity-phev-sb-24-031-recall-23v858.md
  - raw/honda/clarity-phev-2018-press-kit.md
  - raw/clarity-phev-12v-and-charging-community-2026-11-13.md
tags: [vehicle, honda, clarity, phev, hybrid, can29, iso15765]
confidence: community
---

# 2018 Honda Clarity Plug-in Hybrid (Touring)

Aaron's car (added 2026-11-13). The 2018 Clarity sold in three versions; only the **Plug-in Hybrid**
had a Touring trim and was sold outright (the Electric and Fuel Cell were lease-only). A 1.5 L Atkinson
engine (`LEB3`) and two motors (Honda's two-motor hybrid, no conventional gearbox), a 17 kWh battery of
168 lithium-ion cells under the floor, and an **indirect** TPMS. Diagnosed with the [[obdlink-ex]];
battery data in [[clarity-phev-hv-battery]]. Nothing here has been observed on Aaron's car yet
(DECISIONS O1, O11).

## Recognising it from the VIN

NHTSA vPIC, 2026-11-13 (raw/nhtsa/vpic-clarity-vin-decodes-2026-11-13.md). confidence: verified.

| VIN position | Value | Meaning |
|---|---|---|
| 1–3 | `JHM` | Honda passenger car built in Japan (Sayama) |
| 4–6 | `ZC5` | Clarity Plug-in Hybrid (Honda's Emergency Response Guide says the same) |
| 7 | `F` | — |
| 8 | `1` base, `3` Touring | vPIC trim |
| 10 | `J` | model year 2018 (`K` 2019, `L` 2020) |

`JHMZC5F` + year `J` is the carlink profile. The Clarity Electric is `JHMZC6F` and does not match. The
car reports its VIN over OBD (OBDb's captures show service 09 support; which of the seven ECUs answers
`0902` is not recorded).

## Diagnostic link

| Item | Value | Confidence |
|---|---|---|
| Protocol | **ISO 15765-4 CAN, 29-bit, 500 kbit/s** (`ATDPN` A7). Functional requests `18DB33F1`, replies `18DAF1xx` | community (OBDb 2018 captures) |
| Generic OBD responders | **seven**: `18DAF101`, `102`, `106`, `10E`, `115`, `116`, `12A`. All mirror rpm, speed, coolant, throttle, distance since clear and module voltage. Only **`18DAF10E`** has the full monitor set, fuel trims and O2 data: carlink treats it as the engine computer | community |
| What the others are | `01` and `16`: the two battery condition monitor modules (battery packs A and B; `01` reports the capacity Honda's HDS calls "Battery Pack Capacity"); `15`: charger; `2A`: brake unit (pedal stroke data); `02`, `06`: not named (likely motor/generator control). Each can report its own emissions-related codes to services 03/07/0A | inference from OBDb names |
| Other ECUs (29-bit physical, service 22) | `18DA60F1` meter (odometer `7022`, Maintenance Minder `7060`), `18DA0EF1` PCM, `18DA1DF1` transmission/e-CVT, `18DA28F1` VSA | community |
| Airbag, ABS/VSA, EPS, body codes | not read by carlink (no documented UDS DTC access on this car; DECISIONS O6) | — |

## Power modes (owner's manual pp. 158, 376–377; Honda owner-site excerpt). confidence: verified

- **ACCESSORY**: POWER once, foot off the brake. **ON** (ignition on, not READY): POWER twice, foot off
  the brake. **READY**: brake + POWER until "Ready to Drive". Off: in Park, press POWER.
- In READY **the gasoline engine starts and stops by itself**: a system check after power-on, low
  battery charge (one segment or less always starts it), heavy climate use, very hot or cold weather,
  hard acceleration or hills, and "when the engine has not started for a long time". So a scan in READY
  can show rpm 0 or a running engine, both normal; a clear is done in ON, not READY (carlink still
  refuses if any module reports rpm above 0).
- Below about −22 °F (−30 °C) battery temperature the car will not start or charge.

## 12 V system

The 12 V battery (under the hood) is charged only by the DC-DC converter in the battery unit, from the
high-voltage battery — there is no alternator (Honda service information, Battery System Description).
About 14 V at the OBD port in READY with the engine off is normal. The "12-volt battery charging system"
light means the converter is not charging (owner's manual p. 546: do not switch the car off and on
again, it drains the battery fast). After a 12 V disconnect: power on then off, confirm the car charges;
TPMS and CMBS lights may stay on until driven above 12 mph (pp. 426, 508).

## High-voltage battery health

See [[clarity-phev-hv-battery]] for the byte map.

- **55 Ah new.** Honda service bulletin 17-093 (Jan 2018) shows "Battery Pack Capacity 55.0 Ah" in
  i-HDS on a new car and says: below **36.6 Ah** a battery still under warranty is **eligible for
  warranty replacement**, and "this vehicle does not set a DTC" at that level — the owner gets no
  warning. confidence: verified.
- **Warranty**: 8 years/100,000 miles (S/B 17-093); owners and the clarity-phev capacity guide report
  10 years/150,000 miles in California and other CARB states. Gradual capacity loss is "expected and
  not covered"; greater than normal degradation is (owner's manual p. 573). A 2018 first sold in 2018
  reaches 8 years during 2026. confidence: verified (8/100k), community (CARB 10/150k).
- **Owner readings** (forums, community): 53–54 Ah in the first year or two, 49–52 Ah at 13–19k mi,
  ~48 Ah near 60k mi, 41–44 Ah on 2018s at ~7 years. OBDb's ≈ 40 Ah captures fit the low end.
- **Reset to 55 Ah**: one owner reports the reading jumped back to 55 Ah (and the range estimate up)
  after codes were cleared, then fell again (to 50.5 Ah); the BMS relearns it. OBDb's 95 captures at
  exactly 55.0 Ah fit that. confidence: unverified (single report + inference). carlink therefore treats
  exactly 55.0 Ah as "not a health result", reads capacity before any clear, and saves it in history.
- **DTCs owners report for warranty pack replacements**: P0A7F (hybrid battery deterioration) with
  P1DC7 (battery pack deterioration), sometimes U0296 and P0AE1, with "Plug In Charging System Problem.
  Range Limited". Honda service information also defines **P144A** "HV Battery Pack Deterioration (High
  Severity)" at ≤ 24.9 % usable capacity (MIL and POWER SYSTEM light, two drive cycles). carlink refuses
  to clear while P0A7F, P1DC7 or P144A is stored (DECISIONS D62).

## Readiness monitors and inspections

The engine computer supports catalyst, EVAP, O2 sensor and O2 heater monitors plus the continuous ones
(OBDb `0101`: `07 E5`). They only run while the engine runs, so a car driven almost only on battery may
keep them incomplete; at power-on the check-engine light blinks five times when they are not complete
(owner's manual p. 90). Before an inspection: several trips with the engine running (HV mode).
confidence: verified (blink), inference (HV-mode advice).

## Recalls and known issues

- **23V858** (fuel pump; 2018–2019 Clarity PHEV among many Hondas) — check by VIN. NHTSA lists no other
  recall for the 2018 Clarity PHEV (fetched 2026-11-13). confidence: verified.
- **Loss of power on hills or at highway speed with the battery depleted** — the most common NHTSA
  complaint (engine revs high, car slows). By design: "When the number of bars … becomes one, assist by
  the high-voltage battery is limited and the vehicle output is lowered" (owner's manual p. 123). Use HV
  or HV Charge mode before a long climb. No Honda fix bulletin was found.
- NHTSA complaints also name A/C evaporator leaks, turn-signal/DRL faults, multi-purpose camera
  `U3000-49`, and one oil-dilution report (fuel in the oil) with nothing from Honda. confidence:
  community (complaint text).

### Honda bulletins and product updates (NHTSA copies; verified)

Version 1 of 18-089/18-090 had their titles swapped; forum posts often use the old numbers.

| Bulletin | Symptom / DTCs | Fix |
|---|---|---|
| 18-089 | MIL from a climate-control software problem when running the heater | replace the A/C control unit |
| 18-090 (an emissions recall since v5, 2023) | P26A3 (coolant 3-way valve), P100C (electric VTC motor), P0010 | PCM software update |
| 18-091 | HV range reads absurdly high | meter software |
| 18-097 | will not charge, or stops, on poor mains power: P0D28, P0D2A, P0D3D, P1D23 | charger software (charging up to 1.5× longer) |
| 21-008 | "Hybrid System Failure", P1D8D (generator motor inverter short) | replace the PCU; all 2018–19 |
| 21-017 | A/C condenser corrosion leaks | warranty extended to 10 years |
| recall 23V858 (S/B 24-031) | fuel pump impeller swells; P0087, possible stall | fuel pump |

Owner-reported (community): "Emission System Problem" with P0454/P0455/P0456 after the 23V858 repair
(cap, filler, tank seals; mixed results); on-board charger or inlet failures with P0D27/P0D5C (rule out a
weak 12 V battery and the charge-port pins first); the battery codes above. carlink's known-code notes
carry each of these (`carlink dtc <code> --vehicle clarity-phev-2018`).

## 12 V battery facts (community unless stated)

Group 51R, under the hood (location: ERG, verified). About 14.5 V in READY, about 13.3 V while plugged in,
not charged in ACCESSORY or ON. Owners report failures around 3 years with no warning: no READY, the car
will not start charging, random warning lights.

## Maintenance (owner's manual pp. 475–479, 567)

Maintenance Minder codes: A oil and filter; 0 inspections; 1 tire rotation; 2 cabin filter; 3
transmission fluid (DW-1, or every 47,500 mi / 3 years); 4 spark plugs and valve clearance; 5 engine
coolant; 7 brake fluid (or every 3 years); 8 air cleaner. Change the oil (0W-20, 3.5 US qt with filter)
at least once a year even without a message. Three coolant circuits — engine, inverter, high-voltage
battery — all Honda Type 2; only a dealer refills the battery and inverter circuits. Reset procedure:
`carlink resets --vehicle clarity-phev-2018`.

## TPMS

Indirect (wheel-speed comparison, no sensors). Calibrate after any pressure change, rotation or tire
change: power mode ON, touchscreen HOME > Settings > Vehicle > TPMS Calibration > Calibrate; it
finishes after about 30 minutes of driving at 30–65 mph. Standard tires 235/45R18, 36 psi cold
(owner's manual pp. 425–426, 567; S/B 17-093 §11). confidence: verified.
