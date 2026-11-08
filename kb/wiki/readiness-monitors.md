---
title: Readiness monitors (service 01 PID 01 / 41)
type: concept
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/wikipedia-obd2-pids-2026-11-07.wikitext
  - url:https://en.wikipedia.org/wiki/OBD-II_PIDs
  - url:https://dep.nj.gov/stopthesoot/obd-ii-information-and-assistance/
  - url:https://www.wisconsinvip.org/wivippublic/pdf/downloads/unset_readiness_monitors.pdf
tags: [obd2, readiness, emissions, inspection]
confidence: community
---

# Readiness monitors

A readiness monitor is the ECU's self-test of one emissions system. Each monitor has two bits:
**supported/available** (1 = this car has the test) and **incomplete** (1 = the test has
*not yet* run to completion since the last clear). The polarity is inverted between the two:
for completeness bits, **0 means complete/ready**.

## PID 01 bit layout (4 bytes A B C D)

| Bits | Meaning |
|------|---------|
| A7 | MIL (check-engine lamp) commanded on |
| A6–A0 | Number of confirmed emissions DTCs (`A & 0x7F`) |
| B7 | Reserved (0) |
| B6 / B5 / B4 | **Incomplete**: components / fuel system / misfire |
| B3 | Ignition type: 0 = spark, 1 = compression (diesel). Selects how C and D are read. |
| B2 / B1 / B0 | **Supported**: components / fuel system / misfire |
| C7…C0 | **Supported**, engine-specific monitors |
| D7…D0 | **Incomplete**, engine-specific monitors (same bit order as C) |

Spark ignition (B3 = 0), bit *n* of C (supported) and of D (incomplete):

| Bit | Monitor |
|-----|---------|
| 7 | EGR and/or VVT system |
| 6 | O2 sensor heater |
| 5 | O2 sensor |
| 4 | Reserved in older J1979; recently defined as gasoline particulate filter (often mislabelled "A/C refrigerant") |
| 3 | Secondary air system |
| 2 | Evaporative (EVAP) system |
| 1 | Heated catalyst |
| 0 | Catalyst |

Compression ignition (B3 = 1): 7 EGR/VVT, 6 PM filter, 5 exhaust gas sensor, 4 reserved,
3 boost pressure, 2 reserved, 1 NOx/SCR, 0 NMHC catalyst.

Status per monitor: `!supported` → "n/a"; `supported && !incomplete` → "ready";
`supported && incomplete` → "not ready".

Misfire, fuel system and components are **continuous** monitors and are normally "ready"
almost immediately. The rest are **non-continuous**: each needs specific drive conditions
(cold soak, steady cruise, fuel level band for EVAP, etc.).

PID `41` has the same layout but reports *this drive cycle*. Its byte A is always 0. It is
useful to show "has the catalyst test run on this trip".

## What it means for an emissions inspection

- US EPA guidance (2001), adopted by most state I/M programs (e.g. NJ, WI, PA, NY): a car of
  MY 1996–2000 may have up to **2** non-continuous monitors not ready; MY 2001+ may have
  **1**. More than that and the car is rejected as "not ready" (it does not fail on emissions;
  it is untestable). The continuous monitors must be ready. Unsupported monitors do not count.
- California is stricter: for MY 2000+ gasoline cars, only EVAP may be incomplete (`community`).
  Check the local program's rules.
- Monitors go back to "not ready" after a service 04 clear, after the ECU loses power (battery
  disconnect), or after a reflash. See [[clearing-codes]].
- EVAP and catalyst are typically the slowest to complete. EVAP often needs an overnight cold
  soak and a fuel level between roughly 15% and 85% (manufacturer-specific; `unverified`).

## What the catalyst and EVAP monitors test

- **Catalyst:** compares upstream and downstream O2/A-F sensor activity. A good converter stores
  oxygen, so the downstream sensor stays steady. Failure → P0420 (bank 1) / P0430 (bank 2).
- **EVAP:** seals the fuel vapour system (the canister vent closes) and checks for pressure or
  vacuum decay. Failure → P0440–P0457 family (P0442 small leak, P0455 large leak, P0456 very
  small leak). A loose fuel cap is the classic cause.

## Related
[[obd2-pids]] · [[clearing-codes]] · [[diagnostic-thresholds]] · [[obd2-services]]
