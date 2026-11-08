---
title: OBD-II service 06 — on-board monitoring test results (CAN and K-line formats)
type: concept
created: 2026-11-08
updated: 2026-11-08
sources:
  - raw/icanhack-obd-ii-kb-2026-11-08.md
  - raw/python-obd-mode06-tables-2026-11-08.md
  - raw/elm327-emulator-toyota-auris-mode06-2026-11-08.md
  - raw/carb-13ccr-1968.2-2006-test-results-excerpt.md
  - raw/snapon-global-obd-manual-2013-service05-06-excerpt.md
  - raw/iso15031-5-2015-sample-service06-noncan.md
  - raw/hit-karlsruhe-corolla-e11-kline-mode06-excerpt.md
  - raw/rlescalambre-mode06-test-results.md
  - raw/motor-advanced-mode06-diagnostics-2007-excerpt.md
  - raw/aa1car-obd2-mode06-diagnostics-2005.md
  - raw/mode06-and-load-community-notes-2026-11-08.md
  - url:https://x-engineer.org/on-board-diagnostics-obd-modes-operation-diagnostic-services/
tags: [obd2, j1979, mode06, service06, misfire, catalyst, o2, can, iso9141, decoding]
confidence: community
---

# Service 06 — on-board monitoring test results

Service 06 returns the ECU's own self-test results **with the limits it judged them against**: a
value, a minimum and a maximum per test. In a shop it is worth reading for four things:

- **Per-cylinder misfire counts** (CAN cars), which show a weak cylinder before a P030x sets.
- **Catalyst and EVAP margins**: how close a passing test was to failing.
- **Repair verification** without waiting for the MIL to go out ([[readiness-monitors]]).
- **A baseline** to compare at the next visit.

It is read-only. Service 06 is already in carlink's read list ([[adapter-safety]]). This page replaces
the "phase 2 / layout unverified" note in [[obd2-services]]. For what the two Hondas actually report,
see [[honda-mode06]].

The standard (SAE J1979 / ISO 15031-5 and its J1979-DA tables) is paywalled. Copies of it turned up
on forums but were not opened. The layout below is pieced together from these sources:

- the free ISO 15031-5 publisher sample (K-line format)
- CARB's OBD II regulation
- the Snap-on Global OBD manual
- python-OBD's tables, read for facts only (it is GPL)
- captures from a Toyota (CAN) and a Corolla (K-line)
- manufacturer documents (Honda, GM, Toyota, Ford)

Confidence is `community` unless a line says otherwise.

## Two incompatible formats

| | **CAN** (ISO 15765-4), e.g. 2016 Odyssey | **K-line / J1850** (ISO 9141-2, ISO 14230-4), e.g. 2004 CR-V |
|---|---|---|
| What is requested | `06 <OBDMID>`, a standardized monitor ID | `06 <TID>`, a manufacturer test ID |
| One test result | 9 bytes: `MID TID UASID value(2) min(2) max(2)` | 6 bytes after `46`: `TID TLTCID value(2) limit(2)` |
| Scaling | standardized by the Unit-and-Scaling ID (UASID) | raw counts; the conversion is the manufacturer's |
| Limits | both min and max, always | one limit per message; bit 7 of TLTCID says min or max |
| Per-cylinder misfire | standardized (MID `A2`–`AD`, TID `0B`/`0C`) | not standardized; absent from Honda's tables |

Sources: icanhack, ISO 15031-5 sample, Snap-on manual, MOTOR 2007, RLEscalambre.

## CAN: request and response

### Supported MIDs

Ask `06 00`, `06 20`, `06 40`, `06 60`, `06 80`, `06 A0`, `06 C0` and `06 E0`. Each answer is a single
frame `46 <base> A B C D`. It uses the same bitmap layout as service 01 PID 00 ([[obd2-pids]]): bit A7 =
MID base+1 … bit D0 = MID base+0x20. D0 set means the next bitmap is supported, so keep walking while D0 = 1.

Toyota Auris data. This comes from the ELM327-emulator project's "car" scenario, which its README says
reproduces a Toyota Auris Hybrid's traffic. Whether each frame is a verbatim capture is not stated, so
it is `community`. "Toyota capture" below means this data.

| Request | Reply | Meaning |
|---|---|---|
| `06 00` | `46 00 C0 00 00 01` | MIDs 01, 02, and the 20 range exists |
| `06 20` | `46 20 80 00 80 01` | MIDs 21 (catalyst B1) and 31 (EGR B1), and the 40 range exists |
| `06 40`, `06 60`, `06 80` | `… 00 00 00 01` | nothing in that range; keep going |
| `06 A0` | `46 A0 F8 00 00 00` | MIDs A1–A5: general misfire plus 4 cylinders. End of chain |

Send **one bitmap or one test MID per request**. J1979 may allow several supported-MID IDs in one CAN
request, but that is not verified here, so do not depend on it.

### Test results

`06 <MID>` returns one message, sent once:

```
46 | MID TID UASID TVhi TVlo MINhi MINlo MAXhi MAXlo | MID TID UASID … | …
```

- `46` appears once. After it comes one **9-byte record per test**, and **the MID is repeated at the
  start of every record**. Evidence: python-OBD parses fixed 9-byte blocks starting with the MID, and the
  Toyota capture shows `A2 0B 24 …  A2 0C 24 …`. The icanhack layout table confirms the field order and
  that values are 16-bit big-endian.
- The length is 1 + 9n ≥ 10 bytes, so a result **always needs ISO-TP multi-frame**. The first frame is
  `10 LL`, where LL is the payload length. Then come `21`, `22`, …. Cut the payload at LL bytes to drop
  padding, and drop any partial record left at the end. The adapter sends flow control; see
  [[adapter-response-format]] for how the ELM/STN prints this with headers on and off.
- A functional request can be answered by more than one ECU. **Group by CAN ID.** On the Odyssey only
  `7E8` is expected ([[odyssey-2016]]).
- Unsupported MID: a search excerpt of J1979 says the ECU does not respond (`unverified`). Treat both
  `NO DATA` and a `7F 06 xx` negative response as "not supported". Only ask for MIDs the bitmap lists.

### Worked examples

**Misfire, cylinder 1** (Toyota capture, headers on):

```
> 06A2
7E8 10 13 46 A2 0B 24 00 00
7E8 21 00 00 FF FF A2 0C 24
7E8 22 00 00 00 00 FF FF 00      ← last byte is padding (payload length 0x13 = 19)
```

Payload: `46 | A2 0B 24 0000 0000 FFFF | A2 0C 24 0000 0000 FFFF`

| MID | TID | UASID | value | min | max | Reading |
|---|---|---|---|---|---|---|
| A2 (cyl 1) | 0B | 24 (counts) | 0 | 0 | 65535 | average over the last 10 drive cycles: 0 |
| A2 (cyl 1) | 0C | 24 | 0 | 0 | 65535 | this or the last drive cycle: 0 |

**Same layout with non-zero counts.** This one is built from the numbers in an OdyClub post, so it is
illustrative, not a capture. `46 A5 0B 24 00 3B 0000 FFFF A5 0C 24 00 60 0000 FFFF` decodes to
cylinder 4: average 59, current cycle 96.

**Catalyst bank 1** (Toyota capture): `06 21` → `7E8 10 0A 46 21 A9 86 02 E5` / `7E8 21 02 D0 7F FF 00 00 00`

- Payload: `46 | 21 A9 86 02E5 02D0 7FFF`. TID A9 is Toyota-defined. UASID 86 is signed, 0.000305/bit.
- Value 741 → **0.2260**. Min 720 → **0.2196**. Max 32767 → 9.994.
- Result: **pass**, about 3 % above its minimum.
- The upper limit is `7FFF`, the largest *signed* value, where the unsigned UASID 24 uses `FFFF`. That
  fits the signed/unsigned split of the UASID table.

**O2 sensor B1S1** (python-OBD test vector, standard TIDs):
`46 | 01 01 0A 0BB0 0BB0 0BB0 | 01 05 10 0048 0000 0064 | 01 85 24 0096 004B FFFF`

- TID 01, rich-to-lean threshold: 2992 × 0.122 mV = **365 mV**. A constant reports value = min = max.
- TID 05, rich-to-lean switch time: **72 ms**, limits 0–100 ms. Pass.
- TID 85 (manufacturer-defined): **150 counts**, minimum 75. Pass.

## Pass, fail and "not run"

1. **Value, min and max all `0000` means the test has not run** since codes were cleared. It is not a
   pass. CARB 1968.2 (g)(4.5.4)(D) requires zeros for monitors that have not completed since a clear,
   and the Snap-on manual says the same. For monitors with several pass/fail criteria, the tests not
   used in the latest decision also report zeros (CARB (g)(4.5.4)(E), phased in over MY2009–2011).
   `verified` (regulation)
2. Otherwise, **pass ⇔ min ≤ value ≤ max**. CARB requires both limits, and a value equal to either limit
   passes ((g)(4.5.4)(B)). `verified`
3. **Compare in engineering units after applying the UASID's sign.** For signed IDs (0x81–0xFE) the raw
   words must be read as i16. An unsigned compare treats a negative value (`8000`–`FFFF`) as huge.
4. Results survive key-off. They are replaced by a newer result or zeroed by a service 04 clear
   ((g)(4.5.3) and (D); Snap-on; MOTOR). **Read service 06 before any clear** ([[clearing-codes]]).
5. Check readiness first (PID 01/41, [[readiness-monitors]]). The ISO 15031-5 example says values may be
   invalid if the test has not completed.
6. **Misfire TIDs `0B`/`0C` carry limits 0…65535, so they always "pass".** The counts are the
   information. Do not print PASS next to them. Sources: Toyota capture, Torque Pro readouts, SGMW
   declaration.
7. Makers publish exceptions. Per a search summary of GM's CAN mode $06 document (`unverified`), some
   GM values must be divided by 1000, and for some MID/TID pairs a failing value can equal the limit.

## Standardized OBDMIDs

From python-OBD's table. It is cross-checked against RLEscalambre (MID 21 = catalyst B1 for every
maker; A2 = cylinder 1 … A7 = cylinder 6), x-engineer's examples (31 EGR B1, 21 catalyst B1, 81 fuel
system B1, A4 cylinder 3) and MOTOR (catalyst in MIDs 21–24). CARB forbids the maker-defined range
`E1`–`FF` for required results ((g)(4.5.4)(A)).

| MID | Monitor | MID | Monitor |
|---|---|---|---|
| 00 20 40 60 80 A0 C0 E0 | supported-MID bitmaps | 41–44 | O2 sensor **heater** B1 S1–S4 |
| 01–04 | O2 sensor B1 S1–S4 | 45–48 | O2 heater B2 S1–S4 |
| 05–08 | O2 sensor B2 S1–S4 | 49–4C / 4D–50 | O2 heater B3 / B4 |
| 09–0C / 0D–10 | O2 sensor B3 / B4 | 61–64 | heated catalyst B1–B4 |
| 21–24 | catalyst B1–B4 | 71–74 | secondary air 1–4 |
| 31–34 | EGR B1–B4 | 81–84 | fuel system B1–B4 |
| 35–38 | VVT B1–B4 | 85–86 | boost pressure control B1–B2 |
| 39 | EVAP 0.150″ (cap off) | 90–91 | NOx absorber B1–B2 |
| 3A | EVAP 0.090″ | 98–99 | NOx catalyst B1–B2 |
| 3B | EVAP 0.040″ | **A1** | **misfire, general** |
| 3C | EVAP 0.020″ | **A2–AD** | **misfire cylinder 1–12** |
| 3D | purge flow | B0–B1 | PM filter B1–B2 |
| | | E1–FF | manufacturer-defined |

The O2 MIDs cover wideband A/F sensors too. Other IDs are reserved.

## Standardized test IDs (TIDs)

| TID | Meaning |
|---|---|
| 00 | reserved |
| 01 / 02 | rich→lean / lean→rich sensor threshold voltage (constant) |
| 03 / 04 | low / high sensor voltage for switch-time calculation (constant) |
| 05 / 06 | rich→lean / lean→rich switch time (calculated) |
| 07 / 08 | minimum / maximum sensor voltage for the test cycle |
| 09 | time between sensor transitions |
| 0A | sensor period |
| **0B** | **EWMA misfire count over the last 10 driving cycles**: `0.1 × this cycle + 0.9 × previous`, starting from 0, rounded to an integer |
| **0C** | **misfire count for the current / last driving cycle** |
| 0D–7F | reserved |
| 80–FE | manufacturer-defined |

Sources:

- TIDs 01–0A are service 05's O2 TIDs, reused on CAN under the O2 MIDs (Snap-on).
- The EWMA formula is Snap-on's, matching a search summary of a Toyota manual.
- 80–FE: python-OBD, plus the SGMW declaration via a search summary.
- On TID 0C, a Toyota 2GR-FE manual (via search summary) says it shows the *current* cycle while the
  engine runs and the *last* cycle with the key on and engine off. Read it with the engine running at the
  end of a test drive. `community`
- MID A1 TID 0C is the total for all cylinders in the current cycle (RLEscalambre). Some makers put their
  own TIDs under A1: Ford uses 80–82, total misfire rate against its thresholds.

## Unit and Scaling IDs (UASID)

Decode: take the 16-bit big-endian word. For a signed ID, read it as two's complement (i16). Then
`value = raw × scale + offset`. Apply the same conversion to value, min and max.

The source is python-OBD's table (J1979 Appendix E as implemented). Unsigned IDs run 0x01–0x41, signed
IDs 0x81–0xFE. Many signed IDs reuse the scaling of the unsigned ID 0x80 lower, but not all: 87 is ppm
where 07 is rpm, and 99, AE, B0 and B1 differ too. **Never derive a missing ID from that pattern.**

★ = this UASID appears in the misfire, catalyst or O2 examples gathered here: the Toyota Auris scenario
data, the GM document rows (via search summary), and the python-OBD test vector.

**Unsigned**

| ID | Scale/bit | Unit | ID | Scale/bit | Unit |
|---|---|---|---|---|---|
| 01 | 1 | raw | 21 | 1 | mHz |
| 02 | 0.1 | raw | 22 | 1 | Hz |
| 03 | 0.01 | raw | 23 | 1 | kHz |
| 04 | 0.001 | raw | **24** ★ | 1 | **counts** (misfire) |
| 05 ★ | 0.0000305 | raw (0–1.999; GM catalyst "OSC ratio") | 25 | 1 | km |
| 06 | 0.000305 | raw (0–19.99) | 26 | 0.1 | mV/ms |
| 07 | 0.25 | rpm | 27 | 0.01 | g/s |
| 08 | 0.01 | km/h | 28 | 1 | g/s |
| 09 | 1 | km/h | 29 | 0.25 | Pa/s |
| **0A** ★ | 0.122 | **mV** | 2A | 0.001 | kg/h |
| **0B** ★ | 0.001 | **V** | 2B | 1 | counts (switches) |
| 0C | 0.01 | V | 2C | 0.01 | g/cylinder |
| 0D | 0.00390625 | mA | 2D | 0.01 | mg/stroke |
| 0E | 0.001 | A | 2E | — | boolean (non-zero = true) |
| 0F | 0.01 | A | 2F | 0.01 | % |
| **10** ★ | 1 | **ms** | 30 | 0.001526 | % (0–100) |
| 11 | 100 | ms | 31 | 0.001 | L |
| 12 | 1 | s | 32 | 0.0000305 | inch |
| 13 | 1 | mΩ | 33 | 0.00024414 | λ (0–16) |
| 14 | 1 | Ω | 34 | 1 | min |
| 15 | 1 | kΩ | 35 | 10 | ms |
| 16 | 0.1, **offset −40** | °C | 36 / 37 / 38 | 0.01 / 0.1 / 1 | g |
| 17 | 0.01 | kPa (gauge) | 39 | 0.01, **offset −327.68** | % |
| 18 | 0.0117 | kPa | 3A / 3B | 0.001 / 0.0001 | g |
| 19 | 0.079 | kPa | 3C | 0.1 | µs |
| 1A | 1 | kPa | 3D | 0.01 | mA |
| 1B | 10 | kPa | 3E | 0.00006103516 | mm² |
| 1C | 0.01 | ° | 3F | 0.01 | L |
| 1D | 0.5 | ° | 40 | 1 | ppm |
| 1E | 0.0000305 | λ (0–1.999) | 41 | 0.01 | µA |
| 1F | 0.05 | A/F ratio | | | |
| 20 | 0.00390625 | ratio | | | |

**Signed** (i16)

| ID | Scale/bit | Unit | ID | Scale/bit | Unit |
|---|---|---|---|---|---|
| 81 | 1 | raw | 96 | 0.1 (no offset) | °C |
| 82 / 83 / 84 | 0.1 / 0.01 / 0.001 | raw | 99 | 0.1 | kPa |
| 85 | 0.0000305 | raw | 9C / 9D | 0.01 / 0.5 | ° |
| **86** ★ | 0.000305 | raw (Toyota catalyst) | A8 | 1 | g/s |
| 87 | 1 | ppm | A9 | 0.25 | Pa/s |
| 8A | 0.122 | mV | AD / AE | 0.01 / 0.1 | mg/stroke |
| 8B / 8C | 0.001 / 0.01 | V | AF | 0.01 | % |
| **8D** ★ | 0.00390625 | **mA** (Toyota A/F sensor) | B0 | 0.003052 | % (−100…100) |
| 8E | 0.001 | A | B1 | 2 | mV/s |
| 90 | 1 | ms | FC / FD | 0.01 / 0.001 | kPa |
| | | | FE | 0.25 | Pa |

An ID missing from both tables is unknown. Show the raw hex, and give no unit and no verdict, because the
sign is unknown too.

## K-line / J1850 format (the 2004 CR-V)

- `06 00` → `46 00 FF A B C D`: the supported-TID bitmap, with an `FF` in the TLTCID position. Seen on a
  Toyota Corolla over ISO 9141-2 as `46 00 FF 80 00 00 00`, meaning only TID 01 is supported.
- `06 <TID>` → **one or more** 7-byte messages `46 TID TLTCID TVhi TVlo TLhi TLlo`, one per component and
  limit type. Corolla capture: `46 01 05 00 01 00 73`.
- **TLTCID bit 7 is the limit type.** 0 = the limit is a maximum, so the test fails if value > limit.
  1 = the limit is a minimum, so it fails if value < limit. **Bits 6–0 are the component ID**, which the
  manufacturer defines. Value and limit are unsigned 0–65535 (ISO 15031-5 sample, Table 77–81). `verified`
- ISO sample example: TID 02, TLTCID `84` (minimum, CID 04), value `0010` ≥ limit `0000` → pass.
  TLTCID `16` (maximum, CID 16), value `0032` > limit `0020` → fail.
- Values are **raw and unscaled**. TIDs and CIDs belong to the manufacturer (Snap-on, MOTOR), and the
  same TID means different things per maker: Toyota TID 01 is the catalyst, Ford TID 01 an O2 test.
- Pass/fail can be judged on the raw numbers whenever the maker's conversion is increasing. That holds
  for every conversion in Honda's 2005 table (`k·x` or `k·x − c`, k > 0); it is an inference for other
  makers. The label and units still need the maker's table.
- Every message fits one K-line frame (7 data bytes), so a TID costs one ordinary request/response
  exchange, like a service 01 PID. The adapter cannot know how many component messages will follow, so
  it waits out its response timeout after the last one. The exact time per TID on the CR-V is
  `unverified`.

## What carlink should do

1. **CAN cars:** walk the MID bitmaps, request each supported MID, split the reply into 9-byte records,
   decode with the UASID table, and give each test one verdict: *not run* (all zero), *pass*, *FAIL*, or
   *info* (misfire 0B/0C).
   - Unknown TID → "manufacturer test 0xNN".
   - Unknown UASID → raw hex, no verdict.
2. **Show misfire as a per-cylinder table** (0B average, 0C current), never as pass/fail. Thresholds are
   in [[honda-mode06]] and [[live-data-interpretation]].
3. **Read before clearing, and store the results with the scan**, so a later visit can compare.
4. **K-line cars:** an optional raw dump of TID, CID, limit type, raw value, raw limit and pass/fail. It
   is cheap to add; labels come from the maker's table where one exists ([[honda-mode06]]).
5. EV (Kona): no combustion monitors. Expect service 06 to be unsupported (`unverified`).

## Not verified

- Whether one CAN request may carry several supported-MID IDs, and the exact reply to an unsupported MID.
- The UASID table is python-OBD's implementation of J1979 Appendix E, not the appendix itself. IDs it
  lacks (e.g. 0x42–0x7F, 0x88, 0x89) may exist in newer J1979-DA revisions.
- GM's divide-by-1000 and "fail can equal the limit" exceptions come from a search summary; the GM
  document is now behind a sign-in.
- TID 0C current-versus-last behaviour is documented for Toyota only.

## Related
[[obd2-services]] · [[honda-mode06]] · [[readiness-monitors]] · [[clearing-codes]] ·
[[adapter-response-format]] · [[live-data-interpretation]] · [[obd2-pids]]
