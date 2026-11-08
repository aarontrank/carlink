---
title: Diagnostic thresholds — rules of thumb for interpreting live data
type: concept
created: 2026-11-07
updated: 2026-11-07
sources:
  - url:https://www.samarins.com/diagnose/p0171.html
  - url:https://apextechnation.com/articles/fuel-trim-diagnostics
  - url:https://autodtcs.com/fuel-trim-diagnostics-explained-short-term-vs-long-term-fuel-trim/
  - url:https://www.firestonecompleteautocare.com/blog/batteries/car-battery-voltage/
  - url:https://blog.scrut.my/car-battery-voltage-chart/
  - url:https://www.intangles.ai/guides/what-is-high-engine-coolant-temperature/
  - url:https://engineoiljournal.com/engine-coolant-temperature-chart/
  - url:https://www.aa1car.com/library/o2sensor.htm
  - url:https://www.brakeandfrontend.com/tech-tip-oxygen-sensors-monitoring-converter-efficiency/
  - url:https://forums.ross-tech.com/index.php?threads/14159/
  - url:https://engineerfix.com/what-should-my-maf-sensor-read-at-idle/
tags: [obd2, thresholds, recommendations, fuel-trim, voltage, coolant, o2, maf]
confidence: community
---

# Diagnostic thresholds

These are mechanic rules of thumb from trade press and repair sites, **not** J1979 values.
Manufacturer specs win. Anything observed on Aaron's cars through carlink overrides these. Use
them to phrase recommendations ("worth a look", "likely cause of …"), never as pass/fail
verdicts. Confidence: `community` throughout.

## Fuel trims (PIDs 06–09)

Positive = ECU adding fuel (sees lean). Negative = removing fuel (sees rich). Judge **STFT +
LTFT combined**, warm engine, closed loop (PID 03 = 2).

| Combined trim | Reading |
|---------------|---------|
| within ±5% | Normal |
| ±5–10% | Acceptable; watch the trend |
| beyond ±10% | Investigate even with no MIL |
| ≈ +20–25% / −20–25% | ECU near its limit. P0171/P0174 (lean) or P0172/P0175 (rich) typically sets around here (manufacturer-specific; some set at 15–20%). |

Patterns: high positive trim at idle that falls toward 0 at 2,500 rpm → vacuum leak. Normal at
idle but rising under load → fuel delivery (weak pump, clogged filter). Both banks equally off →
a shared cause (MAF, fuel pressure). One bank only → that bank (injector, manifold leak, O2).

## Coolant temperature (PID 05)

| °C | °F | Reading |
|----|----|---------|
| < ~85 after 15+ min driving | < 185 | Not warming up. Thermostat stuck open (P0128). |
| 88–104 | 190–220 | Normal operating (thermostat opens ~82–90 °C) |
| 105–110 | 220–230 | Running hot; monitor |
| > 110 | > 230 | Overheating. Stop and investigate (warning lamp territory). |
| > 118 | > 245 | Severe. Stop immediately. |

Engines differ (some run 105 °C normally). Compare against the car's own warm baseline once
carlink has recorded one.

## Voltage — battery / charging (PID 42 or ELM `AT RV`)

PID 42 is measured at the ECU and reads a few tenths below the battery terminals. `AT RV` reads
OBD pin 16.

| Condition | Volts | Reading |
|-----------|-------|---------|
| Engine off, rested (several hours) | ≥ 12.6 | Fully charged (flooded lead-acid; AGM rests ~12.8–13.0) |
| | ~12.4 | ~75%; fine |
| | ≤ 12.2 | ~50%; weak or discharged. Charge and load-test. |
| | ≤ 12.0 | ~25% or less; flat |
| Engine running | 13.5–14.7 | Charging OK |
| | < 13.0 | Charging problem likely (alternator, belt, connections) |
| | > 15.0 | Overcharging (regulator) |
| Cranking | must stay > ~9.6 | Below that → weak battery |

**Caveat:** smart-charging systems (the 2016 Odyssey likely has one; `unverified`) deliberately
run at ~12.5–13.0 V when the battery is full. Do not flag low running voltage on those without
context. Resting voltage measures state of charge, not capacity: a 12.6 V battery can still fail
a load test. A reading taken right after driving carries a surface charge, so wait a while.

## O2 sensors (PIDs 14–1B narrowband, 24–2B / 34–3B wideband)

- **Upstream narrowband (S1):** in closed loop, switches between ~0.1 V (lean) and ~0.9 V (rich),
  crossing 0.45 V roughly 0.5–2×/s at idle and faster at 2,500 rpm. Lazy (slow) switching →
  P0133. Stuck low → lean or sensor fault. Stuck high → rich.
- **Downstream (S2):** with a working catalyst, steady around 0.45–0.7 V (slightly rich bias).
  **If S2 mirrors S1's switching, catalyst efficiency is degraded** (P0420/P0430). Replace the
  converter, not the sensor.
- **Wideband/A-F upstream** (PIDs 24+/34+; many Hondas use this): report λ. Expect λ ≈ 1.00
  (± ~0.02) in closed-loop cruise. A sustained λ > 1.05 is lean; < 0.95 is rich.
- Sensor reads `FF` trim byte → not used for trim.

## MAF (PID 10)

- Warm idle, no accessories, near sea level: **≈ 1 g/s per litre of displacement** (sources range
  0.8–2 g/s/L). 2.4 L CR-V ≈ 2.5–4 g/s; 3.5 L Odyssey ≈ 3.5–6 g/s.
- Peak at wide-open throttle ≈ 0.8–1.0 × rated hp in g/s (so hp ≈ 1.25 × peak g/s).
- Much lower than expected with positive fuel trims → dirty or under-reading MAF, or an
  unmetered air leak.
- Altitude lowers readings by a few tenths of a g/s per 500 ft.

## Load and others

- Calculated load (PID 04) at warm idle: typically ~15–35%; high idle load suggests an accessory
  drag or a MAF problem (`unverified`).
- Distance or time since clear (PIDs 31/4E) small, with monitors incomplete → codes were
  cleared recently. Mention it before an inspection.
- Distance with MIL on (PID 21) > 0 → the MIL has been on; how long matters for urgency.

## Related
[[obd2-pids]] · [[readiness-monitors]] · [[dtc-format]] · [[clearing-codes]]
