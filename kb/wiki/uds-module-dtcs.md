---
title: Module trouble codes — UDS 0x19/0x14 and KWP2000 0x18/0x14 through an ELM/STN adapter
type: concept
created: 2026-11-08
updated: 2026-11-08
sources:
  - raw/projectgus-kona-ev-can-logs-diag.md
  - raw/iso14229-1-samples-dtc-excerpts.md
  - raw/py-uds-knowledge-base-excerpts.md
  - raw/kwp2000-daimlerchrysler-rd-2.2-dtc-excerpts.md
  - raw/iso-dis-14230-3-1996-dtc-excerpts.md
  - raw/hyundai-tsb-23-01-073h-1-kona-ev-dtc-inspection.md
  - raw/obdlink-frpm-rev-f.md
  - raw/elm327-datasheet.md
  - raw/module-dtc-community-notes-2026-11-08.md
tags: [uds, kwp2000, iso14229, iso14230, dtc, clearing, safety, sessions]
confidence: community
---

# Module trouble codes (UDS and KWP2000)

OBD services 03/07/0A and 04 ([[obd2-services]], [[clearing-codes]]) only reach the emissions
ECUs. Brakes, airbags, steering, body, climate and the EV charger keep their own fault memory. You
read it with **UDS service 0x19** (ISO 14229) or **KWP2000 service 0x18** (ISO 14230). Both clear
with **service 0x14**, but the two clear requests are formatted differently. This page covers the
services, the byte layouts, how to pick between them, and what must not be cleared. The per-car
address tables are in [[kona-ev-modules]], [[odyssey-2016-modules]] and [[crv-2004-modules]].

Evidence labels used below:
- **Observed**: seen in a real Kona's CAN traffic while the Car Scanner app read and cleared codes
  (raw/projectgus-kona-ev-can-logs-diag.md). These are third-party captures, not Aaron's car, so the
  schema confidence is `community`.
- **Standard**: from ISO sample pages, py-uds, or an OEM KWP spec.
- **Inference**: reasoned from the two, not tested.

## 1. Telling UDS and KWP modules apart

Decide per module from its reply. Do not decide from the address. On the Kona, two modules next to
each other use different protocols (7E2 is KWP, 7E4 is UDS). Observed replies:

| Reply to `19 02 08` | Meaning | Next step |
|---|---|---|
| `59 02 <avail> …` | UDS module, list follows | parse (§2) |
| `7F 19 11` serviceNotSupported, or `7F 19 12` subFunctionNotSupported | not a UDS DTC module. The Kona MCU and OBC send 11, **the Kona VCU sends 12** | send KWP `18 00 FF 00` (§3) |
| `7F 19 31` requestOutOfRange | the mask has bits this module does not support (§2.2) | retry with `08` |
| `7F 19 7F` serviceNotSupportedInActiveSession | UDS, but 0x19 is only served in another session (§5) | report it; see §5 |
| `7F 19 22` conditionsNotCorrect | refused for now (Kona module 783) | report "refused" |
| `7F 19 78` responsePending | still working (Kona BCM 7A0, module 7A5) | keep waiting; not the answer |
| nothing | not fitted, asleep, or it ignores that addressing (the Kona IEB and the 7D2 module never answered functional 7DF requests) | report "no answer". This is not an error |

| Reply to `18 00 FF 00` | Meaning |
|---|---|
| `58 <n> …` | KWP module, list follows (Kona VCU 7E2, MCU 7E3, OBC 7E5) |
| `7F 18 11` | not KWP (every Kona UDS module answered this way) |
| `7F 18 12` | sub-function/status byte not supported. **The Kona KWP modules return this for `18 02 FF 00`** |

The NRC meanings are from py-uds (raw/py-uds-knowledge-base-excerpts.md) and the ISO/DIS 14230-3
response-code table. KWP's `0x12` is "subFunctionNotSupported-invalidFormat" and `0x79` is
"incorrectByteCountDuringBlockTransfer" (raw/iso-dis-14230-3-1996-dtc-excerpts.md).

## 2. UDS read — ReadDTCInformation 0x19, sub-function 0x02

Request: `19 02 <DTCStatusMask>`. Positive response (ISO 14229-1:2013 Table 272; standard):

```
59 02 <DTCStatusAvailabilityMask> [DTC_hi DTC_mid DTC_lo statusOfDTC] × n
```

- When no DTC matches, nothing follows the availability byte (ISO 14229-1:2020 §12.3.1.3). For
  example, the Kona BMS answered `03 59 02 FF` (Observed).
- A DTC is reported when `(statusOfDTC & mask) != 0` (ISO 14229-1:2020 §12.3.1.3).
- Lists longer than 4 bytes are ISO-TP multi-frame. Five DTCs make a 23-byte reply
  (`10 17 59 02 …`), seen from the Kona IEB, IGPM and BCM (Observed). The adapter must send flow
  control (§6).
- ISO says a paged-buffer server may pad a list with `00 00 00 / 00` entries, and the client must
  ignore them (ISO 14229-1:2020 §12.3.1.1). Drop all-zero records.

### 2.1 Status byte (ISO 14229-1 Annex D.2, via py-uds; standard)

| Bit | Mask | Name | 1 means |
|---|---|---|---|
| 0 | 0x01 | testFailed | the most recent test failed (failing now) |
| 1 | 0x02 | testFailedThisOperationCycle | failed at least once this drive/ignition cycle |
| 2 | 0x04 | pendingDTC | failed this cycle or the last completed one |
| 3 | 0x08 | confirmedDTC | stored in long-term memory |
| 4 | 0x10 | testNotCompletedSinceLastClear | test has not run since the last clear |
| 5 | 0x20 | testFailedSinceLastClear | failed at least once since the last clear |
| 6 | 0x40 | testNotCompletedThisOperationCycle | test has not run this cycle |
| 7 | 0x80 | warningIndicatorRequested | the module asks for its warning lamp |

Observed example (Kona BMS, before and after a clear): `D1 28 00 AF` → U1128-00 with failing now,
failed this cycle, pending, confirmed, failed since clear and lamp requested. After the clear the
same fault read `27`, which keeps the failing bits but drops confirmed and lamp. Car Scanner's
labels for those two bytes match this table bit for bit.

### 2.2 Which status mask

- **The ISO rule:** "If the client specifies a status mask that contains bits that the server does
  not support, then the server shall process the DTC information using only the bits that it does
  support." (ISO 14229-1:2020 §12.3.1.3; standard)
- **What Kona modules actually do (Observed):** the cluster (7C6), HVAC (7B3) and modules 733 and
  796 follow the rule. The **MDPS 7D4** (availability `89`) and modules **780** (availability `09`)
  and **783** answered `7F 19 31` to masks `AC` and `AF`, because those masks include bits they do
  not support. So `19 02 FF` would very likely be refused by the MDPS too. That is an inference;
  `FF` itself was never sent in the logs.
- `19 02 08` (confirmedDTC) was accepted by every Kona UDS module that answered it (Observed). The
  exceptions: module 783 refused with `7F 19 22` (conditionsNotCorrect), and 7D2 did not answer.
  Bit 3 is in every availability mask seen: `FF`, `89`, `7F`, `09`, `08`.

**Recommended (inference):**
1. Send `19 02 08` and read the availability mask from byte 3 of the reply.
2. If `avail & 0xAF` has bits other than `0x08`, also send `19 02 <avail & 0xAF>`. That mask holds
   only bits the module says it supports, so even a strict module should accept it (inference;
   not observed). It adds codes that are failing or pending but not yet confirmed.
3. Leave out `0x10` and `0x40`. They mean "test not run yet", not "fault". With them in the mask, a
   module such as the Kona E-Shifter (availability `7F`) would list every code whose test has not
   run.

Car Scanner itself sends `08`, then `AC`, then `AF` (Observed).

## 3. KWP2000 read — ReadDiagnosticTroubleCodesByStatus 0x18

ISO/DIS 14230-3 §8.2 leaves the request parameters to the manufacturer. The DaimlerChrysler KWP2000
Requirements Definition 2.2 §3.6.2 defines `18 <statusOfDTC> <groupHi> <groupLo>`:
- `$00` = identified DTCs and status, SAE J2012 format
- `$02` = identified 2-byte hex DTCs and status
- group `$FF00` = all DTCs
(raw/kwp2000-daimlerchrysler-rd-2.2-dtc-excerpts.md; standard)

- **Hyundai uses `$00`.** The Kona VCU, MCU and OBC answer `18 00 FF 00` with `58 <n> …` and
  answer `18 02 FF 00` with `7F 18 12` (Observed, real 2021 car and bench). A KWP fallback that
  sends `18 02 FF 00` gets nothing from a Kona.
- Response: `58 <numberOfDTC> [DTC_hi DTC_lo statusOfDTC] × n`. It is multi-frame past 2 DTCs.
  Example (bench VCU, Observed):
  `58 04 1A 78 E8 C1 29 E4 C1 46 64 C1 64 64` → P1A78, U0129, U0146, U0164.
  The 2 bytes decode exactly like an OBD DTC ([[dtc-format]]), with no failure-type byte.
- One request returns every listed code whatever its status. After a clear the VCU still listed
  codes that were present but no longer stored.

### 3.1 KWP status byte — manufacturer-specific

ISO/DIS 14230-3 §8.2.1 says the statusOfDTC "Format and length … are vehicle manufacturer specific".
The DaimlerChrysler table puts bits 6–5 as a storage state and bit 7 as warning-lamp request.
**Hyundai's KWP modules do not follow it.** The Car Scanner labels, checked against the raw bytes
before and after a clear on the bench VCU and OBC, give this (Observed):

| Bit | Mask | Hyundai meaning (Car Scanner label) | DaimlerChrysler RD 2.2 |
|---|---|---|---|
| 7 | 0x80 | "Validated fault has been present during this drive cycle" | warning indicator requested |
| 6 | 0x40 | "Validated fault present at time of request" | storage state (high bit) |
| 5 | 0x20 | "Validated and stored in non volatile memory" | storage state (low bit) |
| 4 | 0x10 | not observed | readiness flag (test not complete) |
| 3 | 0x08 | "Test is inhibited by other DTC" | reserved |
| 2 | 0x04 | "Test conditions are met" | reserved |
| 1–0 | | not observed | reserved |

Practical reading: `0x40` set means failing now, `0x20` set means stored, and both set means an
active stored fault. Do not show bit 7 as "warning lamp" for a Hyundai.

## 4. Clearing — ClearDiagnosticInformation 0x14

| Protocol | Request | Positive reply | Source |
|---|---|---|---|
| UDS | `14 FF FF FF` (groupOfDTC = all) | `54` | ISO 14229-1 (via py-uds; 2020 example Table 300/301 uses group `FFFF33`); Observed on the Kona BMS and E-Shifter |
| KWP | `14 FF 00` (groupOfDTC = all) | `54 FF 00` (echoes the group) | DaimlerChrysler RD 2.2 §3.4; Observed on the Kona VCU, MCU and OBC |

- **The forms are not interchangeable (Observed).** Kona UDS modules answer `14 FF 00` with
  `7F 14 13` (incorrect length). Kona KWP modules answer `14 FF FF FF` with `7F 14 79` and
  `14 FF FF` with `7F 14 12`. Pick the form from the protocol the read established. Never guess.
- The 2020 edition adds an optional MemorySelection byte. Do not send it. Do not use the other
  groups (`FFFF33` emissions, `FFFFD0` safety systems). Reading all and clearing all keeps the logic
  simple.
- **What it erases:** "DTC status information shall be cleared upon a successful
  ClearDiagnosticInformation request" (ISO 14229-1:2020 §12.3.1.3), and with it the snapshot (freeze
  frame) data. It does not fix anything. On the real 2021 Kona, the BMS re-read 12 s after `54`
  already showed P0D99-12 with status `27`: failing again, no longer confirmed (Observed).
- **Default session is enough (Observed, bench Kona):** a functional `14 FF FF FF`, sent more than
  2 minutes after any session change, was acknowledged `54` by the BMS and the E-Shifter. A
  functional `14 FF 00` was acknowledged `54 FF 00` by the VCU, MCU and OBC. ISO 14229 lists 0x14
  and 0x19 as available in the default session as well as in non-default ones. That comes from
  ISO 14229:2006 Table 24 as reported in a search excerpt; the table was not fetched (unverified).

## 5. Sessions, tester-present and timing

- **No keep-alive is needed in the default session.** The S3 timer only returns a server from a
  *non-default* session to default after 5000 ms without a request (py-uds `S3Server = 5000ms`;
  standard). The ELM/STN sends no keep-alive on CAN by itself; its automatic keep-alive is for
  K-line only ([[elm327-at-commands]]).
- If a non-default session is ever used (`10 03` → `50 03 P2hi P2lo P2*hi P2*lo`; the Kona BMS sent
  `50 03 00 32 01 F4`, so P2 = 50 ms and P2* = 5 s), the host must send `3E 00` (or `3E 80`, no
  reply) more often than every 5 s (py-uds S3client default 2 s). It ends with `10 01`. One request
  per ELM command is fine: the host interleaves `3E80`. Do not use the STN periodic-message commands
  (`STPPMA`), which [[adapter-safety]] bans.
- **When a session is needed:** NRC `0x7F` "shall only be used when the requested service is known
  to be supported in another session" (py-uds). The Kona module at 7D2, assumed to be the airbag
  unit, answered `19 02 AC` with `7F 19 7F` (Observed). That came after a `20` (stopDiagnosticSession)
  had been answered `60`, which presumably put it back in the default session. A later `20` drew
  `7F 20 7F`, which fits a module already in default. So its DTCs are very likely readable only after `10 03`
  (inference, not observed). The other Kona UDS modules answered `19 02 08` in the default session
  (Observed, functional request at t 24.14 s), with three caveats:
  - The BCM sent only `78`s to that request. It delivered its list later to a physical request
    made after a `20`/`60` exchange, so presumably also in the default session.
  - Module 783 refused with `22`.
  - The IEB 7D1 ignores functional requests, and its only logged read came after `10 03`.
- **responsePending:** the Kona BCM and module 7A5 send `7F 19 78` and answer 0.5–0.7 s later when
  addressed physically. Addressed functionally, the BCM repeated `78` about every 2.5 s
  (Observed). The STN extends its timeout on each `7F xx 78` (FRPM PP 2A, enabled by default). Give
  module reads a host timeout of at least 5 s, reset on each received line, and never treat a `78`
  line as the answer.
- Car Scanner also sends KWP-style `10 90` and `20` (stopDiagnosticSession) to every module. carlink
  needs neither: the Kona KWP modules answer `18 00 FF 00` in the default session (Observed).

## 6. Addressing on the OBDLink EX

- **Use physical addressing, one module at a time.** With functional `7DF`:
  - some modules never answer: the Kona IEB and 7D2 (Observed);
  - the Kona BCM sent only repeated `7F 19 78` and never delivered its list (Observed);
  - multi-frame answers lose their flow control: the 7CC and 778 replies stopped after the first
    frame (Observed);
  - with the default `7DF` header, the STN's automatic flow-control filter is `7E8`/mask `7F8`
    (FRPM §11.2). Only `7E8`–`7EF` are handled as ISO 15765 replies; anything else needs custom
    filters;
  - one functional clear reaches several modules at once (§8).
- **11-bit physical:** `ATSH <req>` and `ATCRA <req+8>`, the request, then restore with **`ATAR`**
  and `ATSH 7DF`. FRPM §11.2 documents only `ATAR` as restoring the automatic filtering mode;
  `ATCRA` without an argument only resets the hardware filter. As of 2026-11-08, carlink's
  `with_header` (src/adapter.rs) sends `ATSH`, `ATCRA`, `ATFCSH`/`ATFCSD300000`/`ATFCSM1`, and
  restores with `ATFCSM0`, `ATAR`, `ATSH7DF`, which matches. The "restore `ATCRA`" wording on
  [[carlink-cli]] is out of date. For 11-bit, the STN sends flow control automatically to
  RxID − 8 (FRPM §12), which is right for every Hyundai pair (reply = request + 8).
- **Order matters (inference from FRPM §11.2):**
  - In automatic filtering mode, `ATSH` re-derives the flow-control filter from the new header.
  - Setting a hardware filter (`ATCRA`) switches automatic mode off.
  So send `ATSH` **before** `ATCRA`, and end each module with `ATAR`. Otherwise the flow-control
  filter stays on whatever it was last derived from: `7E8`/`7F8`, or the previous module's ID. A
  multi-frame reply from, say, `7D9` would then get no flow control and would stop after its first
  frame, silently losing DTCs. Test it once on the adapter with a module that returns a long list
  (the IEB's 23-byte reply).
- The belt-and-braces alternative is the explicit flow-control recipe already used for the BMS in
  [[kona-ev-bms-pids]]: `ATFCSH <req>`, `ATFCSD 300000`, `ATFCSM1`, in that order. `ATFCSH` also
  accepts 29-bit IDs (FRPM). A reply whose ISO-TP length says more bytes than arrived must be
  reported as a failed read, never parsed as a short DTC list.
- **29-bit physical (Honda):** header `18DA<tgt>F1`, reply `18DAF1<tgt>`. Flow control is automatic
  in "normal fixed" addressing: the STN sends it to the reply ID with its last two bytes swapped
  (FRPM §13.2). A 4-byte `ATSH` sets all 29 bits, including priority `18` (FRPM ATSH; ELM DS).

### Temporarily switching an 11-bit session to 29-bit (OBDLink EX)

From FRPM §8.6: "for a given CAN preset, the protocol … and Tx ID size (11/29 bit) are hard-set",
and "All CAN presets are configured to receive both 11-bit and 29-bit messages". Transmitting 29-bit
therefore needs a different preset. Each step and its basis:

```
STPR             → note the current preset (e.g. 33). If `ATDPN` already says 7 / A7, skip the switch
STP 34           → ISO 15765, 29-bit Tx, 500 kbps, DLC 8 (FRPM STP table)
ATSH 18DA11F1    → 29-bit physical header, tester F1 (FRPM ATSH 4-byte form)
ATCRA 18DAF111   → hardware filter: replies from that module only (FRPM ATCRA example "18 DA F1 10")
<request>        → e.g. 0101, 22xxxx, 1902 08
ATAR             → back to automatic filters (FRPM §11.2)
STP 33           → back to the noted preset (33 = ISO 15765 11-bit 500k)
ATSH 7DF         → back to the functional 11-bit header
```

- **Never `ATSP 7`.** ATSP saves to NVM (FRPM ATSP; [[adapter-safety]]).
- **Avoid `ATTP 7`.** It is deprecated on OBDLink. The ELM datasheet says that with memory on (ATM1,
  the OBDLink default) a valid protocol found by TP "will then be stored in memory as the new
  default".
- `STP` is documented without any NVM write. ATM only records protocols that were *detected*. So
  `STP` is volatile by omission, not by explicit statement. Check it once on the adapter: after the
  session, run `ATZ` and confirm `STPR`/`ATDPN` show the old value (unverified on hardware).
- If a module also needs a non-default session on 29-bit, the same §5 rules apply.

## 7. Showing a DTC

- **UDS (3 bytes):** the first 2 bytes give the usual 5 characters ([[dtc-format]]) and the third
  byte is the failure-type byte (FTB). Display it as `C1611-00` or `U1128-00`, the py-uds/SAE J2012
  form "P0123-45" (standard). Hyundai prints the same 7 characters without the dash: `P1AA700` in
  TSB 23-01-073H-1, and `P1A6F00` in Car Scanner's Hyundai descriptions. Car Scanner's list view
  shows `U1128(00) [0xD12800]`. Look descriptions up by the first 5 characters, then by all 7. Keep
  the raw 6 hex digits in the stored record.
- **KWP (2 bytes):** 5 characters, no FTB, e.g. `P1A78` and `U0129` (from `C1 29`).
- **State word:** for UDS, "failing now" if bit 0 is set, else "stored" if bit 3, else "pending" if
  bit 2, else "history" if bit 5. For Hyundai KWP, "failing now" if `0x40`, and "stored" if `0x20`.
  Always keep the raw status byte next to the word.

## 8. What carlink must not clear

| Target | Why | Basis |
|---|---|---|
| **Anything, functionally (`7DF` / `18DB33F1`)**, with 0x14 *or* OBD service 04 | One functional `14 FF FF FF` cleared the BMS and E-Shifter together. A functional `04` was acknowledged `44` by the **BMS** (the VCU, MCU and OBC refused with `7F 04 22`). On a Kona, carlink's existing service-04 clear therefore reaches the BMS | Observed (bench) |
| **Kona BMS 7E4**, by 0x14 or by service 04 | Hyundai campaign 9A4 (TSB 23-01-073H-1) has the dealer read the freeze-frame "Information Code 1–2" of P1AA6 and look for P1AA7–P1AB0 to decide whether the battery pack must be replaced. Those codes cover cell-voltage deviation, insulation resistance, over-temperature, over-/under-voltage and similar faults. A `14 FF FF FF` clear erases exactly that evidence. Whether service 04 alone erases those manufacturer codes is **not established**: the bench sent `04` and `14` within a second of each other. Block both. The BMS also logs crash-related codes, e.g. P1A6F00 "Airbag Deployment Signal Circuit High" on the bench | verified (TSB) + Observed |
| **Airbag / SRS** (Kona 7D2 if that is the SRS; Honda SRS is not reachable) | Clearing turns off the only warning that an airbag may not deploy. Crash records are not removed by a DTC clear anyway (community: one-time variant coding, crash data in EEPROM, vendors sell bench resets). On the Kona even the read path is unproven (`7F 19 7F`) | community / inference |
| **Smart key / immobilizer** (Kona 7A5 per OVMS naming) | No source shows that 0x14 touches key or immobilizer registration; it is DTC memory only. But no repair workflow needs carlink to clear it, so keep it read-only | inference (policy) |
| **Any module while driving** | Clears belong parked, after the repair. ECUs may refuse with `0x22` | policy |

Everything else (IGPM, BCM, cluster, HVAC, IEB, MDPS, E-Shifter, VCU, MCU, OBC) can be cleared only
when all of these hold:
- a human explicitly asks;
- a full read of that module was saved first;
- the request goes to that one module by physical address.

Each clear runs once per module. Re-read afterwards and report anything that came straight back as
**still failing**. The generic consequences in [[clearing-codes]] also apply.

## 9. Not verified

- Whether `19 02 <avail & 0xAF>` is accepted by the strict Kona modules. It is derived from the ISO
  rule; only `08`, `AC` and `AF` were observed.
- The Kona SRS read path, and whether `10 03` is enough or the module wants `10 04`
  (safetySystemDiagnosticSession; py-uds lists 0x04).
- Whether the Kona IEB answers a *physical* `19 02 08` in the default session. Its only logged read
  came after `10 03`.
- ISO 14229's default-session table (0x14/0x19 in default session). It was seen only in a search
  excerpt of the 2006 edition, but is consistent with the observations above.
- That `STP` never persists, on a real OBDLink EX.
- Hyundai KWP status bits 4, 1 and 0.
