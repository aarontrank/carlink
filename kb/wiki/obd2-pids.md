---
title: OBD-II service 01 PIDs — formulas and units
type: concept
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/wikipedia-obd2-pids-2026-11-07.wikitext
  - url:https://en.wikipedia.org/wiki/OBD-II_PIDs
tags: [obd2, j1979, pid, decoding]
confidence: community
---

# Service 01 PIDs

Notation: A, B, C, D are the data bytes after `41 <PID>`, in order. `A7` is the MSB of A.
Formulas come from Wikipedia "OBD-II PIDs", which transcribes J1979/SAE HS-3000 (saved in
`raw/wikipedia-obd2-pids-2026-11-07.wikitext`). They are consistent with python-OBD and the
ELM327 datasheet examples. Confidence: `community`. The same formulas apply to service 02
(freeze frame).

## Supported-PID bitmaps

PIDs `00, 20, 40, 60, 80, A0, C0` each return 4 bytes. The bits A7…D0 map to the next 32 PIDs,
MSB first: PID `00` covers `01`–`20`, PID `20` covers `21`–`40`, and so on. The last bit (D0)
says whether the *next* bitmap PID is supported, so walk the chain until D0 = 0. For PID
*p* ≥ 1: the bitmap PID is `base = ((p-1)/32)*32`, `index = p - base - 1` (0–31), and the flag
is in data byte `index/8`, bit `7 - index%8`. Check: p = 0x0C → base 0x00, index 11, byte B,
bit 4.

Example: `BE 1F A8 13` → supported PIDs 01, 03–07, 0C–11, 13, 15, 1C, 1F, 20.

Several ECUs (`7E8`, `7E9`, …) answer, each with its own bitmap. Union them for display, but
query a PID from an ECU that claims it.

## Useful PIDs

| PID | Bytes | Meaning | Formula | Units / range |
|-----|-------|---------|---------|---------------|
| 01 | 4 | Monitor status since DTCs cleared | bit-encoded — see [[readiness-monitors]] | |
| 03 | 2 | Fuel system status (sys 1 = A, sys 2 = B) | enum: 0 off/not present, 1 OL cold, 2 CL (O2 feedback), 4 OL load/decel fuel-cut, 8 OL system failure, 16 CL with feedback fault | |
| 04 | 1 | Calculated engine load | `A*100/255` | % 0–100 |
| 05 | 1 | Engine coolant temp | `A-40` | °C −40…215 |
| 06 / 07 | 1 | STFT / LTFT bank 1 | `A*100/128 - 100` | % −100 (rich, removing fuel)…+99.2 (lean, adding fuel) |
| 08 / 09 | 1 | STFT / LTFT bank 2 | `A*100/128 - 100` | % |
| 0A | 1 | Fuel pressure (gauge) | `A*3` | kPa 0–765 |
| 0B | 1 | Intake manifold absolute pressure | `A` | kPa 0–255 |
| 0C | 2 | Engine speed | `(A*256+B)/4` | rpm 0–16383.75 |
| 0D | 1 | Vehicle speed | `A` | km/h 0–255 |
| 0E | 1 | Timing advance | `A/2 - 64` | ° before TDC |
| 0F | 1 | Intake air temp | `A-40` | °C |
| 10 | 2 | MAF air flow rate | `(A*256+B)/100` | g/s 0–655.35 |
| 11 | 1 | Throttle position | `A*100/255` | % |
| 1C | 1 | OBD standard conformed to | enum (1 = OBD-II CARB, 3 = OBD and OBD-II, 6 = EOBD, …) | |
| 1F | 2 | Run time since engine start | `A*256+B` | s |
| 21 | 2 | Distance with MIL on | `A*256+B` | km |
| 2F | 1 | Fuel tank level | `A*100/255` | % |
| 30 | 1 | Warm-ups since codes cleared | `A` | count |
| 31 | 2 | Distance since codes cleared | `A*256+B` | km |
| 33 | 1 | Absolute barometric pressure | `A` | kPa |
| 41 | 4 | Monitor status **this drive cycle** | same layout as PID 01; A is always 0 | |
| 42 | 2 | Control module voltage | `(A*256+B)/1000` | V |
| 46 | 1 | Ambient air temp | `A-40` | °C |
| 4D | 2 | Time run with MIL on | `A*256+B` | min |
| 4E | 2 | Time since codes cleared | `A*256+B` | min |
| 5C | 1 | Engine oil temp | `A-40` | °C −40…210 |
| 5E | 2 | Engine fuel rate | `(A*256+B)/20` | L/h |
| A6 | 4 | Odometer | `(A*2^24 + B*2^16 + C*2^8 + D)/10` | km (CARB-mandated from MY2019; often unsupported on older cars) |

Also useful:

- `13`: O2 sensors present. Bits A0–A3 = bank 1 sensors 1–4, A4–A7 = bank 2.
- `14`–`1B`: narrowband O2 sensor. `A/200` V; `B*100/128-100` % trim, where B = `FF` means
  the sensor is not used for trim.
- `24`–`2B` / `34`–`3B`: wideband (A/F) sensors. λ = `(A*256+B)*2/65536`; voltage =
  `(C*256+D)*8/65536` V, or current = `(C*256+D)/256 - 128` mA.
- `3C`–`3F`: catalyst temperature, `(A*256+B)/10 - 40` °C.
- `43`: absolute load, `(A*256+B)*100/255` %.
- `44`: commanded λ, `(A*256+B)*2/65536`.

## Decoding cautions

- Signed trims: use the formula exactly. `0x80` = 0%. Do not treat A as an `i8`.
- 16-bit values are big-endian (A is the high byte).
- `NO DATA` for a PID the bitmap claims is supported happens; treat it as "unavailable", not
  as an error.
- An EV (2020 Kona Electric) supports few or none of the combustion PIDs. Expect most to be
  absent; its useful data lives in manufacturer service `22` PIDs.

## Related
[[obd2-services]] · [[readiness-monitors]] · [[diagnostic-thresholds]] · [[dtc-format]]
