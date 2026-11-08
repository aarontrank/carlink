---
title: DTC encoding (services 03, 07, 0A and 02 PID 02)
type: concept
created: 2026-11-07
updated: 2026-11-08
sources:
  - raw/wikipedia-obd2-pids-2026-11-07.wikitext
  - url:https://en.wikipedia.org/wiki/OBD-II_PIDs
  - url:https://www.elmelectronics.com/wp-content/uploads/2016/07/ELM323DS.pdf
  - url:https://ramn.readthedocs.io/en/latest/userguide/diag_tutorial.html
  - url:https://github.com/ircama/ELM327-emulator
  - url:https://github.com/foerbsnavi/obdex
  - url:https://x-engineer.org/obd-diagnostic-service-mode-03-request-emission-related-diagnostic-trouble-codes-dtcs/
  - url:https://www.scribd.com/document/334162539/ISO9141X
tags: [obd2, dtc, decoding]
confidence: community
---

# DTC format

## Two bytes → five characters

Each DTC is 2 bytes, A and B:

| Bits | Meaning |
|------|---------|
| A7–A6 | System letter: `00` P (powertrain), `01` C (chassis), `10` B (body), `11` U (network) |
| A5–A4 | First digit, 0–3 (`0` = SAE generic, `1` = manufacturer, `2` = generic for P, `3` = mixed/manufacturer) |
| A3–A0 | Second character, a hex digit |
| B7–B4 | Third character, a hex digit |
| B3–B0 | Fourth character, a hex digit |

```
letter = "PCBU"[A >> 6]
code   = format!("{}{:X}{:X}{:X}{:X}", letter, (A>>4)&3, A&0xF, B>>4, B&0xF)
```

Examples: `01 33` → **P0133**; `C1 58` → **U0158**; `03 00` → **P0300**; `41 23` → **C0123**.
A four-character form like "C158" is the undecoded raw hex. Do not show it.

## Response framing differs by protocol

**CAN (ISO 15765-4):** `43 <count> [A B]…`. The byte after the service echo is the **number of
DTCs** from that ECU, followed by exactly `count` pairs, with no padding pairs. ISO-TP
reassembles anything longer than one frame.

- `43 01 01 72` → 1 code: P0172.
- `43 00` → no codes.
- `43 02 01 33 04 20` → P0133, P0420.

**ISO 9141-2 / KWP2000 / J1850 (non-CAN):** no count byte. Each message is `43` + exactly
**6 data bytes = 3 code slots**, padded with `00 00`. More than 3 codes means more messages.

- `43 01 33 00 00 00 00` → P0133 only. The `0000` pairs are filler, not "P0000".
- To learn the count up front on non-CAN, read `01 01`: byte A & 0x7F (stored codes only).

Rules for the parser:

1. Know the protocol (from `AT DPN`, or from the ELM327 having CAN headers/ISO-TP framing).
   Do not infer it from the data.
2. On CAN, take `count` and then exactly `count` pairs.
3. On non-CAN, read pairs and drop `00 00`. P0000 is never a real code.
4. Group by ECU header. Engine (`7E8`) and transmission (`7E9`) can each report codes.
5. `47` (service 07, pending) and `4A` (service 0A, permanent) use the identical format.

Some secondary references (and at least one ELM emulator) mix the two layouts. The rule above
follows Wikipedia, the ELM323 datasheet (non-CAN) and the RAMN UDS/OBD tutorial (CAN).
(`community`)

## When an ECU has no codes

SAE J1979 makes the "no codes" reply protocol-dependent, so silence means different things:

| Protocol | ECU with no codes | Silence means |
|---|---|---|
| ISO 15765-4 (CAN) | answers `43 00` (shall respond with zero DTCs) | a lost reply |
| ISO 14230-4 (KWP) | answers `43 00 00 00 00 00 00` (all three slots $00) | a lost reply |
| ISO 9141-2, SAE J1850 | a reply is **allowed but not required**; the ELM shows `NO DATA` | nothing, unless PID 01 counts codes |

J1979's read procedure: read `01 01` first (byte A & 0x7F = confirmed codes per ECU), then `03`,
and repeat until the codes read equal the count. carlink applies it as: a CAN ECU silent to `03` is a
lost reply; a non-CAN ECU's silence is "no codes" only if its own PID 01 counts zero — on KWP too, since a
lost reply from an ECU that counts zero hides no confirmed code ([[carlink-cli]]).
Real Kona captures fit the CAN row: `03`/`07` were answered `43 00`/`47 00` by every responding ECU
([[kona-ev-modules]]). The CR-V's behaviour (ISO 9141-2) is unconfirmed. Sources: J1979 text as
reproduced in an ISO 9141-2 message-format excerpt (scribd.com/document/334162539) and x-engineer.org's
Service $03 page ("If no DTCs are stored, the ECU may choose not to respond"). (`community`)

## Which list a code is in

| Service | List | Meaning |
|---------|------|---------|
| 07 | Pending | Fault seen once (current or last drive cycle); not yet confirmed; MIL usually off |
| 03 | Confirmed / stored | Fault matured (typically 2 consecutive trips); MIL on for emissions faults |
| 0A | Permanent | Confirmed code with MIL on; survives service 04 and battery disconnect; erased only by the ECU after the monitor passes (CARB requirement, MY2010+) |

## Descriptions

The generic (J2012) titles are in `~/carlink/data/dtc-generic.csv` (9,533 codes: P0, P2, P3,
U0, U3, C0, B0), from OBDex under CC0 1.0. `~/carlink/data/DTC-LICENSE.md` records the source
and license. Codes with first digit 1 (and most 3) are manufacturer-specific. Some makers also
reuse generic-range C/B codes with their own meaning (older GM: C0035–C0050 are its own
wheel-speed circuits). Show "manufacturer-specific" rather than a wrong generic description.

## Related
[[obd2-services]] · [[clearing-codes]] · [[readiness-monitors]]
