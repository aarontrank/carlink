---
title: Overview
type: overview
created: 2026-11-07
updated: 2026-11-13
---

# Overview

This wiki is what an agent needs to diagnose Aaron's four cars through an [[obdlink-ex]] using
[[carlink-cli]]. Everything here was researched on 2026-11-07 and **nothing has yet been observed on
the real cars**; the first scans should confirm or correct the vehicle pages.

## The adapter
The OBDLink EX is an STN22xx-based, ELM327-compatible USB adapter (FTDI `0403:6015`, 115200 8N1,
`/dev/cu.usbserial-<serial>` on macOS, no driver needed). It speaks every legislated OBD-II protocol
(K-line ISO 9141-2/KWP2000, CAN 11/29-bit) but, unlike the MX+, not Honda's or Hyundai's private
body networks ([[macos-serial-setup]], [[elm327-at-commands]], [[stn-st-commands]]). Responses are
parsed with headers on so each ECU's answer is separable ([[adapter-response-format]]). Several
AT/ST commands permanently reconfigure or brick the adapter, so carlink uses an allowlist
([[adapter-safety]]).

## What generic OBD-II gives you
Services 01–0A ([[obd2-services]]): live PIDs ([[obd2-pids]]), readiness monitors
([[readiness-monitors]]), stored/pending/permanent codes ([[dtc-format]]), freeze frame, VIN.
Clearing codes (service 04) also erases freeze frames and resets readiness — the car then fails an
emissions inspection until drive cycles complete ([[clearing-codes]]). Interpretation thresholds are
rules of thumb, not standards ([[diagnostic-thresholds]]).

## The cars
- **[[crv-2004]]** — K-line, PCM only; transmission faults show as P0700 + P07xx. Known patterns:
  P1457 EVAP vent valve, VTEC switch P2646/7, VTC P0011, A/C compressor failure. Takata recalls.
- **[[odyssey-2016]]** — CAN; PCM at 7E8 also handles the transmission. Headline issue: VCM oil
  burning → misfires on cylinders 1–3, covered by Honda extension 25-061 (10 yr from original sale /
  150k mi). Torque-converter judder → bulletin 17-043. Timing belt at Maintenance Minder code 4.
- **[[clarity-phev-2018]]** — plug-in hybrid on 29-bit CAN; seven ECUs answer generic OBD, the engine
  computer is `18DAF10E`. The value is the battery capacity ([[clarity-phev-hv-battery]]): 55 Ah new,
  warranty replacement below 36.6 Ah (Honda S/B 17-093) with no warning light. Recall 23V858 (fuel pump).
- **[[kona-electric-2020]]** — almost nothing on generic OBD; the value is in BMS DIDs
  ([[kona-ev-bms-pids]]): SOC, SOH, 98 cell voltages, temperatures, lifetime kWh, 12 V, isolation.
  Battery-health interpretation in [[kona-ev-battery-health]]; confirm recall **21V-127** (LG pack
  replacement) was done ([[kona-ev-recalls]]).

Reset/relearn procedures are owner-performed on the car, except clearing codes
([[honda-maintenance-minder]], [[odyssey-2016-tpms-calibration]], [[honda-idle-relearn]],
[[kona-ev-resets]]); the full list is in [[honda-common-dtcs]] and the per-car pages.

## Beyond the engine computer (added 2026-11-08)
- **Self-test results (service 06)** are the shop's best misfire tool on CAN cars: per-cylinder
  misfire counts (MIDs A2–A7 on the Odyssey) and each monitor's value against its limits
  ([[obd2-mode06]], [[honda-mode06]]). The CR-V's K-line version has no misfire data.
- **Module trouble codes**: the Kona's 14 diagnosable modules answer UDS `19 02` or KWP `18 00 FF 00`
  ([[uds-module-dtcs]], [[kona-ev-modules]]). Its airbag module only serves codes in a non-default
  session. On both Hondas nothing beyond the PCM is reachable with this adapter
  ([[odyssey-2016-modules]], [[crv-2004-modules]]).
- **What must not be cleared**: on the Kona, OBD service 04 reaches the BMS, whose codes Hyundai's
  battery campaign 9A4 uses to decide on pack replacement; airbag and smart-key modules stay read-only
  ([[uds-module-dtcs]] §8).
- **Live-data interpretation** for a shop (λ/O2, catalyst, fuel-trim idle vs 2,500 rpm, load):
  [[live-data-interpretation]], with the extra PIDs in [[obd2-pids-extended]].

## Open questions (resolve on the first real scans)
- Does the CR-V return a VIN over 0902? Which protocol does `ATDPN` report?
- Odyssey: 11-bit (`7E8`) or 29-bit (`18DAF111`/`18DAF11D`) generic OBD? Its service 06 MID set?
- Kona: do the module reads, explicit flow control and the `ATAR` restore behave as in the
  third-party captures? Rear TPMS left/right order?
- Does the Odyssey use smart charging (low running voltage would then be normal)?

## Contradictions and tensions
- Kona VMCU speed decode: evDash and OBDb disagree; carlink does not use VMCU speed.
- Kona TPMS rear-left/right byte order disputed; carlink labels them `rear_a`/`rear_b`.
- Odyssey crank pattern learn steps come from other Honda models.
- 2026-11-08: the module research contradicted three earlier pages (Kona service 03/04/VIN, Kona
  service 04 scope, Odyssey protocol); the notes are on [[kona-electric-2020]], [[kona-ev-resets]] and
  [[odyssey-2016]].
