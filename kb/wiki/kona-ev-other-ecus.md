---
title: Kona Electric — other ECUs (VMCU, MCU, TPMS, HVAC, cluster, OBC/LDC)
type: concept
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/obdb-hyundai-kona-electric.md
  - raw/evdash-carkiaeniro-src.md
  - raw/jejusoul-kona-niro-ev-pids.md
tags: [kona-electric, vmcu, tpms, odometer, hvac, obc, ldc]
confidence: community
---

# Kona Electric — other ECUs

These are the non-BMS modules on the [[kona-electric-2020]]. The indexing convention is the same one
used in [[kona-ev-bms-pids]]: reassemble ISO-TP, strip the positive-response header, and let `p` be
the 0-based index after it. For service 22 the header is 3 bytes (`62 DD DD`). For KWP-style service
21 it is 2 bytes (`61 LL`). Torque letter A = `p0`. evDash: `p = char/2 − 3` for 22xxxx, and
`p = char/2 − 2` for 21xx. OBDb: `p = bix/8`. `blsb` means little-endian.

OBDb marks the TPMS, cluster, HVAC 220100 and BMS reads `fcm1`: it sends `ATFCSH<req>`,
`ATFCSD300000` and `ATFCSM1` first. See [[kona-ev-bms-pids]] for the AT order rule.

## ECU address map (documented by at least one source)

| Req → Resp | Module | Reads documented |
|---|---|---|
| 7E2 → 7EA | VMCU (vehicle control) | 2101, 2102, 22E004, 22E005 |
| 7E3 → 7EB | MCU (motor/inverter) | 2102, 220102, 22E001, 22E009; **answers generic Service 01** |
| 7E4 → 7EC | BMS | 220101–220106 ([[kona-ev-bms-pids]]); **answers generic Service 01** |
| 7E5 → 7ED | OBC / LDC (charger, 12 V DC-DC) | 2101, 22E001–E003, 22E011 |
| 7A0 → 7A8 | BCM / TPMS | 22C00B (tires), 22B00C, 22C00C–C00F |
| 7B3 → 7BB | HVAC (FATC) | 220100, 220101, 220102 |
| 7C6 → 7CE | Instrument cluster | 22B001–B003 (odometer in B002) |
| 770 → 778 | IGPM (body / lights / doors) | 22BC01–BC09 |
| 7D1 → 7D9 | ABS / ESC / IEB | 22C101–C103 (drive mode), 2101 |
| 7D4 → 7DC | MDPS (steering) | 220101, 2101 |

## VMCU 7E2 — vehicle speed, 12 V battery

**2101 — vehicle speed** (response `61 01`, length 0x18)

| p | Field | Formula | Source / confidence |
|---|---|---|---|
| 13–14 | Vehicle speed | `s16` **little-endian** `/100` → mph | OBDb (bix 104, blsb); 5 real stationary captures decode to 0 |

Disagreement: evDash reads big-endian `p14–15 × 0.0155` → km/h. The two methods share the high byte
(`p14`). On OBDb's stationary captures evDash's method yields 0.6–2.8 km/h, while OBDb's yields 0.
**Use OBDb's.** Note the raw unit is 0.01 mph even on EU cars, which is where evDash's 0.0155 ≈
0.01 × 1.609 comes from. `unverified` until it is checked against GPS on the car.

**2102 — 12 V auxiliary battery** (response `61 02`)

| p | Torque | Field | Formula | Unit |
|---|---|---|---|---|
| 16–17 | Q–R | LDC current (OBDb "AUX amps LDC") | `u16 / 100` (OBDb, big-endian) | A |
| 19–20 | T–U | Aux battery voltage | `s16` LE `/1000` = `(signed(U)·256 + T)/1000` | V |
| 21–22 | V–W | Aux battery current | `s16` LE `/1000` | A (evDash negates it) |
| 23 | X | Aux battery SOC | `u8` | % |

JejuSoul and OBDb agree on p19–23. evDash's SOC byte also matches (`p23`). evDash reads the current
big-endian, which disagrees with the other two; the little-endian reading is preferred.

**22E004** (OBDb only): accelerator pedal `u16 LE p8–9 / 512` %; high-res speed `s16 LE p10–11 ×
156.25/10000` km/h; 12 V `u16 LE p12–13 × 4.883/10000` V. `unverified`; OBDb's dbgfilter suggests it
was not confirmed on MY2020.

## MCU 7E3 — motor and inverter temperatures

| Read | p | Field | Formula | Source |
|---|---|---|---|---|
| 2102 | 14 | Inverter temp | `s8` °C | evDash |
| 2102 | 15 | Motor temp | `s8` °C | evDash |
| 220102 | 11 / 12 | Motor / inverter temp | `s8` °C | OBDb (`dbg`, unconfirmed) |

## TPMS 7A0 → 7A8, DID 22C00B — tire pressure and temperature

Real MY2020 capture (OBDb):

```
7A8101762C00BFFFF00
7A82100BB420100BD41
7A8220100B63E0100B8
7A8233E0100AAAAAAAA
→ 62 C0 0B | FF FF 00 00 BB 42 01 00 BD 41 01 00 B6 3E 01 00 B8 3E 01 00
```

| p | Field | Formula | Capture → value |
|---|---|---|---|
| 4 | Front-left pressure | `u8 / 5` psi (= `u8 × 0.2`) | BB → 37.4 psi |
| 5 | Front-left temp | `u8 − 50` °C | 42 → 16 °C |
| 8 / 9 | Front-right pressure / temp | same | BD → 37.8 psi, 41 → 15 °C |
| 12 / 13 | Rear tire "A" pressure / temp | same | B6 → 36.4 psi, 3E → 12 °C |
| 16 / 17 | Rear tire "B" pressure / temp | same | B8 → 36.8 psi, 3E → 12 °C |

**Rear left/right disagreement.** OBDb labels p12 = rear-left and p16 = rear-right. evDash labels
p12 = rear-right and p16 = rear-left. Neither shows evidence for its choice. Report the rear pair as
"rear (p12)" / "rear (p16)", or settle it on the car by deflating one rear tire slightly.
evDash's bar conversion (`raw / 72.5189`) is the same scale (0.2 psi/bit).

evDash only polls TPMS every 30 s. OBDb's `freq` is 15 s.

## HVAC 7B3 → 7BB

| Read | p | Field | Formula | Sources |
|---|---|---|---|---|
| 220100 | 5 | Indoor temp | `u8/2 − 40` °C | JejuSoul F, evDash, OBDb |
| 220100 | 6 | Outdoor temp | `u8/2 − 40` °C | JejuSoul G, evDash, OBDb |
| 220100 | 7 | Evaporator temp | `u8/2 − 40` °C | evDash |
| 220100 | 29 | Vehicle speed | `u8` km/h | OBDb |
| 220102 | 4 / 5 | Coolant temp 1 / 2 (only if `p3 == 00`) | `u8/2 − 40` °C | evDash; OBDb (p4) |

## Instrument cluster 7C6 → 7CE, DID 22B002 — odometer

Real MY2020 capture from a US car:

```
7CE100F62B002D00000 / 7CE2100009B00000000 / 7CE22A79EAAAAAAAAAA
→ 62 B0 02 | D0 00 00 00 00 9B 00 00 00 00 A7 9E      (p0 … p11; length 0x0F = 15)
```

| p | Field | Formula | Capture |
|---|---|---|---|
| 6–8 | Odometer, km | `u24` | `000000` → 0 (unused on this US car) |
| 9–11 | Odometer, miles | `u24` | `00A79E` → 42 910 mi |

evDash reads only the km field (`p6–8`), because its users are mostly on EU cars. On a US car the km
field reads 0, so **read p9–11 (miles)**, and fall back to p6–8 if the miles field is 0. `p5` varies
between captures (9B…9F) and is not documented.

## OBC / LDC 7E5 → 7ED (OBDb only, `unverified`)

| Read | p | Field | Formula |
|---|---|---|---|
| 2101 | 9 | AC input voltage RMS | `u8 × 64/25` V |
| 2101 | 13 | Charger temp | `u8 − 100` °C |
| 2101 | 25 | AC input current RMS | `u8 × 64/25` A |
| 22E011 | 9 | LDC temp | `u8 − 100` °C |
| 22E011 | 10–11 | LDC output voltage | `u16 / 1000` V |
| 22E011 | 12–13 | LDC output current | `u16 / 100` A |
| 22E011 | 20 | Aux battery SOC | `u8` % |
| 22E011 | 21–22 | Aux battery voltage | `u16/1000 + 6` V |

OBDb's dbgfilter marks these as not confirmed on MY2020; treat them as leads to try, not as facts.

## Related
[[kona-electric-2020]] · [[kona-ev-bms-pids]] · [[kona-ev-resets]] · [[obd2-services]]
