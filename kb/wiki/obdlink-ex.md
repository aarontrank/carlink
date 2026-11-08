---
title: OBDLink EX
type: entity
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/obdlink-frpm-rev-f.md
  - raw/elm327-datasheet.md
  - url:https://www.obdlink.com/products/obdlink-ex/
  - url:https://www.obdlink.com/products/obdlink-mxp/
  - url:https://github.com/nicremo/open-obd-gui/pull/4
  - url:https://www.f150lightningforum.com/forum/threads/guide-forscan-on-apple-silicon-macbook-air.35117/
tags: [device, adapter, stn, usb, ftdi]
confidence: verified
---

# OBDLink EX

ScanTool.net / OBD Solutions' **wired USB** OBD-II adapter, sold as "optimized for Ford /
FORScan". It is the adapter `carlink` talks to. It speaks the [[elm327-at-commands]] set plus
the OBDLink [[stn-st-commands]] extensions over a USB virtual serial port.

## Identity

| Fact | Value | Source | Confidence |
|------|-------|--------|-----------|
| Device IDs | STN2230 (out of production), STN2231, **STN2232 (active)** — "Low cost OBD to USB with support for MS CAN" | FRPM §3.1 | verified |
| Firmware string (`STI`) | e.g. `STN2232 v5.10.3`; one EX on macOS reported `STN2232 v5.12.4` | FRPM §8.4; open-obd-gui PR #4 | verified / community |
| Hardware string (`STDI`) | e.g. `OBDLink EX r1.0.0` (FRPM `STDIX` example); `r2.7.1` seen in the field | FRPM §8.4; PR #4 | verified / community |
| `ATI` response | `ELM327 v1.4b` (the highest ELM version *fully* supported; newer ELM commands may still work) | FRPM §6.0, ATI | verified |
| Serial number (`STSN`) | 12 digits, starts with the device ID (e.g. `2232…`) | FRPM §8.4 STSN | verified |
| USB bridge | **FTDI**, VID `0x0403` / PID `0x6015` (FT-X series) | PR #4 | community |
| Max message (STPX) | 4 KB; max recommended UART baud 2 Mbps | FRPM §8.6 STPX table | verified |
| UART default baud | 115200 bps for USB OBDLink adapters; 8N1 | FRPM §6.0 | verified |

## Networks and protocols

Legislated: ISO 15765-4 CAN (11/29-bit, 250/500 kbps), ISO 14230-4 KWP2000 (5-baud and fast
init), ISO 9141-2, SAE J1850 VPW and PWM. OEM: **Ford MS-CAN** (125 kbps, OBD pins 3/11), with
an electronic HS-CAN/MS-CAN switch. (obdlink.com EX product page; FRPM §8.6 `STP` tables)
— *verified*.

**EX vs MX+:** the MX+ (STN2255/2256) adds **GM Single-Wire CAN (SW-CAN / GMLAN, pin 1,
33.3 kbps)**; the EX does not list SW-CAN anywhere (FRPM §3.1 device descriptions;
obdlink.com MX+ page "MS-CAN and SW-CAN" vs EX page "MS-CAN" only) — *verified by omission*.
Only one CAN channel can be active at a time on any OBDLink (FRPM §8.6).

STP protocol numbers relevant to the EX: 22 = ISO 9141-2, 24/25 = KWP 5-baud/fast, 33/34 =
HS-CAN 11/29-bit 500k, 35/36 = 250k, 53/54 = MS-CAN 11/29-bit 125k (FRPM §8.6). `STP 6x`
(SW-CAN) should be refused with `?` on an EX — *unverified, not tested*.

### Relevance to Aaron's vehicles

- **2004 Honda CR-V** — legislated OBD is K-line (ISO 9141-2 / KWP, pin 7). The EX supports
  this. *Inference from Honda DLC pinouts (pin 7 = K-line); confirm with `ATDPN` on the car.*
- **2016 Honda Odyssey / 2020 Hyundai Kona Electric** — HS-CAN on pins 6/14, which the EX
  supports. *Inference.*
- Honda body network (B-CAN) is **not** known to appear on DLC pins 3/11; owner forums
  describe it as reached through the gateway or via the SCS (pin 9→4) blink-code method. The
  EX's MS-CAN channel should therefore not be assumed to reach Honda body ECUs, and the
  MX+'s SW-CAN is GM-specific. Nothing found says the Kona exposes a non-HS-CAN bus on the
  DLC. — *community/unverified*
  (https://www.odyclub.com/threads/retrieving-b-can-dtcs.151708/,
  https://obdplanet.com/obd2-pinout/)

## Behaviour worth knowing

- The EX is **powered from USB** as well as from DLC pin 16: with the car off/no pin 16 it
  still answers AT/ST commands and reports `CAN ERROR` / `UNABLE TO CONNECT` rather than
  going silent. Check `ATRV`/`STVR` (near 0 V ⇒ no vehicle power) before blaming the bus.
  (open-obd-gui PR #4) — *community*
- Official software support is Windows and Android only; macOS works through Apple's built-in
  FTDI driver — see [[macos-serial-setup]].
- Firmware is updated by OBDLink's own updater over the bootloader; never from `carlink` —
  see [[adapter-safety]].

## See also

[[elm327-at-commands]] · [[stn-st-commands]] · [[adapter-response-format]] ·
[[adapter-safety]] · [[macos-serial-setup]]
