---
title: carlink CLI
type: entity
created: 2026-11-07
updated: 2026-11-07
sources:
  - code:~/carlink
tags: [carlink, tooling]
confidence: verified
---

# carlink CLI

`carlink` (`~/carlink`, Rust) is the program agents use to talk to the [[obdlink-ex]]. It prints
JSON only; the `carlink` skill (`~/carlink/skills/carlink/SKILL.md`) tells an agent how to run and
interpret it. This page maps what the program does onto the rest of this wiki.

| Command | What it does | Wiki basis |
|---|---|---|
| `ports` / `probe` | Finds the FTDI `0403:6015` device at 115200 baud, runs the init sequence, reports adapter, protocol, voltage, VIN | [[macos-serial-setup]], [[elm327-at-commands]], [[stn-st-commands]] |
| `scan` | Supported-PID sweep, monitor status, services 03/07/0A, freeze frame, VIN, live snapshot, vehicle-specific reads, findings; saved to SQLite | [[obd2-services]], [[obd2-pids]], [[readiness-monitors]], [[dtc-format]], [[diagnostic-thresholds]] |
| `clear` | Service 04 after a saved pre-clear scan; needs `--confirm-clear-codes`; refuses with RPM > 0 | [[clearing-codes]] |
| `dtc` | SAE generic description (OBDex, CC0) plus vehicle note | [[dtc-format]] |
| `resets` | Per-vehicle reset/relearn procedures; only code clearing is done by carlink | [[kona-ev-resets]], [[honda-maintenance-minder]], [[odyssey-2016-tpms-calibration]], [[honda-idle-relearn]] |
| `raw` | One read-only request (services 01 02 03 06 07 09 0A 22), optional 11-bit header | [[adapter-safety]] |
| `history` | sessions / show / trend / keys / dtcs over the SQLite store (`~/.local/share/carlink/carlink.db`) | — |

## Parsing

Session init: `ATZ ATE0 ATL0 ATS1 ATH1 ATCAF1 ATAT1 ATSP0`, then `0100` triggers the protocol search
(30 s host timeout), `ATDPN` picks the bus. With headers on, CAN frames are grouped by ID and ISO-TP
reassembled; K-line frames drop the 3 header bytes and checksum ([[adapter-response-format]]).
Physical requests (`with_header`) set `ATSH<id>` and `ATCRA<id+8>`, then restore `ATCRA` / `ATSH7DF`.

## Vehicle-specific

- [[kona-electric-2020]]: BMS 220101–220105 at 7E4, VMCU 2102 at 7E2, TPMS 22C00B at 7A0, odometer
  22B002 at 7C6 ([[kona-ev-bms-pids]], [[kona-ev-other-ecus]]). Decoder tested against the real
  OBDb 2020 capture. Findings from [[kona-ev-battery-health]].
- [[crv-2004]], [[odyssey-2016]]: generic OBD-II plus known-code notes and recalls.

## Not yet verified on the real cars

Everything above runs against the built-in simulator; the first real scans should confirm protocol,
which ECUs answer, and (Kona) whether default flow control works without `ATFCSM1`.
