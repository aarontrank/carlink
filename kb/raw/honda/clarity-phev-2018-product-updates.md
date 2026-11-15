---
title: Honda 2018 Clarity PHEV product updates and related bulletins (NHTSA manufacturer-communication copies)
source_url: https://static.nhtsa.gov/odi/tsbs/2018/MC-10145433-9999.pdf ; https://static.nhtsa.gov/odi/tsbs/2018/MC-10145429-9999.pdf ; https://static.nhtsa.gov/odi/tsbs/2018/MC-10145430-9999.pdf ; https://static.nhtsa.gov/odi/tsbs/2018/MC-10145431-9999.pdf ; https://static.nhtsa.gov/odi/tsbs/2018/MC-10145432-9999.pdf ; https://static.nhtsa.gov/odi/tsbs/2018/MC-10145427-9999.pdf ; https://static.nhtsa.gov/odi/tsbs/2018/MC-10145426-9999.pdf ; https://static.nhtsa.gov/odi/tsbs/2019/MC-10154940-0002.pdf
fetched: 2026-11-13
---
PDFs saved under the session scratchpad (not kept in the repository; fetch it again from source_url) (text in `f.txt`).

## Campaign announcement, Oct 4 2018 (MC-10145433) — verbatim
"Climate Control: In certain operating conditions, a software issue with the climate control unit and PCM software may
lead to incorrect operation of the coolant 3-way valve, causing DTC P26A3 (electric water valve malfunction) to set.
Electric Valve Timing Control (EVTC): In certain operating conditions, an issue with the PCM software may lead to
incorrect operation of the EVTC, causing DTC P100C (electric VTC motor signal circuit malfunction) and/or P0010
(variable valve timing control [VTC] malfunction) to set.
Driver Information Interface: In certain EV mode driving conditions, a software issue with the driver information
interface may lead to the HV range reading unusually high (more than 1,000 miles remaining).
Warranty Information CD: The warranty information CD placed in the vehicle did not list the correct warranty coverage
for the state of Oregon."

## 18-089 v2 (Oct 6 2018) Climate Control Unit Replacement — verbatim
"Supersedes 18-090, dated October 4, 2018, due to an internal error." "BACKGROUND There is a software problem in the
climate control unit when running the heater, which may turn the MIL on." "CORRECTIVE ACTION Replace the A/C control
unit." Part "CPU Assembly Auto A/C Kit 06796-TRW-A00".

## 18-090 v2 (Oct 6 2018) EVTC & 3-Way Valve Software Update (P100C/P0010/P26A3) — verbatim
"Supersedes 18-089, dated October 4, 2018, due to an internal error." "There is a software problem in the PCM, which may
cause the MIL to come on with DTC P26A3 (electric water valve malfunction), DTC P100C (electric VTC motor speed signal
circuit malfunction) and/or P0010 (variable valve timing control [VTC] malfunction)." "CORRECTIVE ACTION Clear the DTC's
and update the PGM-FI software with the latest version." "i-HDS Software Version: 1.004.031 or later"
(Replaces S/B 18-079 "MIL Comes On with DTC P100C and/or P0010", PCM part 37805-5WJ-A61.)

## 18-091 v2 (Oct 6 2018) Driver Display Software Update (Mileage) — verbatim
"There is a software error in the miles per gallon calculation, which affects the HV range display by reading unusually
high." "CORRECTIVE ACTION Update the meter software." (Replaces S/B 18-069 "HV Range in Driver Information Interface
Reads Unusually High".)

## 18-096 v2 — Warranty Information CD (Oregon coverage text). No DTC.

## 18-097 v3 (Mar 12 2019) Clarity Fails To Charge or Finish Charging — verbatim
"AFFECTED VEHICLES 2018 Clarity Plug-in Hybrid ALL ALL; 2017-18 Clarity Electric ALL ALL"
"SYMPTOM The Clarity Plug-In Hybrid and Clarity Electric may not start charging when plugged in or may stop charging
before the high-voltage lithium-ion battery is full."
"POSSIBLE CAUSE The battery charger software does not start charging or may stop before charging is complete in areas
with poor or fluctuating power quality."
"CORRECTIVE ACTION Update the battery charger software to the latest version."
"Once the update is completed, the vehicle may take up to one and a half times longer to fully charge."
"This issue might set one or more of the following DTCs: • DTC P0D28 (battery charger input voltage too high) • DTC
P0D2A: (battery charger input voltage too high) • DTC P0D3D: (battery charger input voltage too high) • Clarity Plug-In
Hybrid: DTC P1D23 (battery charger temperature sensor circuit range/performance) • Clarity Electric: DTC P1D22"
Part "Battery Charger Software 1C601-5WP-A02".

## Note on numbering
Version 1 (Oct 4) had 18-089 = EVTC/3-way valve and 18-090 = climate unit; version 2 (Oct 6) swapped them. Forum
posts often use the v1 numbers. The question's "18-097" is the charging bulletin above, not a 2018-10 product update.

## Summary
Dealer-applied (no OTA). Fixes: MIL with P26A3 / P100C / P0010 (PCM reflash), MIL from heater operation (climate
control unit replacement), absurd HV range (meter reflash), charging refusals with P0D28/P0D2A/P0D3D/P1D23 (charger
reflash; 18-097 applied on request).
