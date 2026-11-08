---
title: Kona Electric — resets and relearns without dealer tools
type: procedure
created: 2026-11-07
updated: 2026-11-08
sources:
  - raw/kona-ev-community-notes-2026-11-07.md
  - raw/obdb-hyundai-kona-electric.md
tags: [kona-electric, tpms, 12v, service-reminder, dtc]
confidence: community
---

# Kona Electric — resets and relearns without dealer tools

This page covers what an owner can do on the [[kona-electric-2020]] with the cluster, a drive, or a
generic [[obdlink-ex]] read. ECU writes (coding, BMS reflash, routine control, TPMS sensor ID
writes) are **dealer GDS tasks**, and `carlink` does not perform them.

## TPMS — direct sensors, automatic position relearn (`community`)

- The 1st-gen Kona Electric has **direct TPMS**: one sensor per wheel, read by the receiver on 7A0
  (22C00B, see [[kona-ev-other-ecus]]). The module locates each sensor partly by signal strength.
- **After a tire rotation** with the same sensors there is nothing to do. Drive, and the display
  moves the pressures to the new positions after roughly 5–20 minutes or about 10 miles. Third-party
  guides say to drive above 25 mph (40 km/h). The owner's-manual wording was not found.
- **Low-pressure light:** inflate to the door-placard pressure and drive. The light clears once the
  sensors report the new pressure. There is no reset button on this car.
- **New sensors or a second wheel set:** the new sensor IDs must be registered with a TPMS tool (or
  at a dealer). Auto-learn of *new* IDs is reported as inconsistent. That is a write, so it is not a
  `carlink` job.
- Caveat: 2nd-gen (2023+) Kona pages describing *indirect* TPMS with a menu reset do **not** apply
  to this car.

## 12 V auxiliary battery (`community`)

- The HV pack charges the 12 V battery through the LDC whenever the car is "ready" or charging. With
  Aux. Battery Saver+ it also does so periodically while parked. On updated cars the saver is
  always on and no longer appears in the menu; the front badge light turns yellow while topping up.
- There is nothing to "reset" after a 12 V replacement that an owner can do. Readings to monitor:
  7E2 `2102` p19–20 voltage (mV resolution), p21–22 current, p23 SOC %; BMS 220101 p29 (0.1 V).
- A dead 12 V means the HV system cannot wake: jump the 12 V terminals and the car starts normally.
  For long storage, Hyundai's storage guidance (NHTSA MC-10141574) says to disconnect the 12 V
  negative terminal.

## Service reminder (`community`, not Kona-EV-specific)

- Hyundai/Kia TSB text describes the cluster path **User Settings → Service Interval**: set or reset
  the distance and months, then confirm. The reminder is not set at the factory. On some models it is
  under the head unit instead: Setup → Vehicle → Cluster → Service Interval.
- Maintenance items that matter on this EV (Hyundai 2022 Kona EV quick reference): coolant first
  replacement 120,000 mi / 120 months. Reducer oil interval: not found. Owners report black reducer
  oil early (see [[kona-ev-recalls]]).

## Clearing DTCs (Service 04 semantics `verified`; Kona ECU list `community`)

- **Contradicted 2026-11-08 ([[kona-ev-modules]], [[uds-module-dtcs]] §8):** on a bench Kona only the
  BMS acknowledged service 04; the VCU, MCU and OBC refused. Because the BMS's codes are the evidence
  Hyundai's battery campaign 9A4 uses, carlink never sends 04 on this car and never clears the BMS.
  Original text follows.
- Generic **Service 04** sent to `7DF` clears emission-related DTCs and freeze frames in the ECUs
  that implement OBD-II services. On this car those are the **MCU (7E3) and BMS (7E4)** (OBDb
  service01 support data). See [[clearing-codes]] and [[obd2-services]].
- Service 04 does **not** reset SOH, deterioration, cumulative counters, the TPMS, the 12 V saver, or
  the service reminder. It also does not touch DTCs in body modules (IGPM, BCM/TPMS, cluster, IEB),
  which keep their codes under UDS service 0x19/0x14. Clearing those is a different (UDS) operation,
  and this wiki has not documented it for the Kona.
- A code whose fault is still present comes back on the next drive or charge cycle. Clearing a BMS
  code does not fix an isolation or cell fault.

## Related
[[kona-electric-2020]] · [[kona-ev-other-ecus]] · [[clearing-codes]] · [[obd2-services]]
