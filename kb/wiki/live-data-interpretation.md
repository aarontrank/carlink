---
title: Interpreting live data in the shop — λ and O2 sensors, catalyst, MAF vs load, fuel-trim tests
type: concept
created: 2026-11-08
updated: 2026-11-08
sources:
  - raw/aa1car-oxygen-sensors-2026-11-08.md
  - raw/tirereview-troubleshooting-wideband-o2-2011.md
  - raw/alflash-wide-range-afs-vs-scan-tool.md
  - raw/motor-lean-on-me-fuel-trim.md
  - raw/gears-maf-sensor-diagnosis.md
  - raw/vehicleservicepros-fuel-trim-analysis-maf-testing.md
  - raw/mode06-and-load-community-notes-2026-11-08.md
  - raw/obdb-saej1979-service01-excerpt-2026-11-08.md
  - raw/honda/honda-mode06-2005-all-models.md
  - raw/wikipedia-variable-cylinder-management-2026-11-08.wikitext
  - raw/charm-crv-2004/readiness-monitors.txt
  - url:https://apextechnation.com/articles/fuel-trim-diagnostics
  - url:https://aviondemand.com/insider/fuel-trim-diagnosis-o2-sensor-performance/
tags: [live-data, o2, wideband, lambda, catalyst, maf, map, load, fuel-trim, vacuum-leak, crv-2004, odyssey-2016]
confidence: community
---

# Interpreting live data in the shop

This page is about **how to read** the numbers. The pass/watch/investigate bands for each PID are in
[[diagnostic-thresholds]], and the decoding is in [[obd2-pids]] and [[obd2-pids-extended]]. Everything
here is trade practice from repair-press and training sources (`community`) unless marked otherwise.
Manufacturer specs, and what Aaron's cars show through carlink, override it.

Two facts about the cars change which rules apply:

| | 2004 CR-V (K24A1) | 2016 Odyssey (J35Z8) |
|---|---|---|
| Air metering | **speed-density: MAP, no MAF** (`community`) | **MAF** (Honda P/N 37980-…; `community`) |
| Upstream sensor | wideband A/F, B1S1 (2004 service manual) | wideband A/F, B1S1 and B2S1 (parts listings: 5-wire "wide-band" front and rear; `community`) |
| Downstream sensor | narrowband HO2S, B1S2 | narrowband HO2S, B1S2 and B2S2 |
| Banks | 1 | 2. **Bank 1 = rear = cylinders 1–3, the bank VCM shuts off** |
| Bus / sample rate | K-line, about 56 ms between requests minimum ([[elm327-at-commands]]) | CAN, much faster |

## The capture sequence

Run each step with the engine fully warm and in closed loop (PID 03 = 2), with no misfire or sensor DTCs
that would invalidate it.

1. **Warm idle, 60 s,** accessories off.
2. **2,500 rpm in Park/Neutral, held steady for 30–60 s.**
3. **Snap to about 4,000 rpm and release** to get a decel fuel cut. This is the lean-response check.

For each step, log: rpm, ECT, PID 03, STFT and LTFT for each bank, MAF or MAP, loads 04 and 43,
commanded λ (44), measured λ (24/34…), downstream O2 voltage, IAT and baro. Steps 1–3 are the Toyota
A/F-sensor check (alflash), MOTOR's vacuum-leak test, and the usual O2 checks.

A **road test** adds steady cruise and one full-throttle pull, which is where fuel-delivery and airflow
faults show. The driver does that; carlink only logs it.

On the Odyssey, VCM cuts the rear bank "during cruising and deceleration" (Wikipedia). Steps 1–3 in
Park are therefore normally six-cylinder data. That is an inference: generic OBD shows no VCM state.
Do bank-to-bank comparisons there, not from cruise data.

## Upstream wideband (A/F) sensor: what λ should do

| Condition | Expected | Problem signs |
|---|---|---|
| Warm idle, closed loop | λ ≈ 1.00 with small dither around commanded λ (PID 44) | Stuck on one value; sustained offset from PID 44 (bands in [[diagnostic-thresholds]]) |
| 2,500 rpm steady | λ ≈ 1.00, steady | Slow wander, bank-to-bank difference |
| Snap and release (decel fuel cut) | λ jumps **lean** (well above 1, toward the top of the range) within moments, then returns when fuelling resumes | No jump or a slow jump means a lazy or contaminated sensor or a heater problem |
| Full throttle (road) | λ **below 1**: open loop for power (PID 03 = 4); PID 44 also below 1 | λ near or above 1 at full throttle means it is lean under load |

- **Direction:** lean gives λ > 1 and positive current; rich gives λ < 1 and negative current. At
  stoichiometry the current is zero (Tire Review).
- **Frozen readings.** A sensor or circuit fault can freeze λ at the end of its range. One 2004 CR-V
  showed "AF LAMBDA = 2.00" and "AF Sensor (mA) = −127.5" in closed loop and with the key on, engine off
  (ScannerDanner thread). That pattern means: check the sensor connector, the circuit and the heater
  before anything else.
- **Judge λ, not the voltage or current field.** Makers fill that field inconsistently; one Toyota showed
  more than 7 V at idle with λ = 1.000 (alflash).
- **For reference only, Toyota's published check:** about 3.3 V on the PCM's own A/F signal at 2,500 rpm
  (0.66 V on generic tools that rescale it to 0–1 V), and at least 3.8 V (0.76 V) on decel. Honda's
  internal scale is not documented here. Do not apply Toyota volts to the Hondas.
- **Contaminated sensors** (coolant from a head gasket, phosphorus from burning oil, silicone) respond
  lazily (Tire Review). **Oil burning from VCM is exactly that risk on the Odyssey.**
- **False lean.** Exhaust-manifold leaks, compression leaks and **misfires** put oxygen past the sensor,
  so it reads lean and trims go positive (Tire Review; MOTOR). **Fix misfires first** ([[honda-mode06]]).

## Narrowband O2 sensors (other cars' upstream; the Hondas' downstream)

- They swing between about 0.1 V (lean) and 0.9 V (rich) around 0.45 V. A good sensor follows a
  propane-rich or vacuum-lean change **within about 100 ms** (AA1Car).
- **Upstream switching rate at 2,500 rpm:** multiport injection is about **5–7 transitions per second**;
  throttle-body injection is 2–3/s (AA1Car).
- **Sampling caveat.** Counting 5–7 Hz switching needs at least ~15 readings per second of that one PID.
  That is plausible on CAN with a single-PID loop (measure the achieved rate; `unverified` for the
  OBDLink EX). It is not possible on the CR-V's K-line: at least 56 ms between requests plus the response
  time means at most about 10/s (derived, `unverified`). On K-line, report min, max and mean over a
  window, not switch counts.

## Downstream sensor and catalyst efficiency

- **Healthy converter:** the downstream sensor is **steady, with few crosscounts, averaging 0.45 V or
  more** (AA1Car). In practice it often sits around 0.6–0.75 V ([[diagnostic-thresholds]]).
- **Failing converter:** the downstream signal **starts to mirror the upstream one** (AA1Car). With a
  wideband upstream sensor, as on both Hondas, "mirroring" means the downstream voltage swings in step with
  the upstream λ dither instead of staying flat.
- **Prove the downstream sensor works before blaming the cat.** It must go high when the mixture is
  driven rich and low on decel fuel cut. Toyota's published check, with fuel volume forced +25 % / −12.5 %:
  above 0.55 V rich, below 0.4 V lean (alflash). AA1Car gives ~0.9 V rich and ~0.2 V lean as the
  general behaviour. A flat-lined downstream sensor can fake a "perfect" cat.
- **Use the ECU's own verdict** when it exists: the catalyst test in service 06 (CR-V TID `62`; Odyssey
  MID `21`/`22`; [[honda-mode06]], [[obd2-mode06]]).
  - Honda's 2005 test descriptions show its catalyst monitors work from the **downstream** sensor's
    behaviour (moving averages, variance and switching period of the secondary HO2S).
  - The margin to the limit says how close a P0420 is.
- Cats die from misfire heat and oil ash. On the Odyssey, misfire counts (A2–A7) and oil consumption come
  before any converter verdict ([[honda-common-dtcs]]).

## MAF, MAP and load sanity checks

**What the load PIDs mean.** These are the J1979 / ISO 15031-5 definitions as quoted by a trade author,
`community`.

- **PID 04, calculated load** = current airflow ÷ peak full-throttle airflow at this rpm, corrected to
  standard conditions. It "reaches 100 % at WOT at any altitude, temperature or rpm", and is roughly the
  percent of available torque.
- **PID 43, absolute load** = air mass per intake stroke ÷ (1.184 g/L × cylinder displacement). It runs
  0 to about 95 % on naturally aspirated engines, and its peak tracks volumetric efficiency.

**Cross-check (derived here from the PID 43 definition):**

```
LOAD_ABS % ≈ 10 135 × MAF[g/s] / (displacement[L] × rpm)
```

- Odyssey, 3.5 L at 700 rpm and 5 g/s: about 21 %.
- CR-V, 2.4 L at 750 rpm and 3 g/s: about 17 %.

On a MAF engine the PCM computes PID 43 from the same air mass, so the two should agree. A large mismatch
means one of them is not air-mass based, not that a part is broken.

**MAF (Odyssey).**

- Warm-idle rules of thumb disagree: **about 1 g/s per litre** ([[diagnostic-thresholds]]; a trade
  guide's cleaned 2.5 L example read 2.6 g/s) versus **about 2 g/s per litre** (GEARS: a 2.0 L Ford at
  4.00 g/s). Calling idle MAF "low" or "high" needs this car's own baseline.
- GEARS calls the idle check "mediocre"; it catches only gross failures.
- **WOT tests** (road):
  - Peak MAF should exceed **40 × displacement** in g/s: more than 140 g/s for the 3.5 L (GEARS).
  - **Volumetric efficiency** = MAF ÷ (displacement × rpm/120 × air density). Air density in g/L is about
    BARO[kPa] / (0.287 × (IAT + 273)), or 1.184 at 25 °C, 101.3 kPa. A naturally aspirated engine should
    exceed **75 %**; GEARS's good example was 86 %.
  - **Low VE with a flat-lean O2** at WOT means the MAF is under-reporting. **Low VE with an O2 that is
    not lean** means a breathing restriction, intake or exhaust (GEARS).
- MOTOR's caution: **do not use MAF or load to decide whether there is a vacuum leak.** The MAF reacts to
  the problem rather than revealing it. Out-of-range load values are often MAF-related.

**MAP (CR-V: no MAF).**

- The PCM computes air from MAP, rpm and its VE tables. So PID 10 is probably absent, and MAP (PID 0B) is
  the airflow proxy.
- A warm idle MAP of roughly **30–45 kPa absolute at sea level** (17–21 inHg vacuum) is the common rule
  of thumb (`unverified`). A forum reading for this engine: MAP sensor ≤ 1.0 V at warm idle.
- High idle MAP points to a vacuum leak, a mechanical problem (valves, timing) or a sensor fault. Compare
  with BARO (PID 33) key-on engine-off: the two should read the same before start.

## Fuel-trim tests

**Total trim per bank = STFT + LTFT** (MOTOR). Judge totals, warm, in closed loop. The bands are in
[[diagnostic-thresholds]].

| Pattern (total trim) | Meaning on a **MAF** engine (Odyssey) | Source |
|---|---|---|
| Lean at idle, **drops by at least 15 %** at 2,500 rpm (e.g. +24 % → about +20 % or lower) | **Vacuum leak** (unmetered air matters less as airflow rises) | MOTOR "Lean On Me" |
| Normal or slightly lean at idle, **lean worsening under real load** (road, WOT) | **Fuel delivery**: pump, filter, regulator, restricted injectors | Apex TechNation; AVI |
| Lean at all speeds and loads, **both banks**, idle MAF low for this engine | **MAF under-reporting** (dirty element) | Apex TechNation; MOTOR; GEARS |
| Rich (negative) everywhere | Leaking injector, high fuel pressure, MAF over-reporting | search summary of CarCodeFinder (`unverified`) |
| **One bank** off, the other normal | Bank-specific: intake leak on that bank's runners, injector, exhaust leak ahead of that bank's A/F sensor, **misfire on that bank** | MOTOR; Tire Review |
| Lean that improves with rpm but no leak found | **PCV / crankcase** leak: crankcase air is unmetered too | MOTOR |

Notes on these patterns:

- MOTOR's "15 %" is **relative**: its example drops from +24 to +20, four points. The 2,500 rpm in Park
  test is a *light-load* test. Fuel-delivery faults need a loaded road test to show.
- **Speed-density engines (the CR-V) play by different rules.** A leak into the intake manifold "is seen
  by the ECM the same as opening the throttle blade a little". MAP rises, the PCM adds fuel, and **trims
  barely move**. The exception is a leak right at one intake runner, which can lean out that one cylinder.
  Expect a **higher idle speed, or the idle air control valve closing to compensate**, instead of lean
  trims. A **leaking EGR valve or a restricted exhaust goes rich** on speed-density, because MAP reads high
  load (Vehicle Service Pros, 2020). So on the CR-V:
  - **do not** rule out a vacuum leak because the trims look normal;
  - check MAP at idle, smoke-test the intake, and read Honda's service 06 test TID `48`, "intake air
    leak", L/min against a limit, if the 2004 supports it ([[honda-mode06]]).
- **Odyssey VCM.**
  - Compare bank 1 (rear, cylinders 1–3) with bank 2 at idle and 2,500 rpm in Park.
  - A **positive bank-1 trim together with misfire counts on A2–A4** is the oil-fouling pattern seen from
    two angles.
  - Misfire on a bank makes that bank read lean, so treat the misfire first, then re-check trims.

## Suggested findings for carlink

These are candidates for `findings.rs`. Each must cite its condition, and the bands come from
[[diagnostic-thresholds]].

| Finding | Condition | Say |
|---|---|---|
| Vacuum-leak signature (MAF cars) | total trim at idle above the investigate band, and the 2,500 rpm total at least 15 % lower in relative terms | "Trims improve with rpm: typical of a vacuum leak. Smoke-test the intake and PCV." |
| Speed-density caveat (CR-V) | MAP-only car, suspected leak | "Normal trims do not rule out a vacuum leak on this engine. Check idle MAP and Honda test 48." |
| Bank imbalance | the two banks' totals differ by more than the watch band, warm, at idle in Park | "Bank N only: injector, intake leak, exhaust leak or misfire on that bank. Check service 06 misfire counts." |
| Frozen wideband | λ the same value (or at range limit) across idle, 2,500 and decel | "Upstream A/F sensor not responding: check heater, wiring, sensor." |
| Downstream mirrors upstream | downstream voltage swings in step with upstream at 2,500 rpm | "Catalyst oxygen storage looks weak; confirm with the service 06 catalyst test before replacing." |
| Load cross-check | PID 43 vs the MAF-derived value differ widely | informational only |

## Not verified

- Warm-idle MAF per litre (1× vs 2× rule) and warm-idle MAP; use the cars' own baselines once recorded.
- Achievable single-PID sample rate on the OBDLink EX over CAN.
- Whether VCM is ever active with the van stationary in Park.
- Which wideband PID family (24 vs 34) each Honda uses; probe the bitmaps ([[obd2-pids-extended]]).

## Related
[[diagnostic-thresholds]] · [[obd2-pids]] · [[obd2-pids-extended]] · [[obd2-mode06]] · [[honda-mode06]] ·
[[crv-2004]] · [[odyssey-2016]] · [[honda-common-dtcs]]
