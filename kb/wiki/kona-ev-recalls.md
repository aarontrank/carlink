---
title: Kona Electric 2019–2021 — recalls, TSBs and known issues
type: analysis
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/nhtsa-recalls-hyundai-kona-electric.md
  - raw/kona-ev-community-notes-2026-11-07.md
  - url:https://api.nhtsa.gov/recalls/recallsByVehicle?make=hyundai&model=kona%20electric&modelYear=2020
  - url:https://static.nhtsa.gov/odi/rcl/2021/RCLRPT-21V127-1095.PDF
  - url:https://static.nhtsa.gov/odi/tsbs/2022/MC-10225461-0001.pdf
tags: [kona-electric, recalls, tsb, known-issues]
confidence: verified
---

# Kona Electric 2019–2021 — recalls and known issues

This page covers the [[kona-electric-2020]]. To query NHTSA, use model `kona electric` (or
`KONA ELECTRIC`). `kona ev` returns nothing, and `kona` returns the *gasoline* Kona's recalls
(21V-301, Nu 2.0L engine), which do not apply. The NHTSA complaints endpoint for "kona electric"
mixes in gasoline-Kona complaints, so do not count them as EV complaints.

## US recalls for MY2020 (NHTSA API, fetched 2026-11-07) — `verified`

| NHTSA # | Hyundai # | Filed | Scope | Defect / remedy |
|---|---|---|---|---|
| **20V-630** | 196 | 2020-10-13 | 2019–2020 Kona Electric | Li-ion battery may short-circuit → fire. Dealer updates BMS software, inspects pack, replaces if necessary. Park outside until done. |
| **21V-127** | 200 | 2021-03-01 | 2019–2020 Kona Electric, 2020 Ioniq Electric | Same short-circuit risk; amends 196. Interim: BMS update (owner via infotainment, or dealer). Final: inspect and replace the **Battery System Assembly**. Owner letters mailed 2021-04-09 (interim) and 2021-06-17 (final). Park outside. |
| **20V-748** | 199 | 2020-12-01 | 2019–2021 Kona Electric, 2019–2020 Nexo | Integrated Electronic Brake (IEB) may detect an abnormal sensor signal → significantly reduced braking. IEB software update. |
| **26V-728** | 311 | 2026-11-03 | 2019–2023 Kona Electric, 2019–2022 Veloster N | Airbag control unit software may deactivate airbags and pretensioners. ACU software update; owner letters expected 2027-01-01; VINs searchable from 2026-11-06. **New, likely still open.** |

MY2021 adds **22V-941** (Hyundai 239, 2021 Kona EV only): EPCU may be improperly sealed and leak
coolant internally; inspect/replace EPCU. This does not cover MY2020.

### 21V-127 details (Part 573 report, via search summary — `community` until the PDF is read in full)
- 4,694 MY2019–2020 Kona Electric built **2018-08-28 to 2020-03-02**, limited to cars not already
  repacked under Recall 196.
- Root cause: cells from LG Energy Solution's **Nanjing** plant with a possibly **folded anode tab**.
  Lithium plating on the tab can contact the cathode, causing an internal short.
- In practice most US cars received a **new pack**. The owner notice reads "update BMS or replace BSA
  if necessary", and press reports agree that most original packs were replaced. Hyundai's global
  program covered about 82,000 cars.
- Implication for `carlink`: if the pack was replaced, SOH, CEC/CED and CCC/CDC restart from the
  replacement, and the BMS operating time likely does too. See [[kona-ev-battery-health]].

### 2026 global BMS-software recall — **not in NHTSA for MY2020 as of 2026-11-07** (`community`)
- KBA ref 16242R / Hyundai 61D004, published 2026-03-17. Covers 104,011 1st-gen Kona Electric built
  2018-01-24 to 2023-07-15. The BMS may not detect thermal instability in time; new software detects
  cell-to-cell voltage differences sooner, and some packs get replaced. Australia (2026-04-10) explicitly
  includes cars that already had a pack replaced in 2020–21.
- Re-check NHTSA periodically. A US filing would appear under `kona electric`.

## TSBs and known issues

| Issue | Source | Confidence | Notes |
|---|---|---|---|
| **Motor / reduction-gear rumble or tapping** | Hyundai TSB **22-EV-001H-1** "Kona EV Motor Rumble Noise", 2019–2021MY | verified (TSB exists on NHTSA) / community (cause) | Repair: noise kit, roll mount, motor bolts, then replace motor and/or reducer. Owners suspect motor-shaft ↔ reducer spline wear and report early black reducer oil. Reducer-only replacement often does not fix it. Not visible over OBD. |
| **12 V auxiliary battery going flat** | owner's manual (Aux. Battery Saver+), forums | community | HV pack tops up 12 V. A software update made it always-on and removed the menu option (badge light turns yellow while charging). Drain still reported. Monitor with 7E2 2102 (V, A, SOC) and 220101 p29. No specific US TSB number was found. |
| **"ICCU failure"** | — | — | **Not applicable.** The ICCU is an E-GMP (Ioniq 5/6, EV6) part. The 1st-gen Kona has a separate OBC and an LDC inside the EPCU (inference from the module map in [[kona-ev-other-ecus]]). |
| **EPCU coolant leak** | 22V-941 | verified | MY2021 only. |
| **Cooling / water pump** | — | not found | No Kona EV-specific recall or TSB was found. OBDb documents EWP current, voltage and speed on 7E3 22E009 (unconfirmed on MY2020), which could be used to watch the pump. |
| **IEB braking software** | 20V-748 | verified | |
| **C-MDPS steering noise** | TSBs 20-ST-003H, 22-ST-005H list 2019–22 Kona EV | community | Steering worm-shaft bearing noise. |

## How to check a specific VIN

The NHTSA VIN lookup is `https://www.nhtsa.gov/recalls?vin=<VIN>` (web), or Hyundai's owner site.
The `recallsByVehicle` API above reports what *could* apply to the model year, not whether this car
has had the remedy. A Recall 196/200 BMS reflash or pack swap is visible indirectly over OBD only
through SOH/CEC resets ([[kona-ev-battery-health]]).

## Related
[[kona-electric-2020]] · [[kona-ev-battery-health]] · [[kona-ev-resets]]
