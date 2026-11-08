---
title: projectgus/hyundai-kona-ev-can-logs — diagnostic (UDS/KWP DTC read and clear) excerpts from real Kona EVs
source_url: https://github.com/projectgus/hyundai-kona-ev-can-logs/tree/83a5cac813debf5e5e97a188e2b8e9c082ee307f
commit: 83a5cac813debf5e5e97a188e2b8e9c082ee307f
fetched: 2026-11-08
license: none stated in the repository (excerpts kept for private research)
---

Real CAN captures (SavvyCAN GVRET CSV) from 1st-generation Hyundai Kona Electric cars, published by
projectgus (Angus Gratton). Three logs contain a phone app (Car Scanner ELM OBD2, with its built-in
Hyundai Kona EV profile, on an ELM327-type dongle) reading and clearing trouble codes, so they show
exactly which diagnostic IDs answer and which services they accept.

How these excerpts were made (2026-11-08): each CSV was downloaded at the commit above and filtered
to 11-bit frames with IDs 0x700–0x7FF. Times are seconds from the first frame of the file
(221210-1) or from the first diagnostic frame (221210-4 and the bench log). `bN` is the logger
channel. The app's periodic `7DF 02 01 00` polls and their `41 00` answers were removed; nothing else
was removed or edited inside a window. Frames are raw CAN data (ISO-TP PCI byte first, `AA`/`00`
padding included).

Files used:
- `221210-1-2021-dcan-car-scanner-elm327.csv` (101 531 bytes)
- `221210-4-2021-pcan-dcan-read-dtcs-with-app.csv` (76 313 909 bytes; only bus 1 = D-CAN shown)
- `bench_kona/2023-11-19T11:57:59.166526-bench_kona.csv` (64 399 647 bytes)
- `bench_kona/2023-11-19T11:57:59-obd-scan-1-before-clearing.txt` and `...-2-after-clearing.txt`
  (Car Scanner's own DTC reports for the same bench session, reproduced verbatim below)

## README.md (excerpts, verbatim)

```markdown
# Kona EV CAN logs

Some CAN logs taken from various CAN buses of Hyundai Kona EV vehicles. To assist with reverse engineering and repurposing for EV conversions and other uses.

Log files are in the SavvyCAN "GVRET" CSV format.

## See also

* A DBC file derived mostly from these logs can be found at https://github.com/projectgus/kona-ev-dbc
* [This Google Sheet](https://docs.google.com/spreadsheets/d/1-9jZafV9eZeBUnPQo7qQHbX2-_4qZfWfRVpidoF1owA/edit#gid=660740603) from 2018 (author unknown) is mostly about Diagnostic OBD2 PIDs, but includes a useful diagram of the vehicle CAN bus architecture showing all the buses connected to the CAN gateway ("CGW" in the diagram, Hyundai seems to more typically call it the Integrated Gateway Power Module "IGPM").

## Vehicles

This repo contains logs from:

* 2019 Kona EV Highlander (crashed). Has VESS & Active Air Flap System missing from P-CAN (connector broken).
* 2021 Kona EV Highlander (crashed).
* 2020 Kona (not crashed, for heater log).

## Logs

### 221210-1-2021-dcan-car-scanner-elm327.csv
Model: 2021

(Log probably has no dropped messages.)

Channel 1: DCAN

Running the [Car Scanner ELM OBD2](https://www.carscanner.info/) app to scan for DTCs, with presets for Kona that ship in that app.

### 221210-2-2021-dcan-car-scanner-elm327-vcu-mcu-only.csv
Model: 2021

(Log probably has no dropped messages.)

Channel 1: DCAN

As above, but with only VCU & MCU selected in the app interface.

### 221210-3-2021-pcan-dcan-in-ready.csv
Model: 2021

(This log appears to become intermittent after TS320.)

Channel 0: PCAN, Channel 1: DCAN

Vehicle sitting in "Ready" but not driving. No fault codes present.

Diagnostic app on Channel 1 is running, session is alive but I think idle the whole time.

### 221210-4-2021-pcan-dcan-read-dtcs-with-app.csv

(TS 1115 the log becomes intermittent.)

### 221210-5-2021-pcan-dcan-car-scanner-read-all-params.csv

(TS 214 the log becomes intermittent.)

## Other Logs

These other logs aren't from complete vehicles:

* `bench_kona` contains some logs my ["bench kona"](https://www.projectgus.com/2024/01/kona-motor-turns/) setup in various stages of development. See README in that directory.
* `bms_diagnostics` contains some logs of Kona BMS diagnostic session, from [Dala's EV Discord](https://www.patreon.com/dala)
```

## bench_kona/README.md (first lines, verbatim)

```markdown
## Bench Kona Logs

These logs are from the [Bench Kona](https://www.projectgus.com/2024/01/kona-motor-turns/) setup as it has evolved over time. Messages in these logs may be spoofed and therefore incorrect, or show the original modules in various failure states.

### 2023-11-20

These are the first logs where the motor started turning, and includes two logs of the motor running away to high rpm.

* 2023-11-19T11:57:59.166526-bench_kona.csv - Going to Drive, no motor turning.
```

## Car Scanner DTC report, bench, before clearing (verbatim)

```
Car Scanner ELM OBD2
Version: 1.101.4/401014/GP
DTC report
Connection profile: Hyundai Kona EV/Niro EV/Soul EVDate: 19/11/2023 12:08:45 PM
VIN: SEVBMS
============================
OBD-IIOBD-II
DTCs: 7
----------------------------
U1128(00) [0xD12800]
Status: Test failed, Test failed (current drive cycle), Pending, Confirmed, Test failed since last DTC clear, Warning indicator requested
----------------------------
P1A6F(00) [0x1A6F00]
Status: Test failed, Test failed (current drive cycle), Pending, Test failed since last DTC clear, Warning indicator requested
----------------------------
P1A78 [0x1A78]
Status: Test is inhibited by other DTC, Validated and stored in non volatile memory, Validated fault present at time of request, Validated fault has been present during this drive cycle
----------------------------
U0129 [0xC129]
Data bus, brake system control module - no communication
Status: Test conditions are met, Validated and stored in non volatile memory, Validated fault present at time of request, Validated fault has been present during this drive cycle
----------------------------
U0146 [0xC146]
Data bus, gateway A - no communication
Status: Test conditions are met, Validated and stored in non volatile memory, Validated fault present at time of request
----------------------------
U0164 [0xC164]
Data bus, AC control module - no communication
Status: Test conditions are met, Validated and stored in non volatile memory, Validated fault present at time of request
----------------------------
P0D59 [0x0D59]
Status: Test is inhibited by other DTC, Validated and stored in non volatile memory

============================
Vehicle Motor control unit
DTCs: 4
----------------------------
P1A78 [0x1A78]  - Check IG3 Relay or Back Up Lamp Circuit
Status: Test is inhibited by other DTC, Validated and stored in non volatile memory, Validated fault present at time of request, Validated fault has been present during this drive cycle
----------------------------
U0129 [0xC129]
Data bus, brake system control module - no communication
Status: Test conditions are met, Validated and stored in non volatile memory, Validated fault present at time of request, Validated fault has been present during this drive cycle
----------------------------
U0146 [0xC146]
Data bus, gateway A - no communication
Status: Test conditions are met, Validated and stored in non volatile memory, Validated fault present at time of request
----------------------------
U0164 [0xC164]
Data bus, AC control module - no communication
Status: Test conditions are met, Validated and stored in non volatile memory, Validated fault present at time of request

============================
Motor control unit
No DTC found.

============================
Battery management system
DTCs: 2
----------------------------
U1128(00) [0xD12800]
Status: Test failed, Test failed (current drive cycle), Pending, Confirmed, Test failed since last DTC clear, Warning indicator requested
Lost Communication With Hybrid/EV Battery Electric Coolant Valve "A" (BAT)
----------------------------
P1A6F(00) [0x1A6F00]
Status: Test failed, Test failed (current drive cycle), Pending, Test failed since last DTC clear, Warning indicator requested
P1A6F00 Airbag Deployment Signal Circuit High

============================
4WD #1 / OnBoard Charger (Hybrid/EV)4WD #1 / OnBoard Charger (Hybrid/EV)
DTCs: 2
----------------------------
P0D59 [0x0D59]
Proximity Detection Circuit High
Status: Test is inhibited by other DTC, Validated and stored in non volatile memory
----------------------------
U0146 [0xC146]
Data bus, gateway A - no communication
Status: Test conditions are met, Validated and stored in non volatile memory, Validated fault present at time of request, Validated fault has been present during this drive cycle
============================
E-ShifterE-Shifter
DTCs: 1
----------------------------
C1614(87) [0x561487]
CAN Time-out HCU/VCU/FCU
Status: Test failed, Test failed (current drive cycle), Pending, Confirmed, Test failed since last DTC clear
```

## Car Scanner DTC report, bench, after clearing (verbatim)

```
Car Scanner ELM OBD2
Version: 1.101.4/401014/GP
DTC report
Connection profile: Hyundai Kona EV/Niro EV/Soul EVDate: 19/11/2023 12:12:17 PM
VIN: SEVBMS
============================
OBD-IIOBD-II
DTCs: 6
----------------------------
P1A6F(00) [0x1A6F00]
Status: Test failed, Test failed (current drive cycle), Pending, Test failed since last DTC clear
----------------------------
U1128(00) [0xD12800]
Status: Test failed, Test failed (current drive cycle), Pending, Test failed since last DTC clear
----------------------------
P1A78 [0x1A78]
Status: Test is inhibited by other DTC, Validated fault present at time of request, Validated fault has been present during this drive cycle
----------------------------
U0129 [0xC129]
Data bus, brake system control module - no communication
Status: Test conditions are met, Validated fault present at time of request
----------------------------
U0146 [0xC146]
Data bus, gateway A - no communication
Status: Test conditions are met, Validated fault present at time of request
----------------------------
U0164 [0xC164]
Data bus, AC control module - no communication
Status: Test conditions are met, Validated fault present at time of request
============================
Vehicle Motor control unitVehicle Motor control unit
DTCs: 4
----------------------------
P1A78 [0x1A78]
Status: Test is inhibited by other DTC, Validated fault present at time of request, Validated fault has been present during this drive cycle
----------------------------
U0129 [0xC129]
Data bus, brake system control module - no communication
Status: Test conditions are met, Validated fault present at time of request
----------------------------
U0146 [0xC146]
Data bus, gateway A - no communication
Status: Test conditions are met, Validated fault present at time of request
----------------------------
U0164 [0xC164]
Data bus, AC control module - no communication
Status: Test conditions are met, Validated fault present at time of request
============================
Motor control unit
No DTC found.
============================
Battery management systemBattery management system
DTCs: 2
----------------------------
P1A6F(00) [0x1A6F00]
Status: Test failed, Test failed (current drive cycle), Pending, Test failed since last DTC clear
----------------------------
U1128(00) [0xD12800]
Status: Test failed, Test failed (current drive cycle), Pending, Test failed since last DTC clear
============================
4WD #1 / OnBoard Charger (Hybrid/EV)4WD #1 / OnBoard Charger (Hybrid/EV)
DTCs: 1
----------------------------
U0146 [0xC146]
Data bus, gateway A - no communication
Status: Test conditions are met, Validated fault present at time of request
============================
E-Shifter
No DTC found.
```

## Diagnostic frame excerpts

### 221210-1 — service 09 identification (functional 7DF), t 2.5–3.1 s

```
    2.785 b1 7DF  02 09 02 00 00 00 00 00
    2.795 b1 7EC  10 14 49 02 01 4F 53 45
    2.795 b1 7E4  30 00 00 00 00 00 00 00
    2.804 b1 7EC  21 56 5F 5F 20 42 4D 53
    2.817 b1 7EC  22 20 00 00 00 00 00 00
    2.891 b1 7DF  02 09 04 00 00 00 00 00
    2.893 b1 7EB  04 49 04 01 00 00 00 00
    2.894 b1 7ED  10 13 49 04 01 45 4F 53
    2.894 b1 7E5  30 00 00 00 00 00 00 00
    2.899 b1 7EC  10 13 49 04 01 4F 53 45
    2.900 b1 7E4  30 00 00 00 00 00 00 00
    2.901 b1 7EA  04 49 04 01 00 00 00 00
    2.904 b1 7ED  21 45 4B 35 4C 2D 4F 53
    2.917 b1 7EC  21 56 42 36 38 32 32 52
    2.917 b1 7ED  22 32 2D 44 30 30 30 00
    2.928 b1 7EC  22 00 00 00 00 00 00 AA
    2.984 b1 7DF  02 09 0A 00 00 00 00 00
    2.992 b1 7EB  04 49 0A 01 00 00 00 00
    2.994 b1 7ED  10 17 49 0A 01 42 43 43
    2.994 b1 7E5  30 00 00 00 00 00 00 00
    2.994 b1 7EC  10 17 49 0A 01 42 45 43
    2.995 b1 7E4  30 00 00 00 00 00 00 00
    3.004 b1 7ED  21 4D 2D 42 2B 43 68 61
    3.004 b1 7EC  21 4D 2D 42 2B 45 6E 65
    3.016 b1 7EC  22 72 67 79 43 74 72 6C
    3.017 b1 7ED  22 72 67 65 72 43 74 72
    3.025 b1 7EC  23 00 00 00 AA AA AA AA
    3.026 b1 7ED  23 6C 00 00 00 00 00 00
    3.071 b1 7DF  02 09 08 00 00 00 00 00
```

### 221210-1 — functional (7DF) OBD 03/07/0A, then UDS 19 02 and KWP 18 requests, t 23.7–24.9 s

```
   23.766 b1 7DF  01 03 00 00 00 00 00 00
   23.767 b1 7EB  02 43 00 00 00 00 00 00
   23.768 b1 7ED  02 43 00 00 00 00 00 00
   23.774 b1 7EC  02 43 00 AA AA AA AA AA
   23.776 b1 7EA  02 43 00 00 00 00 00 00
   23.832 b1 7DF  01 03 00 00 00 00 00 00
   23.835 b1 7EA  02 43 00 00 00 00 00 00
   23.837 b1 7EB  02 43 00 00 00 00 00 00
   23.838 b1 7ED  02 43 00 00 00 00 00 00
   23.839 b1 7EC  02 43 00 AA AA AA AA AA
   23.896 b1 7DF  01 07 00 00 00 00 00 00
   23.905 b1 7EA  02 47 00 00 00 00 00 00
   23.907 b1 7EB  02 47 00 00 00 00 00 00
   23.909 b1 7EC  02 47 00 AA AA AA AA AA
   23.909 b1 7ED  02 47 00 00 00 00 00 00
   23.993 b1 7DF  01 07 00 00 00 00 00 00
   24.005 b1 7EA  02 47 00 00 00 00 00 00
   24.007 b1 7EB  02 47 00 00 00 00 00 00
   24.009 b1 7EC  02 47 00 AA AA AA AA AA
   24.009 b1 7ED  02 47 00 00 00 00 00 00
   24.072 b1 7DF  01 0A 00 00 00 00 00 00
   24.075 b1 7EA  02 4A 00 00 00 00 00 00
   24.076 b1 7EB  02 4A 00 00 00 00 00 00
   24.078 b1 7ED  02 4A 00 00 00 00 00 00
   24.140 b1 7DF  03 19 02 08 00 00 00 00
   24.145 b1 7EA  03 7F 19 12 00 00 00 00
   24.145 b1 7BF  07 59 02 FF 58 12 87 28
   24.146 b1 7EB  03 7F 19 11 00 00 00 00
   24.148 b1 7DC  07 59 02 89 58 12 87 08
   24.150 b1 79E  03 59 02 08 AA AA AA AA
   24.151 b1 7EC  03 59 02 FF AA AA AA AA
   24.151 b1 7CC  10 13 59 02 89 58 12 87
   24.151 b1 7ED  03 7F 19 11 00 00 00 00
   24.152 b1 788  03 59 02 09 AA AA AA AA
   24.152 b1 72D  07 59 02 09 96 03 00 08
   24.153 b1 73B  03 59 02 09 AA AA AA AA
   24.155 b1 7BB  07 59 02 09 92 38 13 09
   24.158 b1 7AD  03 7F 19 78 AA AA AA AA
   24.160 b1 7CE  07 59 02 09 90 01 52 09
   24.162 b1 7A8  03 7F 19 78 AA AA AA AA
   24.163 b1 78B  03 7F 19 22 AA AA AA AA
   24.168 b1 778  10 17 59 02 09 96 03 00
   24.225 b1 7DF  03 19 02 AC 00 00 00 00
   24.230 b1 79E  03 59 02 08 AA AA AA AA
   24.234 b1 7DC  03 7F 19 31 AA AA AA AA
   24.235 b1 7EA  03 7F 19 12 00 00 00 00
   24.235 b1 7BF  07 59 02 FF 58 12 87 28
   24.236 b1 7EB  03 7F 19 11 00 00 00 00
   24.238 b1 7ED  03 7F 19 11 00 00 00 00
   24.238 b1 7EC  03 59 02 FF AA AA AA AA
   24.240 b1 7CE  07 59 02 09 90 01 52 09
   24.243 b1 73B  03 59 02 09 AA AA AA AA
   24.243 b1 7BB  07 59 02 09 92 38 13 09
   24.295 b1 7DF  03 19 02 AF 00 00 00 00
   24.299 b1 79E  03 59 02 08 AA AA AA AA
   24.303 b1 7DC  03 7F 19 31 AA AA AA AA
   24.305 b1 7BF  07 59 02 FF 58 12 87 28
   24.305 b1 7EA  03 7F 19 12 00 00 00 00
   24.306 b1 7EB  03 7F 19 11 00 00 00 00
   24.308 b1 7EC  03 59 02 FF AA AA AA AA
   24.309 b1 7ED  03 7F 19 11 00 00 00 00
   24.310 b1 7CE  07 59 02 09 90 01 52 09
   24.313 b1 73B  03 59 02 09 AA AA AA AA
   24.315 b1 7BB  07 59 02 09 92 38 13 09
   24.375 b1 7DF  04 18 00 FF 00 00 00 00
   24.378 b1 7ED  02 58 00 00 00 00 00 00
   24.383 b1 7DC  03 7F 18 21 AA AA AA AA
   24.392 b1 7EA  02 58 00 00 00 00 00 00
   24.392 b1 7EB  02 58 00 00 00 00 00 00
   24.395 b1 7BB  03 7F 18 11 00 00 00 00
   24.456 b1 7DF  04 18 00 FF 00 00 00 00
   24.458 b1 7ED  02 58 00 00 00 00 00 00
   24.463 b1 7DC  03 7F 18 21 AA AA AA AA
   24.465 b1 7EA  02 58 00 00 00 00 00 00
   24.466 b1 7EB  02 58 00 00 00 00 00 00
   24.473 b1 7BB  03 7F 18 11 00 00 00 00
   24.531 b1 7DF  04 18 02 FF 00 00 00 00
   24.535 b1 7EA  03 7F 18 12 00 00 00 00
   24.536 b1 7EB  03 7F 18 12 00 00 00 00
   24.538 b1 7ED  03 7F 18 12 00 00 00 00
   24.538 b1 7DC  03 7F 18 21 AA AA AA AA
   24.543 b1 7BB  03 7F 18 11 00 00 00 00
   24.827 b1 7AD  07 59 02 08 96 03 00 08
```

### 221210-1 — physical VCU 7E2, t 115.5–116.3 s

```
  115.522 b1 7E2  02 10 03 00 00 00 00 00
  115.533 b1 7EA  03 7F 10 12 00 00 00 00
  115.545 b1 7E2  02 10 90 00 00 00 00 00
  115.553 b1 7EA  02 50 90 00 00 00 00 00
  115.572 b1 7E2  01 20 00 00 00 00 00 00
  115.583 b1 7EA  01 60 00 00 00 00 00 00
  115.641 b1 7E2  03 19 02 AF 00 00 00 00
  115.643 b1 7EA  03 7F 19 12 00 00 00 00
  115.696 b1 7E2  03 19 02 AF 00 00 00 00
  115.703 b1 7EA  03 7F 19 12 00 00 00 00
  115.767 b1 7E2  04 18 00 FF 00 00 00 00
  115.776 b1 7EA  02 58 00 00 00 00 00 00
  115.836 b1 7E2  04 18 02 FF 00 00 00 00
  115.843 b1 7EA  03 7F 18 12 00 00 00 00
  115.911 b1 7E2  04 18 00 80 00 00 00 00
  115.913 b1 7EA  02 58 00 00 00 00 00 00
  115.953 b1 7E2  04 18 02 80 00 00 00 00
  115.963 b1 7EA  03 7F 18 12 00 00 00 00
  116.037 b1 7E2  04 18 20 FF 00 00 00 00
  116.043 b1 7EA  02 58 00 00 00 00 00 00
  116.103 b1 7E2  04 18 60 FF 00 00 00 00
  116.113 b1 7EA  02 58 00 00 00 00 00 00
  116.174 b1 7E2  04 18 00 40 00 00 00 00
  116.183 b1 7EA  02 58 00 00 00 00 00 00
  116.238 b1 7E2  01 20 00 00 00 00 00 00
  116.243 b1 7EA  01 60 00 00 00 00 00 00
```

### 221210-1 — physical BMS 7E4, t 117.7–119.2 s

```
  117.738 b1 7E4  02 10 03 00 00 00 00 00
  117.749 b1 7EC  06 50 03 00 32 01 F4 AA
  117.775 b1 7E4  02 10 90 00 00 00 00 00
  118.211 b1 7E4  01 20 00 00 00 00 00 00
  118.218 b1 7EC  01 60 AA AA AA AA AA AA
  118.278 b1 7E4  02 10 03 00 00 00 00 00
  118.293 b1 7EC  06 50 03 00 32 01 F4 AA
  118.361 b1 7E4  03 19 02 08 00 00 00 00
  118.373 b1 7EC  03 59 02 FF AA AA AA AA
  118.420 b1 7E4  03 19 02 AC 00 00 00 00
  118.428 b1 7EC  03 59 02 FF AA AA AA AA
  118.485 b1 7E4  03 19 02 AF 00 00 00 00
  118.493 b1 7EC  03 59 02 FF AA AA AA AA
  118.621 b1 7E4  04 18 00 FF 00 00 00 00
  118.628 b1 7EC  03 7F 18 11 AA AA AA AA
  118.668 b1 7E4  04 18 02 FF 00 00 00 00
  118.683 b1 7EC  03 7F 18 11 AA AA AA AA
  118.725 b1 7E4  04 18 00 80 00 00 00 00
  118.733 b1 7EC  03 7F 18 11 AA AA AA AA
  118.796 b1 7E4  04 18 02 80 00 00 00 00
  118.802 b1 7EC  03 7F 18 11 AA AA AA AA
  118.859 b1 7E4  04 18 20 FF 00 00 00 00
  118.875 b1 7EC  03 7F 18 11 AA AA AA AA
  118.931 b1 7E4  04 18 60 FF 00 00 00 00
  118.937 b1 7EC  03 7F 18 11 AA AA AA AA
  118.997 b1 7E4  04 18 00 40 00 00 00 00
  119.007 b1 7EC  03 7F 18 11 AA AA AA AA
  119.102 b1 7E4  01 20 00 00 00 00 00 00
  119.112 b1 7EC  01 60 AA AA AA AA AA AA
```

### 221210-1 — physical OBC 7E5, t 119.5–120.6 s

```
  119.532 b1 7E5  02 10 03 00 00 00 00 00
  119.541 b1 7ED  03 7F 10 12 00 00 00 00
  119.567 b1 7E5  02 10 90 00 00 00 00 00
  119.585 b1 7ED  02 50 90 00 00 00 00 00
  119.606 b1 7E5  01 20 00 00 00 00 00 00
  119.611 b1 7ED  01 60 00 00 00 00 00 00
  119.663 b1 7E5  02 10 03 00 00 00 00 00
  119.674 b1 7ED  03 7F 10 12 00 00 00 00
  119.739 b1 7E5  03 19 02 08 00 00 00 00
  119.741 b1 7ED  03 7F 19 11 00 00 00 00
  119.808 b1 7E5  03 19 02 AC 00 00 00 00
  119.811 b1 7ED  03 7F 19 11 00 00 00 00
  119.867 b1 7E5  03 19 02 AF 00 00 00 00
  119.881 b1 7ED  03 7F 19 11 00 00 00 00
  119.977 b1 7E5  04 18 00 FF 00 00 00 00
  119.991 b1 7ED  02 58 00 00 00 00 00 00
  120.046 b1 7E5  04 18 02 FF 00 00 00 00
  120.051 b1 7ED  03 7F 18 12 00 00 00 00
  120.100 b1 7E5  04 18 00 80 00 00 00 00
  120.110 b1 7ED  02 58 00 00 00 00 00 00
  120.175 b1 7E5  04 18 02 80 00 00 00 00
  120.190 b1 7ED  03 7F 18 12 00 00 00 00
  120.238 b1 7E5  04 18 20 FF 00 00 00 00
  120.240 b1 7ED  02 58 00 00 00 00 00 00
  120.294 b1 7E5  04 18 60 FF 00 00 00 00
  120.300 b1 7ED  02 58 00 00 00 00 00 00
  120.360 b1 7E5  04 18 00 40 00 00 00 00
  120.371 b1 7ED  02 58 00 00 00 00 00 00
  120.490 b1 7E5  01 20 00 00 00 00 00 00
  120.500 b1 7ED  01 60 00 00 00 00 00 00
```

### 221210-1 — physical 725, t 120.8–121.3 s

```
  120.841 b1 725  02 10 03 00 00 00 00 00
  120.860 b1 72D  02 50 03 AA AA AA AA AA
  120.878 b1 725  02 10 90 00 00 00 00 00
  120.890 b1 72D  02 50 90 AA AA AA AA AA
  120.914 b1 725  01 20 00 00 00 00 00 00
  120.930 b1 72D  03 7F 20 00 AA AA AA AA
  120.989 b1 725  02 10 03 00 00 00 00 00
  121.000 b1 72D  02 50 03 AA AA AA AA AA
  121.094 b1 725  03 19 02 08 00 00 00 00
  121.110 b1 72D  07 59 02 09 96 03 00 08
  121.200 b1 725  01 20 00 00 00 00 00 00
  121.220 b1 72D  03 7F 20 00 AA AA AA AA
```

### 221210-1 — physical IGPM 770, t 386.5–387.2 s

```
  386.526 b1 770  02 10 03 00 00 00 00 00
  386.555 b1 778  02 50 03 AA AA AA AA AA
  386.586 b1 770  02 10 90 00 00 00 00 00
  386.615 b1 778  02 50 90 AA AA AA AA AA
  386.631 b1 770  01 20 00 00 00 00 00 00
  386.655 b1 778  01 60 AA AA AA AA AA AA
  386.738 b1 770  02 10 03 00 00 00 00 00
  386.765 b1 778  02 50 03 AA AA AA AA AA
  386.954 b1 770  03 19 02 08 00 00 00 00
  386.974 b1 778  10 17 59 02 09 96 03 00
  386.975 b1 770  30 00 05 00 00 00 00 00
  386.984 b1 778  21 08 A8 01 00 08 56 7C
  386.994 b1 778  22 00 08 A2 11 00 08 58
  387.004 b1 778  23 29 00 08 AA AA AA AA
  387.080 b1 770  01 20 00 00 00 00 00 00
  387.104 b1 778  01 60 AA AA AA AA AA AA
```

### 221210-1 — physical 776, t 407.2–407.7 s

```
  407.251 b1 776  02 10 03 00 00 00 00 00
  407.272 b1 77E  02 50 03 AA AA AA AA AA
  407.285 b1 776  02 10 90 00 00 00 00 00
  407.292 b1 77E  02 50 90 AA AA AA AA AA
  407.313 b1 776  01 20 00 00 00 00 00 00
  407.342 b1 77E  01 60 AA AA AA AA AA AA
  407.426 b1 776  02 10 03 00 00 00 00 00
  407.452 b1 77E  02 50 03 AA AA AA AA AA
  407.526 b1 776  03 19 02 08 00 00 00 00
  407.552 b1 77E  07 59 02 09 A8 01 00 08
  407.646 b1 776  01 20 00 00 00 00 00 00
  407.671 b1 77E  01 60 AA AA AA AA AA AA
```

### 221210-1 — physical 780, t 427.4–428.6 s

```
  427.406 b1 780  02 10 03 00 00 00 00 00
  427.429 b1 788  06 50 03 00 32 00 FA AA
  427.442 b1 780  02 10 90 00 00 00 00 00
  427.459 b1 788  03 7F 10 12 AA AA AA AA
  427.481 b1 780  01 20 00 00 00 00 00 00
  427.499 b1 788  03 7F 20 11 AA AA AA AA
  427.572 b1 780  02 10 03 00 00 00 00 00
  427.591 b1 788  06 50 03 00 32 00 FA AA
  427.661 b1 780  03 19 02 08 00 00 00 00
  427.677 b1 788  03 59 02 09 AA AA AA AA
  427.747 b1 780  03 19 02 AC 00 00 00 00
  427.757 b1 788  03 7F 19 31 AA AA AA AA
  427.816 b1 780  03 19 02 AF 00 00 00 00
  427.829 b1 788  03 7F 19 31 AA AA AA AA
  427.900 b1 780  04 18 00 FF 00 00 00 00
  427.916 b1 788  03 7F 18 11 AA AA AA AA
  427.980 b1 780  04 18 02 FF 00 00 00 00
  428.001 b1 788  03 7F 18 11 AA AA AA AA
  428.075 b1 780  04 18 00 80 00 00 00 00
  428.086 b1 788  03 7F 18 11 AA AA AA AA
  428.155 b1 780  04 18 02 80 00 00 00 00
  428.169 b1 788  03 7F 18 11 AA AA AA AA
  428.236 b1 780  04 18 20 FF 00 00 00 00
  428.252 b1 788  03 7F 18 11 AA AA AA AA
  428.349 b1 780  04 18 60 FF 00 00 00 00
  428.367 b1 788  03 7F 18 11 AA AA AA AA
  428.454 b1 780  04 18 00 40 00 00 00 00
  428.468 b1 788  03 7F 18 11 AA AA AA AA
  428.558 b1 780  01 20 00 00 00 00 00 00
  428.582 b1 788  03 7F 20 11 AA AA AA AA
```

### 221210-1 — physical 783, t 428.8–429.5 s

```
  428.889 b1 783  02 10 03 00 00 00 00 00
  428.905 b1 78B  06 50 03 00 32 01 F4 AA
  428.925 b1 783  02 10 90 00 00 00 00 00
  428.948 b1 78B  03 7F 10 12 AA AA AA AA
  428.962 b1 783  01 20 00 00 00 00 00 00
  428.982 b1 78B  03 7F 20 11 AA AA AA AA
  429.059 b1 783  02 10 03 00 00 00 00 00
  429.075 b1 78B  06 50 03 00 32 01 F4 AA
  429.134 b1 783  03 19 02 08 00 00 00 00
  429.154 b1 78B  03 7F 19 22 AA AA AA AA
  429.264 b1 783  03 19 02 AC 00 00 00 00
  429.284 b1 78B  03 7F 19 31 AA AA AA AA
  429.347 b1 783  03 19 02 AF 00 00 00 00
  429.368 b1 78B  03 7F 19 31 AA AA AA AA
  429.429 b1 783  04 18 00 FF 00 00 00 00
  429.444 b1 78B  03 7F 18 11 AA AA AA AA
```

### 221210-1 — physical 796, t 488.8–490.1 s

```
  488.805 b1 796  02 10 03 00 00 00 00 00
  488.812 b1 79E  02 50 03 AA AA AA AA AA
  488.830 b1 796  02 10 90 00 00 00 00 00
  489.274 b1 796  01 20 00 00 00 00 00 00
  489.281 b1 79E  01 60 AA AA AA AA AA AA
  489.340 b1 796  02 10 03 00 00 00 00 00
  489.351 b1 79E  02 50 03 AA AA AA AA AA
  489.407 b1 796  03 19 02 08 00 00 00 00
  489.411 b1 79E  03 59 02 08 AA AA AA AA
  489.461 b1 796  03 19 02 AC 00 00 00 00
  489.471 b1 79E  03 59 02 08 AA AA AA AA
  489.536 b1 796  03 19 02 AF 00 00 00 00
  489.541 b1 79E  03 59 02 08 AA AA AA AA
  489.588 b1 796  04 18 00 FF 00 00 00 00
  489.601 b1 79E  03 7F 18 11 AA AA AA AA
  489.658 b1 796  04 18 02 FF 00 00 00 00
  489.671 b1 79E  03 7F 18 11 AA AA AA AA
  489.729 b1 796  04 18 00 80 00 00 00 00
  489.741 b1 79E  03 7F 18 11 AA AA AA AA
  489.818 b1 796  04 18 02 80 00 00 00 00
  489.828 b1 79E  03 7F 18 11 AA AA AA AA
  489.907 b1 796  04 18 20 FF 00 00 00 00
  489.908 b1 79E  03 7F 18 11 AA AA AA AA
  489.976 b1 796  04 18 60 FF 00 00 00 00
  489.988 b1 79E  03 7F 18 11 AA AA AA AA
  490.043 b1 796  04 18 00 40 00 00 00 00
  490.047 b1 79E  03 7F 18 11 AA AA AA AA
  490.087 b1 796  01 20 00 00 00 00 00 00
  490.097 b1 79E  01 60 AA AA AA AA AA AA
```

### 221210-1 — physical BCM 7A0, t 490.4–506.2 s

```
  490.441 b1 7A0  02 10 03 00 00 00 00 00
  490.451 b1 7A8  02 50 03 AA AA AA AA AA
  490.465 b1 7A0  02 10 90 00 00 00 00 00
  490.481 b1 7A8  02 50 90 AA AA AA AA AA
  490.501 b1 7A0  01 20 00 00 00 00 00 00
  490.511 b1 7A8  01 60 AA AA AA AA AA AA
  490.561 b1 7A0  02 10 03 00 00 00 00 00
  490.571 b1 7A8  02 50 03 AA AA AA AA AA
  490.638 b1 7A0  03 19 02 08 00 00 00 00
  490.661 b1 7A8  03 7F 19 78 AA AA AA AA
  490.730 b1 7A0  01 20 00 00 00 00 00 00
  491.375 b1 7A0  02 10 03 00 00 00 00 00
  491.802 b1 7A0  02 10 90 00 00 00 00 00
  493.158 b1 7A8  03 7F 19 78 AA AA AA AA
  495.654 b1 7A8  03 7F 19 78 AA AA AA AA
  498.151 b1 7A8  03 7F 19 78 AA AA AA AA
  504.311 b1 7A0  02 10 03 00 00 00 00 00
  504.313 b1 7A8  02 50 03 AA AA AA AA AA
  504.331 b1 7A0  02 10 90 00 00 00 00 00
  504.343 b1 7A8  02 50 90 AA AA AA AA AA
  504.356 b1 7A0  01 20 00 00 00 00 00 00
  504.373 b1 7A8  01 60 AA AA AA AA AA AA
  504.467 b1 7A0  03 19 02 08 00 00 00 00
  504.493 b1 7A8  03 7F 19 78 AA AA AA AA
  504.983 b1 7A8  10 17 59 02 09 96 03 00
  504.983 b1 7A0  30 00 05 00 00 00 00 00
  504.993 b1 7A8  21 08 92 10 00 09 92 11
  505.003 b1 7A8  22 00 09 92 12 00 09 92
  505.013 b1 7A8  23 13 00 09 AA AA AA AA
  506.111 b1 7A0  01 20 00 00 00 00 00 00
  506.121 b1 7A8  01 60 AA AA AA AA AA AA
```

### 221210-1 — physical 7A5, t 551.0–553.9 s

```
  551.022 b1 7A5  02 10 03 00 00 00 00 00
  551.030 b1 7AD  02 50 03 AA AA AA AA AA
  551.044 b1 7A5  02 10 90 00 00 00 00 00
  551.060 b1 7AD  02 50 90 AA AA AA AA AA
  551.079 b1 7A5  01 20 00 00 00 00 00 00
  551.090 b1 7AD  01 60 AA AA AA AA AA AA
  551.138 b1 7A5  02 10 03 00 00 00 00 00
  551.140 b1 7AD  02 50 03 AA AA AA AA AA
  551.192 b1 7A5  03 19 02 08 00 00 00 00
  551.210 b1 7AD  03 7F 19 78 AA AA AA AA
  551.259 b1 7A5  01 20 00 00 00 00 00 00
  551.862 b1 7A5  02 10 03 00 00 00 00 00
  551.879 b1 7AD  07 59 02 08 96 03 00 08
  551.896 b1 7A5  02 10 90 00 00 00 00 00
  551.909 b1 7AD  02 50 90 AA AA AA AA AA
  551.962 b1 7A5  01 20 00 00 00 00 00 00
  551.969 b1 7AD  01 60 AA AA AA AA AA AA
  552.058 b1 7A5  03 19 02 08 00 00 00 00
  552.079 b1 7AD  03 7F 19 78 AA AA AA AA
  552.748 b1 7AD  07 59 02 08 96 03 00 08
  553.828 b1 7A5  01 20 00 00 00 00 00 00
  553.837 b1 7AD  01 60 AA AA AA AA AA AA
```

### 221210-1 — physical HVAC 7B3, t 565.7–566.1 s

```
  565.725 b1 7B3  02 10 03 00 00 00 00 00
  565.739 b1 7BB  02 50 03 00 00 00 00 00
  565.750 b1 7B3  02 10 90 00 00 00 00 00
  565.767 b1 7BB  02 50 90 00 00 00 00 00
  565.786 b1 7B3  01 20 00 00 00 00 00 00
  565.797 b1 7BB  01 60 00 00 00 00 00 00
  565.855 b1 7B3  02 10 03 00 00 00 00 00
  565.867 b1 7BB  02 50 03 00 00 00 00 00
  565.933 b1 7B3  03 19 02 08 00 00 00 00
  565.947 b1 7BB  07 59 02 09 92 38 13 09
  566.001 b1 7B3  01 20 00 00 00 00 00 00
  566.017 b1 7BB  01 60 00 00 00 00 00 00
```

### 221210-1 — physical 7B6, t 566.3–567.7 s

```
  566.328 b1 7B6  02 10 03 00 00 00 00 00
  566.342 b1 7BE  06 50 03 00 32 01 F4 00
  566.352 b1 7B6  02 10 90 00 00 00 00 00
  566.784 b1 7B6  01 20 00 00 00 00 00 00
  566.791 b1 7BE  01 60 00 00 00 00 00 00
  566.852 b1 7B6  02 10 03 00 00 00 00 00
  566.861 b1 7BE  06 50 03 00 32 01 F4 00
  566.930 b1 7B6  03 19 02 08 00 00 00 00
  566.941 b1 7BE  03 59 02 7F 00 00 00 00
  566.993 b1 7B6  03 19 02 AC 00 00 00 00
  567.001 b1 7BE  03 59 02 7F 00 00 00 00
  567.062 b1 7B6  03 19 02 AF 00 00 00 00
  567.071 b1 7BE  03 59 02 7F 00 00 00 00
  567.140 b1 7B6  04 18 00 FF 00 00 00 00
  567.151 b1 7BE  03 7F 18 11 00 00 00 00
  567.194 b1 7B6  04 18 02 FF 00 00 00 00
  567.201 b1 7BE  03 7F 18 11 00 00 00 00
  567.253 b1 7B6  04 18 00 80 00 00 00 00
  567.260 b1 7BE  03 7F 18 11 00 00 00 00
  567.329 b1 7B6  04 18 02 80 00 00 00 00
  567.340 b1 7BE  03 7F 18 11 00 00 00 00
  567.394 b1 7B6  04 18 20 FF 00 00 00 00
  567.400 b1 7BE  03 7F 18 11 00 00 00 00
  567.462 b1 7B6  04 18 60 FF 00 00 00 00
  567.470 b1 7BE  03 7F 18 11 00 00 00 00
  567.543 b1 7B6  04 18 00 40 00 00 00 00
  567.550 b1 7BE  03 7F 18 11 00 00 00 00
  567.631 b1 7B6  01 20 00 00 00 00 00 00
  567.650 b1 7BE  01 60 00 00 00 00 00 00
```

### 221210-1 — physical 7B7, t 568.0–568.4 s

```
  568.036 b1 7B7  02 10 03 00 00 00 00 00
  568.042 b1 7BF  02 50 03 AA AA AA AA AA
  568.063 b1 7B7  02 10 90 00 00 00 00 00
  568.072 b1 7BF  02 50 90 AA AA AA AA AA
  568.091 b1 7B7  01 20 00 00 00 00 00 00
  568.102 b1 7BF  01 60 AA AA AA AA AA AA
  568.168 b1 7B7  02 10 03 00 00 00 00 00
  568.172 b1 7BF  02 50 03 AA AA AA AA AA
  568.238 b1 7B7  03 19 02 08 00 00 00 00
  568.241 b1 7BF  07 59 02 FF 58 12 87 28
  568.334 b1 7B7  01 20 00 00 00 00 00 00
  568.341 b1 7BF  01 60 AA AA AA AA AA AA
```

### 221210-1 — physical 7D0 (no reply), t 568.6–571.4 s

```
  568.692 b1 7D0  02 10 03 00 00 00 00 00
  569.122 b1 7D0  02 10 90 00 00 00 00 00
  569.560 b1 7D0  01 20 00 00 00 00 00 00
  570.005 b1 7D0  02 10 03 00 00 00 00 00
  570.444 b1 7D0  02 10 90 00 00 00 00 00
  570.879 b1 7D0  01 20 00 00 00 00 00 00
  571.324 b1 7D0  01 20 00 00 00 00 00 00
```

### 221210-1 — physical IEB 7D1, t 572.1–572.9 s

```
  572.132 b1 7D1  02 10 03 00 00 00 00 00
  572.137 b1 7D9  02 50 03 AA AA AA AA AA
  572.155 b1 7D1  02 10 90 00 00 00 00 00
  572.619 b1 7D1  01 20 00 00 00 00 00 00
  572.638 b1 7D9  01 60 AA AA AA AA AA AA
  572.688 b1 7D1  02 10 03 00 00 00 00 00
  572.692 b1 7D9  02 50 03 AA AA AA AA AA
  572.748 b1 7D1  03 19 02 08 00 00 00 00
  572.752 b1 7D9  10 17 59 02 89 56 B6 87
  572.752 b1 7D1  30 00 05 00 00 00 00 00
  572.754 b1 7D9  21 08 56 38 08 08 58 06
  572.761 b1 7D9  22 87 08 56 14 87 08 56
  572.769 b1 7D9  23 43 08 08 AA AA AA AA
  572.870 b1 7D1  01 20 00 00 00 00 00 00
  572.871 b1 7D9  01 60 AA AA AA AA AA AA
```

### 221210-1 — physical 7D2, t 576.6–579.2 s

```
  576.630 b1 7D2  02 10 03 00 00 00 00 00
  576.643 b1 7DA  02 50 03 AA AA AA AA AA
  576.657 b1 7D2  02 10 90 00 00 00 00 00
  577.106 b1 7D2  01 20 00 00 00 00 00 00
  577.153 b1 7DA  03 7F 20 78 AA AA AA AA
  577.182 b1 7DA  01 60 AA AA AA AA AA AA
  577.297 b1 7D2  02 10 03 00 00 00 00 00
  577.725 b1 7D2  02 10 90 00 00 00 00 00
  578.178 b1 7D2  01 20 00 00 00 00 00 00
  578.229 b1 7DA  03 7F 20 78 AA AA AA AA
  578.240 b1 7DA  01 60 AA AA AA AA AA AA
  578.361 b1 7D2  03 19 02 08 00 00 00 00
  578.824 b1 7D2  03 19 02 AC 00 00 00 00
  578.829 b1 7DA  03 7F 19 7F AA AA AA AA
  578.955 b1 7D2  04 18 60 FF 00 00 00 00
  578.959 b1 7DA  03 7F 18 11 AA AA AA AA
  579.079 b1 7D2  04 18 20 FF 00 00 00 00
  579.089 b1 7DA  03 7F 18 11 AA AA AA AA
  579.144 b1 7D2  01 20 00 00 00 00 00 00
  579.149 b1 7DA  03 7F 20 7F AA AA AA AA
```

### 221210-1 — physical MDPS 7D4, t 579.4–580.2 s

```
  579.484 b1 7D4  02 10 03 00 00 00 00 00
  579.493 b1 7DC  02 50 03 AA AA AA AA AA
  579.512 b1 7D4  02 10 90 00 00 00 00 00
  579.952 b1 7D4  01 20 00 00 00 00 00 00
  579.963 b1 7DC  01 60 AA AA AA AA AA AA
  580.018 b1 7D4  02 10 03 00 00 00 00 00
  580.027 b1 7DC  02 50 03 AA AA AA AA AA
  580.090 b1 7D4  03 19 02 08 00 00 00 00
  580.097 b1 7DC  07 59 02 89 58 12 87 08
  580.153 b1 7D4  01 20 00 00 00 00 00 00
  580.162 b1 7DC  01 60 AA AA AA AA AA AA
```

### 221210-1 — physical 7C4, t 768.9–784.0 s

```
  768.936 b1 7C4  02 10 03 00 00 00 00 00
  768.943 b1 7CC  06 50 03 00 32 01 F4 AA
  768.959 b1 7C4  02 10 90 00 00 00 00 00
  781.668 b1 7C4  02 10 03 00 00 00 00 00
  781.678 b1 7CC  06 50 03 00 32 01 F4 AA
  781.699 b1 7C4  02 10 90 00 00 00 00 00
  782.165 b1 7C4  01 20 00 00 00 00 00 00
  782.608 b1 7C4  02 10 90 00 00 00 00 00
  783.049 b1 7C4  02 10 90 00 00 00 00 00
  783.485 b1 7C4  01 20 00 00 00 00 00 00
  783.948 b1 7C4  01 20 00 00 00 00 00 00
  783.956 b1 7CC  01 60 AA AA AA AA AA AA
```

### 221210-1 — physical cluster 7C6, t 974.7–975.6 s

```
  974.717 b1 7C6  02 10 03 00 00 00 00 00
  974.738 b1 7CE  06 50 03 00 32 01 F4 AA
  974.856 b1 7C6  02 10 90 00 00 00 00 00
  975.294 b1 7C6  01 20 00 00 00 00 00 00
  975.310 b1 7CE  01 60 AA AA AA AA AA AA
  975.360 b1 7C6  02 10 03 00 00 00 00 00
  975.380 b1 7CE  06 50 03 00 32 01 F4 AA
  975.449 b1 7C6  03 19 02 08 00 00 00 00
  975.460 b1 7CE  07 59 02 09 90 01 52 09
  975.538 b1 7C6  01 20 00 00 00 00 00 00
  975.551 b1 7CE  01 60 AA AA AA AA AA AA
```

### 221210-4 — BMS 7E4 read, D-CAN (bus 1), t 190.1–190.9 s

```
  190.114 b1 7E4  02 10 03 00 00 00 00 00
  190.126 b1 7EC  06 50 03 00 32 01 F4 AA
  190.139 b1 7E4  02 10 90 00 00 00 00 00
  190.579 b1 7E4  01 20 00 00 00 00 00 00
  190.588 b1 7EC  01 60 AA AA AA AA AA AA
  190.631 b1 7E4  02 10 03 00 00 00 00 00
  190.648 b1 7EC  06 50 03 00 32 01 F4 AA
  190.708 b1 7E4  03 19 02 08 00 00 00 00
  190.718 b1 7EC  10 0B 59 02 FF 0A 0A 12
  190.718 b1 7E4  30 00 05 00 00 00 00 00
  190.733 b1 7EC  21 A8 0D 99 12 AF AA AA
  190.820 b1 7E4  01 20 00 00 00 00 00 00
  190.838 b1 7EC  01 60 AA AA AA AA AA AA
```

### 221210-4 — BMS 7E4 clear, D-CAN (bus 1), t 471.6–472.8 s

```
  471.621 b1 7E4  02 10 03 00 00 00 00 00
  471.631 b1 7EC  06 50 03 00 32 01 F4 AA
  471.675 b1 7E4  02 10 90 00 00 00 00 00
  472.153 b1 7E4  01 20 00 00 00 00 00 00
  472.167 b1 7EC  01 60 AA AA AA AA AA AA
  472.210 b1 7E4  02 10 03 00 00 00 00 00
  472.221 b1 7EC  06 50 03 00 32 01 F4 AA
  472.268 b1 7E4  01 14 00 00 00 00 00 00
  472.280 b1 7EC  03 7F 14 13 AA AA AA AA
  472.338 b1 7E4  03 14 FF 00 00 00 00 00
  472.354 b1 7EC  03 7F 14 13 AA AA AA AA
  472.397 b1 7E4  03 14 FF FF 00 00 00 00
  472.405 b1 7EC  03 7F 14 13 AA AA AA AA
  472.514 b1 7E4  04 14 FF FF FF 00 00 00
  472.527 b1 7EC  01 54 AA AA AA AA AA AA
  472.574 b1 7E4  03 14 40 00 00 00 00 00
  472.587 b1 7EC  03 7F 14 13 AA AA AA AA
  472.630 b1 7E4  03 14 60 00 00 00 00 00
  472.640 b1 7EC  03 7F 14 13 AA AA AA AA
  472.688 b1 7E4  03 14 20 00 00 00 00 00
  472.700 b1 7EC  03 7F 14 13 AA AA AA AA
  472.765 b1 7E4  01 20 00 00 00 00 00 00
  472.773 b1 7EC  01 60 AA AA AA AA AA AA
```

### 221210-4 — BMS 7E4 re-read after clear, D-CAN (bus 1), t 484.0–484.9 s

```
  484.073 b1 7E4  02 10 03 00 00 00 00 00
  484.086 b1 7EC  06 50 03 00 32 01 F4 AA
  484.101 b1 7E4  02 10 90 00 00 00 00 00
  484.574 b1 7E4  01 20 00 00 00 00 00 00
  484.585 b1 7EC  01 60 AA AA AA AA AA AA
  484.654 b1 7E4  02 10 03 00 00 00 00 00
  484.665 b1 7EC  06 50 03 00 32 01 F4 AA
  484.716 b1 7E4  03 19 02 08 00 00 00 00
  484.725 b1 7EC  03 59 02 FF AA AA AA AA
  484.786 b1 7E4  03 19 02 AC 00 00 00 00
  484.795 b1 7EC  07 59 02 FF 0D 99 12 27
  484.853 b1 7E4  01 20 00 00 00 00 00 00
  484.865 b1 7EC  01 60 AA AA AA AA AA AA
```

### bench 2023-11-19 — functional reads (default session), t 19.7–21.2 s

```
   19.754 b0 7DF  01 03 00 00 00 00 00 00
   19.758 b0 7ED  02 43 00 00 00 00 00 00
   19.761 b0 7EB  02 43 00 00 00 00 00 00
   19.764 b0 7EA  02 43 00 00 00 00 00 00
   19.764 b0 7EC  02 43 00 AA AA AA AA AA
   20.040 b0 7DF  01 03 00 00 00 00 00 00
   20.041 b0 7EB  02 43 00 00 00 00 00 00
   20.044 b0 7EA  02 43 00 00 00 00 00 00
   20.048 b0 7ED  02 43 00 00 00 00 00 00
   20.048 b0 7EC  02 43 00 AA AA AA AA AA
   20.117 b0 7DF  01 07 00 00 00 00 00 00
   20.121 b0 7EB  02 47 00 00 00 00 00 00
   20.123 b0 7EA  02 47 00 00 00 00 00 00
   20.124 b0 7EC  02 47 00 AA AA AA AA AA
   20.128 b0 7ED  02 47 00 00 00 00 00 00
   20.183 b0 7DF  01 07 00 00 00 00 00 00
   20.189 b0 7ED  02 47 00 00 00 00 00 00
   20.191 b0 7EB  02 47 00 00 00 00 00 00
   20.194 b0 7EA  02 47 00 00 00 00 00 00
   20.194 b0 7EC  02 47 00 AA AA AA AA AA
   20.303 b0 7DF  01 0A 00 00 00 00 00 00
   20.308 b0 7ED  02 4A 00 00 00 00 00 00
   20.311 b0 7EB  02 4A 00 00 00 00 00 00
   20.313 b0 7EA  02 4A 00 00 00 00 00 00
   20.374 b0 7DF  03 19 02 08 00 00 00 00
   20.374 b0 71D  03 7F 19 11 00 00 00 00
   20.375 b0 74E  03 7F 19 11 00 00 00 00
   20.377 b0 7ED  03 7F 19 11 00 00 00 00
   20.380 b0 7EB  03 7F 19 11 00 00 00 00
   20.383 b0 7EA  03 7F 19 12 00 00 00 00
   20.384 b0 7EC  07 59 02 FF D1 28 00 AF
   20.590 b0 7DF  03 19 02 AC 00 00 00 00
   20.590 b0 71D  03 7F 19 11 00 00 00 00
   20.590 b0 74E  03 7F 19 11 00 00 00 00
   20.593 b0 7EA  03 7F 19 12 00 00 00 00
   20.593 b0 7EB  03 7F 19 11 00 00 00 00
   20.597 b0 7ED  03 7F 19 11 00 00 00 00
   20.597 b0 7EC  10 0B 59 02 FF D1 28 00
   20.598 b0 7E4  30 00 00 00 00 00 00 00
   20.607 b0 7EC  21 AF 1A 6F 00 A7 AA AA
   20.678 b0 7DF  03 19 02 AF 00 00 00 00
   20.679 b0 74E  03 7F 19 11 00 00 00 00
   20.679 b0 71D  03 7F 19 11 00 00 00 00
   20.680 b0 7EB  03 7F 19 11 00 00 00 00
   20.683 b0 7EA  03 7F 19 12 00 00 00 00
   20.688 b0 7EC  10 0B 59 02 FF D1 28 00
   20.689 b0 7ED  03 7F 19 11 00 00 00 00
   20.689 b0 7E4  30 00 00 00 00 00 00 00
   20.697 b0 7EC  21 AF 1A 6F 00 A7 AA AA
   20.753 b0 7DF  04 18 00 FF 00 00 00 00
   20.754 b0 71D  03 7F 18 11 00 00 00 00
   20.754 b0 74E  03 7F 18 11 00 00 00 00
   20.757 b0 7ED  10 08 58 02 0D 59 28 C1
   20.757 b0 7E5  30 00 00 00 00 00 00 00
   20.760 b0 7EB  02 58 00 00 00 00 00 00
   20.763 b0 7EA  10 0E 58 04 1A 78 E8 C1
   20.763 b0 7E2  30 00 00 00 00 00 00 00
   20.772 b0 7EA  21 29 E4 C1 46 64 C1 64
   20.773 b0 7ED  21 46 E4 00 00 00 00 00
   20.782 b0 7EA  22 64 00 00 00 00 00 00
   20.959 b0 7DF  04 18 00 FF 00 00 00 00
   20.959 b0 71D  03 7F 18 11 00 00 00 00
   20.960 b0 74E  03 7F 18 11 00 00 00 00
   20.962 b0 7EA  10 0E 58 04 1A 78 E8 C1
   20.963 b0 7EB  02 58 00 00 00 00 00 00
   20.963 b0 7E2  30 00 00 00 00 00 00 00
   20.973 b0 7EA  21 29 E4 C1 46 64 C1 64
   20.974 b0 7ED  10 08 58 02 0D 59 28 C1
   20.974 b0 7E5  30 00 00 00 00 00 00 00
   20.977 b0 7ED  21 46 E4 00 00 00 00 00
   20.982 b0 7EA  22 64 00 00 00 00 00 00
   21.093 b0 7DF  04 18 02 FF 00 00 00 00
   21.093 b0 74E  03 7F 18 11 00 00 00 00
   21.094 b0 71D  03 7F 18 11 00 00 00 00
   21.096 b0 7ED  03 7F 18 12 00 00 00 00
   21.099 b0 7EB  03 7F 18 12 00 00 00 00
   21.102 b0 7EA  03 7F 18 12 00 00 00 00
```

### bench 2023-11-19 — physical VCU 7E2, t 30.0–31.0 s

```
   30.064 b0 7E2  02 10 03 00 00 00 00 00
   30.071 b0 7EA  03 7F 10 12 00 00 00 00
   30.087 b0 7E2  02 10 03 00 00 00 00 00
   30.091 b0 7EA  03 7F 10 12 00 00 00 00
   30.112 b0 7E2  03 19 02 08 00 00 00 00
   30.121 b0 7EA  03 7F 19 12 00 00 00 00
   30.191 b0 7E2  03 19 02 AC 00 00 00 00
   30.201 b0 7EA  03 7F 19 12 00 00 00 00
   30.248 b0 7E2  03 19 02 AF 00 00 00 00
   30.251 b0 7EA  03 7F 19 12 00 00 00 00
   30.303 b0 7E2  04 18 00 FF 00 00 00 00
   30.311 b0 7EA  10 0E 58 04 1A 78 E8 C1
   30.311 b0 7E2  30 00 05 00 00 00 00 00
   30.321 b0 7EA  21 29 E4 C1 46 64 C1 64
   30.331 b0 7EA  22 64 00 00 00 00 00 00
   30.401 b0 7E2  04 18 02 FF 00 00 00 00
   30.410 b0 7EA  03 7F 18 12 00 00 00 00
   30.462 b0 7E2  04 18 00 80 00 00 00 00
   30.470 b0 7EA  02 58 00 00 00 00 00 00
   30.535 b0 7E2  04 18 02 80 00 00 00 00
   30.540 b0 7EA  03 7F 18 12 00 00 00 00
   30.587 b0 7E2  04 18 20 FF 00 00 00 00
   30.590 b0 7EA  02 58 00 00 00 00 00 00
   30.661 b0 7E2  04 18 60 FF 00 00 00 00
   30.670 b0 7EA  10 0E 58 04 1A 78 E8 C1
   30.671 b0 7E2  30 00 05 00 00 00 00 00
   30.680 b0 7EA  21 29 E4 C1 46 64 C1 64
   30.690 b0 7EA  22 64 00 00 00 00 00 00
   30.737 b0 7E2  04 18 00 40 00 00 00 00
   30.740 b0 7EA  02 58 00 00 00 00 00 00
   30.970 b0 7E2  01 20 00 00 00 00 00 00
   30.980 b0 7EA  01 60 00 00 00 00 00 00
```

### bench 2023-11-19 — physical 7B6, t 111.3–112.4 s

```
  111.372 b0 7B6  02 10 03 00 00 00 00 00
  111.380 b0 7BE  06 50 03 00 32 01 F4 00
  111.400 b0 7B6  02 10 03 00 00 00 00 00
  111.410 b0 7BE  06 50 03 00 32 01 F4 00
  111.428 b0 7B6  03 19 02 08 00 00 00 00
  111.431 b0 7BE  07 59 02 7F 56 14 87 2F
  111.495 b0 7B6  03 19 02 AC 00 00 00 00
  111.500 b0 7BE  07 59 02 7F 56 14 87 2F
  111.547 b0 7B6  03 19 02 AF 00 00 00 00
  111.550 b0 7BE  07 59 02 7F 56 14 87 2F
  111.596 b0 7B6  04 18 00 FF 00 00 00 00
  111.600 b0 7BE  03 7F 18 11 00 00 00 00
  111.681 b0 7B6  04 18 02 FF 00 00 00 00
  111.690 b0 7BE  03 7F 18 11 00 00 00 00
  111.741 b0 7B6  04 18 00 80 00 00 00 00
  111.750 b0 7BE  03 7F 18 11 00 00 00 00
  111.915 b0 7B6  04 18 02 80 00 00 00 00
  111.920 b0 7BE  03 7F 18 11 00 00 00 00
  111.970 b0 7B6  04 18 20 FF 00 00 00 00
  111.980 b0 7BE  03 7F 18 11 00 00 00 00
  112.033 b0 7B6  04 18 60 FF 00 00 00 00
  112.040 b0 7BE  03 7F 18 11 00 00 00 00
  112.100 b0 7B6  04 18 00 40 00 00 00 00
  112.109 b0 7BE  03 7F 18 11 00 00 00 00
  112.329 b0 7B6  01 20 00 00 00 00 00 00
  112.339 b0 7BE  01 60 00 00 00 00 00 00
```

### bench 2023-11-19 — clear sequence (functional, then physical), t 281.0–285.9 s

```
  281.105 b0 7DF  01 04 00 00 00 00 00 00
  281.108 b0 7EB  03 7F 04 22 00 00 00 00
  281.112 b0 7EA  03 7F 04 22 00 00 00 00
  281.113 b0 7EC  01 44 AA AA AA AA AA AA
  281.115 b0 7ED  03 7F 04 22 00 00 00 00
  281.162 b0 7DF  01 04 00 00 00 00 00 00
  281.162 b0 7EA  03 7F 04 22 00 00 00 00
  281.169 b0 7EB  03 7F 04 22 00 00 00 00
  281.170 b0 7EC  01 44 AA AA AA AA AA AA
  281.170 b0 7ED  03 7F 04 22 00 00 00 00
  281.217 b0 7DF  01 04 00 00 00 00 00 00
  281.222 b0 7EA  03 7F 04 22 00 00 00 00
  281.223 b0 7EC  01 44 AA AA AA AA AA AA
  281.225 b0 7ED  03 7F 04 22 00 00 00 00
  281.227 b0 7EB  03 7F 04 22 00 00 00 00
  281.272 b0 7DF  01 14 00 00 00 00 00 00
  281.272 b0 74E  03 7F 14 11 00 00 00 00
  281.273 b0 71D  03 7F 14 11 00 00 00 00
  281.275 b0 7ED  03 7F 14 79 00 00 00 00
  281.277 b0 7EB  03 7F 14 79 00 00 00 00
  281.279 b0 7BE  03 7F 14 13 00 00 00 00
  281.279 b0 7EC  03 7F 14 13 AA AA AA AA
  281.282 b0 7EA  03 7F 14 79 00 00 00 00
  281.346 b0 7DF  03 14 FF 00 00 00 00 00
  281.354 b0 71D  03 7F 14 11 00 00 00 00
  281.354 b0 74E  03 7F 14 11 00 00 00 00
  281.354 b0 7BE  03 7F 14 13 00 00 00 00
  281.355 b0 7EA  03 54 FF 00 00 00 00 00
  281.355 b0 7EB  03 54 FF 00 00 00 00 00
  281.355 b0 7EC  03 7F 14 13 AA AA AA AA
  281.356 b0 7ED  03 54 FF 00 00 00 00 00
  281.399 b0 7DF  03 14 FF FF 00 00 00 00
  281.399 b0 74E  03 7F 14 11 00 00 00 00
  281.399 b0 71D  03 7F 14 11 00 00 00 00
  281.402 b0 7EA  03 7F 14 12 00 00 00 00
  281.404 b0 7ED  03 7F 14 12 00 00 00 00
  281.407 b0 7EB  03 7F 14 12 00 00 00 00
  281.409 b0 7BE  03 7F 14 13 00 00 00 00
  281.409 b0 7EC  03 7F 14 13 AA AA AA AA
  281.667 b0 7DF  04 14 FF FF FF 00 00 00
  281.668 b0 71D  03 7F 14 11 00 00 00 00
  281.669 b0 74E  03 7F 14 11 00 00 00 00
  281.669 b0 7BE  01 54 00 00 00 00 00 00
  281.671 b0 7EA  03 7F 14 79 00 00 00 00
  281.672 b0 7EC  01 54 AA AA AA AA AA AA
  281.674 b0 7ED  03 7F 14 79 00 00 00 00
  281.677 b0 7EB  03 7F 14 79 00 00 00 00
  282.021 b0 7E2  02 10 03 00 00 00 00 00
  282.031 b0 7EA  03 7F 10 12 00 00 00 00
  282.041 b0 7E2  02 10 03 00 00 00 00 00
  282.054 b0 7EA  03 7F 10 12 00 00 00 00
  282.155 b0 7E2  01 14 00 00 00 00 00 00
  282.161 b0 7EA  03 7F 14 79 00 00 00 00
  282.280 b0 7E2  03 14 FF 00 00 00 00 00
  282.291 b0 7EA  03 54 FF 00 00 00 00 00
  282.353 b0 7E2  03 14 FF FF 00 00 00 00
  282.361 b0 7EA  03 7F 14 12 00 00 00 00
  282.416 b0 7E2  04 14 FF FF FF 00 00 00
  282.421 b0 7EA  03 7F 14 79 00 00 00 00
  282.507 b0 7E2  03 14 40 00 00 00 00 00
  282.510 b0 7EA  03 54 40 00 00 00 00 00
  282.596 b0 7E2  03 14 60 00 00 00 00 00
  282.600 b0 7EA  03 7F 14 12 00 00 00 00
  282.660 b0 7E2  03 14 20 00 00 00 00 00
  282.670 b0 7EA  03 7F 14 12 00 00 00 00
  282.907 b0 7E2  01 20 00 00 00 00 00 00
  282.910 b0 7EA  01 60 00 00 00 00 00 00
  283.081 b0 7E4  02 10 03 00 00 00 00 00
  283.091 b0 7EC  06 50 03 00 32 01 F4 AA
  283.105 b0 7E4  02 10 03 00 00 00 00 00
  283.111 b0 7EC  06 50 03 00 32 01 F4 AA
  283.129 b0 7E4  01 14 00 00 00 00 00 00
  283.136 b0 7EC  03 7F 14 13 AA AA AA AA
  283.195 b0 7E4  03 14 FF 00 00 00 00 00
  283.200 b0 7EC  03 7F 14 13 AA AA AA AA
  283.255 b0 7E4  03 14 FF FF 00 00 00 00
  283.260 b0 7EC  03 7F 14 13 AA AA AA AA
  283.352 b0 7E4  04 14 FF FF FF 00 00 00
  283.360 b0 7EC  01 54 AA AA AA AA AA AA
  283.422 b0 7E4  03 14 40 00 00 00 00 00
  283.430 b0 7EC  03 7F 14 13 AA AA AA AA
  283.527 b0 7E4  03 14 60 00 00 00 00 00
  283.536 b0 7EC  03 7F 14 13 AA AA AA AA
  283.633 b0 7E4  03 14 20 00 00 00 00 00
  283.640 b0 7EC  03 7F 14 13 AA AA AA AA
  283.864 b0 7E4  01 20 00 00 00 00 00 00
  283.870 b0 7EC  01 60 AA AA AA AA AA AA
  284.328 b0 7E5  02 10 03 00 00 00 00 00
  284.331 b0 7ED  03 7F 10 12 00 00 00 00
  284.363 b0 7E5  02 10 03 00 00 00 00 00
  284.371 b0 7ED  03 7F 10 12 00 00 00 00
  284.398 b0 7E5  01 14 00 00 00 00 00 00
  284.401 b0 7ED  03 7F 14 79 00 00 00 00
  284.456 b0 7E5  03 14 FF 00 00 00 00 00
  284.465 b0 7ED  03 54 FF 00 00 00 00 00
  284.523 b0 7E5  03 14 FF FF 00 00 00 00
  284.531 b0 7ED  03 7F 14 12 00 00 00 00
  284.591 b0 7E5  04 14 FF FF FF 00 00 00
  284.601 b0 7ED  03 7F 14 79 00 00 00 00
  284.674 b0 7E5  03 14 40 00 00 00 00 00
  284.680 b0 7ED  03 54 40 00 00 00 00 00
  284.727 b0 7E5  03 14 60 00 00 00 00 00
  284.730 b0 7ED  03 7F 14 12 00 00 00 00
  284.775 b0 7E5  03 14 20 00 00 00 00 00
  284.780 b0 7ED  03 7F 14 12 00 00 00 00
  284.990 b0 7E5  01 20 00 00 00 00 00 00
  285.000 b0 7ED  01 60 00 00 00 00 00 00
  285.117 b0 7B6  02 10 03 00 00 00 00 00
  285.124 b0 7BE  06 50 03 00 32 01 F4 00
  285.139 b0 7B6  02 10 03 00 00 00 00 00
  285.149 b0 7BE  06 50 03 00 32 01 F4 00
  285.164 b0 7B6  01 14 00 00 00 00 00 00
  285.174 b0 7BE  03 7F 14 13 00 00 00 00
  285.230 b0 7B6  03 14 FF 00 00 00 00 00
  285.233 b0 7BE  03 7F 14 13 00 00 00 00
  285.274 b0 7B6  03 14 FF FF 00 00 00 00
  285.284 b0 7BE  03 7F 14 13 00 00 00 00
  285.384 b0 7B6  04 14 FF FF FF 00 00 00
  285.394 b0 7BE  01 54 00 00 00 00 00 00
  285.449 b0 7B6  03 14 40 00 00 00 00 00
  285.453 b0 7BE  03 7F 14 13 00 00 00 00
  285.592 b0 7B6  03 14 60 00 00 00 00 00
  285.593 b0 7BE  03 7F 14 13 00 00 00 00
  285.649 b0 7B6  03 14 20 00 00 00 00 00
  285.653 b0 7BE  03 7F 14 13 00 00 00 00
  285.878 b0 7B6  01 20 00 00 00 00 00 00
  285.883 b0 7BE  01 60 00 00 00 00 00 00
```

### bench 2023-11-19 — re-read after clear (functional), t 302.9–303.8 s

```
  302.995 b0 7DF  01 03 00 00 00 00 00 00
  302.998 b0 7ED  02 43 00 00 00 00 00 00
  303.000 b0 7EB  02 43 00 00 00 00 00 00
  303.002 b0 7EC  02 43 00 AA AA AA AA AA
  303.005 b0 7EA  02 43 00 00 00 00 00 00
  303.058 b0 7DF  01 03 00 00 00 00 00 00
  303.060 b0 7EB  02 43 00 00 00 00 00 00
  303.065 b0 7EA  02 43 00 00 00 00 00 00
  303.066 b0 7EC  02 43 00 AA AA AA AA AA
  303.068 b0 7ED  02 43 00 00 00 00 00 00
  303.115 b0 7DF  01 07 00 00 00 00 00 00
  303.118 b0 7ED  02 47 00 00 00 00 00 00
  303.127 b0 7EA  02 47 00 00 00 00 00 00
  303.128 b0 7EB  02 47 00 00 00 00 00 00
  303.128 b0 7EC  02 47 00 AA AA AA AA AA
  303.198 b0 7DF  01 07 00 00 00 00 00 00
  303.200 b0 7EB  02 47 00 00 00 00 00 00
  303.205 b0 7EA  02 47 00 00 00 00 00 00
  303.206 b0 7EC  02 47 00 AA AA AA AA AA
  303.208 b0 7ED  02 47 00 00 00 00 00 00
  303.269 b0 7DF  01 0A 00 00 00 00 00 00
  303.270 b0 7EB  02 4A 00 00 00 00 00 00
  303.275 b0 7EA  02 4A 00 00 00 00 00 00
  303.277 b0 7ED  02 4A 00 00 00 00 00 00
  303.327 b0 7DF  03 19 02 08 00 00 00 00
  303.327 b0 71D  03 7F 19 11 00 00 00 00
  303.327 b0 74E  03 7F 19 11 00 00 00 00
  303.328 b0 7ED  03 7F 19 11 00 00 00 00
  303.330 b0 7EB  03 7F 19 11 00 00 00 00
  303.335 b0 7EA  03 7F 19 12 00 00 00 00
  303.336 b0 7EC  03 59 02 FF AA AA AA AA
  303.385 b0 7DF  03 19 02 AC 00 00 00 00
  303.385 b0 71D  03 7F 19 11 00 00 00 00
  303.386 b0 74E  03 7F 19 11 00 00 00 00
  303.387 b0 7ED  03 7F 19 11 00 00 00 00
  303.390 b0 7EB  03 7F 19 11 00 00 00 00
  303.391 b0 7EC  10 0B 59 02 FF 1A 6F 00
  303.392 b0 7E4  30 00 00 00 00 00 00 00
  303.395 b0 7EA  03 7F 19 12 00 00 00 00
  303.402 b0 7EC  21 27 D1 28 00 27 AA AA
  303.445 b0 7DF  03 19 02 AF 00 00 00 00
  303.445 b0 71D  03 7F 19 11 00 00 00 00
  303.445 b0 74E  03 7F 19 11 00 00 00 00
  303.447 b0 7ED  03 7F 19 11 00 00 00 00
  303.450 b0 7EB  03 7F 19 11 00 00 00 00
  303.451 b0 7EC  10 0B 59 02 FF 1A 6F 00
  303.452 b0 7E4  30 00 00 00 00 00 00 00
  303.455 b0 7EA  03 7F 19 12 00 00 00 00
  303.462 b0 7EC  21 27 D1 28 00 27 AA AA
  303.572 b0 7DF  04 18 00 FF 00 00 00 00
  303.573 b0 71D  03 7F 18 11 00 00 00 00
  303.574 b0 74E  03 7F 18 11 00 00 00 00
  303.575 b0 7EA  10 0E 58 04 1A 78 C8 C1
  303.575 b0 7E2  30 00 00 00 00 00 00 00
  303.577 b0 7ED  05 58 01 C1 46 44 00 00
  303.580 b0 7EB  02 58 00 00 00 00 00 00
  303.585 b0 7EA  21 29 44 C1 46 44 C1 64
  303.594 b0 7EA  22 44 00 00 00 00 00 00
  303.654 b0 7DF  04 18 00 FF 00 00 00 00
  303.654 b0 74E  03 7F 18 11 00 00 00 00
  303.655 b0 71D  03 7F 18 11 00 00 00 00
  303.657 b0 7ED  05 58 01 C1 46 44 00 00
  303.660 b0 7EB  02 58 00 00 00 00 00 00
  303.664 b0 7EA  10 0E 58 04 1A 78 C8 C1
  303.665 b0 7E2  30 00 00 00 00 00 00 00
  303.674 b0 7EA  21 29 44 C1 46 44 C1 64
  303.684 b0 7EA  22 44 00 00 00 00 00 00
  303.731 b0 7DF  04 18 02 FF 00 00 00 00
  303.731 b0 74E  03 7F 18 11 00 00 00 00
  303.732 b0 71D  03 7F 18 11 00 00 00 00
  303.734 b0 7EA  03 7F 18 12 00 00 00 00
  303.741 b0 7EB  03 7F 18 12 00 00 00 00
  303.742 b0 7ED  03 7F 18 12 00 00 00 00
```
