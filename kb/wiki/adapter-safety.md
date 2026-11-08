---
title: Adapter safety — commands carlink must never send
type: concept
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/obdlink-frpm-rev-f.md
  - raw/elm327-datasheet.md
tags: [safety, nvm, adapter, policy]
confidence: verified
---

# Adapter safety — commands carlink must never send

Two different hazards: (1) commands that **change the [[obdlink-ex]] itself** persistently
(NVM, one-time-programmable memory, firmware) and (2) OBD/UDS requests that **write to the
car**. The second is governed by the schema rule: `carlink` is read-only; writes to an ECU are
dealer/HDS/GDS tasks. This page covers the first and lists the obvious bus-side ones.

## Never send — permanent or one-time (FRPM §7.2, §8.4, §8.5)

| Command | Why |
|---------|-----|
| `AT@3 cccccccccccc` | Sets device identifier — **can only be used once, ever** |
| `STSDI …` | One-time hardware ID string (standalone ICs, but never risk it) |
| `STSAVCAL` | Writes voltage calibration to one-time-programmable memory |

## Never send — writes NVM (survives power cycle)

| Command | Persistent effect | Source |
|---------|-------------------|--------|
| `ATPP xx SV yy`, `ATPP xx ON/OFF`, `ATPP FF ON/OFF` | Programmable parameters: change echo, headers, timeouts, **UART baud (PP 0C)**, CR/LF chars, PowerSave… for every future session and app | FRPM §7.3 |
| `STWBR` | Saves the current UART baud — a wrong value makes the adapter unreachable at 115200 | FRPM §8.3 |
| `STSBR baud` / `STBR baud` / `ATBRD` | Changes UART baud now (volatile until `STWBR`, but a host that does not follow loses the session) | FRPM §8.3 |
| `STRSTNVM` | Factory-resets all NVM: PPs, saved protocol, baud, voltage cal, ID strings, PowerSave | FRPM §8.2 |
| `STSATI`, `STS@1` | Rewrite `ATI` / `AT@1` strings (persist; other apps may key on them) | FRPM §8.4, §8.2 |
| `ATSD hh` | Writes the user NVM data byte (not reset even by STRSTNVM) | FRPM §8.2 |
| `STVCAL`, `ATCV` | Voltage calibration (reset only by STRSTNVM) | FRPM §8.5 |
| `STSL*` setters (`STSLU`, `STSLVL`, `STSLX`, `STSLPA`, …) | "All PowerSave settings are saved in NVM" — can make the adapter sleep or not wake | FRPM §8.11 |
| `STBT*` | Bluetooth config in NVM (not applicable to EX) | FRPM §8.12 |
| `ATSP p` (p ≠ 0) | Saves a fixed protocol to NVM; use `STP p` instead. `ATSP 0` / `ATSP 00` (auto) are allowed | FRPM ATSP |
| `ATM1` (default) side effect | With memory on, every *detected* protocol is written to NVM and tried first after reset. Harmless (auto-search still falls back); `ATM0` would stop it but whether `ATM` itself persists on OBDLink is undocumented — leave it alone | FRPM ATM, ATSP; ELM DS M0/M1 |

## Never send — disruptive

| Command | Why |
|---------|-----|
| Firmware update / bootloader | Done only with OBDLink's own updater app; an interrupted flash can brick the adapter. No `carlink` path. FRPM §4.0 mentions the bootloader; no command for it is documented. |
| `STSLEEP`, `ATLP` | Puts the adapter to sleep; wake-up depends on PowerSave config |
| `STCMM 1` while monitoring | Adapter starts ACKing frames — it becomes an active node |
| `STPPMA` (periodic messages), `STGPOW` | Transmit on the bus / drive GPIO unasked |
| A bare `\r` after a write-type request | Repeats the last command (FRPM §6.0) |

## Never send to the car (read-only policy)

OBD mode `04` (clear DTCs / reset monitors — also erases freeze frames and readiness),
mode `08` (on-board test control), UDS `0x2E` WriteDataByIdentifier, `0x2F` IO control,
`0x31` RoutineControl, `0x27` SecurityAccess, `0x34`–`0x37` download/upload, `0x11`
ECUReset, `0x28` CommunicationControl, `0x85` ControlDTCSetting, `0x14` ClearDiagnosticInfo.
*Service IDs from ISO 14229/SAE J1979 general knowledge, not from a fetched source in this
wiki — unverified.* Clearing DTCs, if ever wanted, needs Aaron's explicit say-so per run.

## Safe to send

Everything in the "Commands carlink needs" table of [[elm327-at-commands]], the identity and
volatile protocol commands in [[stn-st-commands]], `ATPPS` (print only), and OBD modes
`01 02 03 06 07 09 0A` (reads).

## Enforcement suggestion

Gate on an **allowlist** of exact command prefixes in `carlink`, not a denylist: the STN set
is large and grows with firmware. Anything not on the list should be refused loudly.
