---
title: Honda service-manual extracts — DTC P144A (78) HV Battery Pack Deterioration; Battery System Description (Clarity PHEV)
source_url: https://www.insideevsforum.com/community/attachments/p144a-battery-pack-deteriation-clarity-pdf.7415/ ; https://www.insideevsforum.com/community/attachments/battery-system-description-3105-pdf.7411/
fetched: 2026-11-13
---

Provenance: two PDF printouts of Honda service-information pages ("P144A Battery Pack Deteriation
Clarity.pdf", "Battery System Description 3105.pdf") attached by KentuckyKen, Feb 12 2020, in
https://www.insideevsforum.com/community/threads/battery-warranty.8264/. The pages do not print a model
name; the forum poster labels them Clarity. Saved under (a session-scratchpad copy, not kept; see source_url).

## P144A (verbatim)
"DTC P144A (78): HV Battery Pack Deterioration (High Severity)
The usable battery capacity range of the battery pack is calculated from the battery capacity (ΔAh) and
SOC (state of charge) change width (ΔSOC) of a specified section. If the usable battery capacity lowers
to a specified value, the battery condition monitor module A detects a malfunction and stores a DTC.
DTC Type: Two drive cycles, MIL on, POWER SYSTEM indicator on, 12 volt battery charging system indicator off
Enable Conditions: Battery pack temperature 14 °F (-10 °C) – 132 °F (56 °C); Vehicle ON mode, or Plug-in
charge on, or Plug-in quick charge on
Malfunction Threshold: The battery pack capacity is 24.9 % or less.
Possible Cause: Battery pack capacity decreased
Confirmation Procedure: 1. Turn the vehicle to the READY TO DRIVE mode. 2. Repeat accelerations and
decelerations at the vehicle speed between 0 - 31 mph (0 - 50 km/h) for at least 20 minutes.
Conditions for clearing the DTC: The MIL and the POWER SYSTEM indicator are cleared if the malfunction does
not return in three consecutive trips in which the diagnostic runs. ... can be cleared with the scan tool
Clear command or by disconnecting the 12 volt battery."

## Battery System Description (verbatim)
"The battery system is composed of the IPU and the ES cooling line. ... The DC-DC converter which is built
into the IPU converts voltage in the high voltage battery into 12 volts to supply power the 12 volt battery
and 12 volt lines."
Diagram: BATTERY PACK A (modules 1A–7A) with BATTERY CONDITION MONITOR MODULE A, CURRENT SENSOR A,
SERVICE PLUG A; BATTERY PACK B (1B–7B) with MODULE B, CURRENT SENSOR B, SERVICE PLUG B; both packs on the
same power-supply bus (i.e. A and B in parallel, 7 modules each).
"HV Battery Cut Off Control — Once a specific DTC is stored, the battery pack experiencing has contactor
problems is cut off. When this occurs, the vehicle can still be driven with the remaining charge in the
batteries. After replacing the affected battery pack, the SOC may be unbalanced and requires the 'battery
capacity adjustment' before the vehicle is driven."
"The battery condition monitor module A communicates with the PCM to determine if the 12 volt battery needs
to be charged for constant voltage, monitor the DC-DC converter, and control the converter. The PCM
communicates with the gauge control module to turn on the 12 volt battery charging indicator if there is a
problem in the DC-DC converter."
"...they keep the temperature constant by circulating ES coolant and using electric-type battery heater..."

## Summary
- Pack = two parallel sub-packs A and B, 7 modules each, each with its own battery condition monitor
  module (BCM A / BCM B). Matches OBDb's two BMS addresses (18DAF101 / 18DAF116 — mapping to A/B not
  established here).
- 12 V battery is charged only through the IPU DC-DC converter (no belt alternator).
- P144A is a deterioration DTC at ≤24.9 % capacity (distinct from the 36.6 Ah warranty threshold, which
  sets no DTC). Whether P144A applies to the 2018 Clarity specifically is per the forum label only.
