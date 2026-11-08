---
title: Kona Electric community / press / TSB notes gathered by web search
source_url: multiple (listed per item)
fetched: 2026-11-07
---

These are **search-result summaries**, not verbatim page captures. Each item names the page it came
from. Treat as `community` / `unverified` unless an item says it is a Hyundai or NHTSA document.

## SOH behaviour
- insideevsforum "Battery health" (https://www.insideevsforum.com/community/threads/battery-health.19748/):
  owner reads SOH from 7E4 / 220105 bytes Z:AA /10; 100% at ~25k miles; original Sept-2018 pack read
  100% until replaced 4 years later. After pack replacement "Minimum Deterioration" (AC:AD /10) was
  not reset (0.7%), rose to 1.3% over two years while SOH stayed 100%; later SOH 97.5% with minimum
  deterioration 5.8%. One owner: SOH 91% in Dec 2023, a BMS software update reset it to 100%.
- No source found reporting SOH > 100.0% on a Kona; the field appears capped at 1000 raw (100.0%).

## Cell balance / imbalance thresholds (forum opinion, not Hyundai spec)
- speakev "Kona cell balancing" (https://www.speakev.com/threads/kona-cell-balancing.151619/),
  speakev "High Voltage battery cell voltage deviation"
  (https://www.speakev.com/threads/high-voltage-battery-cell-voltage-deviation.168441/),
  insideevsforum "Another BMS recall for the Kona"
  (https://www.insideevsforum.com/community/threads/another-bms-recall-for-the-kona.20946/),
  hyundaikonaforum "Suggestion on battery degradation control"
  (https://www.hyundaikonaforum.com/threads/suggestion-on-battery-degradation-control.9281/):
  - 98 series groups, each 3 cells in parallel (1st gen). Resolution is 0.02 V, so a reading of
    0.02 V may be anything from ~0 to 0.039 V.
  - 0.02 V max-min is treated as normal; KiwiME: investigate anything over 0.04 V, measured at
    >= 80% SoC; a group that is persistently low never re-balancing indicates a self-discharging cell.
  - guesses of 0.05 V for a dash warning; >0.1 V described as pack-replacement territory with BMS
    severely limiting charge.
  - remedy for small imbalance: periodic 100% AC charge (90% will not balance).

## Reduction gear / motor noise
- Hyundai TSB 22-EV-001H-1 "Kona EV Motor Rumble Noise" (NHTSA copy
  https://static.nhtsa.gov/odi/tsbs/2022/MC-10225461-0001.pdf): 2019-2021 Kona Electric (OS EV),
  abnormal rumble from traction motor; repair: rumble-noise kit, roll mount and motor bolt
  replacement, inspection, then replace motor and/or reduction gear.
- Owners (speakev "Kona Electric Drive Train - Funny Noise",
  https://www.speakev.com/threads/kona-electric-drive-train-funny-noise.137774/; insideevsforum
  "2019 Kona EV Motor Noise Issue"): suspected spline wear between motor shaft and reducer input;
  black reducer oil early; replacing only the reducer often did not cure it.

## 12 V auxiliary battery
- Owner's manual: "Aux. Battery Saver+" lets the HV battery top up the 12 V battery
  (https://www.manualslib.com/manual/1485999/Hyundai-Kona-Electric-2018.html?page=66).
- speakev "Auxiliary Battery Saver Mode" (https://www.speakev.com/threads/auxiliary-battery-saver-mode.165236/):
  after a software update the option disappeared from the menu and is always on; front badge light
  turns yellow while the 12 V is being charged.
- Recurrent 12 V drain reports across 2019-2021 (hyundaikonaforum, chargeprosd.com). No specific US
  TSB number for a 12 V fix was found in this pass.
- 1st-gen Kona uses an LDC inside the EPCU plus a separate OBC; the "ICCU" failure widely reported is an
  E-GMP (Ioniq 5/6, EV6) part, not this car. (Inference from OBDb listing separate OBC/LDC signals on
  7E5; not from a Hyundai document.)

## Battery fire recalls (press)
- NHTSA Part 573 21V-127 (https://static.nhtsa.gov/odi/rcl/2021/RCLRPT-21V127-1095.PDF, via search
  summary): 4,694 MY2019-2020 Kona Electric built 2018-08-28 to 2020-03-02 that had not already had a
  pack replaced under Recall 196; cells from LG Energy Solution Nanjing with possibly folded anode tab;
  remedy replacement of the Battery System Assembly.
- Global 2026 BMS-software recall (electrive 2026-03-20
  https://www.electrive.com/2026/03/20/hyundai-recalls-over-100000-kona-electric-vehicles/; The Driven
  2026-05-18): KBA ref 16242R, Hyundai code 61D004, 104,011 1st-gen Kona Electric built 2018-01-24 to
  2023-07-15; software to detect cell voltage differences sooner; Australia recall 2026-04-10 includes
  cars that already had a replacement pack. **Not present in the NHTSA API for MY2020 as of 2026-11-07.**

## TPMS
- hyundaikonaforum "Anyone know how the TPMS system works?"
  (https://www.hyundaikonaforum.com/threads/anyone-know-how-the-tpms-system-works.1502/) and
  "Tire pressure display and tire rotation": direct TPMS; after rotation the car relearns positions by
  itself within ~5 min / ~10 miles of driving. New sensors must be registered with a TPMS tool.

## Service reminder
- Kia/Hyundai TSB text on NHTSA (MC-10220346/MC-10220347): cluster "User Settings > Service Interval"
  sets/resets the reminder; not set at the factory. Not Kona-EV-specific.
- Hyundai 2022 Kona EV quick reference guide (https://www.dgs.ca.gov/-/media/Divisions/OFAM/Statewide-Travel-Program/Forms/Miscellaneous/Car-rental-resources-page/2022-KONA-EV-QRG.pdf):
  coolant first replacement 120,000 mi / 120 months.
