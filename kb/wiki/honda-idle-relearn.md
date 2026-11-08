---
title: Honda ECM/PCM reset, idle learn and CKP pattern learn
type: procedure
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/charm-crv-2004/ecm-reset-and-idle-learn.txt
  - raw/charm-crv-2004/readiness-monitors.txt
  - url:https://www.fitfreak.net/forums/fit-diy-repair-maintenance/7729-ecm-pcm-idle-learn-procedure-3.html
  - url:https://www.odyclub.com/threads/idle-learn-procedure.372675/
  - url:https://www.odyclub.com/threads/problems-solved-by-pcm-reset.307929/
tags: [procedure, honda, idle-learn, ckp, pcm-reset, crv-2004, odyssey-2016]
confidence: community
---

# Honda ECM/PCM reset, idle learn and CKP pattern learn

All three are **driver-performed**: no ECU write is needed beyond the standard Service 04 clear.
That keeps them inside `carlink`'s read-only rule ([[adapter-safety]]). `carlink` can *guide* them
and can *watch* coolant temp (PID 05), RPM (PID 0C) and readiness (PID 01) while they run.

## What a generic Service 04 clear does on a Honda

| Car | Effect of a generic-tool clear | Then do | Confidence |
|---|---|---|---|
| 2004 CR-V | Honda calls it an **ECM/PCM reset**: DTCs, freeze frame and all readiness codes are erased, along with learned idle data. Pulling fuse No. 6 ECU (15 A) for 10 s is equivalent | Idle learn, then a drive cycle to reset readiness | verified (service manual) |
| 2016 Odyssey | On Honda PCMs of this era a generic tool has **one** clear command that wipes DTCs, freeze data, on-board snapshot, readiness, **idle learn and the CKP pattern** together (HDS can clear DTCs alone) | Idle learn, then **CKP pattern learn**, then a drive cycle | community (Honda manual text quoted for a 2009 Fit; same PCM family, not confirmed in a 2016 Odyssey manual) |

Neither car's clear touches the maintenance reminder ([[honda-maintenance-minder]]), the TPMS
([[odyssey-2016-tpms-calibration]]), the radio anti-theft code (battery disconnect only), or
non-PCM modules (ABS/VSA, SRS). See [[clearing-codes]].

**Practical consequence:** after clearing a misfire code on the Odyssey, the misfire monitor is
unreliable until the CKP pattern is relearned. This is relevant to the VCM misfire problem
([[honda-common-dtcs]]). Clear codes only after recording freeze frame and Mode 06 data.

## Idle learn (both cars)

From the 2004 CR-V service manual; Honda uses the same text on later PCMs. confidence: verified for
the CR-V, community for the Odyssey.

**When:** after battery disconnect, ECM/PCM reset or replacement, throttle body or IAC
replacement/cleaning, removing ECU or battery fuses, or disconnecting PCM, fuse-box or ground
connectors. A DTC erase **with HDS** does not require it.

1. Turn off all electrical loads: A/C, audio, rear defogger, lights.
2. Start the engine. Hold **3,000 rpm**, no load, in Park/Neutral, until the **radiator fan comes
   on** or ECT reaches **194 °F (90 °C)**.
3. Release the throttle and let it **idle about 5 minutes** with the throttle fully closed. Do not
   count time while the radiator fan is running.

For the Odyssey, an aftermarket VCM disabler (Muzzler, S-VCM and similar) must be disconnected during
the procedure (OdyClub). Symptoms of a skipped idle learn are a hunting or high idle and stalling
when coming to a stop.

`carlink` assist: poll `010C` (rpm) and `0105` (ECT). Prompt "hold 3000" until ECT reaches 90 °C,
then time 5 minutes of idle.

## CKP (crank) pattern learn: Odyssey (and other CAN-era Hondas)

Needed after a generic clear, PCM replacement or update, CKP sensor replacement, or
flywheel/flexplate or crank work. HDS has "CRANK PATTERN LEARNING". The no-HDS method below is
quoted from Honda manual text posted on FitFreak. confidence: community / unverified for the
2016 Odyssey.

1. Warm up: hold 3,000 rpm with no load in Park/Neutral until the radiator fan comes on.
2. On a level road, select **2** (automatic). From **2,500 rpm**, close the throttle fully and coast
   down to **1,000 rpm**.
3. Repeat step 2 several times.
4. Ignition OFF, then ON and wait 30 s.

This step is only safe where a closed-throttle coast-down in a low gear is safe. `carlink` should
show the steps and never script them.

**2004 CR-V:** the service manual lists only ECM reset, idle learn and ECM update under
"Programming and Relearning". There is **no CKP pattern learn** for this PCM. **P1381** does not appear in the 2004 CR-V DTC chart.
It is believed to be an older Honda cylinder-position code (unverified), and there is no
relearn tied to it on either car.

## Immobilizer note

Replacing the ECM/PCM on either car needs HDS to write the immobilizer code ("To start the
engine, you must rewrite the immobilizer code with the HDS"). This is a dealer task.
