---
title: carlink CLI
type: entity
created: 2026-11-07
updated: 2026-11-13
sources:
  - code:carlink
tags: [carlink, tooling]
confidence: verified
---

# carlink CLI

`carlink` (this repo, Rust) is the program agents use to talk to the [[obdlink-ex]]. It prints one
JSON object per command; the `carlink` skill (`skills/carlink/SKILL.md`) tells an agent how to run and
interpret it. This page maps what the program does onto the rest of this wiki. Decisions behind it
are in the repo's `DECISIONS.md`.

| Command | What it does | Wiki basis |
|---|---|---|
| `ports`, `probe` | Finds the adapter (product string "OBDLink" first, then FTDI `0403:6015`), checks identity (`ATI`) before anything else, power at the OBD port, the car, the VIN | [[macos-serial-setup]], [[elm327-at-commands]], [[stn-st-commands]] |
| `scan` | Supported PIDs, monitors, services 03/07/0A (complete only if every CAN ECU answered 03 — a non-CAN ECU may stay silent only when its PID 01 counts zero confirmed codes — and none counts more confirmed codes than were read), freeze frame, VIN, live snapshot incl. O2/wideband/catalyst PIDs, service 06 (CAN), vehicle-specific reads, module codes, findings, inspection outlook; saved per VIN | [[obd2-services]], [[obd2-pids]], [[obd2-pids-extended]], [[readiness-monitors]], [[dtc-format]], [[diagnostic-thresholds]], [[obd2-mode06]] |
| `tests` | Service 06: CAN standardized MIDs with UASID scaling, per-cylinder misfire counts (A1–AD, TID 0B/0C); K-line raw tests with Honda's CR-V labels | [[obd2-mode06]], [[honda-mode06]] |
| `watch` | Samples PIDs (or the Kona BMS 220101) repeatedly; min/max/avg | [[live-data-interpretation]] |
| `clear` | Engine/emissions (service 04, functional) by default; modules (UDS `14 FF FF FF` / KWP `14 FF 00`, physical) only when named. Needs `--confirm-clear-codes` and a VIN; refuses on identity conflict, running engine, incomplete code read; Kona: never 04, never BMS/SRS/SMK. Outcome only from a fresh scan | [[clearing-codes]], [[uds-module-dtcs]], [[kona-ev-modules]] |
| `dtc` | Validated code → SAE description (OBDex, CC0), J2012 range, vehicle note | [[dtc-format]] |
| `resets` | Per-vehicle procedures; only code clearing is done by carlink | [[kona-ev-resets]], [[honda-maintenance-minder]], [[odyssey-2016-tpms-calibration]], [[honda-idle-relearn]] |
| `raw` | One read-only request (01 02 03 06 07 09 0A 18 19 21 22), optional 11-bit header | [[adapter-safety]] |
| `history` | vehicles / sessions / show / trend / keys / dtcs / compare over SQLite (`~/.local/share/carlink/carlink.db`; simulator runs go to `simulator.db`) | — |

## Protocol handling

- Session setup: `ATI` (identity) → `ATZ` (banner checked) → `ATE0 ATL0 ATS1 ATH1 ATCAF1 ATAT1 ATSP0`,
  each answer must be `OK`; `0100` must contain `41 00`; `ATDPN` parsed strictly ([[elm327-at-commands]]).
- Replies: CAN frames grouped by ID, ISO-TP sequence-checked (a lost frame is an error, never shorter
  data); K-line drops the 3 header bytes and checksum ([[adapter-response-format]]). A reply of the
  wrong shape (adapter rebooted, headers off) triggers re-initialisation and one retry.
- Timing: the deadline restarts on every received byte (response pending, K-line init); after a
  timeout the adapter is interrupted with a space and drained; a bare CR is never sent.
- A lost link (USB disconnect, or two timeouts in a row, e.g. a request and its retry) stops all traffic: the
  command fails with `adapter_disconnected` / `adapter_timeout` instead of returning a partial scan.
- Physical addressing (`with_header`): 11-bit `ATSH<id> ATCRA<id+8> ATFCSH<id> ATFCSD300000 ATFCSM1`,
  restored with `ATFCSM0 ATAR ATSH7DF`; 29-bit `ATSH18DAxxF1 ATCRA18DAF1xx ATFCSH18DAxxF1 ATFCSD300000
  ATFCSM1`, restored with `ATFCSM0 ATAR ATSH18DB33F1`. A header of the other size is refused before
  anything is sent; a failed restore blocks every later request until repaired.
- Engine ECU (bare history keys, findings): `7E8`, `18DAF110`, `18DAF111`, the Clarity's `18DAF10E`, or
  K-line `10`.
- Before a clear the engine must be off: every module that reports rpm must report 0.

## Vehicle-specific

- [[kona-electric-2020]]: BMS 220101–220105 at 7E4 (cells checked against the BMS summary before any
  spread is reported), VMCU 2102 at 7E2, TPMS 22C00B at 7A0, odometer 22B002 at 7C6
  ([[kona-ev-bms-pids]], [[kona-ev-other-ecus]]); module codes from 14 modules ([[kona-ev-modules]]).
- [[clarity-phev-2018]]: BMS 22 202A and 2028 at `18DA01F1` and `18DA16F1` (capacity, SOC, 168 cells),
  odometer 22 7022 at `18DA60F1` ([[clarity-phev-hv-battery]]); capacity judged against Honda's 36.6 Ah
  warranty line; no clear while P0A7F, P1DC7 or P144A is stored.
- [[crv-2004]], [[odyssey-2016]]: generic OBD-II, service 06, known-code notes, recalls; no modules
  beyond the PCM are reachable ([[crv-2004-modules]], [[odyssey-2016-modules]]).

## Not yet verified on the real cars

Everything above runs against the built-in simulator, which models adapter faults and the module
behaviours observed in third-party captures. `--trace <file>` records every request and raw reply, and
the repository's `docs/first-real-session.md` is the per-car checklist. The first real sessions should confirm, per car:
protocol (Odyssey 11- or 29-bit), responding ECUs (Clarity: seven expected), whether the explicit flow control and `ATAR`
restore behave as documented, and the Kona rear TPMS order.
