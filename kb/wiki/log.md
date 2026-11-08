---
title: Log
type: log
created: 2026-11-07
updated: 2026-11-07
---

# Log

## [2026-11-07] init | Wiki created
Created wiki for OBDLink EX diagnostics of the CR-V 2004, Odyssey 2016 and Kona Electric 2020.

## [2026-11-07] research | Initial research pass (4 parallel agents)
Sources: OBDLink FRPM rev F, ELM327 datasheet, Wikipedia OBD-II PIDs, OBDex (CC0 DTC list),
JejuSoul/evDash/OBDb Kona sources, NHTSA API, 2004 CR-V service manual (charm.li), Honda memos.
Created: [[obdlink-ex]], [[macos-serial-setup]], [[elm327-at-commands]], [[stn-st-commands]],
[[adapter-response-format]], [[adapter-safety]], [[obd2-services]], [[obd2-pids]],
[[readiness-monitors]], [[dtc-format]], [[clearing-codes]], [[diagnostic-thresholds]],
[[crv-2004]], [[odyssey-2016]], [[honda-common-dtcs]], [[honda-maintenance-minder]],
[[odyssey-2016-tpms-calibration]], [[honda-idle-relearn]], [[kona-electric-2020]],
[[kona-ev-bms-pids]], [[kona-ev-other-ecus]], [[kona-ev-battery-health]], [[kona-ev-recalls]],
[[kona-ev-resets]], [[carlink-cli]]. Updated: [[index]], [[overview]].

## [2026-11-07] move | Wiki moved into the carlink repo
Moved from `~/wikikb/vehicle-diagnostics/` to `~/carlink/kb/` so it is versioned with the code.
Content unchanged; [[carlink-cli]] and `schema.md` note the new home.

## [2026-11-08] research | Service 06, extended PIDs, live-data interpretation
Sources: CARB 13 CCR 1968.2, ISO 15031-5 sample, Snap-on Global OBD manual, Honda "Mode $06 Test
Information: All 2005 Hondas", Toyota emulator captures, OBDb, trade articles. Created: [[obd2-mode06]],
[[honda-mode06]], [[obd2-pids-extended]], [[live-data-interpretation]].

## [2026-11-08] research | Module trouble codes (UDS/KWP) and per-car module maps
Sources: projectgus Kona CAN logs (Car Scanner reads/clears), OBDb Kona/Odyssey/Pilot captures,
opendbc, OVMS, ISO 14229/14230 samples, Hyundai TSB 23-01-073H-1, Honda job aid 06932, 2004 CR-V
service manual. Created: [[uds-module-dtcs]], [[kona-ev-modules]], [[odyssey-2016-modules]],
[[crv-2004-modules]]. Contradictions flagged on [[kona-electric-2020]], [[kona-ev-resets]] and
[[odyssey-2016]] (see the notes added there).

## [2026-11-08] update | carlink hardening pass
[[carlink-cli]] rewritten for the new commands and protocol handling; [[diagnostic-thresholds]]
charging rows aligned with carlink's findings.


## [2026-11-08] update | K-line code-read completeness
Sources: url:x-engineer.org (Service $03), url:scribd.com/document/334162539 (J1979 excerpt). [[dtc-format]]
gained "When an ECU has no codes": on ISO 9141-2/J1850 a no-code reply is optional, on CAN and KWP it is
required. [[carlink-cli]]: a K-line ECU silent to 03 counts as "no codes" only when its PID 01 counts zero.

## [2026-11-08] research | Kona power states
Source: raw/hyundai-kona-electric-erg-power-button.md (Hyundai's Kona Electric Emergency Response Guide). [[kona-electric-2020]] gained the
OFF → ACC → ON → OFF button cycle (no brake) vs READY (brake), used by the skill's clear step.
