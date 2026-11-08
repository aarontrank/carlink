---
title: STN (OBDLink) ST commands
type: concept
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/obdlink-frpm-rev-f.md
tags: [stn, st-commands, obdlink, adapter]
confidence: verified
---

# STN (OBDLink) ST commands

OBDLink's parallel command set, available alongside AT commands on the [[obdlink-ex]]. All
facts from the OBDLink FRPM rev F §8 unless noted. ELM327 clones do **not** implement these —
`STI` returning `?` is a good clone detector.

## Identity (read-only, safe)

| Command | Output example | Note |
|---------|----------------|------|
| `STI` | `STN2232 v5.10.3` | firmware ID (§8.4) |
| `STIX` | `STN2232 v5.10.3 [2024.02.01]` | + release date |
| `STDI` | `OBDLink EX r1.0.0` | hardware ID; pre-v5.6.19 firmware used `rX.Y` |
| `STDIX` | multi-line `Device:`, `Firmware:`, `Mfr:`, `Serial #:`, `BL Ver:`, `POR Count:`, … | format may change between firmware versions |
| `STSN` | `223010296101` | 12-digit factory serial, starts with device ID |
| `STMFR` | `OBD Solutions LLC` | |
| `STVR [n]` | `12.34` | DLC voltage, default 2 decimals; `--.--` if > 65.534 V (§8.5) |
| `STPR` / `STPRS` | protocol number / string | replaces `ATDPN` / `ATDP` (§8.6) |
| `STPBRR` | `500000` | actual bus bit rate; `?` while protocol is AUTO |
| `ATPPS` | PP summary table | print only; safe |

## Protocol control (volatile)

| Command | Effect |
|---------|--------|
| `STP p` | Select protocol preset (does not open it). Returns `?` if unsupported. Presets: 11 J1850 PWM, 12 J1850 VPW, 21 ISO 9141 no-init, **22 ISO 9141-2 5-baud**, 23 KWP no-init, 24 KWP 5-baud, 25 KWP fast, 31/32 raw CAN 500k 11/29, **33/34 ISO 15765 500k 11/29**, 35/36 ISO 15765 250k, 41/42 J1939, 51–54 MS-CAN 125k, 61–64 SW-CAN 33.3k (not on EX). Presets without autoinit send no keep-alives. |
| `STPO` / `STPC` | Open / close the current protocol |
| `STPTO ms` | Request timeout in ms (1–65535; 0 = infinite). Default 102 ms (PP 03). First frame always waits the full timeout; ATAT governs later frames. |
| `STPTOT ms` | Bus-access timeout before `BUS BUSY`: 300 ms ISO/J1850, 50 ms CAN |
| `STPTRQ ms` | Min time between last response and next request (ISO P3), default 56 ms ISO, 0 others |
| `STCTOR fc,cf` | ISO-TP flow-control / consecutive-frame RX timeouts, default 75 / 150 ms |
| `STITW …` | 5-baud init timings, defaults R1 301 (sync byte), R2/R3 21 (key bytes), T4 30, R4 51, T5 305 ms |
| `STIP1X ms` / `STIP4 ms` | ISO P1max (default 20 ms) / P4 (default 5 ms) |
| `STCSEGR 1` | Reassemble received ISO-TP multi-frame messages into one line (default 0) |
| `STCFCPA tx,rx` | Add flow-control address pair; without pairs, 11-bit FC goes to RxID − 8 (7E8 → 7E0) (§12) |
| `STCAF f[,tt]` | CAN addressing: 0 normal, 1 extended, 2 mixed |

## Raw send

`STPX h:<hdr>, d:<data>, t:<ms>, r:<count>` sends one message with per-call header, timeout
and expected response count, e.g. `STPX h:7E0, d:0100, t:100, r:1`. `r` lets the adapter
return as soon as N responses arrive instead of waiting out the timeout. `l:<len>` mode
prompts `DATA>` for long payloads (max 4 KB on EX). (§8.6)

## Monitoring and filters

`STMA [n]` monitors all traffic (CAN treated as ISO 15765); `STM [n]` uses current filters.
Stop by sending any single character; wait for `STOPPED` then `>` (§8.9). Monitoring defaults
to **silent** (no CAN ACKs; `STCMM 0`). Filters: `STFPA` pass, `STFBA` block, `STFFCA` flow
control, `STFAC` clear all; each allocates RAM and can fail with `OUT OF MEMORY` (§8.10).
For read-only diagnosis, never set `STCMM 1` unless the adapter is one of only two nodes.

## Commands that write NVM or are one-time — see [[adapter-safety]]

`STWBR`, `STSBR`/`STBR` (+`STWBR`), `STRSTNVM`, `STSATI`, `STS@1`, `STSDI`, `STSAVCAL`,
`STVCAL`, all `STSL*` PowerSave setters, `STBT*` (Bluetooth; n/a on EX), plus AT `ATPP`,
`ATSD`, `AT@3`, `ATSP n`.

Related: [[elm327-at-commands]] · [[adapter-response-format]]
