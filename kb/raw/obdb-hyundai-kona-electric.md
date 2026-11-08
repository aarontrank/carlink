---
title: OBDb Hyundai-Kona-Electric — signalset excerpts and real-vehicle test captures (2019/2020)
source_url: https://github.com/OBDb/Hyundai-Kona-Electric/tree/fc3ae847eda89590bf17c5322a0bf18b3f666ed5
commit: fc3ae847eda89590bf17c5322a0bf18b3f666ed5
fetched: 2026-11-07
---

OBDb (Clutch Engineering, used by the Sidecar/Pelican apps). Signal `fmt.bix` is a **bit index**
counted from the first byte after the positive-response header (after `62 DID_hi DID_lo` for service 22,
after `61 id` for service 21); bit 0 = MSB of that byte. `len` in bits; big-endian unless `blsb: true`
(little-endian). `fcm1: true` = the app sends ATFCSH<hdr>, ATFCSD300000, ATFCSM1 before the command
(see https://pelican.clutch.engineering/scanning/extended-pids/). Test captures are headers-on,
spaces-off ELM output from real cars with the expected decoded values.

## service01/modelyears.json (which ECUs answered generic Service 01 PIDs, per model year)

```json
{
  "2019": {
    "7E3": [ "01", "21", "30", "31" ],
    "7E4": [ "01", "21", "30", "31", "5B" ]
  },
  "2021": {
    "7E3": [ "01", "21", "30", "31" ],
    "7E4": [ "01", "21", "30", "31", "5B" ]
  },
  "2022": {
    "7E3": [ "01", "21", "30", "31" ],
    "7E4": [ "01", "21", "30", "31", "5B" ]
  },
  "2024": {
    "7E4": [ "01", "21", "30", "31", "5B", "9A" ],
    "7E6": [ "01" ]
  }
}
```

## tests/test_cases/2019/command_support.yaml

```yaml
model_year: 2019
supported_commands_by_ecu:
  "7E3":
  - 0101:ACRF_RDY,ACRF_SUP,AIR_RDY,AIR_SUP,CAT_RDY,CAT_SUP,CCM_RDY,CCM_SUP,CIM_SUP,DTC_CNT,EGR_RDY,EGR_SUP,EVAP_RDY,EVAP_SUP,FUEL_RDY,FUEL_SUP,HCAT_RDY,HCAT_SUP,HTR_RDY,HTR_SUP,MIL,MIS_RDY,MIS_SUP,O2S_RDY,O2S_SUP
  - 0121:MIL_DIST
  - 0130:WARM_UPS
  - 0131:CLR_DIST
  "7E4":
  - 0101:ACRF_RDY,ACRF_SUP,AIR_RDY,AIR_SUP,CAT_RDY,CAT_SUP,CCM_RDY,CCM_SUP,CIM_SUP,DTC_CNT,EGR_RDY,EGR_SUP,EVAP_RDY,EVAP_SUP,FUEL_RDY,FUEL_SUP,HCAT_RDY,HCAT_SUP,HTR_RDY,HTR_SUP,MIL,MIS_RDY,MIS_SUP,O2S_RDY,O2S_SUP
  - 0121:MIL_DIST
  - 0130:WARM_UPS
  - 0131:CLR_DIST
  - 015B:BAT_SOC

```

## tests/test_cases/2020/command_support.yaml

```yaml
model_year: 2020
supported_commands_by_ecu:
  "7A0":
  - 7A0.7A8.22C00B|fc=1:KONAEV_TP_FL,KONAEV_TP_FR,KONAEV_TP_RL,KONAEV_TP_RR,KONAEV_TT_FL,KONAEV_TT_FR,KONAEV_TT_RL,KONAEV_TT_RR
  "7C6":
  - 7C6.7CE.22B002|fc=1:KONAEV_ODO_KM,KONAEV_ODO_MI
  "7E2":
  - 7E2.7EA.2101:KONAEV_VSS
  "7E3":
  - 0101:ACRF_RDY,ACRF_SUP,AIR_RDY,AIR_SUP,CAT_RDY,CAT_SUP,CCM_RDY,CCM_SUP,CIM_SUP,DTC_CNT,EGR_RDY,EGR_SUP,EVAP_RDY,EVAP_SUP,FUEL_RDY,FUEL_SUP,HCAT_RDY,HCAT_SUP,HTR_RDY,HTR_SUP,MIL,MIS_RDY,MIS_SUP,O2S_RDY,O2S_SUP
  - 0121:MIL_DIST
  - 0130:WARM_UPS
  - 0131:CLR_DIST
  "7E4":
  - 0101:ACRF_RDY,ACRF_SUP,AIR_RDY,AIR_SUP,CAT_RDY,CAT_SUP,CCM_RDY,CCM_SUP,CIM_SUP,DTC_CNT,EGR_RDY,EGR_SUP,EVAP_RDY,EVAP_SUP,FUEL_RDY,FUEL_SUP,HCAT_RDY,HCAT_SUP,HTR_RDY,HTR_SUP,MIL,MIS_RDY,MIS_SUP,O2S_RDY,O2S_SUP
  - 0121:MIL_DIST
  - 0130:WARM_UPS
  - 0131:CLR_DIST
  - 015B:BAT_SOC
  - 7E4.7EC.220101|fc=1:KONAEV_BMS_MAIN_RELAY,KONAEV_C_V_MAX,KONAEV_C_V_MAX_ID,KONAEV_C_V_MIN,KONAEV_C_V_MIN_ID,KONAEV_DRIVE_TIME,KONAEV_HVBAT_CHARGING,KONAEV_HVBAT_CHG_P_AVAIL,KONAEV_HVBAT_CHRG_TOT_C,KONAEV_HVBAT_CHRG_TOT_E,KONAEV_HVBAT_CURR,KONAEV_HVBAT_DCHG_P_AVAIL,KONAEV_HVBAT_DSCH_TOT_C,KONAEV_HVBAT_DSCH_TOT_E,KONAEV_HVBAT_FAN,KONAEV_HVBAT_FAN_STATUS,KONAEV_HVBAT_INLET_T,KONAEV_HVBAT_MOD1_T,KONAEV_HVBAT_MOD2_T,KONAEV_HVBAT_MOD3_T,KONAEV_HVBAT_MOD4_T,KONAEV_HVBAT_PLUG_NORM,KONAEV_HVBAT_PLUG_RAPD,KONAEV_HVBAT_SOC,KONAEV_HVBAT_T_MAX,KONAEV_HVBAT_T_MIN,KONAEV_HVBAT_VDC,KONAEV_IGN,KONAEV_INVRT_CV,KONAEV_ISO_R,KONAEV_RPM,KONAEV_RPM_2,KONAEV_VPWR
  - 7E4.7EC.220102|fc=1:KONAEV_HVBAT_CMU001_VOLT,KONAEV_HVBAT_CMU002_VOLT,KONAEV_HVBAT_CMU003_VOLT,KONAEV_HVBAT_CMU004_VOLT,KONAEV_HVBAT_CMU005_VOLT,KONAEV_HVBAT_CMU006_VOLT,KONAEV_HVBAT_CMU007_VOLT,KONAEV_HVBAT_CMU008_VOLT,KONAEV_HVBAT_CMU009_VOLT,KONAEV_HVBAT_CMU010_VOLT,KONAEV_HVBAT_CMU011_VOLT,KONAEV_HVBAT_CMU012_VOLT,KONAEV_HVBAT_CMU013_VOLT,KONAEV_HVBAT_CMU014_VOLT,KONAEV_HVBAT_CMU015_VOLT,KONAEV_HVBAT_CMU016_VOLT,KONAEV_HVBAT_CMU017_VOLT,KONAEV_HVBAT_CMU018_VOLT,KONAEV_HVBAT_CMU019_VOLT,KONAEV_HVBAT_CMU020_VOLT,KONAEV_HVBAT_CMU021_VOLT,KONAEV_HVBAT_CMU022_VOLT,KONAEV_HVBAT_CMU023_VOLT,KONAEV_HVBAT_CMU024_VOLT,KONAEV_HVBAT_CMU025_VOLT,KONAEV_HVBAT_CMU026_VOLT,KONAEV_HVBAT_CMU027_VOLT,KONAEV_HVBAT_CMU028_VOLT,KONAEV_HVBAT_CMU029_VOLT,KONAEV_HVBAT_CMU030_VOLT,KONAEV_HVBAT_CMU031_VOLT,KONAEV_HVBAT_CMU032_VOLT
  - 7E4.7EC.220103|fc=1:KONAEV_HVBAT_CMU033_VOLT,KONAEV_HVBAT_CMU034_VOLT,KONAEV_HVBAT_CMU035_VOLT,KONAEV_HVBAT_CMU036_VOLT,KONAEV_HVBAT_CMU037_VOLT,KONAEV_HVBAT_CMU038_VOLT,KONAEV_HVBAT_CMU039_VOLT,KONAEV_HVBAT_CMU040_VOLT,KONAEV_HVBAT_CMU041_VOLT,KONAEV_HVBAT_CMU042_VOLT,KONAEV_HVBAT_CMU043_VOLT,KONAEV_HVBAT_CMU044_VOLT,KONAEV_HVBAT_CMU045_VOLT,KONAEV_HVBAT_CMU046_VOLT,KONAEV_HVBAT_CMU047_VOLT,KONAEV_HVBAT_CMU048_VOLT,KONAEV_HVBAT_CMU049_VOLT,KONAEV_HVBAT_CMU050_VOLT,KONAEV_HVBAT_CMU051_VOLT,KONAEV_HVBAT_CMU052_VOLT,KONAEV_HVBAT_CMU053_VOLT,KONAEV_HVBAT_CMU054_VOLT,KONAEV_HVBAT_CMU055_VOLT,KONAEV_HVBAT_CMU056_VOLT,KONAEV_HVBAT_CMU057_VOLT,KONAEV_HVBAT_CMU058_VOLT,KONAEV_HVBAT_CMU059_VOLT,KONAEV_HVBAT_CMU060_VOLT,KONAEV_HVBAT_CMU061_VOLT,KONAEV_HVBAT_CMU062_VOLT,KONAEV_HVBAT_CMU063_VOLT,KONAEV_HVBAT_CMU064_VOLT
  - 7E4.7EC.220104|fc=1:KONAEV_HVBAT_CMU065_VOLT,KONAEV_HVBAT_CMU066_VOLT,KONAEV_HVBAT_CMU067_VOLT,KONAEV_HVBAT_CMU068_VOLT,KONAEV_HVBAT_CMU069_VOLT,KONAEV_HVBAT_CMU070_VOLT,KONAEV_HVBAT_CMU071_VOLT,KONAEV_HVBAT_CMU072_VOLT,KONAEV_HVBAT_CMU073_VOLT,KONAEV_HVBAT_CMU074_VOLT,KONAEV_HVBAT_CMU075_VOLT,KONAEV_HVBAT_CMU076_VOLT,KONAEV_HVBAT_CMU077_VOLT,KONAEV_HVBAT_CMU078_VOLT,KONAEV_HVBAT_CMU079_VOLT,KONAEV_HVBAT_CMU080_VOLT,KONAEV_HVBAT_CMU081_VOLT,KONAEV_HVBAT_CMU082_VOLT,KONAEV_HVBAT_CMU083_VOLT,KONAEV_HVBAT_CMU084_VOLT,KONAEV_HVBAT_CMU085_VOLT,KONAEV_HVBAT_CMU086_VOLT,KONAEV_HVBAT_CMU087_VOLT,KONAEV_HVBAT_CMU088_VOLT,KONAEV_HVBAT_CMU089_VOLT,KONAEV_HVBAT_CMU090_VOLT,KONAEV_HVBAT_CMU091_VOLT,KONAEV_HVBAT_CMU092_VOLT,KONAEV_HVBAT_CMU093_VOLT,KONAEV_HVBAT_CMU094_VOLT,KONAEV_HVBAT_CMU095_VOLT,KONAEV_HVBAT_CMU096_VOLT
  - 7E4.7EC.220105|fc=1:KONAEV_AIRBAG_DUTY,KONAEV_HVBAT_CMU097_VOLT,KONAEV_HVBAT_CMU098_VOLT,KONAEV_HVBAT_C_DEV_V,KONAEV_HVBAT_DET_MAX_ID,KONAEV_HVBAT_DET_MIN,KONAEV_HVBAT_DET_MIN_ID,KONAEV_HVBAT_HTR1_T,KONAEV_HVBAT_HTR2_T,KONAEV_HVBAT_SOC_DISP,KONAEV_HVBAT_SOH

```

## tests/test_cases/2020/commands/7A0.7A8.22C00B|fc=1.yaml

```yaml
command_id: 7A0.7A8.22C00B|fc=1
test_cases:
- expected_values:
    KONAEV_TP_FL: 37.4
    KONAEV_TP_FR: 37.8
    KONAEV_TP_RL: 36.4
    KONAEV_TP_RR: 36.8
    KONAEV_TT_FL: 16
    KONAEV_TT_FR: 15
    KONAEV_TT_RL: 12
    KONAEV_TT_RR: 12
  response: |-
    7A8101762C00BFFFF00
    7A82100BB420100BD41
    7A8220100B63E0100B8
    7A8233E0100AAAAAAAA

```

## tests/test_cases/2020/commands/7C6.7CE.22B002|fc=1.yaml

```yaml
command_id: 7C6.7CE.22B002|fc=1
test_cases:
- expected_values:
    KONAEV_ODO_KM: 0
    KONAEV_ODO_MI: 42910
  response: |-
    7CE100F62B002D00000
    7CE2100009B00000000
    7CE22A79EAAAAAAAAAA
- expected_values:
    KONAEV_ODO_KM: 0
    KONAEV_ODO_MI: 42910
  response: |-
    7CE100F62B002D00000
    7CE2100009C00000000
    7CE22A79EAAAAAAAAAA
- expected_values:
    KONAEV_ODO_KM: 0
    KONAEV_ODO_MI: 42910
  response: |-
    7CE100F62B002D00000
    7CE2100009D00000000
    7CE22A79EAAAAAAAAAA
- expected_values:
    KONAEV_ODO_KM: 0
    KONAEV_ODO_MI: 42910
  response: |-
    7CE100F62B002D00000
    7CE2100009F00000000
    7CE22A79EAAAAAAAAAA

```

## tests/test_cases/2020/commands/7E2.7EA.2101.yaml

```yaml
command_id: 7E2.7EA.2101
test_cases:
- expected_values:
    KONAEV_VSS: 0
  response: |-
    7EA10186101FFF80000
    7EA2109218001060403
    7EA2200000000286800
    7EA2300202005000000
- expected_values:
    KONAEV_VSS: 0
  response: |-
    7EA10186101FFF80000
    7EA21092180F705FF02
    7EA2200000000B56500
    7EA2300202005000000
- expected_values:
    KONAEV_VSS: 0
  response: |-
    7EA10186101FFF80000
    7EA21092180F705FF02
    7EA2200000000C86500
    7EA2300202005000000
- expected_values:
    KONAEV_VSS: 0
  response: |-
    7EA10186101FFF80000
    7EA21092180FA050203
    7EA22000000006F6500
    7EA2300202005000000
- expected_values:
    KONAEV_VSS: 0
  response: |-
    7EA10186101FFF80000
    7EA21092180FA050203
    7EA2200000000A56600
    7EA2300202005000000

```

## tests/test_cases/2020/commands/7E4.7EC.220101|fc=1.yaml

```yaml
command_id: 7E4.7EC.220101|fc=1
test_cases:
- expected_values:
    KONAEV_BMS_MAIN_RELAY: 0
    KONAEV_C_V_MAX: 3.6
    KONAEV_C_V_MAX_ID: 2
    KONAEV_C_V_MIN: 3.6
    KONAEV_C_V_MIN_ID: 1
    KONAEV_DRIVE_TIME: 8061459
    KONAEV_HVBAT_CHARGING: 1
    KONAEV_HVBAT_CHG_P_AVAIL: 0
    KONAEV_HVBAT_CHRG_TOT_C: 5614.8
    KONAEV_HVBAT_CHRG_TOT_E: 2108.1
    KONAEV_HVBAT_CURR: 0
    KONAEV_HVBAT_DCHG_P_AVAIL: 0
    KONAEV_HVBAT_DSCH_TOT_C: 5557.3
    KONAEV_HVBAT_DSCH_TOT_E: 2031.1
    KONAEV_HVBAT_FAN: 0
    KONAEV_HVBAT_FAN_STATUS: 0
    KONAEV_HVBAT_INLET_T: 7
    KONAEV_HVBAT_MOD1_T: 8
    KONAEV_HVBAT_MOD2_T: 8
    KONAEV_HVBAT_MOD3_T: 9
    KONAEV_HVBAT_MOD4_T: 8
    KONAEV_HVBAT_PLUG_NORM: 0
    KONAEV_HVBAT_PLUG_RAPD: 0
    KONAEV_HVBAT_SOC: 36.5
    KONAEV_HVBAT_T_MAX: 9
    KONAEV_HVBAT_T_MIN: 9
    KONAEV_HVBAT_VDC: 354.5
    KONAEV_IGN: 0
    KONAEV_INVRT_CV: 0
    KONAEV_ISO_R: 3000
    KONAEV_RPM: 0
    KONAEV_RPM_2: 0
    KONAEV_VPWR: 12.6
  response: |-
    7EC103E620101FFF7E7
    7EC21FF490000000080
    7EC2200000DD9090908
    7EC23080908000007B4
    7EC2402B40100007E00
    7EC2500DB540000D915
    7EC260000525900004F
    7EC2757007B02130000
    7EC2800000000000BB8
- expected_values:
    KONAEV_BMS_MAIN_RELAY: 0
    KONAEV_C_V_MAX: 3.6
    KONAEV_C_V_MAX_ID: 2
    KONAEV_C_V_MIN: 3.6
    KONAEV_C_V_MIN_ID: 1
    KONAEV_DRIVE_TIME: 8059427
    KONAEV_HVBAT_CHARGING: 1
    KONAEV_HVBAT_CHG_P_AVAIL: 0
    KONAEV_HVBAT_CHRG_TOT_C: 5614.8
    KONAEV_HVBAT_CHRG_TOT_E: 2108.1
    KONAEV_HVBAT_CURR: 0
    KONAEV_HVBAT_DCHG_P_AVAIL: 0
    KONAEV_HVBAT_DSCH_TOT_C: 5557.3
    KONAEV_HVBAT_DSCH_TOT_E: 2031.1
    KONAEV_HVBAT_FAN: 0
    KONAEV_HVBAT_FAN_STATUS: 0
    KONAEV_HVBAT_INLET_T: 7
    KONAEV_HVBAT_MOD1_T: 8
    KONAEV_HVBAT_MOD2_T: 8
    KONAEV_HVBAT_MOD3_T: 9
    KONAEV_HVBAT_MOD4_T: 8
    KONAEV_HVBAT_PLUG_NORM: 0
    KONAEV_HVBAT_PLUG_RAPD: 0
    KONAEV_HVBAT_SOC: 36.5
    KONAEV_HVBAT_T_MAX: 9
    KONAEV_HVBAT_T_MIN: 9
    KONAEV_HVBAT_VDC: 354.5
    KONAEV_IGN: 0
    KONAEV_INVRT_CV: 0
    KONAEV_ISO_R: 3000
    KONAEV_RPM: 0
    KONAEV_RPM_2: 0
    KONAEV_VPWR: 12.7
  response: |-
    7EC103E620101FFF7E7
    7EC21FF490000000080
    7EC2200000DD9090908
    7EC23080908000007B4
    7EC2402B40100007F00
    7EC2500DB540000D915
    7EC260000525900004F
    7EC2757007AFA230000
    7EC2800000000000BB8
- expected_values:
    KONAEV_BMS_MAIN_RELAY: 0
    KONAEV_C_V_MAX: 3.6
    KONAEV_C_V_MAX_ID: 3
    KONAEV_C_V_MIN: 3.6
    KONAEV_C_V_MIN_ID: 1
    KONAEV_DRIVE_TIME: 8061407
    KONAEV_HVBAT_CHARGING: 1
    KONAEV_HVBAT_CHG_P_AVAIL: 0
    KONAEV_HVBAT_CHRG_TOT_C: 5614.8
    KONAEV_HVBAT_CHRG_TOT_E: 2108.1
    KONAEV_HVBAT_CURR: 0
    KONAEV_HVBAT_DCHG_P_AVAIL: 0
    KONAEV_HVBAT_DSCH_TOT_C: 5557.3
    KONAEV_HVBAT_DSCH_TOT_E: 2031.1
    KONAEV_HVBAT_FAN: 0
    KONAEV_HVBAT_FAN_STATUS: 0
    KONAEV_HVBAT_INLET_T: 7
    KONAEV_HVBAT_MOD1_T: 8
    KONAEV_HVBAT_MOD2_T: 8
    KONAEV_HVBAT_MOD3_T: 9
    KONAEV_HVBAT_MOD4_T: 8
    KONAEV_HVBAT_PLUG_NORM: 0
    KONAEV_HVBAT_PLUG_RAPD: 0
    KONAEV_HVBAT_SOC: 36.5
    KONAEV_HVBAT_T_MAX: 9
    KONAEV_HVBAT_T_MIN: 9
    KONAEV_HVBAT_VDC: 354.5
    KONAEV_IGN: 0
    KONAEV_INVRT_CV: 0
    KONAEV_ISO_R: 3000
    KONAEV_RPM: 0
    KONAEV_RPM_2: 0
    KONAEV_VPWR: 12.6
  response: |-
    7EC103E620101FFF7E7
    7EC21FF490000000080
    7EC2200000DD9090908
    7EC23080908000007B4
    7EC2403B40100007E00
    7EC2500DB540000D915
    7EC260000525900004F
    7EC2757007B01DF0000
    7EC2800000000000BB8
- expected_values:
    KONAEV_BMS_MAIN_RELAY: 0
    KONAEV_C_V_MAX: 3.6
    KONAEV_C_V_MAX_ID: 6
    KONAEV_C_V_MIN: 3.6
    KONAEV_C_V_MIN_ID: 1
    KONAEV_DRIVE_TIME: 8060877
    KONAEV_HVBAT_CHARGING: 1
    KONAEV_HVBAT_CHG_P_AVAIL: 0
    KONAEV_HVBAT_CHRG_TOT_C: 5614.8
    KONAEV_HVBAT_CHRG_TOT_E: 2108.1
    KONAEV_HVBAT_CURR: 0
    KONAEV_HVBAT_DCHG_P_AVAIL: 0
    KONAEV_HVBAT_DSCH_TOT_C: 5557.3
    KONAEV_HVBAT_DSCH_TOT_E: 2031.1
    KONAEV_HVBAT_FAN: 0
    KONAEV_HVBAT_FAN_STATUS: 0
    KONAEV_HVBAT_INLET_T: 7
    KONAEV_HVBAT_MOD1_T: 8
    KONAEV_HVBAT_MOD2_T: 8
    KONAEV_HVBAT_MOD3_T: 9
    KONAEV_HVBAT_MOD4_T: 8
    KONAEV_HVBAT_PLUG_NORM: 0
    KONAEV_HVBAT_PLUG_RAPD: 0
    KONAEV_HVBAT_SOC: 36.5
    KONAEV_HVBAT_T_MAX: 9
    KONAEV_HVBAT_T_MIN: 9
    KONAEV_HVBAT_VDC: 354.5
    KONAEV_IGN: 0
    KONAEV_INVRT_CV: 0
    KONAEV_ISO_R: 3000
    KONAEV_RPM: 0
    KONAEV_RPM_2: 0
    KONAEV_VPWR: 12.7
  response: |-
    7EC103E620101FFF7E7
    7EC21FF490000000080
    7EC2200000DD9090908
    7EC23080908000007B4
    7EC2406B40100007F00
    7EC2500DB540000D915
    7EC260000525900004F
    7EC2757007AFFCD0000
    7EC2800000000000BB8
- expected_values:
    KONAEV_BMS_MAIN_RELAY: 0
    KONAEV_C_V_MAX: 3.6
    KONAEV_C_V_MAX_ID: 7
    KONAEV_C_V_MIN: 3.6
    KONAEV_C_V_MIN_ID: 11
    KONAEV_DRIVE_TIME: 8060017
    KONAEV_HVBAT_CHARGING: 1
    KONAEV_HVBAT_CHG_P_AVAIL: 0
    KONAEV_HVBAT_CHRG_TOT_C: 5614.8
    KONAEV_HVBAT_CHRG_TOT_E: 2108.1
    KONAEV_HVBAT_CURR: 0
    KONAEV_HVBAT_DCHG_P_AVAIL: 0
    KONAEV_HVBAT_DSCH_TOT_C: 5557.3
    KONAEV_HVBAT_DSCH_TOT_E: 2031.1
    KONAEV_HVBAT_FAN: 0
    KONAEV_HVBAT_FAN_STATUS: 0
    KONAEV_HVBAT_INLET_T: 7
    KONAEV_HVBAT_MOD1_T: 8
    KONAEV_HVBAT_MOD2_T: 8
    KONAEV_HVBAT_MOD3_T: 9
    KONAEV_HVBAT_MOD4_T: 8
    KONAEV_HVBAT_PLUG_NORM: 0
    KONAEV_HVBAT_PLUG_RAPD: 0
    KONAEV_HVBAT_SOC: 36.5
    KONAEV_HVBAT_T_MAX: 9
    KONAEV_HVBAT_T_MIN: 9
    KONAEV_HVBAT_VDC: 354.5
    KONAEV_IGN: 0
    KONAEV_INVRT_CV: 0
    KONAEV_ISO_R: 3000
    KONAEV_RPM: 0
    KONAEV_RPM_2: 0
    KONAEV_VPWR: 12.7
  response: |-
    7EC103E620101FFF7E7
    7EC21FF490000000080
    7EC2200000DD9090908
    7EC23080908000007B4
    7EC2407B40B00007F00
    7EC2500DB540000D915
    7EC260000525900004F
    7EC2757007AFC710000
    7EC2800000000000BB8
- expected_values:
    KONAEV_BMS_MAIN_RELAY: 0
    KONAEV_C_V_MAX: 3.6
    KONAEV_C_V_MAX_ID: 12
    KONAEV_C_V_MIN: 3.6
    KONAEV_C_V_MIN_ID: 1
    KONAEV_DRIVE_TIME: 8059232
    KONAEV_HVBAT_CHARGING: 1
    KONAEV_HVBAT_CHG_P_AVAIL: 0
    KONAEV_HVBAT_CHRG_TOT_C: 5614.8
    KONAEV_HVBAT_CHRG_TOT_E: 2108.1
    KONAEV_HVBAT_CURR: 0
    KONAEV_HVBAT_DCHG_P_AVAIL: 0
    KONAEV_HVBAT_DSCH_TOT_C: 5557.3
    KONAEV_HVBAT_DSCH_TOT_E: 2031.1
    KONAEV_HVBAT_FAN: 0
    KONAEV_HVBAT_FAN_STATUS: 0
    KONAEV_HVBAT_INLET_T: 7
    KONAEV_HVBAT_MOD1_T: 8
    KONAEV_HVBAT_MOD2_T: 8
    KONAEV_HVBAT_MOD3_T: 9
    KONAEV_HVBAT_MOD4_T: 8
    KONAEV_HVBAT_PLUG_NORM: 0
    KONAEV_HVBAT_PLUG_RAPD: 0
    KONAEV_HVBAT_SOC: 36.5
    KONAEV_HVBAT_T_MAX: 9
    KONAEV_HVBAT_T_MIN: 9
    KONAEV_HVBAT_VDC: 354.5
    KONAEV_IGN: 0
    KONAEV_INVRT_CV: 1
    KONAEV_ISO_R: 3000
    KONAEV_RPM: 0
    KONAEV_RPM_2: 0
    KONAEV_VPWR: 12.6
  response: |-
    7EC103E620101FFF7E7
    7EC21FF490000000080
    7EC2200000DD9090908
    7EC23080908000007B4
    7EC240CB40100007E00
    7EC2500DB540000D915
    7EC260000525900004F
    7EC2757007AF9600100
    7EC2801000000000BB8
- expected_values:
    KONAEV_BMS_MAIN_RELAY: 0
    KONAEV_C_V_MAX: 3.6
    KONAEV_C_V_MAX_ID: 14
    KONAEV_C_V_MIN: 3.6
    KONAEV_C_V_MIN_ID: 1
    KONAEV_DRIVE_TIME: 8059134
    KONAEV_HVBAT_CHARGING: 1
    KONAEV_HVBAT_CHG_P_AVAIL: 0
    KONAEV_HVBAT_CHRG_TOT_C: 5614.8
    KONAEV_HVBAT_CHRG_TOT_E: 2108.1
    KONAEV_HVBAT_CURR: 0
    KONAEV_HVBAT_DCHG_P_AVAIL: 0
    KONAEV_HVBAT_DSCH_TOT_C: 5557.3
    KONAEV_HVBAT_DSCH_TOT_E: 2031.1
    KONAEV_HVBAT_FAN: 0
    KONAEV_HVBAT_FAN_STATUS: 0
    KONAEV_HVBAT_INLET_T: 7
    KONAEV_HVBAT_MOD1_T: 8
    KONAEV_HVBAT_MOD2_T: 8
    KONAEV_HVBAT_MOD3_T: 9
    KONAEV_HVBAT_MOD4_T: 8
    KONAEV_HVBAT_PLUG_NORM: 0
    KONAEV_HVBAT_PLUG_RAPD: 0
    KONAEV_HVBAT_SOC: 36.5
    KONAEV_HVBAT_T_MAX: 9
    KONAEV_HVBAT_T_MIN: 9
    KONAEV_HVBAT_VDC: 354.5
    KONAEV_IGN: 0
    KONAEV_INVRT_CV: 200
    KONAEV_ISO_R: 3000
    KONAEV_RPM: 0
    KONAEV_RPM_2: 0
    KONAEV_VPWR: 12.9
  response: |-
    7EC103E620101FFF7E7
    7EC21FF490000000080
    7EC2200000DD9090908
    7EC23080908000007B4
    7EC240EB40100008100
    7EC2500DB540000D915
    7EC260000525900004F
    7EC2757007AF8FE0100
    7EC28C8000000000BB8
- expected_values:
    KONAEV_BMS_MAIN_RELAY: 0
    KONAEV_C_V_MAX: 3.6
    KONAEV_C_V_MAX_ID: 23
    KONAEV_C_V_MIN: 3.6
    KONAEV_C_V_MIN_ID: 1
    KONAEV_DRIVE_TIME: 8059204
    KONAEV_HVBAT_CHARGING: 1
    KONAEV_HVBAT_CHG_P_AVAIL: 0
    KONAEV_HVBAT_CHRG_TOT_C: 5614.8
    KONAEV_HVBAT_CHRG_TOT_E: 2108.1
    KONAEV_HVBAT_CURR: 0
    KONAEV_HVBAT_DCHG_P_AVAIL: 0
    KONAEV_HVBAT_DSCH_TOT_C: 5557.3
    KONAEV_HVBAT_DSCH_TOT_E: 2031.1
    KONAEV_HVBAT_FAN: 0
    KONAEV_HVBAT_FAN_STATUS: 0
    KONAEV_HVBAT_INLET_T: 7
    KONAEV_HVBAT_MOD1_T: 8
    KONAEV_HVBAT_MOD2_T: 8
    KONAEV_HVBAT_MOD3_T: 9
    KONAEV_HVBAT_MOD4_T: 8
    KONAEV_HVBAT_PLUG_NORM: 0
    KONAEV_HVBAT_PLUG_RAPD: 0
    KONAEV_HVBAT_SOC: 36.5
    KONAEV_HVBAT_T_MAX: 9
    KONAEV_HVBAT_T_MIN: 9
    KONAEV_HVBAT_VDC: 354.5
    KONAEV_IGN: 0
    KONAEV_INVRT_CV: 3
    KONAEV_ISO_R: 3000
    KONAEV_RPM: 0
    KONAEV_RPM_2: 0
    KONAEV_VPWR: 12.6
  response: |-
    7EC103E620101FFF7E7
    7EC21FF490000000080
    7EC2200000DD9090908
    7EC23080908000007B4
    7EC2417B40100007E00
    7EC2500DB540000D915
    7EC260000525900004F
    7EC2757007AF9440100
    7EC2803000000000BB8

```

## tests/test_cases/2020/commands/7E4.7EC.220102|fc=1.yaml

```yaml
command_id: 7E4.7EC.220102|fc=1
test_cases:
- expected_values:
    KONAEV_HVBAT_CMU001_VOLT: 3.6
    KONAEV_HVBAT_CMU002_VOLT: 3.6
    KONAEV_HVBAT_CMU003_VOLT: 3.6
    KONAEV_HVBAT_CMU004_VOLT: 3.6
    KONAEV_HVBAT_CMU005_VOLT: 3.6
    KONAEV_HVBAT_CMU006_VOLT: 3.6
    KONAEV_HVBAT_CMU007_VOLT: 3.6
    KONAEV_HVBAT_CMU008_VOLT: 3.6
    KONAEV_HVBAT_CMU009_VOLT: 3.6
    KONAEV_HVBAT_CMU010_VOLT: 3.6
    KONAEV_HVBAT_CMU011_VOLT: 3.6
    KONAEV_HVBAT_CMU012_VOLT: 3.6
    KONAEV_HVBAT_CMU013_VOLT: 3.6
    KONAEV_HVBAT_CMU014_VOLT: 3.6
    KONAEV_HVBAT_CMU015_VOLT: 3.6
    KONAEV_HVBAT_CMU016_VOLT: 3.6
    KONAEV_HVBAT_CMU017_VOLT: 3.6
    KONAEV_HVBAT_CMU018_VOLT: 3.6
    KONAEV_HVBAT_CMU019_VOLT: 3.6
    KONAEV_HVBAT_CMU020_VOLT: 3.6
    KONAEV_HVBAT_CMU021_VOLT: 3.6
    KONAEV_HVBAT_CMU022_VOLT: 3.6
    KONAEV_HVBAT_CMU023_VOLT: 3.6
    KONAEV_HVBAT_CMU024_VOLT: 3.6
    KONAEV_HVBAT_CMU025_VOLT: 3.6
    KONAEV_HVBAT_CMU026_VOLT: 3.6
    KONAEV_HVBAT_CMU027_VOLT: 3.6
    KONAEV_HVBAT_CMU028_VOLT: 3.6
    KONAEV_HVBAT_CMU029_VOLT: 3.6
    KONAEV_HVBAT_CMU030_VOLT: 3.6
    KONAEV_HVBAT_CMU031_VOLT: 3.6
    KONAEV_HVBAT_CMU032_VOLT: 3.6
  response: |-
    7EC1027620102FFFFFF
    7EC21FFB4B4B4B4B4B4
    7EC22B4B4B4B4B4B4B4
    7EC23B4B4B4B4B4B4B4
    7EC24B4B4B4B4B4B4B4
    7EC25B4B4B4B4B4AAAA

```

## tests/test_cases/2020/commands/7E4.7EC.220105|fc=1.yaml

```yaml
command_id: 7E4.7EC.220105|fc=1
test_cases:
- expected_values:
    KONAEV_AIRBAG_DUTY: 100
    KONAEV_HVBAT_CMU097_VOLT: 3.6
    KONAEV_HVBAT_CMU098_VOLT: 3.6
    KONAEV_HVBAT_C_DEV_V: 0
    KONAEV_HVBAT_DET_MAX_ID: 50
    KONAEV_HVBAT_DET_MIN: 4.9
    KONAEV_HVBAT_DET_MIN_ID: 55
    KONAEV_HVBAT_HTR1_T: 7
    KONAEV_HVBAT_HTR2_T: 75
    KONAEV_HVBAT_SOC_DISP: 37.5
    KONAEV_HVBAT_SOH: 99.2
  response: |-
    7EC102E620105003FFF
    7EC2190000000000000
    7EC220000077E000041
    7EC238C3E7800006407
    7EC244B03E032003137
    7EC254B0000B4B40000
    7EC260900000000AAAA
- expected_values:
    KONAEV_AIRBAG_DUTY: 100
    KONAEV_HVBAT_CMU097_VOLT: 3.6
    KONAEV_HVBAT_CMU098_VOLT: 3.6
    KONAEV_HVBAT_C_DEV_V: 0
    KONAEV_HVBAT_DET_MAX_ID: 50
    KONAEV_HVBAT_DET_MIN: 4.9
    KONAEV_HVBAT_DET_MIN_ID: 55
    KONAEV_HVBAT_HTR1_T: 7
    KONAEV_HVBAT_HTR2_T: 75
    KONAEV_HVBAT_SOC_DISP: 37.5
    KONAEV_HVBAT_SOH: 99.2
  response: |-
    7EC102E620105003FFF
    7EC2190000000000000
    7EC220000077E010041
    7EC238C3E7800006407
    7EC244B03E032003137
    7EC254B0000B4B40000
    7EC260900000000AAAA
- expected_values:
    KONAEV_AIRBAG_DUTY: 100
    KONAEV_HVBAT_CMU097_VOLT: 3.6
    KONAEV_HVBAT_CMU098_VOLT: 3.6
    KONAEV_HVBAT_C_DEV_V: 0
    KONAEV_HVBAT_DET_MAX_ID: 50
    KONAEV_HVBAT_DET_MIN: 4.9
    KONAEV_HVBAT_DET_MIN_ID: 55
    KONAEV_HVBAT_HTR1_T: 7
    KONAEV_HVBAT_HTR2_T: 75
    KONAEV_HVBAT_SOC_DISP: 37.5
    KONAEV_HVBAT_SOH: 99.2
  response: |-
    7EC102E620105003FFF
    7EC2190000000000000
    7EC220000077E030041
    7EC238C3E7800006407
    7EC244B03E032003137
    7EC254B0000B4B40000
    7EC260900000000AAAA
- expected_values:
    KONAEV_AIRBAG_DUTY: 100
    KONAEV_HVBAT_CMU097_VOLT: 3.6
    KONAEV_HVBAT_CMU098_VOLT: 3.6
    KONAEV_HVBAT_C_DEV_V: 0
    KONAEV_HVBAT_DET_MAX_ID: 50
    KONAEV_HVBAT_DET_MIN: 4.9
    KONAEV_HVBAT_DET_MIN_ID: 55
    KONAEV_HVBAT_HTR1_T: 7
    KONAEV_HVBAT_HTR2_T: 75
    KONAEV_HVBAT_SOC_DISP: 37.5
    KONAEV_HVBAT_SOH: 99.2
  response: |-
    7EC102E620105003FFF
    7EC2190000000000000
    7EC220000077F000041
    7EC238C3E7800006407
    7EC244B03E032003137
    7EC254B0000B4B40000
    7EC260900000000AAAA
- expected_values:
    KONAEV_AIRBAG_DUTY: 100
    KONAEV_HVBAT_CMU097_VOLT: 3.6
    KONAEV_HVBAT_CMU098_VOLT: 3.6
    KONAEV_HVBAT_C_DEV_V: 0
    KONAEV_HVBAT_DET_MAX_ID: 50
    KONAEV_HVBAT_DET_MIN: 4.9
    KONAEV_HVBAT_DET_MIN_ID: 55
    KONAEV_HVBAT_HTR1_T: 7
    KONAEV_HVBAT_HTR2_T: 75
    KONAEV_HVBAT_SOC_DISP: 37.5
    KONAEV_HVBAT_SOH: 99.2
  response: |-
    7EC102E620105003FFF
    7EC2190000000000000
    7EC220000077F0F0041
    7EC238C3E7800006407
    7EC244B03E032003137
    7EC254B0000B4B40000
    7EC260900000000AAAA
- expected_values:
    KONAEV_AIRBAG_DUTY: 100
    KONAEV_HVBAT_CMU097_VOLT: 3.6
    KONAEV_HVBAT_CMU098_VOLT: 3.6
    KONAEV_HVBAT_C_DEV_V: 0
    KONAEV_HVBAT_DET_MAX_ID: 50
    KONAEV_HVBAT_DET_MIN: 4.9
    KONAEV_HVBAT_DET_MIN_ID: 55
    KONAEV_HVBAT_HTR1_T: 7
    KONAEV_HVBAT_HTR2_T: 75
    KONAEV_HVBAT_SOC_DISP: 37.5
    KONAEV_HVBAT_SOH: 99.2
  response: |-
    7EC102E620105003FFF
    7EC2190000000000000
    7EC2200000781C50041
    7EC238C3E7800006407
    7EC244B03E032003137
    7EC254B0000B4B40000
    7EC260900000000AAAA

```

## signalsets/v3/default.json — commands relevant to carlink (extracted with python json, verbatim objects)

```json
[
 {
  "hdr": "7A0",
  "rax": "7A8",
  "fcm1": true,
  "cmd": {
   "22": "B00C"
  },
  "freq": 5,
  "dbgfilter": {
   "to": 2020,
   "from": 2022
  },
  "signals": [
   {
    "id": "KONAEV_HEATED_WHEEL",
    "path": "Climate",
    "fmt": {
     "bix": 34,
     "len": 1,
     "max": 1,
     "unit": "offon"
    },
    "name": "Heated steering wheel"
   }
  ]
 },
 {
  "hdr": "7A0",
  "rax": "7A8",
  "fcm1": true,
  "cmd": {
   "22": "C00B"
  },
  "freq": 15,
  "dbgfilter": {
   "years": [
    2019
   ],
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_TP_FL",
    "path": "Tires",
    "fmt": {
     "bix": 32,
     "len": 8,
     "max": 51,
     "div": 5,
     "unit": "psi"
    },
    "name": "Front left tire pressure",
    "suggestedMetric": "frontLeftTirePressure"
   },
   {
    "id": "KONAEV_TT_FL",
    "path": "Tires",
    "fmt": {
     "bix": 40,
     "len": 8,
     "max": 205,
     "min": -50,
     "add": -50,
     "unit": "celsius"
    },
    "name": "Front left tire temperature"
   },
   {
    "id": "KONAEV_TP_FR",
    "path": "Tires",
    "fmt": {
     "bix": 64,
     "len": 8,
     "max": 51,
     "div": 5,
     "unit": "psi"
    },
    "name": "Front right tire pressure",
    "suggestedMetric": "frontRightTirePressure"
   },
   {
    "id": "KONAEV_TT_FR",
    "path": "Tires",
    "fmt": {
     "bix": 72,
     "len": 8,
     "max": 205,
     "min": -50,
     "add": -50,
     "unit": "celsius"
    },
    "name": "Front right tire temperature"
   },
   {
    "id": "KONAEV_TP_RL",
    "path": "Tires",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 51,
     "div": 5,
     "unit": "psi"
    },
    "name": "Rear left tire pressure",
    "suggestedMetric": "rearLeftTirePressure"
   },
   {
    "id": "KONAEV_TT_RL",
    "path": "Tires",
    "fmt": {
     "bix": 104,
     "len": 8,
     "max": 205,
     "min": -50,
     "add": -50,
     "unit": "celsius"
    },
    "name": "Rear left tire temperature"
   },
   {
    "id": "KONAEV_TP_RR",
    "path": "Tires",
    "fmt": {
     "bix": 128,
     "len": 8,
     "max": 51,
     "div": 5,
     "unit": "psi"
    },
    "name": "Rear right tire pressure",
    "suggestedMetric": "rearRightTirePressure"
   },
   {
    "id": "KONAEV_TT_RR",
    "path": "Tires",
    "fmt": {
     "bix": 136,
     "len": 8,
     "max": 205,
     "min": -50,
     "add": -50,
     "unit": "celsius"
    },
    "name": "Rear right tire temperature"
   }
  ]
 },
 {
  "hdr": "7A0",
  "rax": "7A8",
  "fcm1": true,
  "cmd": {
   "22": "C00C"
  },
  "freq": 30,
  "dbgfilter": {
   "to": 2020,
   "years": [
    2022,
    2023
   ],
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_SENS_1_ODO_EVT_1",
    "path": "Trips",
    "fmt": {
     "bix": 272,
     "len": 24,
     "max": 1677721.5,
     "div": 10,
     "unit": "kilometers"
    },
    "name": "Sensor 1 odometer event 1"
   },
   {
    "id": "KONAEV_SENS_1_ODO_EVT_2",
    "path": "Trips",
    "fmt": {
     "bix": 296,
     "len": 24,
     "max": 1677721.5,
     "div": 10,
     "unit": "kilometers"
    },
    "name": "Sensor 1 odometer event 2"
   },
   {
    "id": "KONAEV_SENS_1_ODO_EVT_3",
    "path": "Trips",
    "fmt": {
     "bix": 320,
     "len": 24,
     "max": 1677721.5,
     "div": 10,
     "unit": "kilometers"
    },
    "name": "Sensor 1 odometer event 3"
   }
  ]
 },
 {
  "hdr": "7A0",
  "rax": "7A8",
  "fcm1": true,
  "cmd": {
   "22": "C00D"
  },
  "freq": 30,
  "dbgfilter": {
   "to": 2020,
   "years": [
    2022,
    2023
   ],
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_SENS_2_ODO_EVT_1",
    "path": "Trips",
    "fmt": {
     "bix": 272,
     "len": 24,
     "max": 1677721.5,
     "div": 10,
     "unit": "kilometers"
    },
    "name": "Sensor 2 odometer event 1"
   },
   {
    "id": "KONAEV_SENS_2_ODO_EVT_2",
    "path": "Trips",
    "fmt": {
     "bix": 296,
     "len": 24,
     "max": 1677721.5,
     "div": 10,
     "unit": "kilometers"
    },
    "name": "Sensor 2 odometer event 2"
   },
   {
    "id": "KONAEV_SENS_2_ODO_EVT_3",
    "path": "Trips",
    "fmt": {
     "bix": 320,
     "len": 24,
     "max": 1677721.5,
     "div": 10,
     "unit": "kilometers"
    },
    "name": "Sensor 2 odometer event 3"
   }
  ]
 },
 {
  "hdr": "7A0",
  "rax": "7A8",
  "fcm1": true,
  "cmd": {
   "22": "C00E"
  },
  "freq": 30,
  "dbgfilter": {
   "to": 2020,
   "years": [
    2022,
    2023
   ],
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_SENS_3_ODO_EVT_1",
    "path": "Trips",
    "fmt": {
     "bix": 272,
     "len": 24,
     "max": 1677721.5,
     "div": 10,
     "unit": "kilometers"
    },
    "name": "Sensor 3 odometer event 1"
   },
   {
    "id": "KONAEV_SENS_3_ODO_EVT_2",
    "path": "Trips",
    "fmt": {
     "bix": 296,
     "len": 24,
     "max": 1677721.5,
     "div": 10,
     "unit": "kilometers"
    },
    "name": "Sensor 3 odometer event 2"
   },
   {
    "id": "KONAEV_SENS_3_ODO_EVT_3",
    "path": "Trips",
    "fmt": {
     "bix": 320,
     "len": 24,
     "max": 1677721.5,
     "div": 10,
     "unit": "kilometers"
    },
    "name": "Sensor 3 odometer event 3"
   }
  ]
 },
 {
  "hdr": "7A0",
  "rax": "7A8",
  "fcm1": true,
  "cmd": {
   "22": "C00F"
  },
  "freq": 30,
  "dbgfilter": {
   "to": 2020,
   "years": [
    2022,
    2023
   ],
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_SENS_4_ODO_EVT_1",
    "path": "Trips",
    "fmt": {
     "bix": 272,
     "len": 24,
     "max": 1677721.5,
     "div": 10,
     "unit": "kilometers"
    },
    "name": "Sensor 4 odometer event 1"
   },
   {
    "id": "KONAEV_SENS_4_ODO_EVT_2",
    "path": "Trips",
    "fmt": {
     "bix": 296,
     "len": 24,
     "max": 1677721.5,
     "div": 10,
     "unit": "kilometers"
    },
    "name": "Sensor 4 odometer event 2"
   },
   {
    "id": "KONAEV_SENS_4_ODO_EVT_3",
    "path": "Trips",
    "fmt": {
     "bix": 320,
     "len": 24,
     "max": 1677721.5,
     "div": 10,
     "unit": "kilometers"
    },
    "name": "Sensor 4 odometer event 3"
   }
  ]
 },
 {
  "hdr": "7B3",
  "rax": "7BB",
  "fcm1": true,
  "cmd": {
   "22": "0100"
  },
  "freq": 0.25,
  "dbgfilter": {
   "to": 2020,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_TEMP_INT",
    "path": "Climate",
    "fmt": {
     "bix": 40,
     "len": 8,
     "max": 80,
     "min": -40,
     "div": 2,
     "add": -40,
     "unit": "celsius"
    },
    "name": "Interior temperature"
   },
   {
    "id": "KONAEV_TEMP_EXT",
    "path": "Climate",
    "fmt": {
     "bix": 48,
     "len": 8,
     "max": 80,
     "min": -40,
     "div": 2,
     "add": -40,
     "unit": "celsius"
    },
    "name": "Exterior temperature"
   },
   {
    "id": "KONAEV_VSS_7B3",
    "path": "Movement",
    "fmt": {
     "bix": 232,
     "len": 8,
     "max": 255,
     "unit": "kilometersPerHour"
    },
    "name": "Vehicle speed",
    "suggestedMetric": "speed"
   }
  ]
 },
 {
  "hdr": "7B3",
  "rax": "7BB",
  "cmd": {
   "22": "0101"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_HVAC_IAT_PASSENGER_VENT",
    "path": "Climate",
    "fmt": {
     "bix": 64,
     "len": 8,
     "max": 87.5,
     "min": -40,
     "div": 2,
     "add": -40,
     "unit": "celsius"
    },
    "name": "Interior temperature, passenger vent"
   },
   {
    "id": "KONAEV_HVAC_IAT_PASSENGER_LEGS_VENT",
    "path": "Climate",
    "fmt": {
     "bix": 72,
     "len": 8,
     "max": 87.5,
     "min": -40,
     "div": 2,
     "add": -40,
     "unit": "celsius"
    },
    "name": "Interior temperature, passenger legs vent"
   }
  ]
 },
 {
  "hdr": "7B3",
  "rax": "7BB",
  "cmd": {
   "22": "0102"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2020,
   "years": [
    2022,
    2023
   ],
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_HVAC_COOLANT_T",
    "path": "Engine",
    "fmt": {
     "bix": 32,
     "len": 8,
     "max": 87.5,
     "min": -40,
     "div": 2,
     "add": -40,
     "unit": "celsius"
    },
    "name": "Coolant temperature"
   },
   {
    "id": "KONAEV_HVAC_INVERTER_T",
    "path": "Engine",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 87.5,
     "min": -40,
     "div": 2,
     "add": -40,
     "unit": "celsius"
    },
    "name": "Inverter temperature"
   }
  ]
 },
 {
  "hdr": "7C6",
  "rax": "7CE",
  "fcm1": true,
  "cmd": {
   "22": "B002"
  },
  "freq": 5,
  "dbgfilter": {
   "years": [
    2019
   ],
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_ODO_KM",
    "path": "Trips",
    "fmt": {
     "bix": 48,
     "len": 24,
     "max": 16777215,
     "unit": "kilometers"
    },
    "name": "Odometer, km",
    "suggestedMetric": "odometer"
   },
   {
    "id": "KONAEV_ODO_MI",
    "path": "Trips",
    "fmt": {
     "bix": 72,
     "len": 24,
     "max": 16777215,
     "unit": "miles"
    },
    "name": "Odometer, miles",
    "suggestedMetric": "odometer"
   }
  ]
 },
 {
  "hdr": "7E2",
  "rax": "7EA",
  "cmd": {
   "21": "01"
  },
  "freq": 0.25,
  "dbgfilter": {
   "to": 2019,
   "from": 2024
  },
  "signals": [
   {
    "id": "KONAEV_VSS",
    "path": "Movement",
    "fmt": {
     "bix": 104,
     "len": 16,
     "blsb": true,
     "max": 180,
     "min": -180,
     "div": 100,
     "sign": true,
     "unit": "milesPerHour"
    },
    "name": "Vehicle speed",
    "suggestedMetric": "speed",
    "description": "Current vehicle speed."
   }
  ]
 },
 {
  "hdr": "7E2",
  "rax": "7EA",
  "cmd": {
   "21": "02"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2020,
   "from": 2022
  },
  "signals": [
   {
    "id": "KONAEV_AUX_AMPS_LDC",
    "path": "Engine",
    "fmt": {
     "bix": 128,
     "len": 16,
     "max": 655,
     "div": 100,
     "unit": "amps"
    },
    "name": "AUX amps LDC"
   },
   {
    "id": "KONAEV_AUX_V_7E2",
    "path": "Battery",
    "fmt": {
     "bix": 152,
     "len": 16,
     "blsb": true,
     "max": 32.767,
     "min": -32.768,
     "div": 1000,
     "sign": true,
     "unit": "volts"
    },
    "name": "Auxiliary battery voltage, vehicle control unit"
   },
   {
    "id": "KONAEV_AUXILIARY_BAT",
    "path": "Battery",
    "fmt": {
     "bix": 168,
     "len": 16,
     "blsb": true,
     "max": 32.767,
     "min": -32.768,
     "div": 1000,
     "sign": true,
     "unit": "amps"
    },
    "name": "Auxiliary battery current"
   },
   {
    "id": "KONAEV_AUXILIARY_BAT_ST_CHG",
    "path": "Battery",
    "fmt": {
     "bix": 184,
     "len": 8,
     "max": 255,
     "unit": "percent"
    },
    "name": "Auxiliary battery state of charge"
   }
  ]
 },
 {
  "hdr": "7E2",
  "rax": "7EA",
  "cmd": {
   "22": "E004"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_ACCELERATOR_PEDAL_POSITION",
    "path": "Control",
    "fmt": {
     "bix": 64,
     "len": 16,
     "blsb": true,
     "max": 100,
     "div": 512,
     "unit": "percent"
    },
    "name": "Accelerator pedal position"
   },
   {
    "id": "KONAEV_VSS_HD",
    "path": "Movement",
    "fmt": {
     "bix": 80,
     "len": 16,
     "blsb": true,
     "max": 511,
     "min": -512,
     "mul": 156.25,
     "div": 10000,
     "sign": true,
     "unit": "kilometersPerHour"
    },
    "name": "Vehicle speed, high resolution",
    "suggestedMetric": "speed"
   },
   {
    "id": "KONAEV_12V_V",
    "path": "Battery",
    "fmt": {
     "bix": 96,
     "len": 16,
     "blsb": true,
     "max": 32,
     "mul": 4.883,
     "div": 10000,
     "unit": "volts"
    },
    "name": "Auxiliary battery voltage",
    "suggestedMetric": "starterBatteryVoltage"
   }
  ]
 },
 {
  "hdr": "7E2",
  "rax": "7EA",
  "cmd": {
   "22": "E005"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_DISCONNECTOR_STROKE_POSITION",
    "path": "Engine",
    "fmt": {
     "bix": 32,
     "len": 16,
     "blsb": true,
     "max": 511,
     "div": 128,
     "unit": "millimeters"
    },
    "name": "Disconnector stroke position"
   },
   {
    "id": "KONAEV_DISCONNECTOR_MEET_POINT",
    "path": "Engine",
    "fmt": {
     "bix": 48,
     "len": 16,
     "blsb": true,
     "max": 511,
     "div": 128,
     "unit": "millimeters"
    },
    "name": "Disconnector meet point"
   },
   {
    "id": "KONAEV_DISCONNECTOR_END_POINT",
    "path": "Engine",
    "fmt": {
     "bix": 64,
     "len": 16,
     "blsb": true,
     "max": 511,
     "div": 128,
     "unit": "millimeters"
    },
    "name": "Disconnector end point"
   },
   {
    "id": "KONAEV_DISCONNECTOR_DISENGAGE_LAST_TIME",
    "path": "Engine",
    "fmt": {
     "bix": 80,
     "len": 8,
     "max": 255,
     "unit": "milliseconds"
    },
    "name": "Disconnector disengage last time"
   },
   {
    "id": "KONAEV_DISCONNECTOR_ACTUATOR_MOTOR_CURRENT",
    "path": "Engine",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 254,
     "min": -256,
     "mul": 2,
     "sign": true,
     "unit": "amps"
    },
    "name": "Disconnector actuator motor current"
   },
   {
    "id": "KONAEV_DISCONNECTOR_INVERTER_PWM_DUTY",
    "path": "Engine",
    "fmt": {
     "bix": 104,
     "len": 16,
     "blsb": true,
     "max": 100,
     "mul": 78.125,
     "div": 10000,
     "sign": true,
     "unit": "percent"
    },
    "name": "Disconnector inverter PWM duty"
   },
   {
    "id": "KONAEV_DISCONNECTOR_ZERO_COUNT_OFFSET",
    "path": "Engine",
    "fmt": {
     "bix": 120,
     "len": 16,
     "blsb": true,
     "max": 32767,
     "min": -32768,
     "sign": true,
     "unit": "scalar"
    },
    "name": "Disconnector zero count offset"
   }
  ]
 },
 {
  "hdr": "7E3",
  "rax": "7EB",
  "dbg": true,
  "cmd": {
   "22": "0102"
  },
  "freq": 5,
  "signals": [
   {
    "id": "KONAEV_MOTOR_T_7E3",
    "path": "Engine",
    "fmt": {
     "bix": 88,
     "len": 8,
     "max": 127,
     "min": -128,
     "sign": true,
     "unit": "celsius"
    },
    "name": "Motor temperature"
   },
   {
    "id": "KONAEV_INVERTER_T_7E3",
    "path": "Engine",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 127,
     "min": -128,
     "sign": true,
     "unit": "celsius"
    },
    "name": "Inverter temperature"
   }
  ]
 },
 {
  "hdr": "7E3",
  "rax": "7EB",
  "cmd": {
   "22": "E001"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_IGN_VOLTAGE",
    "path": "Engine",
    "fmt": {
     "bix": 120,
     "len": 16,
     "blsb": true,
     "max": 65.535,
     "mul": 10,
     "div": 10000,
     "unit": "volts"
    },
    "name": "Ignition voltage"
   },
   {
    "id": "KONAEV_MOTOR_RPM",
    "path": "Engine",
    "fmt": {
     "bix": 136,
     "len": 16,
     "blsb": true,
     "max": 32767,
     "min": -32768,
     "sign": true,
     "unit": "rpm"
    },
    "name": "Motor RPM"
   },
   {
    "id": "KONAEV_NOMINAL_TORQUE_VALUE",
    "path": "Engine",
    "fmt": {
     "bix": 152,
     "len": 16,
     "blsb": true,
     "max": 3276.7,
     "min": -3276.8,
     "div": 10,
     "sign": true,
     "unit": "newtonMeters"
    },
    "name": "Nominal torque value"
   },
   {
    "id": "KONAEV_MOTOR_ACTUAL_TORQUE",
    "path": "Engine",
    "fmt": {
     "bix": 168,
     "len": 16,
     "blsb": true,
     "max": 3276.7,
     "min": -3276.8,
     "div": 10,
     "sign": true,
     "unit": "newtonMeters"
    },
    "name": "Motor actual torque"
   },
   {
    "id": "KONAEV_MOTOR_PHASE_CURRENT_RMS",
    "path": "Engine",
    "fmt": {
     "bix": 184,
     "len": 16,
     "blsb": true,
     "max": 6553.5,
     "div": 10,
     "unit": "amps"
    },
    "name": "Motor phase current"
   },
   {
    "id": "KONAEV_MOTOR_T",
    "path": "Engine",
    "fmt": {
     "bix": 200,
     "len": 16,
     "blsb": true,
     "max": 327.67,
     "min": -327.68,
     "div": 100,
     "sign": true,
     "unit": "celsius"
    },
    "name": "Motor temperature"
   },
   {
    "id": "KONAEV_INVERTER_T",
    "path": "Engine",
    "fmt": {
     "bix": 216,
     "len": 16,
     "blsb": true,
     "max": 65535,
     "unit": "scalar"
    },
    "name": "Inverter temperature, raw bytes"
   },
   {
    "id": "KONAEV_HEAT_SINK_T",
    "path": "Engine",
    "fmt": {
     "bix": 448,
     "len": 16,
     "blsb": true,
     "max": 327.67,
     "min": -327.68,
     "div": 100,
     "sign": true,
     "unit": "celsius"
    },
    "name": "Heat sink temperature"
   }
  ]
 },
 {
  "hdr": "7E3",
  "rax": "7EB",
  "cmd": {
   "22": "E009"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_EWP_CURRENT_DC",
    "path": "Battery",
    "fmt": {
     "bix": 56,
     "len": 8,
     "max": 25.5,
     "div": 10,
     "unit": "amps"
    },
    "name": "Electric water pump, current DC"
   },
   {
    "id": "KONAEV_EWP_VOLTAGE_DC",
    "path": "Battery",
    "fmt": {
     "bix": 64,
     "len": 8,
     "max": 25.5,
     "div": 10,
     "unit": "volts"
    },
    "name": "Electric water pump, voltage DC"
   },
   {
    "id": "KONAEV_EWP_COMMANDED_ROTATION_SPEED",
    "path": "Battery",
    "fmt": {
     "bix": 88,
     "len": 8,
     "max": 6120,
     "mul": 24,
     "unit": "rpm"
    },
    "name": "Electric water pump, commanded rotation speed"
   },
   {
    "id": "KONAEV_EWP_ACTUAL_ROTATION_SPEED",
    "path": "Battery",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 6120,
     "mul": 24,
     "unit": "rpm"
    },
    "name": "Electric water pump, actual rotation speed"
   },
   {
    "id": "KONAEV_EOP_ACTUAL_SPEED",
    "path": "Battery",
    "fmt": {
     "bix": 112,
     "len": 8,
     "max": 5100,
     "mul": 20,
     "unit": "rpm"
    },
    "name": "Electric oil pump, actual speed"
   },
   {
    "id": "KONAEV_EOP_COMMANDED_SPEED",
    "path": "Battery",
    "fmt": {
     "bix": 152,
     "len": 8,
     "max": 5100,
     "mul": 20,
     "unit": "rpm"
    },
    "name": "Electric oil pump, commanded speed"
   },
   {
    "id": "KONAEV_EOP_CURRENT_DC",
    "path": "Battery",
    "fmt": {
     "bix": 176,
     "len": 8,
     "max": 255,
     "unit": "amps"
    },
    "name": "Electric oil pump, current DC"
   },
   {
    "id": "KONAEV_EOP_CURRENT_IQ",
    "path": "Battery",
    "fmt": {
     "bix": 184,
     "len": 8,
     "max": 255,
     "unit": "amps"
    },
    "name": "Electric oil pump, current IQ"
   }
  ]
 },
 {
  "hdr": "7E4",
  "rax": "7EC",
  "fcm1": true,
  "cmd": {
   "22": "0101"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2019,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_HVBAT_SOC",
    "path": "Battery",
    "fmt": {
     "bix": 32,
     "len": 8,
     "max": 100,
     "div": 2,
     "unit": "percent"
    },
    "name": "HV battery charge",
    "suggestedMetric": "stateOfCharge"
   },
   {
    "id": "KONAEV_HVBAT_CHG_P_AVAIL",
    "path": "Battery",
    "fmt": {
     "bix": 40,
     "len": 16,
     "max": 655.35,
     "div": 100,
     "unit": "kilowatts"
    },
    "name": "Available charge power"
   },
   {
    "id": "KONAEV_HVBAT_DCHG_P_AVAIL",
    "path": "Battery",
    "fmt": {
     "bix": 56,
     "len": 16,
     "max": 655.35,
     "div": 100,
     "unit": "kilowatts"
    },
    "name": "Available discharge power"
   },
   {
    "id": "KONAEV_HVBAT_CHARGING",
    "path": "Battery",
    "fmt": {
     "bix": 72,
     "len": 1,
     "max": 1,
     "unit": "offon"
    },
    "name": "HV battery charging",
    "suggestedMetric": "isCharging"
   },
   {
    "id": "KONAEV_HVBAT_PLUG_RAPD",
    "path": "Battery",
    "fmt": {
     "bix": 73,
     "len": 1,
     "max": 1,
     "unit": "offon"
    },
    "name": "Rapid charge port state",
    "suggestedMetric": "pluggedIn"
   },
   {
    "id": "KONAEV_HVBAT_PLUG_NORM",
    "path": "Battery",
    "fmt": {
     "bix": 74,
     "len": 1,
     "max": 1,
     "unit": "offon"
    },
    "name": "Normal charge port state",
    "suggestedMetric": "pluggedIn"
   },
   {
    "id": "KONAEV_BMS_MAIN_RELAY",
    "path": "Battery",
    "fmt": {
     "bix": 79,
     "len": 1,
     "max": 1,
     "unit": "offon"
    },
    "name": "Battery main relay"
   },
   {
    "id": "KONAEV_HVBAT_CURR",
    "path": "Battery",
    "fmt": {
     "bix": 80,
     "len": 16,
     "max": 3276.7,
     "min": -3276.8,
     "div": 10,
     "sign": true,
     "unit": "amps"
    },
    "name": "HV battery current",
    "suggestedMetric": "tractionBatteryCurrent"
   },
   {
    "id": "KONAEV_HVBAT_VDC",
    "path": "Battery",
    "fmt": {
     "bix": 96,
     "len": 16,
     "max": 6553.5,
     "div": 10,
     "unit": "volts"
    },
    "name": "HV battery voltage, DC"
   },
   {
    "id": "KONAEV_HVBAT_T_MAX",
    "path": "Battery",
    "fmt": {
     "bix": 112,
     "len": 8,
     "max": 127,
     "min": -127,
     "sign": true,
     "unit": "celsius"
    },
    "name": "HV battery temperature, maximum"
   },
   {
    "id": "KONAEV_HVBAT_T_MIN",
    "path": "Battery",
    "fmt": {
     "bix": 120,
     "len": 8,
     "max": 127,
     "min": -127,
     "sign": true,
     "unit": "celsius"
    },
    "name": "HV battery temperature, minimum"
   },
   {
    "id": "KONAEV_HVBAT_MOD1_T",
    "path": "Battery",
    "fmt": {
     "bix": 128,
     "len": 8,
     "max": 127,
     "min": -127,
     "sign": true,
     "unit": "celsius"
    },
    "name": "HV battery temperature, module 1"
   },
   {
    "id": "KONAEV_HVBAT_MOD2_T",
    "path": "Battery",
    "fmt": {
     "bix": 136,
     "len": 8,
     "max": 127,
     "min": -127,
     "sign": true,
     "unit": "celsius"
    },
    "name": "HV battery temperature, module 2"
   },
   {
    "id": "KONAEV_HVBAT_MOD3_T",
    "path": "Battery",
    "fmt": {
     "bix": 144,
     "len": 8,
     "max": 127,
     "min": -127,
     "sign": true,
     "unit": "celsius"
    },
    "name": "HV battery temperature, module 3"
   },
   {
    "id": "KONAEV_HVBAT_MOD4_T",
    "path": "Battery",
    "fmt": {
     "bix": 152,
     "len": 8,
     "max": 127,
     "min": -127,
     "sign": true,
     "unit": "celsius"
    },
    "name": "HV battery temperature, module 4"
   },
   {
    "id": "KONAEV_HVBAT_INLET_T",
    "path": "Battery",
    "fmt": {
     "bix": 176,
     "len": 8,
     "max": 127,
     "min": -127,
     "sign": true,
     "unit": "celsius"
    },
    "name": "Battery inlet temperature"
   },
   {
    "id": "KONAEV_C_V_MAX",
    "path": "Battery",
    "fmt": {
     "bix": 184,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "Maximum cell voltage"
   },
   {
    "id": "KONAEV_C_V_MAX_ID",
    "path": "Battery",
    "fmt": {
     "bix": 192,
     "len": 8,
     "max": 255,
     "unit": "scalar"
    },
    "name": "Maximum cell voltage id",
    "description": "The id of the cell with the highest voltage."
   },
   {
    "id": "KONAEV_C_V_MIN",
    "path": "Battery",
    "fmt": {
     "bix": 200,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "Minimum cell voltage"
   },
   {
    "id": "KONAEV_C_V_MIN_ID",
    "path": "Battery",
    "fmt": {
     "bix": 208,
     "len": 8,
     "max": 255,
     "unit": "scalar"
    },
    "name": "Minimum cell voltage id",
    "description": "The id of the cell with the lowest voltage."
   },
   {
    "id": "KONAEV_HVBAT_FAN_STATUS",
    "path": "Battery",
    "fmt": {
     "bix": 216,
     "len": 8,
     "max": 255,
     "unit": "scalar"
    },
    "name": "HV battery fan status"
   },
   {
    "id": "KONAEV_HVBAT_FAN",
    "path": "Battery",
    "fmt": {
     "bix": 224,
     "len": 8,
     "max": 255,
     "unit": "hertz"
    },
    "name": "HV battery fan"
   },
   {
    "id": "KONAEV_VPWR",
    "path": "Battery",
    "fmt": {
     "bix": 232,
     "len": 8,
     "max": 25.5,
     "div": 10,
     "unit": "volts"
    },
    "name": "Battery auxiliary voltage",
    "suggestedMetric": "starterBatteryVoltage"
   },
   {
    "id": "KONAEV_HVBAT_CHRG_TOT_C",
    "path": "Battery",
    "fmt": {
     "bix": 240,
     "len": 32,
     "max": 429496729,
     "div": 10,
     "unit": "ampereHours"
    },
    "name": "Total charge, current"
   },
   {
    "id": "KONAEV_HVBAT_DSCH_TOT_C",
    "path": "Battery",
    "fmt": {
     "bix": 272,
     "len": 32,
     "max": 429496729,
     "div": 10,
     "unit": "ampereHours"
    },
    "name": "Total discharge, current"
   },
   {
    "id": "KONAEV_HVBAT_CHRG_TOT_E",
    "path": "Battery",
    "fmt": {
     "bix": 304,
     "len": 32,
     "max": 429496729,
     "div": 10,
     "unit": "kilowattHours"
    },
    "name": "Total charge, energy"
   },
   {
    "id": "KONAEV_HVBAT_DSCH_TOT_E",
    "path": "Battery",
    "fmt": {
     "bix": 336,
     "len": 32,
     "max": 429496729,
     "div": 10,
     "unit": "kilowattHours"
    },
    "name": "Total discharge, energy"
   },
   {
    "id": "KONAEV_DRIVE_TIME",
    "path": "Stats",
    "fmt": {
     "bix": 368,
     "len": 32,
     "max": 4294967295,
     "unit": "seconds"
    },
    "name": "Total drive time"
   },
   {
    "id": "KONAEV_IGN",
    "path": "Battery",
    "fmt": {
     "bix": 405,
     "len": 1,
     "max": 1,
     "unit": "offon"
    },
    "name": "Ignition"
   },
   {
    "id": "KONAEV_INVRT_CV",
    "path": "Battery",
    "fmt": {
     "bix": 408,
     "len": 16,
     "max": 6553,
     "nullmax": 6553,
     "unit": "volts"
    },
    "name": "Inverter capacitor voltage"
   },
   {
    "id": "KONAEV_RPM",
    "path": "Engine",
    "fmt": {
     "bix": 424,
     "len": 16,
     "max": 32767,
     "min": -32768,
     "sign": true,
     "unit": "rpm"
    },
    "name": "Drive motor speed"
   },
   {
    "id": "KONAEV_RPM_2",
    "path": "Engine",
    "fmt": {
     "bix": 440,
     "len": 16,
     "max": 32767,
     "min": -32768,
     "sign": true,
     "unit": "rpm"
    },
    "name": "Drive motor speed 2"
   },
   {
    "id": "KONAEV_ISO_R",
    "path": "Battery",
    "fmt": {
     "bix": 456,
     "len": 16,
     "max": 65535,
     "unit": "kiloohms"
    },
    "name": "Isolation resistance"
   }
  ]
 },
 {
  "hdr": "7E4",
  "rax": "7EC",
  "fcm1": true,
  "cmd": {
   "22": "0102"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2019,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_HVBAT_CMU001_VOLT",
    "path": "Battery.Modules.001",
    "fmt": {
     "bix": 32,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 001 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU002_VOLT",
    "path": "Battery.Modules.002",
    "fmt": {
     "bix": 40,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 002 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU003_VOLT",
    "path": "Battery.Modules.003",
    "fmt": {
     "bix": 48,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 003 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU004_VOLT",
    "path": "Battery.Modules.004",
    "fmt": {
     "bix": 56,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 004 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU005_VOLT",
    "path": "Battery.Modules.005",
    "fmt": {
     "bix": 64,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 005 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU006_VOLT",
    "path": "Battery.Modules.006",
    "fmt": {
     "bix": 72,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 006 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU007_VOLT",
    "path": "Battery.Modules.007",
    "fmt": {
     "bix": 80,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 007 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU008_VOLT",
    "path": "Battery.Modules.008",
    "fmt": {
     "bix": 88,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 008 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU009_VOLT",
    "path": "Battery.Modules.009",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 009 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU010_VOLT",
    "path": "Battery.Modules.010",
    "fmt": {
     "bix": 104,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 010 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU011_VOLT",
    "path": "Battery.Modules.011",
    "fmt": {
     "bix": 112,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 011 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU012_VOLT",
    "path": "Battery.Modules.012",
    "fmt": {
     "bix": 120,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 012 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU013_VOLT",
    "path": "Battery.Modules.013",
    "fmt": {
     "bix": 128,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 013 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU014_VOLT",
    "path": "Battery.Modules.014",
    "fmt": {
     "bix": 136,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 014 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU015_VOLT",
    "path": "Battery.Modules.015",
    "fmt": {
     "bix": 144,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 015 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU016_VOLT",
    "path": "Battery.Modules.016",
    "fmt": {
     "bix": 152,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 016 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU017_VOLT",
    "path": "Battery.Modules.017",
    "fmt": {
     "bix": 160,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 017 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU018_VOLT",
    "path": "Battery.Modules.018",
    "fmt": {
     "bix": 168,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 018 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU019_VOLT",
    "path": "Battery.Modules.019",
    "fmt": {
     "bix": 176,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 019 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU020_VOLT",
    "path": "Battery.Modules.020",
    "fmt": {
     "bix": 184,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 020 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU021_VOLT",
    "path": "Battery.Modules.021",
    "fmt": {
     "bix": 192,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 021 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU022_VOLT",
    "path": "Battery.Modules.022",
    "fmt": {
     "bix": 200,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 022 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU023_VOLT",
    "path": "Battery.Modules.023",
    "fmt": {
     "bix": 208,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 023 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU024_VOLT",
    "path": "Battery.Modules.024",
    "fmt": {
     "bix": 216,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 024 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU025_VOLT",
    "path": "Battery.Modules.025",
    "fmt": {
     "bix": 224,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 025 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU026_VOLT",
    "path": "Battery.Modules.026",
    "fmt": {
     "bix": 232,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 026 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU027_VOLT",
    "path": "Battery.Modules.027",
    "fmt": {
     "bix": 240,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 027 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU028_VOLT",
    "path": "Battery.Modules.028",
    "fmt": {
     "bix": 248,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 028 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU029_VOLT",
    "path": "Battery.Modules.029",
    "fmt": {
     "bix": 256,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 029 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU030_VOLT",
    "path": "Battery.Modules.030",
    "fmt": {
     "bix": 264,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 030 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU031_VOLT",
    "path": "Battery.Modules.031",
    "fmt": {
     "bix": 272,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 031 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU032_VOLT",
    "path": "Battery.Modules.032",
    "fmt": {
     "bix": 280,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 032 voltage"
   }
  ]
 },
 {
  "hdr": "7E4",
  "rax": "7EC",
  "fcm1": true,
  "cmd": {
   "22": "0103"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2019,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_HVBAT_CMU033_VOLT",
    "path": "Battery.Modules.033",
    "fmt": {
     "bix": 32,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 033 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU034_VOLT",
    "path": "Battery.Modules.034",
    "fmt": {
     "bix": 40,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 034 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU035_VOLT",
    "path": "Battery.Modules.035",
    "fmt": {
     "bix": 48,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 035 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU036_VOLT",
    "path": "Battery.Modules.036",
    "fmt": {
     "bix": 56,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 036 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU037_VOLT",
    "path": "Battery.Modules.037",
    "fmt": {
     "bix": 64,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 037 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU038_VOLT",
    "path": "Battery.Modules.038",
    "fmt": {
     "bix": 72,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 038 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU039_VOLT",
    "path": "Battery.Modules.039",
    "fmt": {
     "bix": 80,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 039 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU040_VOLT",
    "path": "Battery.Modules.040",
    "fmt": {
     "bix": 88,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 040 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU041_VOLT",
    "path": "Battery.Modules.041",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 041 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU042_VOLT",
    "path": "Battery.Modules.042",
    "fmt": {
     "bix": 104,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 042 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU043_VOLT",
    "path": "Battery.Modules.043",
    "fmt": {
     "bix": 112,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 043 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU044_VOLT",
    "path": "Battery.Modules.044",
    "fmt": {
     "bix": 120,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 044 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU045_VOLT",
    "path": "Battery.Modules.045",
    "fmt": {
     "bix": 128,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 045 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU046_VOLT",
    "path": "Battery.Modules.046",
    "fmt": {
     "bix": 136,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 046 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU047_VOLT",
    "path": "Battery.Modules.047",
    "fmt": {
     "bix": 144,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 047 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU048_VOLT",
    "path": "Battery.Modules.048",
    "fmt": {
     "bix": 152,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 048 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU049_VOLT",
    "path": "Battery.Modules.049",
    "fmt": {
     "bix": 160,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 049 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU050_VOLT",
    "path": "Battery.Modules.050",
    "fmt": {
     "bix": 168,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 050 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU051_VOLT",
    "path": "Battery.Modules.051",
    "fmt": {
     "bix": 176,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 051 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU052_VOLT",
    "path": "Battery.Modules.052",
    "fmt": {
     "bix": 184,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 052 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU053_VOLT",
    "path": "Battery.Modules.053",
    "fmt": {
     "bix": 192,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 053 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU054_VOLT",
    "path": "Battery.Modules.054",
    "fmt": {
     "bix": 200,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 054 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU055_VOLT",
    "path": "Battery.Modules.055",
    "fmt": {
     "bix": 208,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 055 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU056_VOLT",
    "path": "Battery.Modules.056",
    "fmt": {
     "bix": 216,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 056 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU057_VOLT",
    "path": "Battery.Modules.057",
    "fmt": {
     "bix": 224,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 057 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU058_VOLT",
    "path": "Battery.Modules.058",
    "fmt": {
     "bix": 232,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 058 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU059_VOLT",
    "path": "Battery.Modules.059",
    "fmt": {
     "bix": 240,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 059 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU060_VOLT",
    "path": "Battery.Modules.060",
    "fmt": {
     "bix": 248,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 060 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU061_VOLT",
    "path": "Battery.Modules.061",
    "fmt": {
     "bix": 256,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 061 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU062_VOLT",
    "path": "Battery.Modules.062",
    "fmt": {
     "bix": 264,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 062 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU063_VOLT",
    "path": "Battery.Modules.063",
    "fmt": {
     "bix": 272,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 063 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU064_VOLT",
    "path": "Battery.Modules.064",
    "fmt": {
     "bix": 280,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 064 voltage"
   }
  ]
 },
 {
  "hdr": "7E4",
  "rax": "7EC",
  "fcm1": true,
  "cmd": {
   "22": "0104"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2019,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_HVBAT_CMU065_VOLT",
    "path": "Battery.Modules.065",
    "fmt": {
     "bix": 32,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 065 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU066_VOLT",
    "path": "Battery.Modules.066",
    "fmt": {
     "bix": 40,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 066 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU067_VOLT",
    "path": "Battery.Modules.067",
    "fmt": {
     "bix": 48,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 067 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU068_VOLT",
    "path": "Battery.Modules.068",
    "fmt": {
     "bix": 56,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 068 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU069_VOLT",
    "path": "Battery.Modules.069",
    "fmt": {
     "bix": 64,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 069 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU070_VOLT",
    "path": "Battery.Modules.070",
    "fmt": {
     "bix": 72,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 070 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU071_VOLT",
    "path": "Battery.Modules.071",
    "fmt": {
     "bix": 80,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 071 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU072_VOLT",
    "path": "Battery.Modules.072",
    "fmt": {
     "bix": 88,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 072 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU073_VOLT",
    "path": "Battery.Modules.073",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 073 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU074_VOLT",
    "path": "Battery.Modules.074",
    "fmt": {
     "bix": 104,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 074 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU075_VOLT",
    "path": "Battery.Modules.075",
    "fmt": {
     "bix": 112,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 075 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU076_VOLT",
    "path": "Battery.Modules.076",
    "fmt": {
     "bix": 120,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 076 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU077_VOLT",
    "path": "Battery.Modules.077",
    "fmt": {
     "bix": 128,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 077 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU078_VOLT",
    "path": "Battery.Modules.078",
    "fmt": {
     "bix": 136,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 078 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU079_VOLT",
    "path": "Battery.Modules.079",
    "fmt": {
     "bix": 144,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 079 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU080_VOLT",
    "path": "Battery.Modules.080",
    "fmt": {
     "bix": 152,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 080 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU081_VOLT",
    "path": "Battery.Modules.081",
    "fmt": {
     "bix": 160,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 081 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU082_VOLT",
    "path": "Battery.Modules.082",
    "fmt": {
     "bix": 168,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 082 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU083_VOLT",
    "path": "Battery.Modules.083",
    "fmt": {
     "bix": 176,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 083 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU084_VOLT",
    "path": "Battery.Modules.084",
    "fmt": {
     "bix": 184,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 084 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU085_VOLT",
    "path": "Battery.Modules.085",
    "fmt": {
     "bix": 192,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 085 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU086_VOLT",
    "path": "Battery.Modules.086",
    "fmt": {
     "bix": 200,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 086 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU087_VOLT",
    "path": "Battery.Modules.087",
    "fmt": {
     "bix": 208,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 087 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU088_VOLT",
    "path": "Battery.Modules.088",
    "fmt": {
     "bix": 216,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 088 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU089_VOLT",
    "path": "Battery.Modules.089",
    "fmt": {
     "bix": 224,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 089 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU090_VOLT",
    "path": "Battery.Modules.090",
    "fmt": {
     "bix": 232,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 090 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU091_VOLT",
    "path": "Battery.Modules.091",
    "fmt": {
     "bix": 240,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 091 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU092_VOLT",
    "path": "Battery.Modules.092",
    "fmt": {
     "bix": 248,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 092 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU093_VOLT",
    "path": "Battery.Modules.093",
    "fmt": {
     "bix": 256,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 093 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU094_VOLT",
    "path": "Battery.Modules.094",
    "fmt": {
     "bix": 264,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 094 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU095_VOLT",
    "path": "Battery.Modules.095",
    "fmt": {
     "bix": 272,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 095 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU096_VOLT",
    "path": "Battery.Modules.096",
    "fmt": {
     "bix": 280,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 096 voltage"
   }
  ]
 },
 {
  "hdr": "7E4",
  "rax": "7EC",
  "fcm1": true,
  "cmd": {
   "22": "0105"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2019,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_HVBAT_C_DEV_V",
    "path": "Battery",
    "fmt": {
     "bix": 160,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV cell deviation, voltage"
   },
   {
    "id": "KONAEV_AIRBAG_DUTY",
    "path": "Battery",
    "fmt": {
     "bix": 176,
     "len": 8,
     "max": 255,
     "unit": "scalar"
    },
    "name": "Airbag hard-wire duty"
   },
   {
    "id": "KONAEV_HVBAT_HTR1_T",
    "path": "Battery",
    "fmt": {
     "bix": 184,
     "len": 8,
     "max": 127,
     "min": -127,
     "sign": true,
     "unit": "celsius"
    },
    "name": "HV battery heater 1 temperature"
   },
   {
    "id": "KONAEV_HVBAT_HTR2_T",
    "path": "Battery",
    "fmt": {
     "bix": 192,
     "len": 8,
     "max": 255,
     "unit": "scalar"
    },
    "name": "HV battery heater 2 temperature, raw byte"
   },
   {
    "id": "KONAEV_HVBAT_SOH",
    "path": "Battery",
    "fmt": {
     "bix": 200,
     "len": 16,
     "max": 100,
     "div": 10,
     "unit": "percent"
    },
    "name": "HV battery state of health",
    "suggestedMetric": "stateOfHealth"
   },
   {
    "id": "KONAEV_HVBAT_DET_MAX_ID",
    "path": "Battery",
    "fmt": {
     "bix": 216,
     "len": 8,
     "max": 98,
     "nullmax": 200,
     "unit": "scalar"
    },
    "name": "HV battery max deterioration cell id"
   },
   {
    "id": "KONAEV_HVBAT_DET_MIN",
    "path": "Battery",
    "fmt": {
     "bix": 224,
     "len": 16,
     "max": 100,
     "div": 10,
     "unit": "percent"
    },
    "name": "HV battery min deterioration"
   },
   {
    "id": "KONAEV_HVBAT_DET_MIN_ID",
    "path": "Battery",
    "fmt": {
     "bix": 240,
     "len": 8,
     "max": 98,
     "nullmax": 200,
     "unit": "scalar"
    },
    "name": "HV battery min deterioration cell id"
   },
   {
    "id": "KONAEV_HVBAT_SOC_DISP",
    "path": "Battery",
    "fmt": {
     "bix": 248,
     "len": 8,
     "max": 100,
     "div": 2,
     "unit": "percent"
    },
    "name": "HV battery charge",
    "suggestedMetric": "stateOfCharge"
   },
   {
    "id": "KONAEV_HVBAT_CMU097_VOLT",
    "path": "Battery.Modules.097",
    "fmt": {
     "bix": 272,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 097 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU098_VOLT",
    "path": "Battery.Modules.098",
    "fmt": {
     "bix": 280,
     "len": 8,
     "max": 4.2,
     "min": 2.8,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 098 voltage"
   }
  ]
 },
 {
  "hdr": "7E4",
  "rax": "7EC",
  "fcm1": true,
  "cmd": {
   "22": "0106"
  },
  "freq": 5,
  "dbgfilter": {
   "to": 2020,
   "years": [
    2022,
    2023
   ],
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_BMS_STATE",
    "path": "Battery",
    "fmt": {
     "len": 8,
     "max": 15,
     "unit": "scalar"
    },
    "name": "Battery management state"
   },
   {
    "id": "KONAEV_HVBAT_WCS_INLET_T",
    "path": "Battery",
    "fmt": {
     "bix": 32,
     "len": 8,
     "max": 127,
     "min": -128,
     "sign": true,
     "unit": "celsius"
    },
    "name": "Battery temperature at WCS input"
   },
   {
    "id": "KONAEV_HVBAT_FAN_DUTY",
    "path": "Battery",
    "fmt": {
     "bix": 64,
     "len": 8,
     "max": 100,
     "unit": "percent"
    },
    "name": "Battery fan duty feedback"
   },
   {
    "id": "KONAEV_AC_COMP_RPM",
    "path": "Climate",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 5100,
     "mul": 20,
     "unit": "rpm"
    },
    "name": "Air conditioning compressor speed"
   },
   {
    "id": "KONAEV_HVBAT_PRA_T",
    "path": "Battery",
    "fmt": {
     "bix": 160,
     "len": 8,
     "max": 254,
     "min": -256,
     "mul": 2,
     "sign": true,
     "unit": "celsius"
    },
    "name": "Battery temperature at the power relay assembly"
   }
  ]
 },
 {
  "hdr": "7E4",
  "rax": "7EC",
  "fcm1": true,
  "cmd": {
   "22": "010A"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_HVBAT_CMU097_VOLT_7E4",
    "path": "Battery.Modules.097",
    "fmt": {
     "bix": 32,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 097 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU098_VOLT_7E4",
    "path": "Battery.Modules.098",
    "fmt": {
     "bix": 40,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 098 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU099_VOLT",
    "path": "Battery.Modules.099",
    "fmt": {
     "bix": 48,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 099 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU100_VOLT",
    "path": "Battery.Modules.100",
    "fmt": {
     "bix": 56,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 100 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU101_VOLT",
    "path": "Battery.Modules.101",
    "fmt": {
     "bix": 64,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 101 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU102_VOLT",
    "path": "Battery.Modules.102",
    "fmt": {
     "bix": 72,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 102 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU103_VOLT",
    "path": "Battery.Modules.103",
    "fmt": {
     "bix": 80,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 103 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU104_VOLT",
    "path": "Battery.Modules.104",
    "fmt": {
     "bix": 88,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 104 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU105_VOLT",
    "path": "Battery.Modules.105",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 105 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU106_VOLT",
    "path": "Battery.Modules.106",
    "fmt": {
     "bix": 104,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 106 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU107_VOLT",
    "path": "Battery.Modules.107",
    "fmt": {
     "bix": 112,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 107 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU108_VOLT",
    "path": "Battery.Modules.108",
    "fmt": {
     "bix": 120,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 108 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU109_VOLT",
    "path": "Battery.Modules.109",
    "fmt": {
     "bix": 128,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 109 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU110_VOLT",
    "path": "Battery.Modules.110",
    "fmt": {
     "bix": 136,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 110 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU111_VOLT",
    "path": "Battery.Modules.111",
    "fmt": {
     "bix": 144,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 111 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU112_VOLT",
    "path": "Battery.Modules.112",
    "fmt": {
     "bix": 152,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 112 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU113_VOLT",
    "path": "Battery.Modules.113",
    "fmt": {
     "bix": 160,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 113 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU114_VOLT",
    "path": "Battery.Modules.114",
    "fmt": {
     "bix": 168,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 114 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU115_VOLT",
    "path": "Battery.Modules.115",
    "fmt": {
     "bix": 176,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 115 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU116_VOLT",
    "path": "Battery.Modules.116",
    "fmt": {
     "bix": 184,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 116 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU117_VOLT",
    "path": "Battery.Modules.117",
    "fmt": {
     "bix": 192,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 117 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU118_VOLT",
    "path": "Battery.Modules.118",
    "fmt": {
     "bix": 200,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 118 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU119_VOLT",
    "path": "Battery.Modules.119",
    "fmt": {
     "bix": 208,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 119 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU120_VOLT",
    "path": "Battery.Modules.120",
    "fmt": {
     "bix": 216,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 120 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU121_VOLT",
    "path": "Battery.Modules.121",
    "fmt": {
     "bix": 224,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 121 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU122_VOLT",
    "path": "Battery.Modules.122",
    "fmt": {
     "bix": 232,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 122 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU123_VOLT",
    "path": "Battery.Modules.123",
    "fmt": {
     "bix": 240,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 123 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU124_VOLT",
    "path": "Battery.Modules.124",
    "fmt": {
     "bix": 248,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 124 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU125_VOLT",
    "path": "Battery.Modules.125",
    "fmt": {
     "bix": 256,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 125 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU126_VOLT",
    "path": "Battery.Modules.126",
    "fmt": {
     "bix": 264,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 126 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU127_VOLT",
    "path": "Battery.Modules.127",
    "fmt": {
     "bix": 272,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 127 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU128_VOLT",
    "path": "Battery.Modules.128",
    "fmt": {
     "bix": 280,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 128 voltage"
   }
  ]
 },
 {
  "hdr": "7E4",
  "rax": "7EC",
  "fcm1": true,
  "cmd": {
   "22": "010B"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_HVBAT_CMU129_VOLT",
    "path": "Battery.Modules.129",
    "fmt": {
     "bix": 32,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 129 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU130_VOLT",
    "path": "Battery.Modules.130",
    "fmt": {
     "bix": 40,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 130 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU131_VOLT",
    "path": "Battery.Modules.131",
    "fmt": {
     "bix": 48,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 131 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU132_VOLT",
    "path": "Battery.Modules.132",
    "fmt": {
     "bix": 56,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 132 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU133_VOLT",
    "path": "Battery.Modules.133",
    "fmt": {
     "bix": 64,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 133 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU134_VOLT",
    "path": "Battery.Modules.134",
    "fmt": {
     "bix": 72,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 134 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU135_VOLT",
    "path": "Battery.Modules.135",
    "fmt": {
     "bix": 80,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 135 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU136_VOLT",
    "path": "Battery.Modules.136",
    "fmt": {
     "bix": 88,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 136 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU137_VOLT",
    "path": "Battery.Modules.137",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 137 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU138_VOLT",
    "path": "Battery.Modules.138",
    "fmt": {
     "bix": 104,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 138 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU139_VOLT",
    "path": "Battery.Modules.139",
    "fmt": {
     "bix": 112,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 139 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU140_VOLT",
    "path": "Battery.Modules.140",
    "fmt": {
     "bix": 120,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 140 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU141_VOLT",
    "path": "Battery.Modules.141",
    "fmt": {
     "bix": 128,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 141 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU142_VOLT",
    "path": "Battery.Modules.142",
    "fmt": {
     "bix": 136,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 142 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU143_VOLT",
    "path": "Battery.Modules.143",
    "fmt": {
     "bix": 144,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 143 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU144_VOLT",
    "path": "Battery.Modules.144",
    "fmt": {
     "bix": 152,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 144 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU145_VOLT",
    "path": "Battery.Modules.145",
    "fmt": {
     "bix": 160,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 145 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU146_VOLT",
    "path": "Battery.Modules.146",
    "fmt": {
     "bix": 168,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 146 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU147_VOLT",
    "path": "Battery.Modules.147",
    "fmt": {
     "bix": 176,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 147 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU148_VOLT",
    "path": "Battery.Modules.148",
    "fmt": {
     "bix": 184,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 148 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU149_VOLT",
    "path": "Battery.Modules.149",
    "fmt": {
     "bix": 192,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 149 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU150_VOLT",
    "path": "Battery.Modules.150",
    "fmt": {
     "bix": 200,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 150 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU151_VOLT",
    "path": "Battery.Modules.151",
    "fmt": {
     "bix": 208,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 151 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU152_VOLT",
    "path": "Battery.Modules.152",
    "fmt": {
     "bix": 216,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 152 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU153_VOLT",
    "path": "Battery.Modules.153",
    "fmt": {
     "bix": 224,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 153 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU154_VOLT",
    "path": "Battery.Modules.154",
    "fmt": {
     "bix": 232,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 154 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU155_VOLT",
    "path": "Battery.Modules.155",
    "fmt": {
     "bix": 240,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 155 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU156_VOLT",
    "path": "Battery.Modules.156",
    "fmt": {
     "bix": 248,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 156 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU157_VOLT",
    "path": "Battery.Modules.157",
    "fmt": {
     "bix": 256,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 157 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU158_VOLT",
    "path": "Battery.Modules.158",
    "fmt": {
     "bix": 264,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 158 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU159_VOLT",
    "path": "Battery.Modules.159",
    "fmt": {
     "bix": 272,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 159 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU160_VOLT",
    "path": "Battery.Modules.160",
    "fmt": {
     "bix": 280,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 160 voltage"
   }
  ]
 },
 {
  "hdr": "7E4",
  "rax": "7EC",
  "fcm1": true,
  "cmd": {
   "22": "010C"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_HVBAT_CMU161_VOLT",
    "path": "Battery.Modules.161",
    "fmt": {
     "bix": 32,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 161 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU162_VOLT",
    "path": "Battery.Modules.162",
    "fmt": {
     "bix": 40,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 162 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU163_VOLT",
    "path": "Battery.Modules.163",
    "fmt": {
     "bix": 48,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 163 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU164_VOLT",
    "path": "Battery.Modules.164",
    "fmt": {
     "bix": 56,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 164 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU165_VOLT",
    "path": "Battery.Modules.165",
    "fmt": {
     "bix": 64,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 165 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU166_VOLT",
    "path": "Battery.Modules.166",
    "fmt": {
     "bix": 72,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 166 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU167_VOLT",
    "path": "Battery.Modules.167",
    "fmt": {
     "bix": 80,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 167 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU168_VOLT",
    "path": "Battery.Modules.168",
    "fmt": {
     "bix": 88,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 168 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU169_VOLT",
    "path": "Battery.Modules.169",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 169 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU170_VOLT",
    "path": "Battery.Modules.170",
    "fmt": {
     "bix": 104,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 170 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU171_VOLT",
    "path": "Battery.Modules.171",
    "fmt": {
     "bix": 112,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 171 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU172_VOLT",
    "path": "Battery.Modules.172",
    "fmt": {
     "bix": 120,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 172 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU173_VOLT",
    "path": "Battery.Modules.173",
    "fmt": {
     "bix": 128,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 173 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU174_VOLT",
    "path": "Battery.Modules.174",
    "fmt": {
     "bix": 136,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 174 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU175_VOLT",
    "path": "Battery.Modules.175",
    "fmt": {
     "bix": 144,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 175 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU176_VOLT",
    "path": "Battery.Modules.176",
    "fmt": {
     "bix": 152,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 176 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU177_VOLT",
    "path": "Battery.Modules.177",
    "fmt": {
     "bix": 160,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 177 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU178_VOLT",
    "path": "Battery.Modules.178",
    "fmt": {
     "bix": 168,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 178 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU179_VOLT",
    "path": "Battery.Modules.179",
    "fmt": {
     "bix": 176,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 179 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU180_VOLT",
    "path": "Battery.Modules.180",
    "fmt": {
     "bix": 184,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 180 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU181_VOLT",
    "path": "Battery.Modules.181",
    "fmt": {
     "bix": 192,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 181 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU182_VOLT",
    "path": "Battery.Modules.182",
    "fmt": {
     "bix": 200,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 182 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU183_VOLT",
    "path": "Battery.Modules.183",
    "fmt": {
     "bix": 208,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 183 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU184_VOLT",
    "path": "Battery.Modules.184",
    "fmt": {
     "bix": 216,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 184 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU185_VOLT",
    "path": "Battery.Modules.185",
    "fmt": {
     "bix": 224,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 185 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU186_VOLT",
    "path": "Battery.Modules.186",
    "fmt": {
     "bix": 232,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 186 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU187_VOLT",
    "path": "Battery.Modules.187",
    "fmt": {
     "bix": 240,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 187 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU188_VOLT",
    "path": "Battery.Modules.188",
    "fmt": {
     "bix": 248,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 188 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU189_VOLT",
    "path": "Battery.Modules.189",
    "fmt": {
     "bix": 256,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 189 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU190_VOLT",
    "path": "Battery.Modules.190",
    "fmt": {
     "bix": 264,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 190 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU191_VOLT",
    "path": "Battery.Modules.191",
    "fmt": {
     "bix": 272,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 191 voltage"
   },
   {
    "id": "KONAEV_HVBAT_CMU192_VOLT",
    "path": "Battery.Modules.192",
    "fmt": {
     "bix": 280,
     "len": 8,
     "max": 5.1,
     "div": 50,
     "unit": "volts"
    },
    "name": "HV battery module 192 voltage"
   }
  ]
 },
 {
  "hdr": "7E4",
  "rax": "7EC",
  "cmd": {
   "22": "0111"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2020,
   "years": [
    2023
   ],
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_NORMAL_CHARGING_COUNT",
    "path": "Battery",
    "fmt": {
     "bix": 32,
     "len": 32,
     "max": 4294967295,
     "unit": "scalar"
    },
    "name": "Normal charging, count"
   },
   {
    "id": "KONAEV_QUICK_CHARGING_COUNT",
    "path": "Battery",
    "fmt": {
     "bix": 64,
     "len": 32,
     "max": 4294967295,
     "unit": "scalar"
    },
    "name": "Quick charging, count"
   },
   {
    "id": "KONAEV_NORMAL_CHARGING_ENERGY_TOT",
    "path": "Battery",
    "fmt": {
     "bix": 96,
     "len": 32,
     "max": 4294967295,
     "unit": "kilowattHours"
    },
    "name": "Normal charging energy, total"
   },
   {
    "id": "KONAEV_QUICK_CHARGING_ENERGY_TOT",
    "path": "Battery",
    "fmt": {
     "bix": 128,
     "len": 32,
     "max": 4294967295,
     "unit": "kilowattHours"
    },
    "name": "Quick charging energy, total"
   },
   {
    "id": "KONAEV_BATTERY_PACK_B17",
    "path": "Battery",
    "fmt": {
     "bix": 208,
     "len": 8,
     "max": 80,
     "min": -40,
     "sign": true,
     "unit": "celsius"
    },
    "name": "Battery pack B17"
   },
   {
    "id": "KONAEV_BATTERY_PACK_B18",
    "path": "Battery",
    "fmt": {
     "bix": 216,
     "len": 8,
     "max": 80,
     "min": -40,
     "sign": true,
     "unit": "celsius"
    },
    "name": "Battery pack B18"
   }
  ]
 },
 {
  "hdr": "7E4",
  "rax": "7EC",
  "cmd": {
   "22": "0114"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_AC_COMPRESSOR_OPERATION_RPM",
    "path": "Climate",
    "fmt": {
     "bix": 208,
     "len": 16,
     "max": 1310700,
     "mul": 20,
     "unit": "rpm"
    },
    "name": "A/C compressor operation RPM"
   }
  ]
 },
 {
  "hdr": "7E5",
  "dbg": true,
  "cmd": {
   "21": "C0"
  },
  "freq": 0.25,
  "signals": [
   {
    "id": "KONAEV_RPM_7E5",
    "path": "Engine",
    "fmt": {
     "bix": 32,
     "len": 16,
     "max": 16383.75,
     "div": 4,
     "unit": "rpm"
    },
    "name": "Engine RPM"
   },
   {
    "id": "KONAEV_VSS_AWD",
    "path": "Movement",
    "fmt": {
     "bix": 48,
     "len": 8,
     "max": 255,
     "unit": "kilometersPerHour"
    },
    "name": "Vehicle speed",
    "suggestedMetric": "speed"
   },
   {
    "id": "KONAEV_APP_REL_AWD",
    "path": "Control",
    "fmt": {
     "bix": 56,
     "len": 8,
     "max": 100,
     "mul": 100,
     "div": 255,
     "unit": "percent"
    },
    "name": "Relative accelerator pedal position"
   },
   {
    "id": "KONAEV_SUPPLY_VOLTAGE",
    "path": "Battery",
    "fmt": {
     "bix": 64,
     "len": 8,
     "max": 25.5,
     "div": 10,
     "unit": "volts"
    },
    "name": "Supply voltage"
   },
   {
    "id": "KONAEV_SYSTEM_VOLTAGE",
    "path": "Battery",
    "fmt": {
     "bix": 72,
     "len": 8,
     "max": 25.5,
     "div": 10,
     "unit": "volts"
    },
    "name": "System voltage"
   },
   {
    "id": "KONAEV_COMMANDED_TORQUE",
    "path": "Engine",
    "fmt": {
     "bix": 80,
     "len": 8,
     "max": 2550,
     "mul": 10,
     "unit": "newtonMeters"
    },
    "name": "Commanded torque"
   },
   {
    "id": "KONAEV_CLUTCH_SOLENOID_DUTY",
    "path": "Engine",
    "fmt": {
     "bix": 88,
     "len": 8,
     "max": 255,
     "unit": "percent"
    },
    "name": "Clutch solenoid duty"
   },
   {
    "id": "KONAEV_HEAT",
    "path": "Engine",
    "fmt": {
     "bix": 96,
     "len": 8,
     "max": 255,
     "unit": "celsius"
    },
    "name": "Heat"
   },
   {
    "id": "KONAEV_FRONT_RIGHT_WHEEL_SPEED",
    "path": "Movement",
    "fmt": {
     "bix": 104,
     "len": 8,
     "max": 255,
     "unit": "kilometersPerHour"
    },
    "name": "Front right wheel speed"
   },
   {
    "id": "KONAEV_FRONT_LEFT_WHEEL_SPEED",
    "path": "Movement",
    "fmt": {
     "bix": 112,
     "len": 8,
     "max": 255,
     "unit": "kilometersPerHour"
    },
    "name": "Front left wheel speed"
   },
   {
    "id": "KONAEV_REAR_RIGHT_WHEEL_SPEED",
    "path": "Movement",
    "fmt": {
     "bix": 120,
     "len": 8,
     "max": 255,
     "unit": "kilometersPerHour"
    },
    "name": "Rear right wheel speed"
   },
   {
    "id": "KONAEV_REAR_LEFT_WHEEL_SPEED",
    "path": "Movement",
    "fmt": {
     "bix": 128,
     "len": 8,
     "max": 255,
     "unit": "kilometersPerHour"
    },
    "name": "Rear left wheel speed"
   },
   {
    "id": "KONAEV_LOCK_SWITCH",
    "path": "Engine",
    "fmt": {
     "bix": 151,
     "len": 1,
     "max": 1,
     "unit": "offon"
    },
    "name": "Lock switch"
   },
   {
    "id": "KONAEV_STEERING_WHEEL_ANGLE",
    "path": "Tires",
    "fmt": {
     "bix": 160,
     "len": 8,
     "max": 167,
     "min": -88,
     "add": -88,
     "unit": "degrees"
    },
    "name": "Steering wheel angle"
   },
   {
    "id": "KONAEV_MALFUNCTION_INDICATOR",
    "path": "Engine",
    "fmt": {
     "bix": 175,
     "len": 1,
     "max": 1,
     "unit": "offon"
    },
    "name": "Malfunction indicator"
   },
   {
    "id": "KONAEV_PRESSURE_SENSOR",
    "path": "Engine",
    "fmt": {
     "bix": 176,
     "len": 8,
     "max": 63.75,
     "div": 4,
     "unit": "bars"
    },
    "name": "Pressure sensor"
   }
  ]
 },
 {
  "hdr": "7E5",
  "rax": "7ED",
  "cmd": {
   "21": "01"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2020,
   "from": 2022
  },
  "signals": [
   {
    "id": "KONAEV_CHARGER_AC_INPUT_V",
    "path": "Battery",
    "fmt": {
     "bix": 72,
     "len": 8,
     "max": 652,
     "mul": 64,
     "div": 25,
     "unit": "volts"
    },
    "name": "Charger AC input voltage, RMS"
   },
   {
    "id": "KONAEV_CHARGER_T",
    "path": "Battery",
    "fmt": {
     "bix": 104,
     "len": 8,
     "max": 155,
     "min": -100,
     "add": -100,
     "unit": "celsius"
    },
    "name": "Charger temperature a"
   },
   {
    "id": "KONAEV_CHARGER_AC_INPUT",
    "path": "Battery",
    "fmt": {
     "bix": 200,
     "len": 8,
     "max": 652,
     "mul": 64,
     "div": 25,
     "unit": "amps"
    },
    "name": "Charger AC input current, RMS"
   },
   {
    "id": "KONAEV_CHARGER_AC_INPUT_V_INSTANT",
    "path": "Battery",
    "fmt": {
     "bix": 304,
     "len": 8,
     "max": 552,
     "min": -100,
     "mul": 64,
     "div": 25,
     "add": -100,
     "unit": "volts"
    },
    "name": "Charger AC input voltage, instant"
   },
   {
    "id": "KONAEV_CHARGER_T_B",
    "path": "Battery",
    "fmt": {
     "bix": 336,
     "len": 8,
     "max": 155,
     "min": -100,
     "add": -100,
     "unit": "celsius"
    },
    "name": "Charger temperature b"
   }
  ]
 },
 {
  "hdr": "7E5",
  "rax": "7ED",
  "cmd": {
   "22": "E001"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_CHARGE_MODE_DC_TARGET_VOLTAGE",
    "path": "Battery",
    "fmt": {
     "bix": 56,
     "len": 16,
     "max": 6553.5,
     "div": 10,
     "unit": "volts"
    },
    "name": "Charge mode DC target voltage"
   },
   {
    "id": "KONAEV_CHARGE_MODE_DC_MAXIMUM_CURRENT_LIMIT",
    "path": "Battery",
    "fmt": {
     "bix": 80,
     "len": 16,
     "max": 6553.5,
     "div": 10,
     "unit": "amps"
    },
    "name": "Charge mode DC maximum current limit"
   },
   {
    "id": "KONAEV_CHARGE_MODE_AC_MAXIMUM_CURRENT_LIMIT",
    "path": "Battery",
    "fmt": {
     "bix": 96,
     "len": 16,
     "max": 6553.5,
     "div": 10,
     "unit": "amps"
    },
    "name": "Charge mode A/C maximum current limit"
   },
   {
    "id": "KONAEV_DISCHARGE_MODE_V2L_AC_TARGET_VOLTAGE",
    "path": "Battery",
    "fmt": {
     "bix": 112,
     "len": 8,
     "max": 255,
     "unit": "volts"
    },
    "name": "Discharge mode A/C target voltage"
   },
   {
    "id": "KONAEV_DISCHARGE_MODE_DC_MAXIMUM_CURRENT_LIMIT",
    "path": "Battery",
    "fmt": {
     "bix": 128,
     "len": 8,
     "max": 255,
     "unit": "amps"
    },
    "name": "Discharge mode DC maximum current limit"
   },
   {
    "id": "KONAEV_DISCHARGE_MODE_AC_MAXIMUM_CURRENT_LIMIT",
    "path": "Battery",
    "fmt": {
     "bix": 136,
     "len": 8,
     "max": 255,
     "unit": "amps"
    },
    "name": "Discharge mode A/C maximum current limit"
   }
  ]
 },
 {
  "hdr": "7E5",
  "rax": "7ED",
  "cmd": {
   "22": "E002"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_OBC_AC_VOLTAGE_A_RMS",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 32,
     "len": 16,
     "max": 6553.5,
     "div": 10,
     "unit": "volts"
    },
    "name": "On-board charger, AC voltage a"
   },
   {
    "id": "KONAEV_OBC_AC_VOLTAGE_B_RMS",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 48,
     "len": 16,
     "max": 6553.5,
     "div": 10,
     "unit": "volts"
    },
    "name": "On-board charger, AC voltage b"
   },
   {
    "id": "KONAEV_OBC_AC_FREQUENCY",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 80,
     "len": 8,
     "max": 255,
     "unit": "hertz"
    },
    "name": "On-board charger, AC frequency"
   },
   {
    "id": "KONAEV_OBC_AC_TOTAL_CURRENT_RMS",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 88,
     "len": 16,
     "max": 327.67,
     "min": -327.68,
     "div": 100,
     "sign": true,
     "unit": "amps"
    },
    "name": "On-board charger, AC total current"
   },
   {
    "id": "KONAEV_OBC_AC_CURRENT_A_RMS",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 104,
     "len": 16,
     "max": 327.67,
     "min": -327.68,
     "div": 100,
     "sign": true,
     "unit": "amps"
    },
    "name": "On-board charger, AC current a"
   },
   {
    "id": "KONAEV_OBC_AC_CURRENT_B_RMS",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 120,
     "len": 16,
     "max": 327.67,
     "min": -327.68,
     "div": 100,
     "sign": true,
     "unit": "amps"
    },
    "name": "On-board charger, AC current b"
   },
   {
    "id": "KONAEV_OBC_DC_VOLTAGE",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 152,
     "len": 16,
     "max": 65535,
     "unit": "scalar"
    },
    "name": "On-board charger, DC voltage, raw bytes"
   },
   {
    "id": "KONAEV_OBC_DC_CURRENT_TOTAL_RMS",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 168,
     "len": 16,
     "max": 327.67,
     "min": -327.68,
     "div": 100,
     "sign": true,
     "unit": "amps"
    },
    "name": "On-board charger, DC current total"
   },
   {
    "id": "KONAEV_OBC_DC_CURRENT_A_RMS",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 184,
     "len": 16,
     "max": 327.67,
     "min": -327.68,
     "div": 100,
     "sign": true,
     "unit": "amps"
    },
    "name": "On-board charger, DC current a"
   },
   {
    "id": "KONAEV_OBC_DC_CURRENT_B_RMS",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 200,
     "len": 16,
     "max": 327.67,
     "min": -327.68,
     "div": 100,
     "sign": true,
     "unit": "amps"
    },
    "name": "On-board charger, DC current b"
   },
   {
    "id": "KONAEV_OBC_T_A",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 216,
     "len": 8,
     "max": 215,
     "min": -40,
     "add": -40,
     "unit": "celsius"
    },
    "name": "On-board charger, temperature a"
   },
   {
    "id": "KONAEV_OBC_T_B",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 224,
     "len": 8,
     "max": 215,
     "min": -40,
     "add": -40,
     "unit": "celsius"
    },
    "name": "On-board charger, temperature b"
   },
   {
    "id": "KONAEV_OBC_AC_INPUT_CURRENT_SENSOR_A_SIGNAL_VOLTAGE",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 296,
     "len": 16,
     "max": 655.35,
     "div": 100,
     "unit": "volts"
    },
    "name": "On-board charger, AC input current sensor a signal voltage"
   },
   {
    "id": "KONAEV_OBC_AC_INPUT_CURRENT_SENSOR_B_SIGNAL_VOLTAGE",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 312,
     "len": 16,
     "max": 655.35,
     "div": 100,
     "unit": "volts"
    },
    "name": "On-board charger, AC input current sensor b signal voltage"
   }
  ]
 },
 {
  "hdr": "7E5",
  "rax": "7ED",
  "cmd": {
   "22": "E003"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_OBC_CHARGE_MODE_DC_TARGET_VOLTAGE",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 64,
     "len": 16,
     "max": 6553.5,
     "div": 10,
     "unit": "volts"
    },
    "name": "On-board charger, charge mode DC target voltage"
   },
   {
    "id": "KONAEV_OBC_CHARGE_MODE_DC_TARGET_CURRENT",
    "path": "Battery.Charging",
    "fmt": {
     "bix": 80,
     "len": 16,
     "max": 6553.5,
     "div": 10,
     "unit": "amps"
    },
    "name": "On-board charger, charge mode DC target current"
   }
  ]
 },
 {
  "hdr": "7E5",
  "rax": "7ED",
  "cmd": {
   "22": "E011"
  },
  "freq": 1,
  "dbgfilter": {
   "to": 2023,
   "from": 2025
  },
  "signals": [
   {
    "id": "KONAEV_LDC_T",
    "path": "Battery",
    "fmt": {
     "bix": 72,
     "len": 8,
     "max": 155,
     "min": -100,
     "add": -100,
     "unit": "celsius"
    },
    "name": "Low-voltage dc-dc converter, temperature"
   },
   {
    "id": "KONAEV_LDC_OUTPUT_VOLTAGE",
    "path": "Battery",
    "fmt": {
     "bix": 80,
     "len": 16,
     "max": 65.535,
     "div": 1000,
     "unit": "volts"
    },
    "name": "Low-voltage dc-dc converter, output voltage"
   },
   {
    "id": "KONAEV_LDC_OUTPUT_CURRENT",
    "path": "Battery",
    "fmt": {
     "bix": 96,
     "len": 16,
     "max": 655.35,
     "mul": 10,
     "div": 1000,
     "unit": "amps"
    },
    "name": "Low-voltage dc-dc converter, output current"
   },
   {
    "id": "KONAEV_LDC_INPUT_VOLTAGE",
    "path": "Battery",
    "fmt": {
     "bix": 112,
     "len": 16,
     "max": 6553.5,
     "div": 10,
     "unit": "volts"
    },
    "name": "Low-voltage dc-dc converter, input voltage"
   },
   {
    "id": "KONAEV_AUX_BATTERY_CURRENT",
    "path": "Battery",
    "fmt": {
     "bix": 144,
     "len": 16,
     "max": 328.35,
     "min": -327,
     "div": 100,
     "add": -327,
     "unit": "amps"
    },
    "name": "Auxiliary battery current"
   },
   {
    "id": "KONAEV_AUX_BATTERY_STATE_OF_CHARGE",
    "path": "Battery",
    "fmt": {
     "bix": 160,
     "len": 8,
     "max": 100,
     "unit": "percent"
    },
    "name": "Auxiliary battery state of charge"
   },
   {
    "id": "KONAEV_AUX_BATTERY_VOLTAGE_7E5",
    "path": "Battery",
    "fmt": {
     "bix": 168,
     "len": 16,
     "max": 71.535,
     "min": 6,
     "div": 1000,
     "add": 6,
     "unit": "volts"
    },
    "name": "Auxiliary battery voltage",
    "suggestedMetric": "starterBatteryVoltage"
   },
   {
    "id": "KONAEV_AUX_BATTERY_T",
    "path": "Battery",
    "fmt": {
     "bix": 184,
     "len": 16,
     "max": 32727.5,
     "min": -40,
     "div": 2,
     "add": -40,
     "unit": "celsius"
    },
    "name": "Auxiliary battery temperature"
   }
  ]
 },
 {
  "hdr": "7E5",
  "rax": "7ED",
  "dbg": true,
  "cmd": {
   "22": "E101"
  },
  "freq": 1,
  "signals": [
   {
    "id": "KONAEV_2025_LDC_T",
    "path": "Battery",
    "fmt": {
     "bix": 72,
     "len": 8,
     "max": 155,
     "min": -100,
     "add": -100,
     "unit": "celsius"
    },
    "name": "Low-voltage dc-dc converter, temperature"
   },
   {
    "id": "KONAEV_2025_LDC_OUTPUT_VOLTAGE",
    "path": "Battery",
    "fmt": {
     "bix": 80,
     "len": 16,
     "max": 65.535,
     "div": 1000,
     "unit": "volts"
    },
    "name": "Low-voltage dc-dc converter, output voltage"
   },
   {
    "id": "KONAEV_2025_LDC_OUTPUT_CURRENT",
    "path": "Battery",
    "fmt": {
     "bix": 96,
     "len": 16,
     "max": 655.35,
     "mul": 10,
     "div": 1000,
     "unit": "amps"
    },
    "name": "Low-voltage dc-dc converter, output current"
   },
   {
    "id": "KONAEV_2025_LDC_INPUT_VOLTAGE",
    "path": "Battery",
    "fmt": {
     "bix": 112,
     "len": 16,
     "max": 6553.5,
     "div": 10,
     "unit": "volts"
    },
    "name": "Low-voltage dc-dc converter, input voltage"
   },
   {
    "id": "KONAEV_2025_AUX_BATTERY_CURRENT",
    "path": "Battery",
    "fmt": {
     "bix": 144,
     "len": 16,
     "max": 328.35,
     "min": -327,
     "div": 100,
     "add": -327,
     "unit": "amps"
    },
    "name": "Auxiliary battery current"
   },
   {
    "id": "KONAEV_2025_AUX_BATTERY_STATE_OF_CHARGE",
    "path": "Battery",
    "fmt": {
     "bix": 160,
     "len": 8,
     "max": 100,
     "unit": "percent"
    },
    "name": "Auxiliary battery state of charge"
   },
   {
    "id": "KONAEV_2025_AUX_BATTERY_VOLTAGE",
    "path": "Battery",
    "fmt": {
     "bix": 168,
     "len": 16,
     "max": 71.535,
     "min": 6,
     "div": 1000,
     "add": 6,
     "unit": "volts"
    },
    "name": "Auxiliary battery voltage",
    "suggestedMetric": "starterBatteryVoltage"
   },
   {
    "id": "KONAEV_2025_AUX_BATTERY_T",
    "path": "Battery",
    "fmt": {
     "bix": 184,
     "len": 16,
     "max": 32727.5,
     "min": -40,
     "div": 2,
     "add": -40,
     "unit": "celsius"
    },
    "name": "Auxiliary battery temperature"
   }
  ]
 }
]

```
