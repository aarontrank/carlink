---
title: OBDb Honda-Clarity — 2018 Clarity PHEV signal set, responding ECUs and real captures
source_url: https://github.com/OBDb/Honda-Clarity/tree/20139af99ec4e2d5b6bbd4ca879cde9cab0c357d
fetched: 2026-11-13
license: CC-BY-SA-4.0 (OBDb repository LICENSE)
---

OBDb writes a 29-bit physical request header as `DAxx` (= `18DAxxF1`, tester F1) with replies from
`18DAF1xx`, and the functional OBD header as `DB33` (= `18DB33F1`). `bix` is a bit offset counted from
the first byte after the positive-response header (`62 xx xx` for service 22) — checked below against
OBDb's own expected values. Files: `signalsets/v3/default.json`, `tests/test_cases/<year>/command_support.yaml`
and `tests/test_cases/2018/commands/*.yaml` (real captures with expected decoded values). Extracted on
2026-11-13 with Ruby's YAML module and Python's json module.

## Which ECUs answer generic OBD (2018 captures)

`DB33.0101.yaml`, every capture: seven ECUs answer the functional `0101` on 29-bit CAN. Count of
identical replies across the file's captures:

`DB33.0101`:
```
18DAF101 06410100040000  x9
18DAF102 06410100040000  x9
18DAF106 06410100040000  x9
18DAF10E 0641010007E500  x9
18DAF115 06410100040000  x9
18DAF116 06410100040000  x9
18DAF12A 06410100040000  x9
```
`DB33.010C`:
```
18DAF101 04410C0000  x12
18DAF101 04410C1040  x1
18DAF101 04410C176C  x2
18DAF101 04410C17D4  x1
18DAF101 04410C180C  x1
18DAF101 04410C1AF8  x1
18DAF101 04410C1F7C  x1
18DAF101 04410C1F80  x1
18DAF101 04410C1FC0  x1
18DAF101 04410C20F4  x1
18DAF101 04410C21AC  x1
18DAF101 04410C2308  x1
18DAF101 04410C2F04  x1
18DAF102 04410C0000  x12
18DAF102 04410C132C  x1
18DAF102 04410C1768  x1
18DAF102 04410C176C  x1
18DAF102 04410C17FC  x1
18DAF102 04410C180C  x1
18DAF102 04410C1B28  x1
18DAF102 04410C1FA4  x1
18DAF102 04410C1FAC  x1
18DAF102 04410C1FB4  x1
18DAF102 04410C20E0  x1
18DAF102 04410C21A4  x1
18DAF102 04410C2318  x1
18DAF102 04410C2F04  x1
18DAF106 04410C0000  x12
18DAF106 04410C132C  x1
18DAF106 04410C1768  x1
18DAF106 04410C176C  x1
18DAF106 04410C17FC  x1
18DAF106 04410C180C  x1
18DAF106 04410C1B28  x1
18DAF106 04410C1FA4  x1
18DAF106 04410C1FAC  x1
18DAF106 04410C1FB4  x1
18DAF106 04410C20E0  x1
18DAF106 04410C21A4  x1
18DAF106 04410C2318  x1
```

Only `18DAF10E` reports a full monitor set (`41 01 00 07 E5 00`: spark ignition, misfire/fuel/components
plus catalyst, EVAP, O2 sensor and heater); the other six report components only (`00 04 00 00`). The same
captures show all seven mirroring rpm (`010C`) within a few rpm of each other. `18DAF10E` is the only
responder in the captures of `0106` (short-term fuel trim) and `0124` (wideband O2).

## Command support per ECU, 2018 (`command_support.yaml`, abridged: service 22 entries only)
```
supported_commands_by_ecu:
  "DA01":
  - DA01.01.222028:CLARITY_CEL_V_A1,CLARITY_CEL_V_A10,CLARITY_CEL_V_A11,CLARITY_CEL_V_A12,CLARITY_CEL_V_A13,CLARITY_CEL_V_A14,CLARITY_CEL_V_A15,CLARITY_CEL_V_A1
  - DA01.01.22202A:CLARITY_AAT_CAB,CLARITY_AAT_OUT,CLARITY_CELL_MAX_SOC,CLARITY_CELL_MIN_SOC,CLARITY_HVB_CAP,CLARITY_HVB_C_TOTAL,CLARITY_HVB_V,CLARITY_SOC
  - DA01.01.22202C:CLARITY_ECT_1,CLARITY_ECT_2,CLARITY_ECT_3,CLARITY_ECT_4,CLARITY_HVB_MOD_1A_T,CLARITY_HVB_MOD_3A_T
  "DA02":
  "DA06":
  "DA0E":
  - DA0E.0E.222240:CLARITY_BAT_DIST
  - DA0E.0E.222610:CLARITY_AC_CLU_PCM,CLARITY_GENERATOR_PCM
  - DA0E.0E.222611:CLARITY_AIRGAS_RATIO_PCM,CLARITY_FUEL_AIR_RAT_PCM
  - DA0E.0E.222612:CLARITY_CURRENT_GEAR_VAR_2,CLARITY_CURRENT_GEAR_VAR_3
  - DA0E.0E.222613:CLARITY_AC_P
  - DA0E.0E.222660:CLARITY_ODO_PCM,CLARITY_RUNTM_PCM
  - DA0E.0E.222661:CLARITY_FUELAIR_CAR_PCM
  - DA0E.0E.222662:CLARITY_KNOCK_CTRL_PCM,CLARITY_VTC_DUTY_PCM
  - DA0E.0E.222663:CLARITY_MIS_1_PCM,CLARITY_MIS_2_PCM,CLARITY_MIS_3_PCM,CLARITY_MIS_4_PCM,CLARITY_MIS_5_PCM,CLARITY_MIS_6_PCM,CLARITY_MIS_CNT_1_VAR_3_PCM,CLARI
  - DA0E.0E.22266C:CLARITY_MIS_CNT_1_VAR_1_PCM,CLARITY_MIS_CNT_2_VAR_1_PCM,CLARITY_MIS_CNT_3_VAR_1_PCM,CLARITY_MIS_CNT_4_VAR_1_PCM,CLARITY_MIS_CNT_5_VAR_1_PCM,C
  - DA0E.0E.22266D:CLARITY_AIR_FUEL_MIXTURE
  "DA10":
  - DA10.10.222660|fc=1:CLARITY_ODO,CLARITY_RUNTM
  - DA10.10.222661:CLARITY_FUELAIR_CAR
  - DA10.10.222663:CLARITY_MIS_1,CLARITY_MIS_2,CLARITY_MIS_3,CLARITY_MIS_4
  "DA11":
  - DA11.11.222612:CLARITY_VTEC_VALVE_CTRL
  - DA11.11.222660:CLARITY_RUNTM_V2
  - DA11.11.222662:CLARITY_CAT_T,CLARITY_KNOCK_CONTROL_V2,CLARITY_VARIABLE_TIM_CTRL
  "DA15":
  - DA15.15.222021:CLARITY_CHG_V_IN,CLARITY_CHG_V_OUT
  - DA15.15.222029:CLARITY_CEL_V_LIM,CLARITY_CHG_V_TGT
  "DA16":
  - DA16.16.222028:CLARITY_CEL_V_B1,CLARITY_CEL_V_B10,CLARITY_CEL_V_B11,CLARITY_CEL_V_B12,CLARITY_CEL_V_B13,CLARITY_CEL_V_B14,CLARITY_CEL_V_B15,CLARITY_CEL_V_B1
  - DA16.16.22202A:CLARITY_CEL_MAX_SOC_B,CLARITY_CEL_MIN_SOC_B,CLARITY_HVB_CAP_B,CLARITY_HVB_V_B
  - DA16.16.22202C:CLARITY_HVB_MOD_1B_T,CLARITY_HVB_MOD_2A_T,CLARITY_HVB_MOD_3B_T,CLARITY_HVB_MOD_4B_T
  "DA1D":
  - DA1D.1D.222201:CLARITY_ATF_T
  - DA1D.1D.222221|fc=1:CLARITY_GEAR_1D
  "DA28":
  - DA28.28.224004:CLARITY_BRK_SW
  - DA28.28.224005:CLARITY_BRK_P
  "DA2A":
  - DA2A.2A.224201:CLARITY_BRK_FLD_P,CLARITY_BRK_PED_STK,CLARITY_MSTR_CYL_STK,CLARITY_SLV_CYL_STK
  "DA60":
  - DA60.60.227022:CLARITY_ODO_V3
  - DA60.60.227028|fc=1:CLARITY_AAT
  - DA60.60.227029:CLARITY_FLV
  - DA60.60.227060:CLARITY_MAINT_AIR_CLN,CLARITY_MAINT_BRK_FLD,CLARITY_MAINT_CABIN_FLT,CLARITY_MAINT_COOL,CLARITY_MAINT_INSP,CLARITY_MAINT_OIL_FLT,CLARITY_MAINT
  - DBEF.222001:CLARITY_BATTERY_COOLANT_TEMPERATURE,CLARITY_BATTERY_MODULE_POWER,CLARITY_CABIN_AIR_TEMPERATURE,CLARITY_GENERATOR_MOTOR_SPEED,CLARITY_HV_BATTERY_
  - DBEF.222010:CLARITY_DC_DC_CONVERTER_TEMPERATURE,CLARITY_HV_BATTERY_CURRENT
  - DBEF.222012:CLARITY_HV_BATTERY_CELL_MAX_STATE_OF_CHARGE,CLARITY_HV_BATTERY_USABLE_CAPACITY,CLARITY_STATE_OF_CHARGE
unsupported_commands_by_ecu:   # requests these ECUs refused or ignored
  "DA01":
  - DA01.000001.0104
  - DA01.000001.010C
  - DA01.000001.010D
  - DA01.000001.0111
  - DA01.000001.0131
  - DA01.000001.0142
  - DA01.000001.015B
  "DA0E":
  - DA0E.00000E.0106
  - DA0E.00000E.0124
  - DA0E.00000E.0143
  - DA0E.00000E.0144
  - DA0E.00000E.222666
  - DA0E.0E.222615:CLARITY_ACTIVE_CYL_PCM
  "DA10":
  - DA10.000010.222614
  - DA10.000010.222668
  - DA10.10.222610:CLARITY_AC_CLUTCH,CLARITY_GENERATOR
  - DA10.10.222611:CLARITY_AIRGAS_RATIO,CLARITY_FUEL_AIR_RATIO
  - DA10.10.222612:CLARITY_VTEC_PSW,CLARITY_VTEC_SOL
  - DA10.10.222615:CLARITY_ACTIVE_CYL
  - DA10.10.222662:CLARITY_KNOCK_CONTROL,CLARITY_VTC_DUTY
  - DA10.10.222666:CLARITY_EOT_VAR_3,CLARITY_EOT_VAR_4
  - DA10.222660
  "DA11":
  - DA11.000011.222613
  - DA11.000011.222614
  - DA11.000011.222615
  - DA11.000011.222661
  - DA11.000011.222668
  - DA11.11.222610:CLARITY_AC_CLU_DA11,CLARITY_GENERATOR_DA11
  - DA11.11.222611:CLARITY_AIRGAS_RATIO_V2
  - DA11.11.222663:CLARITY_MIS_CNT_1_VAR_2,CLARITY_MIS_CNT_1_VAR_3,CLARITY_MIS_CNT_2_VAR_2,CLARITY_MIS_CNT_2_VAR_3,CLARITY_MIS_CNT_3_VAR_2,CLARITY_MIS_CNT_3_VAR
  - DA11.11.222666:CLARITY_EOT_VAR_1,CLARITY_EOT_VAR_2
  - DA11.11.22266C:CLARITY_MIS_CNT_1_VAR_1,CLARITY_MIS_CNT_2_VAR_1,CLARITY_MIS_CNT_3_VAR_1,CLARITY_MIS_CNT_4_VAR_1,CLARITY_MIS_CNT_5_VAR_1,CLARITY_MIS_CNT_6_VAR
  "DA1D":
  - DA1D.1D.222203:CLARITY_TRANS_OIL_P2
  - DA1D.1D.222660:CLARITY_ODO_V2
  - DA1D.1D.222663:CLARITY_MIS_CNT_1,CLARITY_MIS_CNT_2,CLARITY_MIS_CNT_3,CLARITY_MIS_CNT_4
  - DA1D.222221
  "DA1E":
  - DA1E.00001E.222201
  - DA1E.00001E.223083
  - DA1E.00001E.223086
  - DA1E.223083
  - DA1E.223086
  "DA26":
  - DA26.000026.226001
  - DA26.226001
  "DA60":
  - DA60.227028
  - DBEF.222222:CLARITY_HV_BATTERY_MAX_CELL_VOLTAGE
```

## Signal definitions carlink uses (`signalsets/v3/default.json`, verbatim)
```
{"hdr": "DA01", "cmd": {"22": "2028"}, "signals (first and last; 72 defined)": [{"id": "CLARITY_CEL_V_A1", "path": "Battery.Cells", "fmt": {"bix": 216, "len": 16, "max": 5000, "div": 5, "unit": "millivolts"}, "name": "Individual cell voltage A1"}, {"id": "CLARITY_CEL_V_A72", "path": "Battery.Cells", "fmt": {"bix": 1352, "len": 16, "max": 5000, "div": 5, "unit": "millivolts"}, "name": "Individual cell voltage A72"}]}
{"hdr": "DA01", "cmd": {"22": "202A"}, "signals": [{"id": "CLARITY_SOC", "path": "Battery", "fmt": {"bix": 400, "len": 8, "max": 100, "unit": "percent"}, "name": "State of charge", "suggestedMetric": "stateOfCharge"}, {"id": "CLARITY_HVB_C_TOTAL", "path": "Battery", "fmt": {"bix": 456, "len": 16, "max": 200, "min": -100, "div": 50, "sign": true, "unit": "amps"}, "name": "High voltage battery total current", "suggestedMetric": "tractionBatteryCurrent"}, {"id": "CLARITY_HVB_V", "path": "Battery", "fmt": {"bix": 488, "len": 16, "max": 400, "div": 20, "unit": "volts"}, "name": "High voltage battery voltage", "suggestedMetric": "tractionBatteryVoltage"}, {"id": "CLARITY_CELL_MAX_SOC", "path": "Battery", "fmt": {"bix": 584, "len": 16, "max": 100, "div": 100, "unit": "percent"}, "name": "High voltage battery cells, max state of charge"}, {"id": "CLARITY_CELL_MIN_SOC", "path": "Battery", "fmt": {"bix": 600, "len": 16, "max": 100, "div": 100, "unit": "percent"}, "name": "High voltage battery cells, max state of charge"}, {"id": "CLARITY_AAT_CAB", "path": "Engine", "fmt": {"bix": 832, "len": 8, "max": 127, "min": -40, "sign": true, "unit": "celsius"}, "name": "Ambient air temperature, cabin"}, {"id": "CLARITY_AAT_OUT", "path": "Engine", "fmt": {"bix": 840, "len": 8, "max": 127, "min": -40, "sign": true, "unit": "celsius"}, "name": "Ambient air temperature, outside"}, {"id": "CLARITY_HVB_CAP", "path": "Battery", "fmt": {"bix": 1336, "len": 16, "max": 655.35, "div": 100, "unit": "ampereHours"}, "name": "High voltage battery total capacity"}]}
{"hdr": "DA16", "cmd": {"22": "2028"}, "signals (first and last; 84 defined)": [{"id": "CLARITY_CEL_V_B1", "path": "Battery.Cells", "fmt": {"bix": 216, "len": 16, "max": 5000, "div": 5, "unit": "millivolts"}, "name": "Cell voltage, 1"}, {"id": "CLARITY_CEL_V_B84", "path": "Battery.Cells", "fmt": {"bix": 1544, "len": 16, "max": 5000, "div": 5, "unit": "millivolts"}, "name": "Cell voltage, 84"}]}
{"hdr": "DA16", "cmd": {"22": "202A"}, "signals": [{"id": "CLARITY_HVB_V_B", "path": "Battery", "fmt": {"bix": 488, "len": 16, "max": 400, "div": 20, "unit": "volts"}, "name": "High voltage battery voltage, line b", "suggestedMetric": "starterBatteryVoltage"}, {"id": "CLARITY_CEL_MAX_SOC_B", "path": "Battery.Cells", "fmt": {"bix": 584, "len": 16, "max": 100, "div": 100, "unit": "percent"}, "name": "Individual cell max SOC, line b"}, {"id": "CLARITY_CEL_MIN_SOC_B", "path": "Battery.Cells", "fmt": {"bix": 600, "len": 16, "max": 100, "div": 100, "unit": "percent"}, "name": "Individual cell min SOC, line b"}, {"id": "CLARITY_HVB_CAP_B", "path": "Battery.Cells", "fmt": {"bix": 1336, "len": 16, "max": 655.35, "div": 100, "unit": "ampereHours"}, "name": "Battery capacity, line b"}]}
{"hdr": "DA60", "cmd": {"22": "7022"}, "signals": [{"id": "CLARITY_ODO_V3", "path": "Trips", "fmt": {"bix": 48, "len": 24, "max": 200000, "unit": "miles"}, "name": "Odometer, variation 3"}]}
```

OBDb names 72 cells for `DA01 2028` and 84 for `DA16 2028`, but every capture of both carries 84
16-bit cell values from bit 216 followed by zeros (see the verification below).

## Verification of the bit offsets against OBDb's expected values (Python, 2026-11-13)

Every capture in the four files was reassembled from its ISO-TP frames and each defined signal decoded
as `raw × mul / div + add` from `bix`:

| File | captures | values checked | mismatches |
|---|---|---|---|
| DA01.01.22202A | 147 | 1176 | 10 |
| DA16.16.22202A | 88 | 352 | 4 |
| DA01.01.222028 | 116 | 8352 | 0 |
| DA16.16.222028 | 106 | 8904 | 0 |
| DA60.60.227022 | 7 | 7 | 0 |

The 202A mismatches are OBDb clamping values above a signal's `max` (`CLARITY_HVB_C_TOTAL` 216 and 271 A
expected as 200) and one malformed capture in each file (values like 309.75 Ah).

## Capacity readings across all 2018 captures

`DA01.01.22202A` `bix 1336` ÷ 100 (Ah), value: count — 39.8: 1, 39.9: 19, 40.1: 9, 40.2: 9, 40.4: 10, 54.5: 3, 55.0: 95, 309.8: 1

`DA16.16.22202A` `bix 1336` ÷ 100 (Ah), value: count — 20.2: 2, 20.5: 3, 20.7: 4, 20.8: 1, 20.9: 11, 21.8: 8, 27.5: 58, 309.8: 1

## Captures used by the carlink simulator

Chosen so the pack voltages roughly agree with each other (they are from different moments):
`DA01.01.22202A` test case 0 (318.2 V, 40.16 Ah), `DA16.16.22202A` test case 27 (318.7 V, 20.48 Ah),
`DA01.01.222028` test case 1 (84 cells sum to 318.5 V, A1 = 3791.8 mV, spread 10 mV),
`DA16.16.222028` test case 0 (sum 320.0 V, B1 = 3810.6 mV, spread 6 mV), `DA60.60.227022` test case 0
(90033 mi). Their reassembled payloads are in `src/vehicles/clarity.rs` (`SIM_REPLIES`).
