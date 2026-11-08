---
title: Service 06 on the Hondas — Odyssey misfire counters (CAN) and CR-V K-line test IDs
type: analysis
created: 2026-11-08
updated: 2026-11-08
sources:
  - raw/honda/honda-mode06-2005-all-models.md
  - raw/honda/honda-mode06-2005-all-models.pdf
  - raw/carb-13ccr-1968.2-2006-test-results-excerpt.md
  - raw/mode06-and-load-community-notes-2026-11-08.md
  - raw/aa1car-obd2-mode06-diagnostics-2005.md
  - raw/motor-advanced-mode06-diagnostics-2007-excerpt.md
  - raw/rlescalambre-mode06-test-results.md
  - raw/wikipedia-variable-cylinder-management-2026-11-08.wikitext
  - raw/tirereview-troubleshooting-wideband-o2-2011.md
  - raw/motor-lean-on-me-fuel-trim.md
  - raw/obdb-hyundai-kona-electric.md
  - raw/honda/a25-061-vcm-misfire-warranty-extension-memo-2025-06-17.pdf
tags: [honda, odyssey-2016, crv-2004, kona-electric-2020, mode06, misfire, vcm, evap, catalyst]
confidence: community
---

# Service 06 on Aaron's cars

The format and decoding rules are in [[obd2-mode06]]. This page covers what each car is expected to
report and how to read it.

| Car | Bus | What service 06 gives | Value |
|---|---|---|---|
| [[odyssey-2016]] | CAN | Standardized MIDs, including **per-cylinder misfire counts A2–A7** | **High.** This is the VCM misfire tool |
| [[crv-2004]] | ISO 9141-2 | Honda-defined TID/CID results, raw values, **no misfire** | Low–medium |
| [[kona-electric-2020]] | CAN | Probably nothing (no combustion monitors) | None |

## 2016 Odyssey: per-cylinder misfire counters

**Why they should be there.** CARB's OBD II rule makes CAN cars from MY2005 report misfire results "in
the standardized format specified in SAE J1979" (13 CCR 1968.2 (g)(4.5.4)(C); `verified`). That means MID
`A1` (all cylinders), `A2`–`A7` (cylinders 1–6), TID `0B` (10-cycle average) and TID `0C` (this cycle),
with UASID `24` (counts).

OdyClub owners post exactly these readouts from Odysseys. One example: "A5 0B 59 counts, A5 0C 96 counts …
A7 0B 31 counts … A2 through A4 zero counts" alongside P0304/P0305/P0306. A 2015 owner, same generation
as the 2016, saw low counts on all six cylinders before any code set. `community`. The exact MID and TID
set on this van is **unverified until it is read**. Record the `06 A0` bitmap here when it is.

| MID | Cylinder | Bank | VCM |
|---|---|---|---|
| A1 | all (TID 0C = total this cycle) | — | — |
| A2 / A3 / A4 | 1 / 2 / 3 | **bank 1, rear** (firewall side) | **deactivated** in 3-cylinder mode |
| A5 / A6 / A7 | 4 / 5 / 6 | bank 2, front (radiator side) | always firing in 3-cylinder mode |

Bank 1 = rear is from the P3400 "bank 1 rear" text in [[honda-common-dtcs]]. Wikipedia says VCM
deactivates the rear bank of three. The 2011–2017 Odyssey has VCM-2, which also has a 4-cylinder mode;
which cylinders that mode drops is not documented here.

### Reading the counts

- **TID 0B** is an exponentially weighted average: each new cycle adds 0.1 × its count to 0.9 × the old
  average. One bad drive with 100 misfires raises 0B by about 10, and 0B then fades over several clean
  drives. **After a repair, judge by 0C on a fresh drive, not by 0B.**
- **TID 0C** is this cycle while the engine runs. With the engine off it is probably the *last* cycle,
  as on Toyota (`community`). Read it with the engine running at the end of the test drive.
- Levels (AA1Car):
  - zero or close to it is normal;
  - "a few occasional misfires (say less than 100 over a given time period)" is usually acceptable;
  - "several hundred or a thousand misfires for a given cylinder" means an ignition, fuel or compression
    problem;
  - a P030x typically sets once one cylinder's misfire **rate** passes about 2 %. `community`
- CARB judges misfire as a percentage of firings over 200-revolution windows (catalyst damage) and
  1000-revolution windows (emissions). `verified`
- **Scale, derived here and `unverified`:** a 20-minute drive at about 1,800 rpm is about 36,000 crank
  revolutions, which is about 18,000 firings per cylinder (each cylinder fires every second revolution).
  At 2 %, that is about 360 misfires on one cylinder in one drive. So counts in the low hundreds on one
  cylinder are already near code-setting territory on a short drive.
- **Deactivated cylinders are not misfires.** CARB's definition excludes "lack of combustion events in
  non-active cylinders due to … cylinder deactivation strategies", so VCM operation itself adds no counts.
  `verified`

### Patterns → next step

| Pattern | Likely meaning | Next step |
|---|---|---|
| Counts mostly on **A2–A4 (cylinders 1–3)**, 0C rising | **VCM oil-fouling pattern**: oil past the rings on the deactivated bank fouls plugs | Check oil consumption and the dipstick; pull plugs 1–3; check warranty extension **25-061** ([[honda-common-dtcs]]) |
| One cylinder only | Coil, plug or injector on that cylinder | Swap that coil with a neighbour, drive, re-read **0C**: if the count moves with the coil, it is the coil |
| Whole **bank 2** (A5–A7) | Bank-wide cause. OdyClub replies: low compression in one cylinder plus per-bank trims, or bank-2 fuel delivery | Compression test; bank-2 trims ([[live-data-interpretation]]) |
| Low counts on all six | Shared cause: fuel, ignition supply, crank signal, possibly VCM transitions (OdyClub, disputed) | Fuel trims and fuel pressure; do not replace parts on counts alone |

Misfire leaves unburned oxygen in the exhaust, so **a misfiring bank reads lean and its fuel trim goes
positive** (Tire Review; MOTOR "Lean On Me"). Cylinder 1–3 misfire counts plus a positive bank-1 trim tell
one consistent story. Read the counters **before any clear**: a clear zeroes them, as it does every
service 06 result (CARB (g)(4.5.4)(D)).

### Honda-defined tests on the Odyssey

No public source names Honda's manufacturer TIDs for the 2016's catalyst, O2, EVAP or misfire monitors,
or any TID aimed at VCM. MOTOR (2007) says Honda publishes its mode $06 definitions on its service-
information site. HDS shows live per-cylinder misfire counters through Honda's enhanced data
([[odyssey-2016]]). Until Honda's chart is found, show unknown TIDs as "Honda test 0xNN" with their
standardized UASID scaling. The pass/fail verdict is still valid without the name.

## 2004 CR-V: K-line service 06

The CR-V uses the pre-CAN format: `46 TID TLTCID value limit`, with Honda's own TIDs and conversions
([[obd2-mode06]]). The closest public document is Honda's "SAE J1979 Mode $06 Test Information: All
2005 Hondas" (raw/honda/honda-mode06-2005-all-models.md). Its **CR-V column covers the 2005 CR-V**, the
same K24A1 generation. The model columns were assigned from character positions in the PDF.

| Monitor | TID | TLTCID (limit) | DTC | Honda description (shortened) | Conversion of value and limit |
|---|---|---|---|---|---|
| Catalyst B1 | `62` | `04` (max) | P0420 | moving average from the downstream O2 sensor ("standard value … after passing through catalyst") | no unit |
| A/F sensor B1 | `76` | `80` (doc lists a max limit; see note) | P0133 | A/F sensor response integration value | no unit |
| A/F sensor B1 | `78` | `80` (min) | P1172 | A/F sensor output during A/F control | raw × 0.00390625 − 128 **mA** |
| A/F heater B1 | `70` / `71` / `72` | `80` (min) / `00` (max) / `00` (max) | P0135 | heater current with heater on / on / off | raw × 0.01 **A** |
| Rear HO2S heater | `18` / `19` | `89` (min) / `09` (max) | P0141 | heater current | raw × 0.01 A |
| Intake air leak | `48` | `01` (max) | P2279 | estimated intake air vs air through the idle air control valve at idle | raw × 0.10 **L/min** |
| Thermostat | `4D` / `4E` / `4F` | `81` (min) | P0128 | coolant temperature under three driving-condition criteria | raw − 40 **°C** |
| Thermostat | `50` | `01` (max) | P0128 | estimated minus actual coolant temperature | raw × 1 °C |
| EVAP | `81` | `80` (min) | P0496 | tank pressure, vent shut valve closed after a cold start | raw × 0.1 − 3276.8 **mmHg** |
| EVAP | `82` / `8F` | `80` (min) | P2422 | tank pressure during purge / at engine start | same |
| EVAP | `88` | `00` (max) | P0497 | tank pressure when the tank fails to decompress after a cold start | same |
| EVAP | `8B` / `8D` | `00` (max) | P0457 | decompression failure / leak-check pressure difference | same |
| EVAP | `90`, `91`, `92`, `93` | `00`, `80`, `80`, `00` | P0442 | leak check: pressures (90–92) and pressure change rate (93) | `90`–`92` as above; `93` raw × 0.1 **mmHg/min** |
| EVAP | `94`, `95`, `96`, `97` | `00`, `80`, `00`, `00` | P0456 | same tests, very small leak | `94`/`95` as above; `96`/`97` raw × 0.1 mmHg/min |

Notes:

- **No misfire tests** appear anywhere in Honda's 2005 pre-CAN tables. CARB's test-result requirement
  excludes misfire for these cars ((g)(4.5.1)); the standardized misfire results came with CAN. So **the
  CR-V cannot give per-cylinder misfire counts over generic OBD.** Use P030x codes, freeze frames and a
  coil swap.
- **2004 ≠ 2005 for EVAP.** The 2004 CR-V's EVAP faults are **P1456/P1457** ([[honda-common-dtcs]]). In
  the 2005 document those codes belong to the older-design TIDs that other models still used:
  - P1456: `21`, `26`, `27`, `38`, `3A`, `3B`, `3C`
  - P1457: `29`, `2E`, `2F`, `3E`
  - `2D` is P0497

  They are scaled ×0.488281 mmHg, some with −62.5 mmHg, and `3C` has no unit. So the 2004 probably
  reports those instead of `81`–`97` (inference, `unverified`). The engine-side rows (catalyst, A/F,
  heaters, thermostat, intake leak) probably carry over, also `unverified`.
- **TID `76` with TLTCID `80`.** Bit 7 says the limit is a minimum, but Honda's text lists only a
  "highest limit". Judge pass/fail by bit 7, which is what the PCM encodes, and show Honda's wording as
  the label.
- **TID `48` (intake air leak)** is the useful one for this speed-density engine. A vacuum leak barely
  moves the CR-V's fuel trims ([[live-data-interpretation]]), but this test reports the PCM's own
  estimate of unmetered idle air in L/min against its limit. If the 2004 supports it, read it first when
  a leak is suspected.
- **Worth implementing?** A raw dump (`06 00` bitmap, then one request per supported TID, parse each
  7-byte message, pass/fail from bit 7) is small and protocol-generic. The labels above are a best effort
  for a 2004. Priority is medium-low next to the Odyssey's misfire counters. Read `06 00` once on the car
  and record the bitmap here.

## 2020 Kona Electric

There is no engine, so there are no catalyst, O2, EVAP or misfire monitors. OBDb's real-car support
lists for the Kona's ECUs contain only service 01 PIDs `01`, `21`, `30`, `31` and `5B`
([[kona-electric-2020]]). Expect `06 00` to return `NO DATA`. `unverified`

## Related
[[obd2-mode06]] · [[odyssey-2016]] · [[crv-2004]] · [[honda-common-dtcs]] · [[live-data-interpretation]] ·
[[clearing-codes]]
