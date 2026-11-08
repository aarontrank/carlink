---
title: Clearing codes (service 04) — what it erases and the consequences
type: concept
created: 2026-11-07
updated: 2026-11-07
sources:
  - url:https://x-engineer.org/obd-diagnostic-service-mode-04-clear-reset-emission-related-diagnostic-information/
  - url:https://en.wikipedia.org/wiki/OBD-II_PIDs
  - url:https://dep.nj.gov/stopthesoot/obd-ii-information-and-assistance/
  - url:https://ricksfreeautorepairadvice.com/obdii-scan-tool-modes/
tags: [obd2, service04, dtc, readiness, safety]
confidence: community
---

# Clearing codes (service 04)

Request: `04`. Positive response: `44`. It is sent to all ECUs (functional `7DF` on CAN).
Every emissions ECU should accept it with **ignition on, engine off**. An ECU may refuse while
the engine runs with `7F 04 22` (conditionsNotCorrect).

## What it erases (J1979, via x-engineer's table — `community`)

- MIL status and the confirmed-DTC count (01 PID 01, byte A)
- Confirmed DTCs (service 03) and **pending** DTCs (service 07)
- The freeze-frame DTC (02 PID 02) and all freeze-frame data (service 02)
- **Readiness monitor status**: every supported non-continuous monitor goes back to
  "incomplete" (01 PID 01 / 41)
- O2 sensor test results (service 05) and **on-board monitor test results** (service 06,
  including misfire counters)
- Counters: distance with MIL on (PID 21), warm-ups since clear (30), distance since clear (31),
  time with MIL on (4D), time since clear (4E). All reset to 0.

## What it does NOT erase

- **Permanent DTCs (service 0A).** Only the ECU erases these, after the relevant monitor runs
  and passes. Battery disconnect does not erase them either. They exist so a clear just before
  inspection cannot hide a fault.
- The fault itself. An unrepaired fault returns once the monitor runs again.
- Generally not the learned adaptations (e.g. long-term fuel trim tables, idle learn), though
  this varies by manufacturer (`unverified`).

## Consequences to warn the user about

1. **The car is "not ready" for an emissions inspection** until the drive cycles complete. With
   more than 1 (MY2001+) or 2 (MY1996–2000) non-continuous monitors incomplete, an I/M station
   rejects it. See [[readiness-monitors]]. Completing them can take a few days of mixed driving.
   EVAP and catalyst are usually last.
2. **The diagnostic evidence is gone**: freeze frame, pending codes and mode 06 results. Always
   read and save a full scan (03, 07, 0A, freeze frame, PID 01, mode 06) *before* clearing.
3. A permanent code still shows in service 0A, and inspection programs read it. A clear does
   not make a car with an unrepaired fault pass.
4. If the MIL comes straight back, the fault is current. Clearing again achieves nothing.

carlink should treat `clear` as the one destructive command: confirm interactively, refuse with
the engine running (RPM > 0), and auto-save a pre-clear scan.

## Related
[[dtc-format]] · [[readiness-monitors]] · [[obd2-services]]
