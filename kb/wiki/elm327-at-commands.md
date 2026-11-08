---
title: ELM327 AT commands (as implemented by OBDLink)
type: concept
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/obdlink-frpm-rev-f.md
  - raw/elm327-datasheet.md
tags: [elm327, at-commands, adapter, init]
confidence: verified
---

# ELM327 AT commands (as implemented by OBDLink)

The [[obdlink-ex]] emulates the ELM327 command set and identifies as `ELM327 v1.4b`. Facts
below come from the OBDLink FRPM rev F §6–7 (authoritative for the STN chip) and the Elm
Electronics ELM327 datasheet (ELM327DSK). Where they differ, **the FRPM wins** for this
adapter. Several AT commands are "deprecated" on OBDLink — still supported, but an ST
equivalent exists ([[stn-st-commands]]).

## Wire protocol (FRPM §6.0) — verified

- Commands are ASCII, terminated by **CR (0x0D)**. Spaces are ignored; case-insensitive.
- Only alphanumerics, backspace and CR are accepted. OBD requests are hex digits only.
- Responses end with CR; `ATL1` adds LF. The adapter prints **`>`** when ready — always wait
  for it before sending the next command.
- A bare CR **repeats the last command** — never send an empty line as a "ping" after a
  request you did not mean to repeat.
- Any character received while a request is in progress aborts it and prints `STOPPED`
  (line feeds are ignored by current firmware) (FRPM §9.0).

## Commands carlink needs

| Command | Effect | Response | Notes / source |
|---------|--------|----------|----------------|
| `ATZ` | Full reboot (LED test) | blank line(s), `ELM327 v1.4b`, `>` | FRPM ATZ. Resets UART to the *saved* baud. |
| `ATWS` | Warm start, skips LED test, keeps current baud | `ELM327 v1.4b` | FRPM ATWS. Prefer over ATZ. |
| `ATD` | Restore runtime defaults without reboot (headers, filters, timeouts, protocol) | `OK` | FRPM ATD |
| `ATE0` | Echo off (default on) | `OK` (may be echoed once) | FRPM ATE |
| `ATL0` | No LF after CR (default off) | `OK` | FRPM ATL |
| `ATS0` / `ATS1` | Spaces off/on between bytes (default on) | `OK` | FRPM ATS. S1 is easier to parse/log; S0 is faster. |
| `ATH1` | Show headers, check byte (non-CAN) and CAN PCI byte | `OK` | FRPM ATH. Needed to separate ECUs. |
| `ATCAF1` | CAN auto-formatting (default on): adds PCI to requests, strips PCI/padding from replies | `OK` | FRPM ATCAF. **ATH1 + CAF1 is the FRPM-recommended combination.** |
| `ATAT1` | Adaptive timing, normal (default). `ATAT2` aggressive, `ATAT0` fixed | `OK` | FRPM ATAT |
| `ATST hh` | No-response timeout = hh × 4 ms (OBDLink default 102 ms via PP 03 = 0x19) | `OK` | FRPM ATST, PP 03. Deprecated → `STPTO ms`. ELM's own default is 0x32 ≈ 200 ms. |
| `ATSP 0` | Protocol = automatic; **saved to NVM** | `OK` | FRPM ATSP. See note below. |
| `ATTP n` | Try protocol without writing NVM until it connects | `OK` | ELM DS "Selecting Protocols"; deprecated on OBDLink → `STP`. |
| `ATDP` | Describe protocol, e.g. `AUTO, ISO 9141-2` | text | ELM DS DP. Deprecated → `STPRS`. |
| `ATDPN` | Protocol number, `A` prefix if auto, e.g. `A3` / `A6` | text | ELM DS DPN. Deprecated → `STPR`. |
| `ATRV` | Voltage at DLC pin 16, e.g. `12.5V` (one decimal) | text | ELM DS RV; FRPM: deprecated → `STVR`. |
| `ATSH hdr` | Set request header. 11-bit CAN `ATSH 7E0`; 29-bit `ATSH 18 DA 10 F1`; ISO 9141 `ATSH 68 6A F1` | `OK` | FRPM ATSH |
| `ATCRA id` | CAN receive filter (pattern with mask all-1s; `X` = don't care, e.g. `ATCRA 7EX`). `ATCRA` with no arg resets to auto | `OK` | FRPM ATCRA |
| `ATAR` | Reset filters to automatic | `OK` | FRPM §11 |
| `ATI` / `AT@1` | ID string / device description | text | FRPM |
| `ATKW` | Print ISO keywords from last init | e.g. `1:08 2:08` | FRPM ATKW (format from ELM DS — unverified on EX) |
| `ATSI` / `ATFI` | Force ISO 5-baud / KWP fast init now | `BUS INIT: ...OK` | FRPM; ELM DS "Bus Initiation" |
| `ATSW hh` | ISO/KWP keep-alive interval hh × 20 ms; `ATSW 00` stops keep-alive | `OK` | FRPM ATSW |
| `ATCS` | CAN TX/RX error counters | text | FRPM |

### ELM protocol numbers (ELM DS Figure 2; FRPM ATSP)

`0` auto · `1` J1850 PWM · `2` J1850 VPW · `3` ISO 9141-2 (5-baud init, 10.4k) · `4` KWP
5-baud init · `5` KWP fast init · `6` CAN 11-bit 500k · `7` CAN 29-bit 500k · `8` CAN 11-bit
250k · `9` CAN 29-bit 250k · `A` J1939 · `B` USER1 CAN · `C` USER2 CAN.

### `ATSP` writes non-volatile memory

On OBDLink, `ATSP p` "set[s] the OBD protocol preset and save[s] it in NVM"; with memory on
(`ATM1`, default) the *last detected* protocol is also saved and tried first after reset.
`ATSP 00` clears the saved value back to automatic (FRPM ATSP, ATM). `ATSP 0` restores the
factory behaviour and is safe; `ATSP <non-zero>` leaves the adapter pinned to one protocol
for every later session and app — use `STP p` (volatile) to force a protocol instead.

## Recommended init sequence

Derived from the FRPM recommendations (ATH1+CAF1, wait for prompt, ATWS keeps baud) — the
exact order is a design choice, not quoted from a source:

```
\r              → flush; read until '>'  (ignore whatever came back)
ATWS            → expect "ELM327 v1.4b" then '>'   (use ATZ only if ATWS fails)
ATE0            → "OK"  (the ATE0 line itself may still echo)
ATL0            → "OK"
ATS1            → "OK"   (keep spaces: unambiguous parsing)
ATH1            → "OK"
ATCAF1          → "OK"   (default; set explicitly)
ATAT1           → "OK"
STI             → "STN2232 v5.x.y"   (confirms an STN chip; ELM clones answer "?")
STVR            → "12.34"  (or ATRV → "12.3V"); < ~3 V ⇒ no vehicle power on pin 16
ATSP0           → "OK"   (auto search; writes the factory-default value to NVM — harmless)
0100            → may print "SEARCHING..." first, then responses, or an error
ATDPN / STPR    → which protocol was found
```

For a known car, skip the search: e.g. `STP 22` (ISO 9141-2) or `STP 33` (CAN 11-bit 500k),
then `STPO` or just send the request. See [[stn-st-commands]].

## Timing and keep-alive

| Item | Value | Source / confidence |
|------|-------|---------------------|
| ISO 9141 slow (5-baud) init | "2 to 3 second" process; KWP fast init much quicker (25 ms low + 25 ms high + wake message) | ELM DS "Bus Initiation"; FRPM STIFI, PP 1A/1B — verified |
| Keep-alive | Bus sleeps if idle ~5 s; adapter sends wake-up `68 6A F1 01 00` (ISO 9141) / `C1 33 F1 3E` (KWP) every ~3 s automatically (OBDLink PP 17 default 0x92 × 20.48 ms ≈ 2.99 s); responses are not printed. STP presets 21/23 (no autoinit) send none | ELM DS; FRPM PP 17, STP — verified |
| No-response timeout | 102 ms default (PP 03); max via `ATST FF` ≈ 1 s or `STPTO` up to 65 s | FRPM — verified |
| Response pending | adapter extends to 5 s per `7F xx 78` (CAN, KWP) | ELM DS; FRPM PP 2A — verified |
| Auto search | tries protocols in J1978 order with built-in delays (e.g. 999 ms between 1 and 2, 819 ms after 5) — a full `ATSP0` search with ISO init attempts can take many seconds | FRPM PP 13/18/19 — verified; total duration *unverified* |
| ISO P3 (gap before next request) | 56 ms default | FRPM STPTRQ — verified |

**Host-side read timeouts (recommendation, not from a source):** 2 s for AT/ST commands;
5 s for `ATZ`; 5 s for an ordinary request on an open protocol (covers one response-pending
extension — reset the timer on every received byte); **20–30 s for the first request after
`ATSP0`** or any request that triggers an ISO 5-baud init. Never send another byte while
waiting — it aborts the command with `STOPPED`.

## Related

[[adapter-response-format]] · [[adapter-safety]] · [[macos-serial-setup]]
