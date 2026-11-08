---
title: OBDb Hyundai-Kona-Electric — every request header in the signalset, per-year supported/refused ECUs, and real captures from 770/7D1/7D4/7E5
source_url: https://github.com/OBDb/Hyundai-Kona-Electric/tree/fc3ae847eda89590bf17c5322a0bf18b3f666ed5
commit: fc3ae847eda89590bf17c5322a0bf18b3f666ed5
fetched: 2026-11-08
---

Complements `raw/obdb-hyundai-kona-electric.md` (same commit), which only excerpted the BMS, VMCU,
TPMS and cluster commands. Here: every command object's request header (`hdr`), response header
(`rax`), command, `dbg` flag and `dbgfilter` (OBDb's per-model-year "not yet confirmed" filter),
extracted with Python's json module from `signalsets/v3/default.json`; the ECU keys of each
`tests/test_cases/<year>/command_support.yaml` (supported vs. refused/unanswered); and the first
test case of the four 2021 captures that prove 770, 7D1, 7D4 and 7E5 answer. 2024 is the 2nd
generation (SX2); 2019–2023 are the 1st generation (OS EV) per `generations.yaml`
("First Generation start_year: 2018 end_year: 2023").

## signalsets/v3/default.json — all commands (hdr rax cmd dbg dbgfilter fcm1)

```
730 738 {"22": "F010"} dbg= True dbgfilter= null fcm1= None filter= null
744 74C {"22": "E001"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= True filter= null
744 74C {"22": "E002"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= None filter= null
744 74C {"22": "E003"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= True filter= null
744 74C {"22": "E004"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= None filter= null
770 778 {"22": "BC03"} dbg= None dbgfilter= {"to": 2020, "from": 2024} fcm1= None filter= null
770 778 {"22": "BC04"} dbg= None dbgfilter= {"to": 2020, "years": [2022], "from": 2025} fcm1= None filter= null
770 778 {"22": "BC05"} dbg= None dbgfilter= {"to": 2020, "from": 2025} fcm1= None filter= null
770 778 {"22": "BC06"} dbg= None dbgfilter= {"to": 2020, "years": [2023], "from": 2025} fcm1= None filter= null
770 778 {"22": "BC07"} dbg= None dbgfilter= {"to": 2020, "from": 2024} fcm1= None filter= null
770 778 {"22": "BC08"} dbg= None dbgfilter= {"to": 2020, "from": 2025} fcm1= None filter= null
770 778 {"22": "BC09"} dbg= None dbgfilter= {"to": 2021, "from": 2025} fcm1= None filter= null
7A0 7A8 {"22": "B00C"} dbg= None dbgfilter= {"to": 2020, "from": 2022} fcm1= True filter= null
7A0 7A8 {"22": "C00B"} dbg= None dbgfilter= {"years": [2019], "from": 2025} fcm1= True filter= null
7A0 7A8 {"22": "C00C"} dbg= None dbgfilter= {"to": 2020, "years": [2022, 2023], "from": 2025} fcm1= True filter= null
7A0 7A8 {"22": "C00D"} dbg= None dbgfilter= {"to": 2020, "years": [2022, 2023], "from": 2025} fcm1= True filter= null
7A0 7A8 {"22": "C00E"} dbg= None dbgfilter= {"to": 2020, "years": [2022, 2023], "from": 2025} fcm1= True filter= null
7A0 7A8 {"22": "C00F"} dbg= None dbgfilter= {"to": 2020, "years": [2022, 2023], "from": 2025} fcm1= True filter= null
7B3 7BB {"22": "0100"} dbg= None dbgfilter= {"to": 2020, "from": 2025} fcm1= True filter= null
7B3 7BB {"22": "0101"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= None filter= null
7B3 7BB {"22": "0102"} dbg= None dbgfilter= {"to": 2020, "years": [2022, 2023], "from": 2025} fcm1= None filter= null
7C6 7CE {"22": "B002"} dbg= None dbgfilter= {"years": [2019], "from": 2025} fcm1= True filter= null
7D1 7D9 {"21": "01"} dbg= True dbgfilter= null fcm1= None filter= null
7D1 7D9 {"22": "0104"} dbg= None dbgfilter= {"to": 2020, "from": 2025} fcm1= None filter= null
7D1 7D9 {"22": "4000"} dbg= True dbgfilter= null fcm1= None filter= null
7D1 7D9 {"22": "C101"} dbg= True dbgfilter= null fcm1= None filter= null
7D4 7DC {"21": "01"} dbg= True dbgfilter= null fcm1= None filter= null
7D4 7DC {"22": "0101"} dbg= None dbgfilter= {"to": 2020, "years": [2022], "from": 2025} fcm1= None filter= null
7D6 7DE {"21": "06"} dbg= True dbgfilter= null fcm1= None filter= null
7E2 7EA {"21": "01"} dbg= None dbgfilter= {"to": 2019, "from": 2024} fcm1= None filter= null
7E2 7EA {"21": "02"} dbg= None dbgfilter= {"to": 2020, "from": 2022} fcm1= None filter= null
7E2 7EA {"22": "E004"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= None filter= null
7E2 7EA {"22": "E005"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= None filter= null
7E3 7EB {"22": "0102"} dbg= True dbgfilter= null fcm1= None filter= null
7E3 7EB {"22": "E001"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= None filter= null
7E3 7EB {"22": "E009"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= None filter= null
7E4 7EC {"22": "0101"} dbg= None dbgfilter= {"to": 2019, "from": 2025} fcm1= True filter= null
7E4 7EC {"22": "0102"} dbg= None dbgfilter= {"to": 2019, "from": 2025} fcm1= True filter= null
7E4 7EC {"22": "0103"} dbg= None dbgfilter= {"to": 2019, "from": 2025} fcm1= True filter= null
7E4 7EC {"22": "0104"} dbg= None dbgfilter= {"to": 2019, "from": 2025} fcm1= True filter= null
7E4 7EC {"22": "0105"} dbg= None dbgfilter= {"to": 2019, "from": 2025} fcm1= True filter= null
7E4 7EC {"22": "0106"} dbg= None dbgfilter= {"to": 2020, "years": [2022, 2023], "from": 2025} fcm1= True filter= null
7E4 7EC {"22": "010A"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= True filter= null
7E4 7EC {"22": "010B"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= True filter= null
7E4 7EC {"22": "010C"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= True filter= null
7E4 7EC {"22": "0111"} dbg= None dbgfilter= {"to": 2020, "years": [2023], "from": 2025} fcm1= None filter= null
7E4 7EC {"22": "0114"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= None filter= null
7E5 None {"21": "C0"} dbg= True dbgfilter= null fcm1= None filter= null
7E5 7ED {"21": "01"} dbg= None dbgfilter= {"to": 2020, "from": 2022} fcm1= None filter= null
7E5 7ED {"22": "E001"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= None filter= null
7E5 7ED {"22": "E002"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= None filter= null
7E5 7ED {"22": "E003"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= None filter= null
7E5 7ED {"22": "E011"} dbg= None dbgfilter= {"to": 2023, "from": 2025} fcm1= None filter= null
7E5 7ED {"22": "E101"} dbg= True dbgfilter= null fcm1= None filter= null
7E7 7EF {"22": "C101"} dbg= True dbgfilter= null fcm1= None filter= null
```

## tests/test_cases/<year>/command_support.yaml — ECU keys only

- **2019** supported: 7E3 7E4 
  refused/unanswered: 
- **2020** supported: 7A0 7C6 7E2 7E3 7E4 
  refused/unanswered: 
- **2021** supported: 770 7A0 7B3 7C6 7D1 7D4 7E2 7E3 7E4 7E5 
  refused/unanswered: 730 740 744 770 7B3 7D1 7D4 7D6 7E0 7E1 7E2 7E3 7E4 7E5 7E7 
- **2022** supported: 770 7A0 7B3 7C6 7D1 7E0 7E1 7E2 7E3 7E4 7E6 
  refused/unanswered: 730 744 7B3 7D1 7D4 7E0 7E1 7E3 7E4 
- **2023** supported: 770 7A0 7B3 7C6 7D1 7D4 7E2 7E4 
  refused/unanswered: 744 7D1 7D4 7E0 7E1 7E2 7E3 7E4 7E7 
- **2024** supported: 744 770 7A0 7B3 7C6 7D1 7D4 7E2 7E3 7E4 7E5 7E6 
  refused/unanswered: 730 740 770 7A0 7D1 7D4 7D6 7E0 7E1 7E2 7E3 7E5 7E7 

2021 refused/unanswered entries for the chassis modules (verbatim lines):

```
  "730":
  - 730.738.22F010:KONAEV_LATERAL_ACCELERATION,KONAEV_LONGITUDINAL_ACCELERATION,KONAEV_STEERING_ANGLE,KONAEV_WHEEL_FL_VSS_HD,KONAEV_WHEEL_FR
  "740":
  - 740.748.2201A1
  - 740.748.2201A3
  "744":
  - 744.74C.22E001|fc=1:KONAEV_AC_CHRG_STATE,KONAEV_AUX_BATTERY_VOLTAGE,KONAEV_DC_CHRG_STATE,KONAEV_ESTIMATED_CHRG_POWER,KONAEV_OBC_CHRG_AC_
--
  "7D1":
  - 7D1.7D9.2101:KONAEV_FL_VSS,KONAEV_FR_VSS,KONAEV_RL_VSS,KONAEV_RR_VSS
  - 7D1.7D9.224000:KONAEV_TIRE_FL_VSS,KONAEV_TIRE_FR_VSS,KONAEV_TIRE_RL_VSS,KONAEV_TIRE_RR_VSS
  - 7D1.7D9.22C101:KONAEV_FL_WHEEL_VSS,KONAEV_FR_WHEEL_VSS,KONAEV_RL_WHEEL_VSS,KONAEV_RR_WHEEL_VSS
  "7D4":
  - 7D4.7DC.2101:KONAEV_STEER_ANGLE
  "7D6":
  - 7D6.7DE.2106:KONAEV_FL_TP,KONAEV_FL_TT,KONAEV_FR_TP,KONAEV_FR_TT,KONAEV_RL_TP,KONAEV_RL_TT,KONAEV_RR_TP,KONAEV_RR_TT
  "7E0":
  - 7E0.7D8.2101
  - 7E0.7D8.2118
```

## Real 2021 captures (first test case of each file, verbatim)

### tests/test_cases/2021/commands/770.778.22BC03.yaml

```yaml
command_id: 770.778.22BC03
test_cases:
- expected_values:
    KONAEV_LOW_BEAMS: 0
  response: |-
    778100B62BC03FDEE3C
    778217300040000AAAA
```

### tests/test_cases/2021/commands/7D1.7D9.220104.yaml

```yaml
command_id: 7D1.7D9.220104
test_cases:
- expected_values:
    KONAEV_WHEEL_FL_VSS: 0
    KONAEV_WHEEL_FR_VSS: 0
    KONAEV_WHEEL_RL_VSS: 0
    KONAEV_WHEEL_RR_VSS: 0
  response: |-
    7D9102D620104FFFFFF
    7D921F80000000001E1
    7D92200000000FF9CFF
    7D92380FFC5000000F0
    7D924D5000000551000
    7D92521F5FFF8000000
    7D92600FC0006AAAAAA
```

### tests/test_cases/2021/commands/7D4.7DC.220101.yaml

```yaml
command_id: 7D4.7DC.220101
test_cases:
- expected_values:
    KONAEV_MDPS_MOTOR_CURRENT: 16
    KONAEV_WHEEL_ANGLE_ABS: 0
  response: |-
    7DC1012620101838701
    7DC2100000000000000
    7DC220010101D21AAAA
```

### tests/test_cases/2021/commands/7E5.7ED.2101.yaml

```yaml
command_id: 7E5.7ED.2101
test_cases:
- expected_values:
    KONAEV_CHARGER_AC_INPUT: 0
    KONAEV_CHARGER_AC_INPUT_V: 0
    KONAEV_CHARGER_AC_INPUT_V_INSTANT: -100
    KONAEV_CHARGER_T: 14
    KONAEV_CHARGER_T_B: 14
  response: |-
    7ED103A6101FFFE57FC
    7ED2100000000000000
    7ED22000072003401BD
    7ED2305412100000000
    7ED2400000000000000
    7ED25000001F4000000
    7ED2600000072000000
    7ED27053B0C0F3A0000
    7ED2800000000000000
```

## PR #367 (merged 2026-10-25) — description as returned by WebFetch on 2026-11-08

https://github.com/OBDb/Hyundai-Kona-Electric/pull/367 — "Remove the engine, transmission and 4WD
reads the Kona Electric refused" (author jverkoey):

> The Kona Electric's signalset asked its `7E0`, `7E1` and `740` modules 57 gas and diesel engine,
> transmission and 4WD reads: diesel particulate filter, injection, turbo boost, knock, dual-clutch
> gears. Every one was asked by the 2021-2024 captures and refused, and none was ever answered. The
> Kona Electric has no engine computer, transmission control module or 4WD module. This removes all
> 57: 50 on `7E0`, five on `7E1` and two on `740`. No fixture changed.
