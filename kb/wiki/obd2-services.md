---
title: OBD-II services (SAE J1979 modes 01–0A)
type: concept
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/wikipedia-obd2-pids-2026-11-07.wikitext
  - url:https://en.wikipedia.org/wiki/OBD-II_PIDs
  - url:https://x-engineer.org/obd-diagnostic-service-mode-04-clear-reset-emission-related-diagnostic-information/
  - url:http://m0agx.eu/2018/01/02/reading-obd2-data-without-elm327-part-2-k-line/
  - url:https://ramn.readthedocs.io/en/latest/userguide/diag_tutorial.html
tags: [obd2, j1979, protocol]
confidence: community
---

# OBD-II services (SAE J1979)

SAE J1979 (= ISO 15031-5) defines ten emissions diagnostic services. Before 2002 they were
called "modes". The standard is paywalled; everything here comes from Wikipedia's "OBD-II
PIDs" page (saved as `raw/wikipedia-obd2-pids-2026-11-07.wikitext`) and cross-checked against
secondary sources. Confidence is `community` unless marked otherwise.

## Request/response framing

- A request is `<service> [<PID> ...]`. The positive response echoes **service + 0x40**, then
  the PID, then the data bytes: `01 0C` → `41 0C 1A F8`.
- A negative response is `7F <service> <NRC>`. Example: `7F 04 22` means conditionsNotCorrect,
  which is how an ECU refuses a clear while the engine is running.
- On CAN (ISO 15765-4), the tester sends to functional ID `7DF`. ECUs answer from `7E8`–`7EF`
  (physical request ID + 8). Engine/PCM is usually `7E8`. Several ECUs can answer one request,
  so **group every response by header**. Payloads over 7 bytes use ISO-TP multi-frame
  messages (first frame `10 LL`, consecutive frames `21`, `22`, …). The ELM327 reassembles
  these when headers are off.

## The services

| Svc | Name | Request | Positive response | Changes ECU state? |
|-----|------|---------|-------------------|--------------------|
| 01 | Current powertrain data | `01 PID` (CAN allows up to 6 PIDs) | `41 PID data…` | No |
| 02 | Freeze-frame data | `02 PID FRAME` (frame normally `00`) | `42 PID FRAME data…` | No |
| 03 | Confirmed (stored) DTCs | `03` | `43 …` — see [[dtc-format]] | No |
| 04 | Clear/reset emissions diagnostic info | `04` | `44` (no data) | **Yes — destructive**, see [[clearing-codes]] |
| 05 | O2 sensor monitoring test results | `05 TID O2S` | `45 …` | No. **Non-CAN only.** On CAN this data moved into service 06. |
| 06 | On-board monitoring test results | `06 MID` | `46 MID [TID UASID val min max]…` | No |
| 07 | Pending DTCs (current or last drive cycle) | `07` | `47 …` (same format as 03) | No |
| 08 | Control on-board system/test/component | `08 TID data` | `48 …` | **Yes — actuates hardware** (e.g. seals the EVAP system for a leak test) |
| 09 | Vehicle information | `09 PID` | `49 PID …` | No |
| 0A | Permanent DTCs | `0A` | `4A …` (same format as 03) | No |

Read-only: 01, 02, 03, 05, 06, 07, 09, 0A. State-changing: **04** (clears data) and **08**
(actuates components). A read-only tool should not send 08 at all, and should send 04 only
after the user explicitly confirms.

Manufacturers add their own services above these: `22` (read data by identifier, from SAE
J2190/UDS) on Ford/GM/Hyundai, and `21` on Toyota/Honda. Those are not part of J1979.

## Service 01 / 02

PID formulas are in [[obd2-pids]]. Service 02 takes the same PIDs and meanings as 01, but
returns the snapshot stored when a DTC set. Rules:

- The request carries a **frame number** after the PID: `02 0C 00`. Frame `00` is the
  OBD-required frame. Most ECUs store only that one.
- `02 02 00` returns the DTC that caused the freeze frame, decoded as in [[dtc-format]]. If it
  is `00 00`, there is no freeze frame and the rest of the service 02 data is meaningless.
- PID `01` (monitor status) exists only in service 01. PID `02` exists only in service 02.
- `02 00 00` returns the supported-PID bitmap *for the freeze frame*.

## Service 06 — worth implementing?

On CAN vehicles each record is 9 bytes. The **MID** says which monitor and component
(standardised, e.g. `$21` = catalyst bank 1; `$A2`–`$AD` = misfire cylinders 1–12). Then:
TID (manufacturer-defined test), Unit-and-Scaling ID, 16-bit test value, 16-bit min limit,
16-bit max limit. A test passes when min ≤ value ≤ max. MIDs `00/20/40/…` are support bitmaps.
Pre-CAN mode 06 was manufacturer-specific (TID/CID) and not portable.

For carlink, mode 06 is a **phase-2 feature**:

- Raw pass/fail is generic and cheap to show.
- Misfire counts (MID `A2+`, TID `0B` = average of recent drive cycles, `0C` = this drive cycle)
  are the most useful part.
- Meaningful labels and units need the J1979 Unit-and-Scaling table. TIDs are
  manufacturer-defined, and makers publish corrections (GM, for example, says some values must
  be divided by 1000). (`unverified` on the exact byte layout: from model knowledge plus
  secondary sources, not the J1979 text.)

## Service 09 — vehicle information

| PID | Content | Format |
|-----|---------|--------|
| 00 | Supported PIDs 01–20 | bitmap, like 01 00 |
| 01 | VIN message count (non-CAN only) | usually 5 |
| 02 | **VIN** | 17 ASCII chars |
| 03 | CALID message count (non-CAN only) | multiple of 4 |
| 04 | **Calibration ID(s)** | 16 ASCII bytes each, null-padded; may contain several |
| 06 | Calibration verification numbers | 4 bytes each, shown as hex |
| 08 | In-use performance tracking (spark) | 16-bit counters |
| 0A | **ECU name** | 20 ASCII bytes, null-padded on the right |

**VIN on CAN:** one ISO-TP message, `49 02 01` + 17 ASCII bytes (20 bytes total). The `01` is
the *number of data items*, not a sequence number. Raw example:
`7E8 10 14 49 02 01 57 44 42` / `7E8 21 52 46 36 31 4A 37 36` / `7E8 22 41 30 31 32 33 34 35`.

**VIN on ISO 9141-2 / KWP2000 / J1850:** five separate messages, each
`49 02 <seq> b1 b2 b3 b4`, where `seq` = 01…05 **is** a frame sequence number. The VIN is
left-padded with three `00` bytes to fill 20 bytes: `49 02 01 00 00 00 31`,
`49 02 02 48 47 43 4D`, …. Parse it by dropping `49 02 nn`, concatenating in `nn` order and
stripping the leading NULs.

Strip NULs from CALID and ECU name too. **A robust parser handles both layouts by detecting the
protocol rather than guessing from the bytes.** (The 2004 CR-V is pre-CAN; the 2016 Odyssey
and 2020 Kona are CAN.)

## Related
[[obd2-pids]] · [[dtc-format]] · [[clearing-codes]] · [[readiness-monitors]] · [[diagnostic-thresholds]]
