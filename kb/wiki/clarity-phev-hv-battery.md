---
title: 2018 Clarity Plug-in Hybrid — HV battery and meter DIDs (service 22, 29-bit)
type: concept
created: 2026-11-13
updated: 2026-11-13
sources:
  - raw/obdb-honda-clarity-2026-11-13.md
  - raw/honda/clarity-phev-2018-sb-17-093-pdi.md
  - raw/honda/clarity-phev-sm-p144a-battery-system.md
tags: [clarity, phev, honda, bms, uds, service22, can29]
confidence: community
---

# 2018 Clarity Plug-in Hybrid — HV battery and meter DIDs

What carlink reads beyond generic OBD on the [[clarity-phev-2018]], and how. Every offset here comes
from OBDb's Honda-Clarity signal set and was checked against OBDb's own expected values for its real
2018 captures (`raw/obdb-honda-clarity-2026-11-13.md`: 0 mismatches over 17,256 cell values and 7
odometer readings). Nothing here has been seen on Aaron's car yet (DECISIONS O1). confidence: community.

## Addressing

29-bit physical requests, ISO 15765-4 at 500 kbit/s, tester address F1: request header `18DAxxF1`,
reply `18DAF1xx`. carlink sends `ATSH18DAxxF1`, `ATCRA18DAF1xx`, `ATFCSH18DAxxF1`, `ATFCSD300000`,
`ATFCSM1`, the request, then `ATFCSM0`, `ATAR`, `ATSH18DB33F1` to return to functional addressing (all
volatile, documented in the OBDLink FRPM: `ATSH` 4-byte 29-bit form, `ATCRA`, `ATFCSH`; see
[[adapter-response-format]], [[adapter-safety]]). Only read service 22 is sent.

| ECU | Request header | What carlink reads | OBDb name for the ECU |
|---|---|---|---|
| `01` | `18DA01F1` | `22 202A` pack summary, `22 2028` cell voltages string A | main BMS ("line A") |
| `16` | `18DA16F1` | `22 2028` cell voltages string B | second BMS unit ("line b") |
| `60` | `18DA60F1` | `22 7022` odometer | meter (gauge cluster) |

## Byte maps

Offsets are bytes after the 3-byte `62 xx xx` reply header (OBDb's `bix` ÷ 8). Big-endian.

### `22 202A` from `01` (reply 246 bytes)

| Byte | Size | Formula | carlink key | Meaning |
|---|---|---|---|---|
| 50 | u8 | % | `hv_soc` | battery state of charge as the BMS keeps it (not the dashboard bars) |
| 57 | s16 | ÷ 50 A | `pack_current` | pack current, + = discharging (OBDb "total current") |
| 61 | u16 | ÷ 20 V | `pack_voltage` | pack voltage |
| 73 | u16 | ÷ 100 % | `cell_soc_max` | highest cell state of charge |
| 75 | u16 | ÷ 100 % | `cell_soc_min` | lowest cell state of charge |
| 167 | u16 | ÷ 100 Ah | `hv_capacity_ah` | battery capacity as the BMS estimates it |

OBDb also maps cabin (byte 104) and outside (byte 105) air temperature; carlink does not use them.

### `22 202A` from `16` — deliberately not read

Same layout (byte 61 voltage ÷ 20 V, byte 167 capacity ÷ 100 Ah). carlink does not read it: its
capacity is about half the pack's (one string), and in a weak-model trial an agent compared that half
figure with Honda's 36.6 Ah pack line and nearly told an owner the battery qualified for replacement
(DECISIONS T1). Honda's "Battery Pack Capacity" comes from battery condition monitor module A, i.e. `01`
(S/B 17-093).

### `22 2028` from `01` and from `16` (cells)

84 cells per reply, from byte 27, 2 bytes each, value ÷ 5 = mV (`cell_v_a01`…`cell_v_a84`,
`cell_v_b01`…`cell_v_b84`). OBDb names only 72 cells for `01`, but every one of its 116 captures
carries 84 non-zero values there, then zeros, exactly like `16`. The 84 cells of one reply sum to the
pack voltage (e.g. 318.5 V against 318.2 V from `202A`), so each reply is one series string of 84
cells; the pack is two such strings (168 cells).

### `22 7022` from `60` (meter)

Bytes 6–8, u24 = odometer in **miles** (OBDb `CLARITY_ODO_V3`, captures 90,033–103,943 mi).

## Interpreting the capacity

- `01` reports about **twice** what `16` reports in the same file (40.2 Ah vs 20.5 Ah; 55.0 Ah vs
  27.5 Ah): `01` is the whole pack (two strings in parallel; Honda's battery-system page shows packs A
  and B in parallel) and `16` one string. The 55.0 Ah new value matches Honda's i-HDS screenshot in
  S/B 17-093 ("received by Battery Condition Monitor Module A"), so `hv_capacity_ah` is that figure.
- The captures cluster at two values: **55.0 Ah** (95 captures of `01`; 27.5 Ah in 58 of `16`) and
  **≈ 40 Ah** (48 captures). 17 kWh ÷ ≈ 311 V (84 × 3.7 V) ≈ 55 Ah, so 55.0 Ah is consistent with a
  new pack — but a value that never varies across 95 captures could also be a default the BMS reports
  before it has learned the capacity (one owner reports exactly that after a code clear;
  [[clarity-phev-2018]] "High-voltage battery health" has the warranty line and owner readings).

## Cell spread

`cell_delta_v` is max − min **within** each string, the worse of the two (cells in one string carry the
same current; the two strings are read a moment apart). Across OBDb's 2018 captures (all at unknown
SOC and load) the spread within a string was 3–46 mV, median ≈ 10 mV (Python over every capture,
2026-11-13). carlink computes it only when all 168 cells read 2.5–4.3 V.
