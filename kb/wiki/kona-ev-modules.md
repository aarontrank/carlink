---
title: Kona Electric — diagnostic modules, CAN IDs and the DTC service each one speaks
type: concept
created: 2026-11-08
updated: 2026-11-08
sources:
  - raw/projectgus-kona-ev-can-logs-diag.md
  - raw/obdb-kona-electric-modules-2026-11-08.md
  - raw/opendbc-fw-query-addresses-2026-11-08.md
  - raw/ovms-kianiroev-ecus-2026-11-08.md
  - raw/evdash-carkiaeniro-src.md
  - raw/hyundai-tsb-23-01-073h-1-kona-ev-dtc-inspection.md
  - raw/module-dtc-community-notes-2026-11-08.md
tags: [kona-electric, uds, kwp2000, dtc, ecu-addresses, abs, airbag, tpms, obc]
confidence: community
---

# Kona Electric — diagnostic modules and their DTC services

Which modules on the [[kona-electric-2020]] answer on the OBD port, which trouble-code service each
one speaks, and how they behave. The services, byte layouts and the do-not-clear list are on
[[uds-module-dtcs]]. Live-data reads (DIDs) for some of these modules are on [[kona-ev-other-ecus]]
and [[kona-ev-bms-pids]].

**Where the evidence comes from.** The best source is a set of public CAN captures by projectgus of
a **2021 Kona Electric "Highlander"** and of a **bench rig built from Kona modules**. The car is
crash-damaged, and the trim name suggests an Australian-market car. In those captures the Car
Scanner app scans every module for codes and then clears them
(raw/projectgus-kona-ev-can-logs-diag.md). It is the same 1st generation (OS EV) as Aaron's 2020 US
car: opendbc groups "Hyundai Kona Electric 2018-21", and OBDb's first generation is 2018–2023.
Fitted modules can still vary by market, trim and damage. A module that does not answer is not an
error. Nothing here has been observed on Aaron's car yet.

Status labels (the ones carlink asked for):
- **verified-for-Kona**: answered on a 1st-gen Kona in a real capture.
- **seen-on-sibling**: only documented for e-Niro, Soul EV or Ioniq, or for Hyundai in general.
- **guess**: no source.

Schema confidence for log-based rows is `community`: third-party captures, not Aaron's car.

## Module table (11-bit, 500 kbit/s, reply = request + 8)

| Req → Resp | Module | DTC service | Availability mask | Read in default session? | Status | Evidence |
|---|---|---|---|---|---|---|
| 7D1 → 7D9 | **IEB**: integrated electronic brake (ABS/ESC), firmware "OS IEB … 58520-K4000" | UDS | `89` | untested. It ignores functional requests; its read was logged after `10 03` | verified-for-Kona | 2021 log: `59 02 89` + 5 DTCs; opendbc `HYUNDAI_KONA_EV` 0x7d1; OBDb 2021–23 captures from 7D9; evDash; OVMS |
| 7D4 → 7DC | **MDPS**: electric power steering, "OS MDPS C" | UDS | `89`. **Rejects masks with unsupported bits** (`7F 19 31`) | yes (functional `19 02 08`) | verified-for-Kona | 2021 log; opendbc 0x7d4; OBDb 2021/2023 captures from 7DC |
| 7D2 → 7DA | module present. **Assumed SRS/airbag (SRSCM)**, a name with no source | UDS (`10 03` → `50 03`) | unknown | **no**: `7F 19 7F`. It ignores functional requests | present: verified-for-Kona; name: guess | 2021 log only |
| 7C4 → 7CC | **LKAS front camera**, "OSE LKAS" | UDS | `89` | yes (19-byte reply; the functional read lost its flow control) | verified-for-Kona | 2021 log; opendbc 0x7c4 |
| 7D0 → 7D8 | **SCC front radar**, "OSev SCC" | unknown (answers UDS-style `22 F100`) | — | no answer on the logged car (crash-damaged front; fitment by trim is unverified) | seen on Kona by opendbc; absent on the logged car | opendbc 0x7d0 |
| 7B7 → 7BF | unknown. openpilot calls 0x7B7 "cornerRadar" on Hyundai generally | UDS | `FF` | yes | present: verified-for-Kona; name: guess | 2021 log; opendbc `extra_ecus` |
| 7E2 → 7EA | **VCU / VMCU**: vehicle control | **KWP** (`18 00 FF 00`). UDS 0x19 → `7F 19 12` | — | yes | verified-for-Kona | 2021 log + bench; OBDb; OVMS (KWP `1A 80` VIN poll) |
| 7E3 → 7EB | **MCU**: motor inverter | **KWP**. UDS 0x19 → `7F 19 11` | — | yes | verified-for-Kona | 2021 log + bench; OBDb |
| 7E4 → 7EC | **BMS**. Service 09 0A name "BECM-B+EnergyCtrl" | UDS | `FF` | yes | verified-for-Kona | 2021 log + bench; TSB 23-01-073H-1 prints its DTCs as 3-byte UDS codes (`P1AA700`) |
| 7E5 → 7ED | **OBC / charger**. Service 09 0A name "BCCM-B+ChargerCtrl" | **KWP**. UDS 0x19 → `7F 19 11` | — | yes | verified-for-Kona | 2021 log + bench; OBDb 2021 capture |
| 770 → 778 | **IGPM**: integrated gateway & power module (body) | UDS | `09` | yes (23-byte reply) | verified-for-Kona | 2021 log; OBDb 2021/23; evDash; OVMS |
| 7A0 → 7A8 | **BCM**, which also holds the TPMS data (22C00B) | UDS. **Slow:** `7F 19 78` then the list ~0.5 s later (physical). To the functional request it sent only `78`s | `09` | yes, physically, after a `20`/`60` exchange | verified-for-Kona | 2021 log; OBDb; OVMS |
| 7A5 → 7AD | **Smart key unit (SMK)**, the name in OVMS's Soul EV header | UDS. `7F 19 78` then the list | `08` | yes | present: verified-for-Kona; name: seen-on-sibling | 2021 log; OVMS `kia_common.h` |
| 7B3 → 7BB | **HVAC (FATC)** | UDS | `09` | yes | verified-for-Kona | 2021 log; OBDb; evDash |
| 7B6 → 7BE | **E-Shifter**: shift-by-wire | UDS | `7F` | yes. Its **clear** was also accepted in the default session | verified-for-Kona | 2021 log + bench (Car Scanner's "E-Shifter" C1614(87) = 7BE `56 14 87`) |
| 7C6 → 7CE | **Instrument cluster** | UDS | `09` | yes | verified-for-Kona | 2021 log; OBDb; evDash |
| 725 → 72D | unknown | UDS | `09` | yes | present: verified-for-Kona; name unknown | 2021 log |
| 733 → 73B | unknown | UDS | `09` | yes (functional only) | present: verified-for-Kona; name unknown | 2021 log |
| 776 → 77E | unknown | UDS | `09` | logged after `10 03` | present: verified-for-Kona; name unknown | 2021 log |
| 780 → 788 | unknown | UDS. **Rejects masks with unsupported bits** | `09` | yes | present: verified-for-Kona; name unknown | 2021 log |
| 783 → 78B | unknown | UDS. `19 02 08` → `7F 19 22` (conditionsNotCorrect); `AC`/`AF` → `7F 19 31` | unknown | refused | present: verified-for-Kona; name unknown | 2021 log |
| 796 → 79E | unknown | UDS | `08` | yes | present: verified-for-Kona; name unknown | 2021 log |

**Do not scan (no module on the 1st-gen Kona):**
- 7E0, 7E1, 740: every read refused across the 2021–2024 captures, "no engine computer,
  transmission control module or 4WD module" (OBDb PR #367).
- 744: answers only in OBDb's 2024 (2nd-generation) captures.
- 7E6: no reply on the bench. OBDb lists it only in 2024, plus a 2022 support list that also claims
  7E0/7E1 generic Service 01. That 2022 list conflicts with PR #367 and with the logs, and probably
  mixes in captures from another Kona.
- 730, 7D6, 7E7: refused in OBDb; no reply in the log.
- 73E, 751, 752, 755, 757, 760, 771, 777, 791, 793, 794, 7A1–7A4, 7A6, 7A7, 7B1, 7C1, 7C5, 7C7,
  7D3, 7D5: Car Scanner probed each with `10 03` / `10 90` / `20` (some also with DTC requests) and
  got nothing.

Correction to [[kona-ev-other-ecus]]: its "7D4 → 7DC MDPS" row had no raw source. It is now backed
by OBDb's 2021 and 2023 captures and by opendbc.

## How the modules behaved (Observed in the 2021 log unless noted)

- **Default-session reads work.** Car Scanner's first request was a functional `19 02 08` on 7DF,
  before any session change. Lists came back from the BMS, MDPS, LKAS (first frame only), cluster,
  HVAC, IGPM (first frame only), 7A5 (after `78`), 7B7, 725, 733, 780 and 796. 783 refused it with
  `22`.
  - The BCM answered only with `78`s, repeated about every 2.5 s, and never delivered the list. It
    did deliver to a physical request.
  - The IEB and 7D2 never answered any functional request.
  So read every module by physical address.
- **KWP modules answer `18 00 FF 00` in the default session**: the VCU, MCU and OBC returned `58 00`
  to the functional request at t 24.375 s. They reject `18 02 FF 00` with `7F 18 12`.
- **Strict masks:** `19 02 AC` and `19 02 AF` → `7F 19 31` from the MDPS, 780 and 783. The cluster,
  HVAC, 733 and 796 accepted the same masks. Use `19 02 08` first ([[uds-module-dtcs]] §2.2).
- **Timing:** most replies arrive within about 30 ms. The BCM and 7A5 take 0.5–0.7 s behind
  `7F 19 78`. The UDS session reply `50 03 00 32 01 F4` (P2 50 ms, P2* 5 s) came from the BMS, 783,
  7B6, 7C4 and 7C6; 780 sent `50 03 00 32 00 FA` (P2* 2.5 s).
- **Multi-frame lists** come from the IEB, IGPM and BCM (23 bytes, 5 DTCs) and the LKAS (19
  bytes). The adapter must send flow control. The STN does it automatically to request ID =
  reply − 8, provided the filters are set in the order given in [[uds-module-dtcs]] §6.
- **7D2 (assumed SRS):**
  - `10 03` → `50 03`;
  - after `20` → `7F 20 78`, then `60`, a second `10 03` got no reply;
  - `19 02 08` got no reply;
  - `19 02 AC` → `7F 19 7F` (serviceNotSupportedInActiveSession);
  - `18 …` → `7F 18 11`.
  So it is UDS, and its DTCs are probably served only in a non-default session. That is an
  inference. The car's crash history may also matter.

### Example codes seen (formatting only; not interpreted)

The captured cars were crash-damaged or on a bench, so these are not typical Kona faults.

| Module | Raw record | Displayed as |
|---|---|---|
| IEB 7D9 | `56 B6 87 08`, `56 14 87 08` | C16B6-87, C1614-87 (confirmed) |
| MDPS 7DC | `58 12 87 08` | C1812-87 (confirmed) |
| IGPM 778 | `96 03 00 08`, `A8 01 00 08` | B1603-00, B2801-00 |
| BCM 7A8 | `92 10 00 09` | B1210-00 (failing now + confirmed) |
| Cluster 7CE | `90 01 52 09` | B1001-52 |
| HVAC 7BB | `92 38 13 09` | B1238-13 |
| BMS 7EC (2021 car) | `0A 0A 12 A8`, `0D 99 12 AF` | P0A0A-12, P0D99-12 |
| VCU 7EA (bench, KWP) | `1A 78 E8` | P1A78. KWP status `E8` = present during this cycle + present now + stored + test inhibited |

## Generic OBD on the Kona (Observed; adds to [[kona-electric-2020]])

| Request (7DF) | Who answered |
|---|---|
| `0100` | 7EB, 7EC only (matches OBDb) |
| `03`, `07` | **7EA, 7EB, 7EC and 7ED** (all `43 00` / `47 00` on these cars) |
| `0A` | 7EA, 7EB, 7ED (not 7EC) |
| `0902` (VIN) | only 7EC, and it returns the placeholder **`OSEV__ BMS `**, not a VIN. Ask the user for the VIN |
| `0904` (CALID) | 7EC `OSEVB6822R` or `OSEVB7022R` (it differed between two sessions); 7ED `EOSEK5L-OS2-D000` or `EOPE1DL-OS1-D000`; 7EA and 7EB empty |
| `090A` (ECU name) | 7EC "BECM-B+EnergyCtrl", 7ED "BCCM-B+ChargerCtrl"; 7EB empty |
| `04` (bench) | **7EC answered `44`**. 7EA, 7EB and 7ED answered `7F 04 22` |

This contradicts [[kona-electric-2020]] on two points:
- It expects Service 03 only from 7E3/7E4; the VCU (7EA) and OBC (7ED) also answer.
- It says Service 09 VIN is undocumented. VIN is in fact not available: only the BMS answers, with
  a placeholder.

It also contradicts [[kona-ev-resets]] on Service 04: that page says it clears the MCU and BMS. On
the bench only the BMS accepted it.

## Clearing on the Kona

Observed requests and replies:
- `14 FF FF FF` → `54`: BMS (real car and bench), E-Shifter (bench).
- `14 FF 00` → `54 FF 00`: VCU, MCU, OBC (bench).
- `14 40 00` → `54 40 00`: VCU and OBC, chassis group (bench).

The wrong form for the protocol is refused (`7F 14 13` / `7F 14 79` / `7F 14 12`). After a clear,
anything still faulty is back immediately. The 2021 BMS re-read P0D99-12 with status `27` within
12 s.

**Never clear the BMS (7E4).** That includes the generic Service 04 clear, which the BMS accepts.
Hyundai TSB 23-01-073H-1 (campaign 9A4, 2019–23 Kona Electric) bases the battery-replacement
decision on P1AA6's freeze-frame "Information Code 1–2" and on P1AA7–P1AB0. Leave the 7D2 module
(assumed SRS) and 7A5 (smart key) read-only as well. See [[uds-module-dtcs]] §8 and
[[kona-ev-recalls]].

## Suggested carlink module list (Kona)

Scan by physical address, in this order: 7E4 (UDS), 7E2 / 7E3 / 7E5 (KWP), 7D1, 7D4, 7D2, 7C4, 7D0,
7B7, 770, 7A0, 7A5, 7B3, 7B6, 7C6, 725, 733, 776, 780, 783, 796 (all UDS). A module that does not
answer goes in the report as "no answer" (not fitted, or asleep). Allow at least 5 s for 7A0 and
7A5.

## Not verified

- Every row on Aaron's US 2020.
- The 7D2 identity and how to read it.
- The names of 725, 733, 776, 780, 783, 796 and 7B7.
- Whether the IEB answers a physical `19 02 08` without `10 03`.
- Whether the strict modules accept `19 02 <avail & 0xAF>`.
- Whether the Service 04 refusals by the VCU, MCU and OBC (`7F 04 22`) happen on a complete car or
  only on the bench rig.
