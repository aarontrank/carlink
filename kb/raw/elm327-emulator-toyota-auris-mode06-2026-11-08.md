---
title: ELM327-emulator "car" scenario — service 06 request/response frames from a Toyota Auris Hybrid
source_url: https://github.com/Ircama/ELM327-emulator/blob/dd3b1d2b19368875830ea8c86cea070040c26caf/elm/obd_message.py
commit: dd3b1d2b19368875830ea8c86cea070040c26caf
fetched: 2026-11-08
license: CC BY-NC-SA 4.0 (project README badge). Excerpt of data only, with attribution.
note: >
  The README says the 'car' scenario reproduces "the basic communication of a Toyota Auris Hybrid
  car". The frames below are the scenario's canned answers (CAN 11-bit, ECU 7E8, headers on,
  ELM327 format: header, PCI byte(s), data). Whether every frame is a verbatim capture is not
  stated; treat as community data. HD/SZ/DT are the emulator's header/size/data helpers, flattened
  here into the bytes they produce.
---

Format per line: `request → frames`. In a first frame `10 LL`, LL is the ISO-TP payload length; the
final consecutive frame is padded (here with `00`) past that length.

```
0600: |7E8 06 46 00 C0 00 00 01
0601: |7E8 10 13 46 01 8E 0B 02 27
0601: |7E8 21 00 A9 4D BA 01 91 8D
0601: |7E8 22 01 9D 00 B4 02 2F 00
0620: |7E8 06 46 20 80 00 80 01
0621: |7E8 10 0A 46 21 A9 86 02 E5
0621: |7E8 21 02 D0 7F FF 00 00 00
0631: |7E8 10 0A 46 31 BD 17 07 47
0631: |7E8 21 00 63 FF FF 00 00 00
0640: |7E8 06 46 40 00 00 00 01
0660: |7E8 06 46 60 00 00 00 01
0680: |7E8 06 46 80 00 00 00 01
06A0: |7E8 06 46 A0 F8 00 00 00
06A1: |7E8 10 13 46 A1 0B 24 00 00
06A1: |7E8 21 00 00 FF FF A1 0C 24
06A1: |7E8 22 00 00 00 00 FF FF 00
06A2: |7E8 10 13 46 A2 0B 24 00 00
06A2: |7E8 21 00 00 FF FF A2 0C 24
06A2: |7E8 22 00 00 00 00 FF FF 00
06A3: |7E8 10 13 46 A3 0B 24 00 00
06A3: |7E8 21 00 00 FF FF A3 0C 24
06A3: |7E8 22 00 00 00 00 FF FF 00
06A4: |7E8 10 13 46 A4 0B 24 00 00
06A4: |7E8 21 00 00 FF FF A4 0C 24
06A4: |7E8 22 00 00 00 00 FF FF 00
06A5: |7E8 10 13 46 A5 0B 24 00 00
06A5: |7E8 21 00 00 FF FF A5 0C 24
06A5: |7E8 22 00 00 00 00 FF FF 00
```
