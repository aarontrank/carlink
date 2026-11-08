---
title: 2004 CR-V — ABS and SRS codes are not reachable with an ELM/STN adapter
type: concept
created: 2026-11-08
updated: 2026-11-08
sources:
  - raw/charm-crv-2004/abs-srs-dtc-retrieval.md
  - raw/assets/crv-2004-dlc-connector-view.png
  - raw/honda-job-aid-06932-dlc-circuit-troubleshooting-2017.md
  - raw/module-dtc-community-notes-2026-11-08.md
tags: [crv-2004, honda, abs, srs, k-line, dlc]
confidence: verified
---

# 2004 CR-V — modules other than the PCM

**No.** On the [[crv-2004]] only the PCM can be reached with an ELM-type adapter, over ISO 9141-2 on
pin 7. The ABS and SRS units cannot.

The 2004 service manual gives two ways to read ABS codes and two to read SRS codes:
- the Honda PGM Tester's own menus, through the 16-pin DLC;
- blink codes on the ABS or SRS lamp, after the tester grounds the SCS line (DLC pin 9, circuit F28).

For a generic scan tool it says only "Honda does not provide instructions for retrieving ABS DTCs"
and "Manufacturer does not supply information" for SRS (verified, raw/charm-crv-2004/abs-srs-dtc-retrieval.md).

The DLC view shows pins 6 and 14 are not CAN on this car. Pin 6 is unused. **Pin 14 carries Honda
circuit F55**, a circuit the manual leaves unnamed. On the 2002–04 Odyssey the shop manual calls pin
14 "Circuit F29 [ABS/TCS and SRS control modules]" (community), so pin 14 is probably the ABS/SRS
line here too (inference). Honda's 2017 DLC job aid lists the CR-V's contemporaries (2001–05 Civic,
2003–06 Element, 1999–06 Odyssey) as "K-Line Only" (verified). The 2002–06 CR-V itself is not on
that list.

carlink should therefore offer nothing beyond PCM codes on this car. It should tell the user that
ABS and SRS codes need a Honda-capable scan tool or the SCS blink-code procedure. The OBDLink EX
cannot ground pin 9.

A side note (inference): OBDLink's automatic search uses the SAE J1978 order (FRPM, `ATSS`). The
ELM datasheet says that order "follows the number order", so ISO 9141-2 (protocol 3) is tried
before the CAN protocols (6–9). carlink's `ATSP0` search should therefore normally find the PCM
without driving pins 6/14. If auto-detect ever fails on this car, force the
K-line protocol (`STP 22`) rather than letting the search run its CAN attempts on pin 14.
