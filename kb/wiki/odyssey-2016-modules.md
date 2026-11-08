---
title: 2016 Odyssey — modules beyond the PCM, 29-bit addressing, and what an OBDLink EX can reach
type: concept
created: 2026-11-08
updated: 2026-11-08
sources:
  - raw/obdb-honda-odyssey-pilot-29bit-2026-11-08.md
  - raw/opendbc-fw-query-addresses-2026-11-08.md
  - raw/honda-job-aid-06932-dlc-circuit-troubleshooting-2017.md
  - raw/module-dtc-community-notes-2026-11-08.md
  - raw/obdlink-frpm-rev-f.md
  - raw/elm327-datasheet.md
tags: [odyssey, honda, can, 29-bit, srs, vsa, eps, tpms, uds]
confidence: community
---

# 2016 Odyssey — modules and 29-bit access

**Bottom line.** For the 4th-generation Odyssey (2011–2017, which includes the
[[odyssey-2016]]), the only units documented on 29-bit CAN at the OBD port are the PCM's two
logical addresses: **0x11 (engine)** and **0x1D (automatic transmission)**. **No source documents
SRS, VSA/ABS, EPS, TPMS or gauge-cluster access over pins 6/14 on a 2016.**

- Those addresses (0x53, 0x28, 0x30, 0x26, 0x60) come from newer Hondas: the 2016+ Pilot and the
  2018+ Odyssey.
- On OBDb's real captures of 2014 and 2015 Odysseys and of 2009 and 2013 Pilots (same era), every
  read OBDb tried at these addresses was refused or never answered. It tried 0x26, 0x28 and 0x60 on
  all four, and 0x30 and 0x53 on the 2009 Pilot only.

So carlink cannot read airbag, ABS/VSA, steering or TPMS codes on this van by any documented means.
Report that plainly rather than guessing addresses.

## Honda's own description of the DLC (verified)

American Honda Job Aid 06932 (June 2017), "DLC Circuit Troubleshooting":
- It lists the **2007–14 Odyssey** under "Equipped with both F-CAN and K-Line".
- It states that "all models are equipped with a K-Line to DLC circuit, and some are equipped with an
  additional F-CAN to DLC circuit".
- Its F-CAN procedure checks the F-CAN pair between the PCM and the **gauge control module** (steps
  9–12).
- It also has the technician "identify all of the control modules on the F-CAN circuit" and
  disconnect them one at a time, "but do not disconnect the PCM and gauge control module" (steps
  6–7). So other modules share that CAN.
- Its K-line procedure likewise says to "identify all of the control modules on the K-Line circuit".

The HDS on this generation therefore uses **both** a CAN link and a K-line. The job aid does not say
which module is diagnosed over which link. Its list stops at 2014; whether the 2015–2017 Odyssey is
wired the same is not stated (unverified, but it is the same generation).

The STN's K-line modes are ISO 9141-2 and ISO 14230 framing, with optional custom init and baud. No
source found describes the protocol Honda's HDS uses on the K-line for body and chassis modules, so
treat those modules as unreachable (inference).

## Generic OBD on this generation is probably 29-bit

- OBDb's captures of **2011, 2013, 2014 and 2015 Odysseys** sent generic Service 01 on the 29-bit
  functional ID `18DB33F1`. **Two** units answered, `18DAF111` and `18DAF11D`; the 2018 car answered
  from `18DAF110` and `18DAF11E` (raw/obdb-honda-odyssey-pilot-29bit-2026-11-08.md).
- Owners' generic scan tools list the same two "modules", **$18DAF111** and **$18DAF11D**:
  - on a 2008 Odyssey, a 2012 and 2013 Pilot, and a 2010 Accord;
  - one OdyClub scan reports "P0118 … on Module $18DAF111" (community).
- An older Honda Fit log shows `AT SP 7` / "ISO 15765-4 (CAN 29/500)" with replies from
  `18 DA F1 10` (community).
- One OBDb 2011 entry also lists 11-bit `7E0` Service 01 support, but has no capture file. Some
  PCMs may answer both, so this is not settled.

**This contradicts [[odyssey-2016]]**, which expects 11-bit (`ATSP6`), a single `7E8`, and "probably
no `7E9`". The A/T side of the PCM is its own OBD responder (0x1D). The first `carlink probe` on the
van settles it:
- `ATDPN` → `A7`, or `STPR` → 34, means 29-bit.
- Mode 03/07/0A answers may then come from two IDs.

## Address table

| 29-bit request → reply | Unit | DTC service | Status for a 2016 | Evidence |
|---|---|---|---|---|
| `18DA11F1` → `18DAF111` | PCM, engine (PGM-FI) | unknown beyond OBD 03/07/0A. Service `22` DIDs work (`22 2611/2612/2660/2662`) | verified for the 4th generation (2011–2015 captures) | OBDb Odyssey 2011/2013/2014/2015; OdyClub |
| `18DA1DF1` → `18DAF11D` | PCM, automatic-transmission side | unknown beyond OBD 03/07/0A. `22 2201` (ATF temperature) and `22 2221` (gear) work | verified for the 4th generation | OBDb Odyssey 2011–2015 |
| `18DA10F1`, `18DA1EF1` | engine ECU and separate TCM on the 2018+ | — | not this generation: refused on 2011–2015 | OBDb Odyssey 2018 vs 2011–2015 |
| `18DA28F1` VSA (ABS), `18DA30F1` EPS, `18DA53F1` SRS, `18DA60F1` gauges, `18DAEFF1` gateway, `18DA26F1` TPMS, `18DA0BF1` shift-by-wire | newer-generation modules | unknown (newer Hondas answer UDS `22 F181`) | **seen-on-sibling only**. Unanswered or refused on 2014/2015 Odyssey (0x26, 0x28, 0x60), 2009 Pilot (0x26, 0x28, 0x30, 0x53, 0x60) and 2013 Pilot (0x26, 0x28, 0x60) | opendbc `HONDA_PILOT` (2016–22) and `HONDA_ODYSSEY` (2018–20); OBDb Pilot 2017 answers at 0x26/0x28/0x60 |

The PCM's own trouble codes, including A/T codes reported by 0x1D, already come back through
generic Service 03/07/0A, which `carlink scan` reads. A module-DTC feature adds nothing on this van
unless a path to the SRS or VSA is found.

## Read-only experiments worth running once on the van

None of these is documented; all are reads. Record the results here.

1. `ATDPN` and `STPR` after the normal probe, to settle 11-bit vs 29-bit.
2. To `18DA11F1` and `18DA1DF1`: `19 02 08`. `59 02 …` means UDS. `7F 19 11` means not UDS; then try
   `18 00 FF 00` and look for `58 …`.
3. To `18DA28F1`, `18DA30F1`, `18DA53F1`, `18DA60F1`: `22 F181`, the UDS application-software
   identifier that openpilot asks newer Hondas. Any reply proves that module is on CAN at the DLC.
   Silence fits the K-line / gateway picture above.

**Never send 0x14 to a Honda module.** Clear only via the generic Service 04 path that
[[clearing-codes]] already covers. TPMS on this van uses direct sensors. HDS talks to the TPMS unit
for sensor-ID registration, by a path that is not documented. Its warning clears by itself after
correct inflation and a drive; see [[odyssey-2016-tpms-calibration]].

## Talking 29-bit through the OBDLink EX

- **If the van is already on 29-bit** (`A7`/34): no protocol change is needed. Send `ATSH 18DA11F1`
  and `ATCRA 18DAF111`, then the request. Restore with `ATAR` and `ATSH 18DB33F1`, the 29-bit
  functional header.
- **If the session is 11-bit:** switch with `STP 34` and back with `STP 33`. The full sequence is in
  [[uds-module-dtcs]] §6. **Never `ATSP 7`** (it writes NVM), and avoid `ATTP 7` (it is saved once
  it connects, because ATM1 is the default) ([[adapter-safety]]).
- Flow control for multi-frame replies is automatic on 29-bit. The STN sends it to the reply ID with
  its last two bytes swapped, `18DA11F1` (FRPM §13.2).

## Not verified

- The 2016's OBD protocol: 11-bit or 29-bit.
- Which DTC service, if any, 0x11 and 0x1D answer.
- Whether any non-PCM module is on CAN at the DLC on 2015–2017.
- Which modules Honda put on the K-line, and with what protocol.
