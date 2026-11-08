---
title: OBDb Honda-Odyssey and Honda-Pilot — 29-bit request headers, per-year supported/refused ECUs, real 2011–2015 Odyssey captures
source_url: https://github.com/OBDb/Honda-Odyssey/tree/035b49cd9466b99986180f0ac3e8772ad4a1bbb3
source_url_2: https://github.com/OBDb/Honda-Pilot/tree/55d57a26f06c54344cb0b4e482545ff82651ca58
fetched: 2026-11-08
---

OBDb writes a 29-bit physical header as `DAxx` (= `18DAxxF1`, tester F1) with `rax: xx` (reply
from `18DAF1xx`), and the 29-bit functional OBD header as `DB33` (= `18DB33F1`). Generations per
`generations.yaml` of Honda-Odyssey: 4th generation 2011–2017, 5th generation 2018–. Extracted
with Python's json module and grep on 2026-11-08.

## Honda-Odyssey signalsets/v3/default.json — headers and commands

```
('DA0E', '0E', None, None, None) 11
      ('{"22": "2610"}', 'ODYSSEY_GENERATOR_PCM', 'dbg', '')
      ('{"22": "2611"}', 'ODYSSEY_AIRGAS_RATIO_PCM', 'dbg', '')
      ('{"22": "2612"}', 'ODYSSEY_VTEC_VALVE_CTRL_PCM', '', '{"to": 2011, "from": 2013}')
      ('{"22": "2613"}', 'ODYSSEY_AC_P', '', '{"to": 2011, "from": 2013}')
      ('{"22": "2615"}', 'ODYSSEY_ACTIVE_CYL_PCM', 'dbg', '')
      ('{"22": "2660"}', 'ODYSSEY_ODO_PCM', 'dbg', '')
('DA10', '10', None, None, None) 9
      ('{"22": "2610"}', 'ODYSSEY_GENERATOR', '', '{"years": [2019, 2022], "from": 2026}')
      ('{"22": "2611"}', 'ODYSSEY_AIRGAS_RATIO', '', '{"years": [2019, 2022], "from": 2026}')
      ('{"22": "2612"}', 'ODYSSEY_VTEC_PSW', '', '{"years": [2019, 2022], "from": 2024}')
      ('{"22": "2615"}', 'ODYSSEY_ACTIVE_CYL', '', '{"years": [2019, 2022], "from": 2026}')
      ('{"22": "2660"}', 'ODYSSEY_ODO', '', '{"years": [2019, 2022], "from": 2024}')
      ('{"22": "2661"}', 'ODYSSEY_FUELAIR_CAR', '', '{"years": [2019, 2022], "from": 2026}')
('DA11', '11', None, None, None) 8
      ('{"22": "2610"}', 'ODYSSEY_GENERATOR_DA11', '', '{"to": 2012, "from": 2014}')
      ('{"22": "2611"}', 'ODYSSEY_AIRGAS_RATIO_V2', '', '{"to": 2009, "years": [2012], "from": 2016}')
      ('{"22": "2612"}', 'ODYSSEY_VTEC_VALVE_CTRL', '', '{"to": 2009, "years": [2012], "from": 2016}')
      ('{"22": "2660"}', 'ODYSSEY_RUNTM_V2', '', '{"to": 2009, "years": [2012], "from": 2016}')
      ('{"22": "2662"}', 'ODYSSEY_KNOCK_CONTROL_V2', '', '{"to": 2009, "years": [2012], "from": 2016}')
      ('{"22": "2663"}', 'ODYSSEY_MISFIRE_COUNT_1_VAR_3', '', '{"to": 2012, "from": 2014}')
('DA1D', '1D', None, None, None) 3
      ('{"22": "2201"}', 'ODYSSEY_ATF_T', '', '{"to": 2009, "years": [2012], "from": 2016}')
      ('{"22": "2221"}', 'ODYSSEY_GEAR', '', '{"to": 2009, "years": [2012], "from": 2016}')
      ('{"22": "2663"}', 'ODYSSEY_MISFIRE_COUNT_1', 'dbg', '')
('DA1E', '1E', None, None, None) 3
      ('{"22": "2201"}', 'ODYSSEY_ATF_T_VAR_4', '', '{"to": 2020, "from": 2022}')
      ('{"22": "3083"}', 'ODYSSEY_ATF_T_V2', '', '{"to": 2017, "from": 2019}')
      ('{"22": "3086"}', 'ODYSSEY_GEAR_V2', '', '{"to": 2017, "from": 2019}')
('DA26', '26', None, None, None) 1
      ('{"22": "6001"}', 'ODYSSEY_TP_FR', '', '{"years": [2019, 2022], "from": 2024}')
('DA28', '28', None, None, None) 2
      ('{"22": "4004"}', 'ODYSSEY_BRK_SW', '', '{"years": [2019, 2022], "from": 2024}')
      ('{"22": "4005"}', 'ODYSSEY_BRK_P', '', '{"years": [2019, 2022], "from": 2024}')
('DA60', '60', None, None, None) 3
      ('{"22": "7022"}', 'ODYSSEY_ODO_V3', '', '{"years": [2019, 2022], "from": 2024}')
      ('{"22": "7028"}', 'ODYSSEY_AAT', '', '{"years": [2019, 2022], "from": 2024}')
      ('{"22": "7029"}', 'ODYSSEY_FLV', '', '{"years": [2019, 2022], "from": 2026}')
```

## Honda-Odyssey tests — reply IDs seen in the functional (DB33) OBD captures, per model year

- **2011**: 18DAF111 (51 lines) 18DAF11D (51 lines) 
- **2013**: 18DAF111 (36 lines) 18DAF11D (36 lines) 
- **2014**: 18DAF111 (32 lines) 18DAF11D (32 lines) 
- **2015**: 18DAF111 (19 lines) 18DAF11D (19 lines) 
- **2018**: 18DAF110 (10 lines) 18DAF11E (10 lines) 

## Honda-Odyssey tests — ECU keys per model year (supported vs refused/unanswered)

- **2011** supported: 7E0 DA11 DA1D DB33 
  refused/unanswered: DA01 DA0E DA10 DA15 DA16 DA1D DA1E DA26 DA28 DA2A DA60 
- **2012** supported: DA0E DB33 
  refused/unanswered: DA01 DA0E DA10 DA11 DA15 DA16 DA1D DA1E DA26 DA28 DA2A DA33 DA60 DB33 
- **2013** supported: DA11 DA1D DB33 
  refused/unanswered: DA0E DA10 DA11 DA1D DA1E DA26 DA60 
- **2014** supported: DA11 DA1D DB33 
  refused/unanswered: 7E0 7E2 7E4 7E7 DA01 DA0E DA10 DA15 DA16 DA1D DA1E DA26 DA28 DA2A DA60 DB33 
- **2015** supported: DA11 DA1D DB33 
  refused/unanswered: DA01 DA0E DA10 DA15 DA16 DA1D DA1E DA26 DA28 DA2A DA60 
- **2018** supported: DA10 DA1E DA26 DA28 DA60 DB33 
  refused/unanswered: DA01 DA0E DA11 DA15 DA16 DA1D DA2A DA60 

2014 Odyssey refused/unanswered reads for chassis/body addresses (verbatim lines):

```
  "DA01":
  - DA01.000001.222028
  - DA01.000001.22202A
  - DA01.000001.22202C
  "DA0E":
  - DA0E.00000E.222240
  - DA0E.0E.222613:ODYSSEY_AC_P
  "DA10":
  - DA10.000010.222610
  - DA10.000010.222611
  - DA10.000010.222612
  - DA10.000010.222615
  - DA10.000010.222660
--
  "DA1D":
  - DA1D.00001D.222203
  - DA1D.00001D.222660
  "DA1E":
  - DA1E.1E.223083:ODYSSEY_ATF_T_V2
  - DA1E.1E.223086:ODYSSEY_GEAR_V2
  "DA26":
  - DA26.000026.226001
  "DA28":
  - DA28.000028.224004
  - DA28.000028.224005
  "DA2A":
  - DA2A.00002A.224201
  "DA60":
  - DA60.000060.227022
  - DA60.000060.227028
  - DA60.000060.227029
  - DA60.000060.227060
  "DB33":
```

## Real 2014 Odyssey captures (first test case of each file, verbatim)

### tests/test_cases/2014/commands/DB33.0101.yaml

```yaml
command_id: DB33.0101
test_cases:
- expected_values:
    ACRF_RDY: 0
    ACRF_SUP: 0
    AIR_RDY: 0
    AIR_SUP: 0
    CAT_RDY: 0
    CAT_SUP: 1
    CCM_RDY: 0
    CCM_SUP: 1
    CIM_SUP: SPARK
    DTC_CNT: 1
    EGR_RDY: 0
    EGR_SUP: 1
    EVAP_RDY: 0
    EVAP_SUP: 1
    FUEL_RDY: 0
    FUEL_SUP: 1
    HCAT_RDY: 0
    HCAT_SUP: 0
    HTR_RDY: 0
    HTR_SUP: 1
    MIL: 0
    MIS_RDY: 0
    MIS_SUP: 1
    O2S_RDY: 0
    O2S_SUP: 1
  response: |-
    18DAF11D06410100040000
    18DAF1110641010107E500
```

### tests/test_cases/2014/commands/DA1D.1D.222201|f=-2017.yaml

```yaml
command_id: DA1D.1D.222201|f=-2017
test_cases:
- expected_values:
    ODYSSEY_ATF_T: 70
  response: |-
    18DAF11D1027622201FFFFFF
    18DAF11D2180000000000000
    18DAF11D22000000172E5800
    18DAF11D233118267A946594
    18DAF11D2465446E79000000
    18DAF11D2500000000005555
```

### tests/test_cases/2014/commands/DA11.11.222660|f=-2017.yaml

```yaml
command_id: DA11.11.222660|f=-2017
test_cases:
- expected_values:
    ODYSSEY_RUNTM_V2: 190
  response: |-
    18DAF1111039622660801FFF
    18DAF11121F0F7F001000000
    18DAF1112200000000000000
    18DAF11123006101971E0001
    18DAF1112400040001018300
    18DAF111250103B900000000
    18DAF111260364047700023E
    18DAF111273C00BE00280000
    18DAF1112800005555555555
```

## Honda-Pilot tests — ECU keys per model year (2009/2013 = 2nd generation, same era as the 4th-gen Odyssey; 2017 = 3rd generation)

- **2009** supported: DA0E DA10 DA11 DA1D DB33 
  refused/unanswered: DA0E DA10 DA11 DA1D DA1E DA26 DA28 DA2A DA30 DA53 DA60 
- **2013** supported: 7E0 7E2 7E5 DA11 DA1D DB33 
  refused/unanswered: DA01 DA0E DA10 DA15 DA16 DA1D DA1E DA26 DA28 DA2A DA60 
- **2017** supported: DA0E DA10 DA1D DA1E DA26 DA28 DA60 DB33 
  refused/unanswered: DA01 DA0E DA10 DA11 DA15 DA16 DA1D DA1E DA26 DA2A DA33 DA60 DB33 

2009 Pilot refused/unanswered reads at VSA/EPS/SRS/gauge/TPMS addresses (verbatim lines):

```
  "DA26":
  - DA26.26.226001:PILOT_TP_FL,PILOT_TP_FR,PILOT_TP_RL,PILOT_TP_RR,PILOT_TT_FL,PILOT_TT_FR,PILOT_TT_RL,PILOT_TT_RR
  - DA26.26.226010:PILOT_RAW_26_6010
  "DA28":
  - DA28.000028.22F100
  - DA28.000028.22F110
  - DA28.000028.22F112
--
  "DA30":
  - DA30.30.223004:PILOT_RAW_30_3004
  - DA30.30.223008:PILOT_RAW_30_3008
  "DA53":
  - DA53.53.220006:PILOT_RAW_53_0006
  - DA53.53.228000:PILOT_RAW_53_8000
  - DA53.53.228001:PILOT_RAW_53_8001
--
  "DA60":
  - DA60.000060.22F110
  - DA60.000060.22F112
  - DA60.000060.22F181
```
