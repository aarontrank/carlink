---
title: Adapter response format (ELM327 / OBDLink)
type: concept
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/obdlink-frpm-rev-f.md
  - raw/elm327-datasheet.md
  - url:https://ereding.etsi.us.es/bibing/proyectos/abreproy/90766/fichero/Simulador+de+la+ECU+de+un+veh%EDculo+sobre+OBDII+con+protocolo+ISO+9141-2.pdf
  - url:https://obd-cable.com/kwp2000-code-explained-k-line-scanner/
tags: [parsing, can, iso-tp, iso9141, kwp, headers]
confidence: verified
---

# Adapter response format (ELM327 / OBDLink)

How the [[obdlink-ex]] prints bus traffic, so `carlink` can split a response into per-ECU
messages. Assumes the init in [[elm327-at-commands]]: `ATE0 ATL0 ATS1 ATH1 ATCAF1`.

## General framing — verified (FRPM §6.0, §9.0)

- Each line ends in CR (CR LF with `ATL1`). Blank lines occur; skip them.
- A response is complete when **`>`** arrives (it is not followed by CR).
- Status lines that may precede data: `SEARCHING...` (auto protocol search, ELM DS
  "Selecting Protocols"), `BUS INIT: ...` followed by `OK` or an error (ISO/KWP when not
  auto-searching; dots only for slow 5-baud init, ELM DS "Bus Initiation").
- Terminal/error lines — treat as the whole result: see table at the end.
- If echo is still on, the first line is the command itself; strip a line equal to the
  command sent.

## CAN (ISO 15765-4), headers on

Line = `<CAN ID> <PCI> <data…>`. 11-bit IDs print as 3 hex digits (`7E8`), 29-bit as 4 bytes
(`18 DA F1 10`). With `ATD1` a DLC digit appears after the ID (off by default). Padding is
stripped under CAF1. (FRPM ATH, ATCAF, ATD)

- 11-bit OBD: requests go to functional `7DF` (or physical `7E0`–`7E7`); ECU *n* answers on
  `7E8`–`7EF` (`7E8` = ECU#1 / engine). Auto flow-control replies to RxID − 8 (FRPM §12;
  ELM DS "Multiline Responses").
- 29-bit OBD: responses `18 DA F1 xx` where `xx` is the ECU address; FC filter default
  `18DAF100/1FFFFF00` (FRPM §11.2).

**PCI byte (first byte after the ID)** — ISO 15765-2, as printed with ATH1 (ELM DS "CAN
Message Types"; FRPM ATCAF):

| PCI high nibble | Frame | Layout |
|-----------------|-------|--------|
| `0` | Single frame | low nibble = data length (1–7) |
| `1` | First frame | 12-bit total length = low nibble + next byte (e.g. `10 14` = 0x014 = 20 bytes) |
| `2` | Consecutive | low nibble = sequence 1…F, then 0… wraps |
| `3` | Flow control | normally not printed (adapter sends FC itself) |

Single frame example (`0100`, two ECUs, FRPM/ELM style):
```
7E8 06 41 00 BE 3E B8 11
7E9 06 41 00 80 10 80 00
```
Multi-frame, **ATH1 + CAF1** (FRPM ATCAF example):
```
>0902
7E8 10 14 49 02 01 31 47 31
7E8 21 4A 43 35 34 34 34 52
7E8 22 37 32 35 32 33 36 37
```
Frames from different ECUs **interleave** — group by CAN ID first, then order by sequence
nibble (ELM DS 09 04 example shows `7E8`/`7E9` frames mixed).

**ATH0 + CAF1** (no headers) prints the byte count on its own line and prefixes frames with
`n:` — unusable for separating ECUs (ELM DS shows two ECUs' groups mixed):
```
014
0: 49 02 01 31 47 31
1: 4A 43 35 34 34 34 52
2: 37 32 35 32 33 36 37
```
**CAF0**: you must supply the PCI byte in requests (`02 01 00`), and replies print raw
including padding (FRPM ATCAF). Not recommended. `STCSEGR 1` instead makes the adapter
reassemble multi-frame replies into one line (FRPM §8.8) — *not yet tested here*.

Response-pending: `7F xx 78` (negative response, "busy"). With PP 2A bit 2 set (default)
the adapter extends its timeout to 5 s for CAN and KWP; keep reading until `>` (ELM DS
"Response Pending Messages"; FRPM PP 2A) — *verified*.

## ISO 9141-2 / ISO 14230-4 (K-line), headers on

Line = `<H1> <H2> <H3> <data…> <CS>`: three header bytes, up to 7 data bytes, one checksum.
H1 = priority/format, H2 = target, H3 = **source** (transmitter) (ELM DS Figure 3 "An OBD
Message"; ELM DS 01 00 example: "the third byte shows ECU 10 … and ECU 18").

- ISO 9141-2 request header default `68 6A F1` (tester F1); responses `48 6B <ECU>`
  (ELM DS AT WM default "68 6A F1 01 00"; etsi.us.es thesis citing SAE J1979) — *verified
  for request, community for response pattern*.
- KWP2000 default request header `C1 33 F1` (fast-init wake message `C1 33 F1 81` + CS);
  responses typically `8x F1 <ECU>` where the low 6 bits of H1 give the length (FRPM ATSH
  KWP header table; FRPM auto filter `80F100/C0FF00` = format 10xxxxxx, target F1).
- **Checksum** (last byte) = sum of all preceding bytes mod 256 (ISO 9141-2 / 14230; not a
  CRC) — *community, two sources*. J1850 uses CRC-8 instead, so the ELM DS `48 6B 10 41 00
  … FA` example (J1850) does not sum-check.
- Example (computed): `48 6B 10 41 0C 1A F8 22` — 0x48+0x6B+0x10+0x41+0x0C+0x1A+0xF8 =
  0x222 → CS `22`. Engine ECU `10` reporting RPM `1A F8`.
- The adapter already validates the checksum; a failing one is printed with `<DATA ERROR`
  after the line (FRPM §9). Parsers should still drop the last byte as CS, not data.
- Multi-ECU: one line per ECU, separated by H3. Long ISO 9141 responses (e.g. VIN `0902`)
  arrive as several lines with a message-count byte after the PID: `49 02 01 …`, `49 02 02
  …` (ELM DS multiline VIN example).

## Parser rules (summary)

1. Read until `>`; split on CR/LF; drop empty lines and the echo of the command.
2. If any line equals a known error string (table below), return that error.
3. Drop `SEARCHING...`; treat `BUS INIT: ...OK` as progress, `BUS INIT: ...ERROR` as error.
4. Tokenise remaining lines on spaces into hex bytes. If the protocol (from `STPR`/`ATDPN`)
   is CAN 11-bit: first token is the 3-digit ID; 29-bit: first 4 tokens are the ID. Then PCI,
   then data; reassemble ISO-TP per ID.
5. K-line: first 3 tokens header, last token checksum (verify), middle = data; key by H3.
6. A data line followed by `<DATA ERROR` / `<RX ERROR` is corrupt — discard and retry.

## Status and error strings (FRPM §9.0; ELM DS "Error Messages and Alerts")

| String | Meaning |
|--------|---------|
| `?` | invalid / out-of-context command |
| `OK` | AT/ST command accepted |
| `NO DATA` | no reply before timeout (unsupported PID, filters, timeout too short) |
| `UNABLE TO CONNECT` | protocol detection failed (ignition off, no pin-16 power, wiring) |
| `CAN ERROR` | CAN TX/RX trouble: not on bus, wrong rate, wiring — also seen with car off on an EX |
| `BUS INIT: ...ERROR` | ISO/KWP init failed (FRPM ATKW) |
| `BUS BUSY` / `BUS ERROR` / `FB ERROR` | bus stuck / not toggling / K-line feedback mismatch |
| `DATA ERROR` / `<DATA ERROR` | framing error / previous line failed checksum/CRC |
| `<RX ERROR` | CAN receive error, usually wrong baud |
| `BUFFER FULL` | adapter RX buffer overflow (use filters, ATS0/ATH0, higher UART baud) |
| `STOPPED` | a UART character interrupted the command |
| `FC RX TIMEOUT` | no ISO-TP flow-control frame (OBDLink only) |
| `OUT OF MEMORY` | filters/messages exhausted RAM (OBDLink only) |
| `UART RX OVERFLOW` | host sent too much during ISO keep-alives (OBDLink only) |
| `LV RESET` | brown-out reset; adapter rebooted — re-run init |
| `ACT ALERT`, `LP ALERT` (opt. `!` prefix) | PowerSave about to sleep |
| `ERRxx` (e.g. `ERR94`) | ELM327 internal error codes; not listed in the OBDLink FRPM — treat as fatal |

Related: [[stn-st-commands]] · [[adapter-safety]]
