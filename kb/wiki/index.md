---
title: Index
type: index
created: 2026-11-07
updated: 2026-11-07
---

# Index

## Devices
- [[obdlink-ex]] — the adapter: STN22xx chip, FTDI USB, protocols, EX vs MX+
- [[carlink-cli]] — the Rust CLI agents use; maps commands onto this wiki

## Vehicles
- [[crv-2004]] — 2004 Honda CR-V (K24A1): ISO 9141-2, PCM only, recalls, maintenance
- [[odyssey-2016]] — 2016 Honda Odyssey (J35Z8, VCM): CAN 500k, VCM misfire extension 25-061
- [[kona-electric-2020]] — 2020 Hyundai Kona Electric: generic OBD behaviour, ECU map
- [[kona-ev-bms-pids]] — BMS DIDs 220101–220106 byte map, tested against a real capture
- [[kona-ev-other-ecus]] — VMCU, TPMS, odometer, HVAC, OBC/LDC
- [[kona-ev-battery-health]] — SOH, deterioration, cell-imbalance thresholds
- [[kona-ev-recalls]] — recalls (21V-127 pack replacement, 26V-728 open), TSBs
- [[honda-common-dtcs]] — common codes and failure patterns for both Hondas

## Protocols & commands
- [[elm327-at-commands]] — AT command set, init sequence, timing
- [[stn-st-commands]] — STN extensions, persistent-memory warnings
- [[adapter-response-format]] — CAN/ISO-TP and K-line framing, error strings
- [[adapter-safety]] — commands carlink must never send
- [[obd2-services]] — SAE J1979 services 01–0A
- [[obd2-pids]] — service 01 PID formulas
- [[readiness-monitors]] — PID 01 bit layout and inspection rules
- [[dtc-format]] — DTC encoding on CAN vs K-line
- [[clearing-codes]] — what service 04 erases and the consequences
- [[diagnostic-thresholds]] — rules of thumb for live data

## Procedures
- [[macos-serial-setup]] — finding and opening the adapter on macOS
- [[honda-maintenance-minder]] — MAINT REQ'D / Maintenance Minder reset
- [[odyssey-2016-tpms-calibration]] — TPMS warning reset and sensor registration
- [[honda-idle-relearn]] — PCM reset, idle learn, crank pattern learn
- [[kona-ev-resets]] — TPMS, 12 V, service reminder, clearing codes on the Kona

## Sources
Raw sources live in `raw/` (OBDLink FRPM rev F, ELM327 datasheet, Wikipedia OBD-II PIDs, JejuSoul,
evDash and OBDb Kona sources, NHTSA recall/complaint JSON, 2004 CR-V service manual excerpts, Honda
memos). Pages cite them in their `sources` frontmatter.

## Analyses
_(scan findings from the real cars go here)_
