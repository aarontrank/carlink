---
title: python-OBD — service 06 tables (MIDs, standard TIDs, Unit-and-Scaling IDs) and decoder behaviour, restated as data
source_url: https://github.com/brendan-w/python-OBD/tree/a378bdd81d58c67d08050e4244173a9a7dbda73d
files: obd/UnitsAndScaling.py, obd/codes.py (TEST_IDS), obd/commands.py (__mode6__), obd/decoders.py (monitor, parse_monitor_test, evap_pressure), obd/OBDResponse.py (MonitorTest.passed), tests/test_decoders.py (test_monitor)
commit: a378bdd81d58c67d08050e4244173a9a7dbda73d
fetched: 2026-11-08
license: GPL-2.0 (the project). No code is copied here; this file restates the factual tables (IDs, scale factors, signedness, units) and describes behaviour in prose.
---

python-OBD marks mode 06 as experimental and CAN-only. These tables are the only complete,
openly readable copy of the J1979 Appendix D/E style data found; treat them as `community`
(an implementation, not the standard).

## Decoder behaviour (prose)

- Drops the `46` byte, then reads the rest in **9-byte blocks**: MID, TID, UASID, value (2 bytes),
  min (2 bytes), max (2 bytes). The MID is therefore expected **once per block** (repeated). A trailing
  partial block is discarded.
- Unknown TID: still decoded, named "Unknown". Unknown UASID: the test is dropped.
- Value, min and max are big-endian 16-bit; for signed UAS IDs the 16-bit two's complement is taken;
  then `value * scale + offset`.
- Pass = `min <= value <= max` (after scaling). No special case for an all-zero record.
- Test vector in tests/test_decoders.py (data after the 46 byte):
  `01 01 0A 0BB0 0BB0 0BB0 | 01 05 10 0048 0000 0064 | 01 85 24 0096 004B FFFF`
  → TID 01 = 365 mV / 365 / 365 (UAS 0A); TID 05 = 72 ms, min 0, max 100 ms (UAS 10);
  TID 85 = 150 counts, min 75, max 65535 (UAS 24).

## Service 01 PID 32 decoder (for the formula-disagreement record)

`evap_pressure` signs byte A **and byte B separately** (each as 8-bit two's complement), then
computes (A*256 + B)/4 Pa. PID 53 = (256A+B)/200 kPa. PID 54 = (256A+B) − 32767 Pa (offset binary).
PIDs 24–2B voltage = (256C+D)*8/65535 (denominator 65535, not 65536); λ not decoded for 24–2B.
PIDs 34–3B current = (256C+D)/256 − 128 mA.

## Standard TIDs (codes.py TEST_IDS)

| TID | Name | Description |
|---|---|---|
| 01 | RTL_THRESHOLD_VOLTAGE | Rich to lean sensor threshold voltage |
| 02 | LTR_THRESHOLD_VOLTAGE | Lean to rich sensor threshold voltage |
| 03 | LOW_VOLTAGE_SWITCH_TIME | Low sensor voltage for switch time calculation |
| 04 | HIGH_VOLTAGE_SWITCH_TIME | High sensor voltage for switch time calculation |
| 05 | RTL_SWITCH_TIME | Rich to lean sensor switch time |
| 06 | LTR_SWITCH_TIME | Lean to rich sensor switch time |
| 07 | MIN_VOLTAGE | Minimum sensor voltage for test cycle |
| 08 | MAX_VOLTAGE | Maximum sensor voltage for test cycle |
| 09 | TRANSITION_TIME | Time between sensor transitions |
| 0A | SENSOR_PERIOD | Sensor period |
| 0B | MISFIRE_AVERAGE | Average misfire counts for last ten driving cycles |
| 0C | MISFIRE_COUNT | Misfire counts for last/current driving cycles |

TID 00 is reserved (not listed). TIDs not in this table decode as "Unknown".

## Mode 06 MIDs (commands.py __mode6__)

| MID | Name | Description |
|---|---|---|
| 00 | MIDS_A | Supported MIDs [01-20] |
| 01 | MONITOR_O2_B1S1 | O2 Sensor Monitor Bank 1 - Sensor 1 |
| 02 | MONITOR_O2_B1S2 | O2 Sensor Monitor Bank 1 - Sensor 2 |
| 03 | MONITOR_O2_B1S3 | O2 Sensor Monitor Bank 1 - Sensor 3 |
| 04 | MONITOR_O2_B1S4 | O2 Sensor Monitor Bank 1 - Sensor 4 |
| 05 | MONITOR_O2_B2S1 | O2 Sensor Monitor Bank 2 - Sensor 1 |
| 06 | MONITOR_O2_B2S2 | O2 Sensor Monitor Bank 2 - Sensor 2 |
| 07 | MONITOR_O2_B2S3 | O2 Sensor Monitor Bank 2 - Sensor 3 |
| 08 | MONITOR_O2_B2S4 | O2 Sensor Monitor Bank 2 - Sensor 4 |
| 09 | MONITOR_O2_B3S1 | O2 Sensor Monitor Bank 3 - Sensor 1 |
| 0A | MONITOR_O2_B3S2 | O2 Sensor Monitor Bank 3 - Sensor 2 |
| 0B | MONITOR_O2_B3S3 | O2 Sensor Monitor Bank 3 - Sensor 3 |
| 0C | MONITOR_O2_B3S4 | O2 Sensor Monitor Bank 3 - Sensor 4 |
| 0D | MONITOR_O2_B4S1 | O2 Sensor Monitor Bank 4 - Sensor 1 |
| 0E | MONITOR_O2_B4S2 | O2 Sensor Monitor Bank 4 - Sensor 2 |
| 0F | MONITOR_O2_B4S3 | O2 Sensor Monitor Bank 4 - Sensor 3 |
| 10 | MONITOR_O2_B4S4 | O2 Sensor Monitor Bank 4 - Sensor 4 |
| 20 | MIDS_B | Supported MIDs [21-40] |
| 21 | MONITOR_CATALYST_B1 | Catalyst Monitor Bank 1 |
| 22 | MONITOR_CATALYST_B2 | Catalyst Monitor Bank 2 |
| 23 | MONITOR_CATALYST_B3 | Catalyst Monitor Bank 3 |
| 24 | MONITOR_CATALYST_B4 | Catalyst Monitor Bank 4 |
| 31 | MONITOR_EGR_B1 | EGR Monitor Bank 1 |
| 32 | MONITOR_EGR_B2 | EGR Monitor Bank 2 |
| 33 | MONITOR_EGR_B3 | EGR Monitor Bank 3 |
| 34 | MONITOR_EGR_B4 | EGR Monitor Bank 4 |
| 35 | MONITOR_VVT_B1 | VVT Monitor Bank 1 |
| 36 | MONITOR_VVT_B2 | VVT Monitor Bank 2 |
| 37 | MONITOR_VVT_B3 | VVT Monitor Bank 3 |
| 38 | MONITOR_VVT_B4 | VVT Monitor Bank 4 |
| 39 | MONITOR_EVAP_150 | EVAP Monitor (Cap Off / 0.150") |
| 3A | MONITOR_EVAP_090 | EVAP Monitor (0.090") |
| 3B | MONITOR_EVAP_040 | EVAP Monitor (0.040") |
| 3C | MONITOR_EVAP_020 | EVAP Monitor (0.020") |
| 3D | MONITOR_PURGE_FLOW | Purge Flow Monitor |
| 40 | MIDS_C | Supported MIDs [41-60] |
| 41 | MONITOR_O2_HEATER_B1S1 | O2 Sensor Heater Monitor Bank 1 - Sensor 1 |
| 42 | MONITOR_O2_HEATER_B1S2 | O2 Sensor Heater Monitor Bank 1 - Sensor 2 |
| 43 | MONITOR_O2_HEATER_B1S3 | O2 Sensor Heater Monitor Bank 1 - Sensor 3 |
| 44 | MONITOR_O2_HEATER_B1S4 | O2 Sensor Heater Monitor Bank 1 - Sensor 4 |
| 45 | MONITOR_O2_HEATER_B2S1 | O2 Sensor Heater Monitor Bank 2 - Sensor 1 |
| 46 | MONITOR_O2_HEATER_B2S2 | O2 Sensor Heater Monitor Bank 2 - Sensor 2 |
| 47 | MONITOR_O2_HEATER_B2S3 | O2 Sensor Heater Monitor Bank 2 - Sensor 3 |
| 48 | MONITOR_O2_HEATER_B2S4 | O2 Sensor Heater Monitor Bank 2 - Sensor 4 |
| 49 | MONITOR_O2_HEATER_B3S1 | O2 Sensor Heater Monitor Bank 3 - Sensor 1 |
| 4A | MONITOR_O2_HEATER_B3S2 | O2 Sensor Heater Monitor Bank 3 - Sensor 2 |
| 4B | MONITOR_O2_HEATER_B3S3 | O2 Sensor Heater Monitor Bank 3 - Sensor 3 |
| 4C | MONITOR_O2_HEATER_B3S4 | O2 Sensor Heater Monitor Bank 3 - Sensor 4 |
| 4D | MONITOR_O2_HEATER_B4S1 | O2 Sensor Heater Monitor Bank 4 - Sensor 1 |
| 4E | MONITOR_O2_HEATER_B4S2 | O2 Sensor Heater Monitor Bank 4 - Sensor 2 |
| 4F | MONITOR_O2_HEATER_B4S3 | O2 Sensor Heater Monitor Bank 4 - Sensor 3 |
| 50 | MONITOR_O2_HEATER_B4S4 | O2 Sensor Heater Monitor Bank 4 - Sensor 4 |
| 60 | MIDS_D | Supported MIDs [61-80] |
| 61 | MONITOR_HEATED_CATALYST_B1 | Heated Catalyst Monitor Bank 1 |
| 62 | MONITOR_HEATED_CATALYST_B2 | Heated Catalyst Monitor Bank 2 |
| 63 | MONITOR_HEATED_CATALYST_B3 | Heated Catalyst Monitor Bank 3 |
| 64 | MONITOR_HEATED_CATALYST_B4 | Heated Catalyst Monitor Bank 4 |
| 71 | MONITOR_SECONDARY_AIR_1 | Secondary Air Monitor 1 |
| 72 | MONITOR_SECONDARY_AIR_2 | Secondary Air Monitor 2 |
| 73 | MONITOR_SECONDARY_AIR_3 | Secondary Air Monitor 3 |
| 74 | MONITOR_SECONDARY_AIR_4 | Secondary Air Monitor 4 |
| 80 | MIDS_E | Supported MIDs [81-A0] |
| 81 | MONITOR_FUEL_SYSTEM_B1 | Fuel System Monitor Bank 1 |
| 82 | MONITOR_FUEL_SYSTEM_B2 | Fuel System Monitor Bank 2 |
| 83 | MONITOR_FUEL_SYSTEM_B3 | Fuel System Monitor Bank 3 |
| 84 | MONITOR_FUEL_SYSTEM_B4 | Fuel System Monitor Bank 4 |
| 85 | MONITOR_BOOST_PRESSURE_B1 | Boost Pressure Control Monitor Bank 1 |
| 86 | MONITOR_BOOST_PRESSURE_B2 | Boost Pressure Control Monitor Bank 1 |
| 90 | MONITOR_NOX_ABSORBER_B1 | NOx Absorber Monitor Bank 1 |
| 91 | MONITOR_NOX_ABSORBER_B2 | NOx Absorber Monitor Bank 2 |
| 98 | MONITOR_NOX_CATALYST_B1 | NOx Catalyst Monitor Bank 1 |
| 99 | MONITOR_NOX_CATALYST_B2 | NOx Catalyst Monitor Bank 2 |
| A0 | MIDS_F | Supported MIDs [A1-C0] |
| A1 | MONITOR_MISFIRE_GENERAL | Misfire Monitor General Data |
| A2 | MONITOR_MISFIRE_CYLINDER_1 | Misfire Cylinder 1 Data |
| A3 | MONITOR_MISFIRE_CYLINDER_2 | Misfire Cylinder 2 Data |
| A4 | MONITOR_MISFIRE_CYLINDER_3 | Misfire Cylinder 3 Data |
| A5 | MONITOR_MISFIRE_CYLINDER_4 | Misfire Cylinder 4 Data |
| A6 | MONITOR_MISFIRE_CYLINDER_5 | Misfire Cylinder 5 Data |
| A7 | MONITOR_MISFIRE_CYLINDER_6 | Misfire Cylinder 6 Data |
| A8 | MONITOR_MISFIRE_CYLINDER_7 | Misfire Cylinder 7 Data |
| A9 | MONITOR_MISFIRE_CYLINDER_8 | Misfire Cylinder 8 Data |
| AA | MONITOR_MISFIRE_CYLINDER_9 | Misfire Cylinder 9 Data |
| AB | MONITOR_MISFIRE_CYLINDER_10 | Misfire Cylinder 10 Data |
| AC | MONITOR_MISFIRE_CYLINDER_11 | Misfire Cylinder 11 Data |
| AD | MONITOR_MISFIRE_CYLINDER_12 | Misfire Cylinder 12 Data |
| B0 | MONITOR_PM_FILTER_B1 | PM Filter Monitor Bank 1 |
| B1 | MONITOR_PM_FILTER_B2 | PM Filter Monitor Bank 2 |

(The description of 0x86 says "Bank 1" in python-OBD; by position it is bank 2.)

## Unit and Scaling IDs (UnitsAndScaling.py UAS_IDS)

| UASID | Signed | Scale per bit | Offset | Unit |
|---|---|---|---|---|
| 01 | no | 1 | 0 | count |
| 02 | no | 0.1 | 0 | count |
| 03 | no | 0.01 | 0 | count |
| 04 | no | 0.001 | 0 | count |
| 05 | no | 0.0000305 | 0 | count |
| 06 | no | 0.000305 | 0 | count |
| 07 | no | 0.25 | 0 | rpm |
| 08 | no | 0.01 | 0 | kph |
| 09 | no | 1 | 0 | kph |
| 0A | no | 0.122 | 0 | millivolt |
| 0B | no | 0.001 | 0 | volt |
| 0C | no | 0.01 | 0 | volt |
| 0D | no | 0.00390625 | 0 | milliampere |
| 0E | no | 0.001 | 0 | ampere |
| 0F | no | 0.01 | 0 | ampere |
| 10 | no | 1 | 0 | millisecond |
| 11 | no | 100 | 0 | millisecond |
| 12 | no | 1 | 0 | second |
| 13 | no | 1 | 0 | milliohm |
| 14 | no | 1 | 0 | ohm |
| 15 | no | 1 | 0 | kiloohm |
| 16 | no | 0.1 | -40.0 | celsius |
| 17 | no | 0.01 | 0 | kilopascal |
| 18 | no | 0.0117 | 0 | kilopascal |
| 19 | no | 0.079 | 0 | kilopascal |
| 1A | no | 1 | 0 | kilopascal |
| 1B | no | 10 | 0 | kilopascal |
| 1C | no | 0.01 | 0 | degree |
| 1D | no | 0.5 | 0 | degree |
| 1E | no | 0.0000305 | 0 | ratio |
| 1F | no | 0.05 | 0 | ratio |
| 20 | no | 0.00390625 | 0 | ratio |
| 21 | no | 1 | 0 | millihertz |
| 22 | no | 1 | 0 | hertz |
| 23 | no | 1 | 0 | kilohertz |
| 24 | no | 1 | 0 | count |
| 25 | no | 1 | 0 | kilometer |
| 26 | no | 0.1 | 0 | millivolt / millisecond |
| 27 | no | 0.01 | 0 | grams_per_second |
| 28 | no | 1 | 0 | grams_per_second |
| 29 | no | 0.25 | 0 | pascal / second |
| 2A | no | 0.001 | 0 | kilogram / hour |
| 2B | no | 1 | 0 | count |
| 2C | no | 0.01 | 0 | gram |
| 2D | no | 0.01 | 0 | milligram |
| 2E | no | — | — | boolean: true if any byte non-zero |
| 2F | no | 0.01 | 0 | percent |
| 30 | no | 0.001526 | 0 | percent |
| 31 | no | 0.001 | 0 | liter |
| 32 | no | 0.0000305 | 0 | inch |
| 33 | no | 0.00024414 | 0 | ratio |
| 34 | no | 1 | 0 | minute |
| 35 | no | 10 | 0 | millisecond |
| 36 | no | 0.01 | 0 | gram |
| 37 | no | 0.1 | 0 | gram |
| 38 | no | 1 | 0 | gram |
| 39 | no | 0.01 | -327.68 | percent |
| 3A | no | 0.001 | 0 | gram |
| 3B | no | 0.0001 | 0 | gram |
| 3C | no | 0.1 | 0 | microsecond |
| 3D | no | 0.01 | 0 | milliampere |
| 3E | no | 0.00006103516 | 0 | millimeter ** 2 |
| 3F | no | 0.01 | 0 | liter |
| 40 | no | 1 | 0 | ppm |
| 41 | no | 0.01 | 0 | microampere |
| 81 | yes | 1 | 0 | count |
| 82 | yes | 0.1 | 0 | count |
| 83 | yes | 0.01 | 0 | count |
| 84 | yes | 0.001 | 0 | count |
| 85 | yes | 0.0000305 | 0 | count |
| 86 | yes | 0.000305 | 0 | count |
| 87 | yes | 1 | 0 | ppm |
| 8A | yes | 0.122 | 0 | millivolt |
| 8B | yes | 0.001 | 0 | volt |
| 8C | yes | 0.01 | 0 | volt |
| 8D | yes | 0.00390625 | 0 | milliampere |
| 8E | yes | 0.001 | 0 | ampere |
| 90 | yes | 1 | 0 | millisecond |
| 96 | yes | 0.1 | 0 | celsius |
| 99 | yes | 0.1 | 0 | kilopascal |
| 9C | yes | 0.01 | 0 | degree |
| 9D | yes | 0.5 | 0 | degree |
| A8 | yes | 1 | 0 | grams_per_second |
| A9 | yes | 0.25 | 0 | pascal / second |
| AD | yes | 0.01 | 0 | milligram |
| AE | yes | 0.1 | 0 | milligram |
| AF | yes | 0.01 | 0 | percent |
| B0 | yes | 0.003052 | 0 | percent |
| B1 | yes | 2 | 0 | millivolt / second |
| FC | yes | 0.01 | 0 | kilopascal |
| FD | yes | 0.001 | 0 | kilopascal |
| FE | yes | 0.25 | 0 | pascal |

Comments in the source mark 0x2C as "per-cylinder" (g/cylinder) and 0x2D/0xAD/0xAE as "per-stroke"
(mg/stroke). IDs absent from this table (e.g. 0x42–0x7F, 0x88, 0x89, 0x8F, 0x91–0x95, 0x97, 0x98,
0x9A, 0x9B, 0x9E–0xA7, 0xAA–0xAC, 0xB2–0xFB) are treated as unknown.
