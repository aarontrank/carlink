---
title: macOS serial setup for the OBDLink EX
type: procedure
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/obdlink-frpm-rev-f.md
  - url:https://github.com/nicremo/open-obd-gui/pull/4
  - url:https://github.com/pololu/libusbp/issues/8
  - url:https://www.ftdichip.com/Support/Documents/AppNotes/AN_134_FTDI_Drivers_Installation_Guide_for_MAC_OSX.pdf
  - url:https://www.f150lightningforum.com/forum/threads/guide-forscan-on-apple-silicon-macbook-air.35117/
  - url:https://support.obdlink.com/support/solutions/articles/43000754740-can-i-use-obdwiz-on-a-macos-device-
tags: [macos, serial, ftdi, usb]
confidence: community
---

# macOS serial setup for the OBDLink EX

## Driver

None needed on macOS 11+. The [[obdlink-ex]] uses an FTDI USB-UART (VID `0x0403`, PID
`0x6015`) which binds to Apple's built-in DriverKit extension
`com.apple.DriverKit-AppleUSBFTDI` (open-obd-gui PR #4 — *community, observed on a real EX*;
Apple's DriverKit FTDI naming per pololu/libusbp #8). FTDI's own VCP driver is only a
fallback; OBDLink support points Mac users to ftdichip.com VCP drivers but does not
officially support macOS (support.obdlink.com) — *verified that it is unsupported*.

## Device node

- Apple's FTDI driver publishes `IOCalloutDevice = /dev/cu.usbserial-<FTDI serial>` and
  `IODialinDevice = /dev/tty.usbserial-<FTDI serial>` (libusbp #8 ioreg dump; FTDI AN_134).
- On an EX the suffix is the adapter's 12-digit serial, e.g. `cu.usbserial-223230293267`
  (F-150 Lightning forum, EX on Apple Silicon) — consistent with `STSN` starting with the
  device ID `2232`. — *community*
- **Use `/dev/cu.*`**, not `/dev/tty.*`: `tty.*` is the dial-in node and `open()` blocks
  waiting for carrier detect. *Standard macOS behaviour; unverified for this device.*
- Discovery: match by USB VID/PID (`0403:6015`) via IOKit / `serialport::available_ports()`
  rather than hard-coding a name; fall back to globbing `/dev/cu.usbserial-*`. Confirm the
  device by sending `STI` and expecting `STN22xx v…`.

## Serial parameters

| Setting | Value | Source |
|---------|-------|--------|
| Baud | **115200** (USB OBDLink default) | FRPM §6.0 — verified |
| Framing | 8 data bits, no parity, 1 stop | FRPM §6.0 — verified |
| Flow control | none (`STUFC` applies only to standalone ICs) | FRPM §8.3 — verified |
| Line ending sent | `\r` (0x0D) only | FRPM §6.0 — verified |

**Does baud matter over USB?** The STN's UART runs at the configured rate between the chip
and the FTDI bridge, so yes — the host must open the port at the adapter's rate (115200 unless
someone persisted another rate with `STWBR` or PP 0C). Raising it (`STBR`) speeds bulk
monitoring but is unnecessary for request/response diagnostics. If 115200 yields garbage,
try 9600, 38400, 57600, 230400, 500000 in turn and send `\r` / `ATI` at each — *the list is a
heuristic, not from a source*. Do **not** "fix" a wrong rate by persisting a new one; see
[[adapter-safety]].

## Practical notes

- The FTDI bridge buffers small reads (FTDI's default latency timer is 16 ms; Apple's driver
  does not expose it) — expect a few ms of extra latency per response; read until `>` rather
  than relying on read timing. *community (OpenBCI forum on macOS FTDI latency), unverified on EX.*
- Some USB-3 / USB-C ports misbehave with FTDI adapters; OBDLink suggests a USB 2.0 hub or
  adapter if the device does not enumerate (support.obdlink.com) — *verified as advice*.
- After opening, flush input, send `\r`, wait for `>` (the adapter may still be printing a
  startup banner or a prior response).

See [[elm327-at-commands]] for the init sequence.
