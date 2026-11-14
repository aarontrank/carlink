---
title: Honda Service Bulletin 17-093 v3 (Jan 25, 2018) — 2018 Clarity Plug-In Hybrid PDI and New Model Service Information
source_url: https://www.insideevsforum.com/community/attachments/clarity-pdi-1-pdf.7406/
fetched: 2026-11-13
---

Provenance: Honda service bulletin PDF ("© 2017 American Honda Motor Co., Inc.", "Page n of 13"),
obtained as a forum attachment ("Clarity PDI(1).PDF", posted by KentuckyKen, Feb 12 2020, in
https://www.insideevsforum.com/community/threads/battery-warranty.8264/). Not fetched from
techinfo.honda.com directly (site down). Saved at (a session-scratchpad copy, not kept; see source_url).

## Verbatim excerpts

p.1: "Service Bulletin 17-093 January 25, 2018 08048 Version 3 — 2018 Clarity Plug-In Hybrid: PDI and
New Model Service Information. Supersedes 17-093, dated December 9, 2017"

p.4 (§6): "This vehicle has two cooling systems. One system cools the engine; the other one cools the
inverter system. Both systems use the usual Honda Long Life Antifreeze/Coolant Type 2."

p.6 (§11 Start the TPMS Calibration Process): "This vehicle has an indirect TPMS. Unlike other systems
that directly measure air pressure, an indirect TPMS uses the VSA wheel speed sensors to monitor and
compare tire characteristics while driving and determine when one or more tires are significantly
underinflated. To ensure proper operation, you must inflate all four tires to the cold inflation
pressures listed on the driver's doorjamb label and start the TPMS calibration process during the PDI
using this procedure:
1. With the transmission in Park, turn the ignition to ON.
2. On the Display Audio screen, select HOME, then Settings.
3. Select Vehicle.
4. Select TPMS Calibration.
5. Select Calibrate. The message TPMS has been initialized will appear, and the screen will return to
the customization menu screen. Once started, the calibration will complete on its own."

p.7 (§14): "NOTE: This procedure [remote climate] will not work if the high-voltage lithium-ion battery
is very low on charge."

p.12 (§20): "AC 240 Volt (Level II) 2.5 (full charge); AC 120 Volt (Level I) 12.0 (full charge)" ...
"NOTE: Charging times will be longer as the temperature drops. You cannot charge the battery or start
the engine below -22°F."

p.13 (New model info §4 Checking Battery Pack Capacity): "The high-voltage lithium-ion battery in this
vehicle has an 8-year or 100,000-mile warranty. But unlike earlier vehicles with high-voltage batteries,
this vehicle does not set a DTC when the battery pack capacity value has reached the level for battery
replacement under warranty. To check battery pack capacity in this vehicle, you must use the i-HDS. Just
go to the Electric Powertrain Data List, and look at the Battery Pack Capacity signal. If its value reads
less than 36.6 Ah and the battery is still under warranty, the battery is eligible for warranty
replacement."

p.13 screenshot (HDS Ver 1.004.005, Electric Powertrain snapshot): "Battery Pack Capacity  55.0  Ah" —
help text "This parameter is received by Battery Condition Monitor Module A."

## Summary
- Authoritative Honda statement: warranty-replacement threshold is Battery Pack Capacity < 36.6 Ah
  (HDS Electric Powertrain data list), no DTC is set at that level; 8 yr / 100k mi stated (this bulletin
  does not mention the CARB-state 10/150k terms).
- New-car value shown in Honda's screenshot: 55.0 Ah.
- TPMS is indirect (wheel-speed based), calibrated from Display Audio Settings > Vehicle > TPMS Calibration.
- Two coolant circuits (engine + inverter/PCU), both Honda Type 2 coolant.
