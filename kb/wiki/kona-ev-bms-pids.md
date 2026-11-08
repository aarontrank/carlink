---
title: Kona Electric BMS PIDs (7E4, DIDs 220101-220106) — byte map
type: concept
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/obdb-hyundai-kona-electric.md
  - raw/evdash-carkiaeniro-src.md
  - raw/jejusoul-kona-niro-ev-pids.md
  - raw/elm327-datasheet.md
tags: [kona-electric, bms, uds, service-22, byte-map]
confidence: community
---

# Kona Electric BMS PIDs — byte map

Battery Management System of the [[kona-electric-2020]] (64 kWh, 98 series cell groups). Read with
UDS service 0x22 (ReadDataByIdentifier) through the [[obdlink-ex]]. Interpretation of the numbers is in
[[kona-ev-battery-health]]; other modules are in [[kona-ev-other-ecus]].

**Confidence.** Every offset below agrees across three independent sources — JejuSoul's Torque CSVs,
evDash's C++ decoder, and OBDb's signalset — *and* reproduces OBDb's expected values from real
MY2020 captures (worked example below). That makes them `community` with strong agreement, not
`verified`: none of it has been observed yet on Aaron's car via `carlink`. Where the sources
disagree, the disagreement is flagged in place.

## Addressing and adapter setup

| Item | Value | Source |
|---|---|---|
| Protocol | ISO 15765-4 CAN, 11-bit, 500 kbit/s (`ATSP6`) | evDash command queue |
| Request ID | `7E4` (`ATSH7E4`) | all three |
| Response ID | `7EC` | OBDb test captures |
| Session | default session; no `10 03` needed | evDash, OBDb (no `din`) |
| Flow control | OBDb sets `ATFCSH7E4`, `ATFCSD300000`, `ATFCSM1` before these reads (`fcm1: true`). evDash uses the ELM default (mode 0) and works. | OBDb signalset; evDash |

ELM327 order rule: `AT FC SH` and `AT FC SD` must be sent **before** `AT FC SM 1`, otherwise `SM 1`
answers `?` (ELM327 datasheet, "Altering Flow Control Messages"; see [[adapter-response-format]]).
OBDb's note on why it uses mode 1: on some reads, default mode 0 returns only the First Frame. A
safe recipe is to use mode 1 with `FCSH` equal to the request header. `ATCRA7EC` is optional; no
source needs it, because 7E4 is the only responder to a physical 7E4 request.

Recommended sequence (inferred from evDash plus OBDb; not observed yet on the car):

```
ATZ  ATE0  ATL0  ATS0  ATH1  ATSP6  ATST16(optional)
ATSH7E4  ATFCSH7E4  ATFCSD300000  ATFCSM1
220101  220102  220103  220104  220105  220106
```

The car must be awake (ignition ON or charging) for most BMS reads. evDash's sleep-mode queue still
polls `220105` alone (`commandAllowed()`), which suggests the BMS answers at least 220105 while
asleep or idle.

## Indexing convention (read this before using any offset)

1. Reassemble the ISO-TP message. Drop the PCI bytes (`10 LL` first frame, `2N` consecutive frames)
   and truncate to the length `0xLLL` given in the first frame. Trailing `AA` bytes are CAN padding.
2. The reassembled message starts `62 01 0X` (positive response + DID).
3. **`p` = 0-based index into the bytes *after* those 3 header bytes.** Every table below uses `p`.

How the community sources map onto `p`:

| Source | Its notation | Conversion to `p` |
|---|---|---|
| JejuSoul / Torque Pro | letters `A`,`B`,…,`Z`,`AA`,`AB`,… (A = first byte after `62 01 0X`) | A=0 … Z=25, AA=26 … AZ=51, BA=52 … |
| evDash | `hexToDecFromResponse(from,to,…)` = hex-char slice of merged string *including* `620101` | `p = from/2 − 3` |
| OBDb | `bix` = bit index after the header, MSB-first | `p = bix/8`; bit `b` of byte `p` (LSB=0) is `bix = 8p + (7−b)` |

Multi-byte values are big-endian. "s8" means two's-complement signed byte.

## Response lengths (reassembled, including the 3 header bytes)

| DID | Length | Frames | Source |
|---|---|---|---|
| 220101 | 0x03E = 62 | FF + 8 CF | OBDb capture, evDash sample |
| 220102 / 0103 / 0104 | 0x027 = 39 (3 header + 4 bytes + 32 cells) | FF + 5 CF | OBDb capture |
| 220105 | 0x02E = 46 | FF + 6 CF | OBDb capture |
| 220106 | ≥ 27 (evDash sample is 27 bytes) | — | evDash test data |

## 220101 — pack summary

| p | Torque | Field | Formula | Unit / sign |
|---|---|---|---|---|
| 0–3 | A–D | (constant `FF F7 E7 FF` in all samples) | — | — |
| 4 | E | SOC (BMS, true) | `u8 / 2` | % |
| 5–6 | F–G | Available charge (regen) power | `u16 / 100` | kW |
| 7–8 | H–I | Available discharge power | `u16 / 100` | kW |
| 9 | J | Status bits: bit7 = "charging", bit6 = rapid (CCS) plug, bit5 = normal (J1772) plug, bit0 = BMS main relay | bits | — |
| 10–11 | K–L | Pack current | `s16 / 10` | A; **positive = discharge**, negative = charging |
| 12–13 | M–N | Pack DC voltage | `u16 / 10` | V |
| 14 | O | Battery max temp | `s8` | °C |
| 15 | P | Battery min temp | `s8` | °C |
| 16–19 | Q–T | Module 1–4 temperature | `s8` each | °C |
| 20–21 | U–V | (unused / 0 in samples) | — | — |
| 22 | W | Battery inlet temp | `s8` | °C |
| 23 | X | Max cell voltage | `u8 / 50` | V (0.02 V steps) |
| 24 | Y | Max cell number | `u8` | 1–98 |
| 25 | Z | Min cell voltage | `u8 / 50` | V |
| 26 | AA | Min cell number | `u8` | 1–98 |
| 27 | AB | Battery fan status | `u8` | mode code |
| 28 | AC | Battery fan feedback | `u8` | Hz |
| 29 | AD | Aux (12 V) voltage seen by the BMS | `u8 / 10` | V (0.1 V steps) |
| 30–33 | AE–AH | CCC, cumulative charge current | `u32 / 10` | Ah |
| 34–37 | AI–AL | CDC, cumulative discharge current | `u32 / 10` | Ah |
| 38–41 | AM–AP | CEC, cumulative energy charged | `u32 / 10` | kWh |
| 42–45 | AQ–AT | CED, cumulative energy discharged | `u32 / 10` | kWh |
| 46–49 | AU–AX | Operating time | `u32` | seconds (÷3600 → h) |
| 50 | AY | bit2 = BMS ignition | bit | — |
| 51–52 | AZ–BA | Inverter capacitor voltage | `u16` | V |
| 53–54 | BB–BC | Drive motor speed 1 | `s16` | rpm |
| 55–56 | BD–BE | Drive motor speed 2 | `s16` | rpm |
| 57–58 | BF–BG | Isolation resistance | `u16` | kΩ |

Notes and disagreements:

- **Current sign.** Raw `K:L` is positive while discharging. evDash negates it (`batPowerAmp = -raw/10`)
  so that charging reads positive in its UI. Pack power = V × I / 1000 kW.
- **Max/min temperature.** evDash ignores `O`/`P` and computes min/max over modules 1–4 itself. It
  calls its version "more accurate", and uses it for the DC-charge cold gates.
- **Module count.** JejuSoul, OBDb and evDash (`batModuleTempCount = 4`) all read only 4 module temps
  for this pack.
- **Status bits.** The bits in `J` come from JejuSoul `{j:7}` etc. and OBDb bix 72/73/74/79. In OBDb's
  real capture, bit7 "charging" = 1 while both plug bits = 0 and current = 0, so bit7 is not a
  reliable "charging now" flag. For charging state, prefer 220105 `V` (below, from evDash).

## 220102 / 220103 / 220104 — cell voltages 1–96

| DID | p = 0–3 | p = 4 … 35 |
|---|---|---|
| 220102 | `FF FF FF FF` | cells **1–32**: cell n at `p = n + 3` |
| 220103 | `FF FF FF FF` | cells **33–64**: cell n at `p = n − 29` |
| 220104 | `FF FF FF FF` | cells **65–96**: cell n at `p = n − 61` |

Each cell is `u8 / 50` V (Torque `E`…`AJ`). evDash only accepts 220102 if `p=3` is `FF`. It does
this because 7B3 (HVAC) also has a DID 0102, and the check guards against a mis-routed reply.

## 220105 — SOH, displayed SOC, cells 97–98

| p | Torque | Field | Formula | Unit |
|---|---|---|---|---|
| 0–19 | A–T | (mostly constant / unknown) | — | — |
| 20 | U | Cell voltage deviation (max−min) | `u8 / 50` | V |
| 21 | V | Charging flags: bit6 = AC charger connected, bit5 = DC charger connected; charging when (bit5 or bit6) and low nibble in 5…9 | bits | — (evDash only) |
| 22 | W | Airbag hard-wire duty | `u8` | % (≈100 normal) |
| 23 | X | Battery heater 1 temp | `s8` | °C |
| 24 | Y | Battery heater 2 temp | JejuSoul `s8`; OBDb calls it a "raw byte" (75 in a 7 °C capture, so probably not a temp) | ? |
| 25–26 | Z–AA | **SOH** | `u16 / 10` | % |
| 27 | AB | Max deterioration cell number | `u8` | 1–98 |
| 28–29 | AC–AD | Min deterioration | `u16 / 10` | % |
| 30 | AE | Min deterioration cell number | `u8` | 1–98 |
| 31 | AF | **Displayed SOC** (cluster) | `u8 / 2` | % |
| 34 | AI | Cell 97 voltage | `u8 / 50` | V |
| 35 | AJ | Cell 98 voltage | `u8 / 50` | V |

evDash also reads `p=12` (s8, "bmsUnknownTempA") and `p=38` (s8, "bmsUnknownTempB"). Its heater
temp is `p=23` (`batHeaterC`), which matches JejuSoul `X`. JejuSoul's ABRP file labels the SOH field
"Max Deterioration". The naming is muddled, but the bytes are the same Z:AA.

## 220106 — cooling and battery management mode (evDash, OBDb)

| p | Field | Formula | Source |
|---|---|---|---|
| 4 | Cooling-water temp at battery inlet (WCS) | `s8` °C | evDash `coolingWaterTempC`; OBDb `HVBAT_WCS_INLET_T` bix 32 |
| 6 | unknown temp C | `s8` | evDash |
| 8 | Battery fan duty feedback | `u8` % | OBDb bix 64 |
| 12 | A/C compressor speed | `u8 × 20` rpm | OBDb bix 96 |
| 14 | Battery management mode, low nibble: 1 = LTR cooling, 3 = LTR, 4 = cooling, 6 = off, 0xE = PTC heater | nibble | evDash |
| 20 | Temp at power relay assembly | OBDb: `s8 × 2` °C (bix 160); evDash reads the same byte as plain `s8` "unknown temp D" | OBDb, evDash (scale disagrees) |

`unverified`: only two sources, and nobody has a real capture with known values.

## Worked example (real MY2020 capture from OBDb, headers on, `ATS0`)

Request `220101` to `7E4`. Raw adapter output:

```
7EC103E620101FFF7E7
7EC21FF490000000080
7EC2200000DD9090908
7EC23080908000007B4
7EC2402B40100007E00
7EC2500DB540000D915
7EC260000525900004F
7EC2757007B02130000
7EC2800000000000BB8
```

The first frame `10 3E` gives length 62. Reassembled, it is `62 01 01` followed by 59 payload bytes,
ten per row:

```
p00-p09: FF F7 E7 FF 49 00 00 00 00 80
p10-p19: 00 00 0D D9 09 09 08 08 09 08
p20-p29: 00 00 07 B4 02 B4 01 00 00 7E
p30-p39: 00 00 DB 54 00 00 D9 15 00 00
p40-p49: 52 59 00 00 4F 57 00 7B 02 13
p50-p58: 00 00 00 00 00 00 00 0B B8
```

| Field | Bytes | Decoded | OBDb expected |
|---|---|---|---|
| SOC BMS | p4 `49` = 73 | 36.5 % | 36.5 |
| Status J | p9 `80` | charging bit = 1, plugs 0, relay 0 | 1 / 0 / 0 / 0 |
| Current | p10–11 `0000` | 0.0 A | 0 |
| Voltage | p12–13 `0DD9` = 3545 | 354.5 V | 354.5 |
| Max / min temp | p14 `09`, p15 `09` | 9 / 9 °C | 9 / 9 |
| Modules 1–4 | p16–19 `08 08 09 08` | 8, 8, 9, 8 °C | 8, 8, 9, 8 |
| Inlet | p22 `07` | 7 °C | 7 |
| Max cell | p23 `B4`=180, p24 `02` | 3.60 V, cell 2 | 3.6 / 2 |
| Min cell | p25 `B4`, p26 `01` | 3.60 V, cell 1 | 3.6 / 1 |
| Aux V | p29 `7E` = 126 | 12.6 V | 12.6 |
| CCC | p30–33 `0000DB54` = 56148 | 5614.8 Ah | 5614.8 |
| CDC | p34–37 `0000D915` | 5557.3 Ah | 5557.3 |
| CEC | p38–41 `00005259` | 2108.1 kWh | 2108.1 |
| CED | p42–45 `00004F57` | 2031.1 kWh | 2031.1 |
| Op time | p46–49 `007B0213` = 8 061 459 | 8 061 459 s = 2239.3 h | 8061459 |
| Isolation | p57–58 `0BB8` | 3000 kΩ | 3000 |

220105, same car (46 bytes):

```
7EC102E620105003FFF / 7EC2190000000000000 / 7EC220000077E000041 / 7EC238C3E7800006407
7EC244B03E032003137 / 7EC254B0000B4B40000 / 7EC260900000000AAAA
→ 62 01 05, then 43 payload bytes:
p00-p09: 00 3F FF 90 00 00 00 00 00 00
p10-p19: 00 00 07 7E 00 00 41 8C 3E 78
p20-p29: 00 00 64 07 4B 03 E0 32 00 31
p30-p39: 37 4B 00 00 B4 B4 00 00 09 00
p40-p42: 00 00 00
```

Decoded values: p20 `00` deviation 0.00 V; p22 `64` airbag duty 100; p23 `07` heater 7 °C;
p25–26 `03E0` = 992, so **SOH 99.2 %**; p27 `32` max-det cell 50; p28–29 `0031` min-det 4.9 %;
p30 `37` min-det cell 55; p31 `4B` = 75, so **displayed SOC 37.5 %**; p34/p35 `B4`, so cells 97/98 =
3.60 V. All of these equal OBDb's expected values.

A second, independent sample (from evDash `loadTestData`, merged form) decodes to SOC BMS 89.5 %,
displayed SOC 93.5 %, 399.5 V, +1.8 A, cells 4.08 V (#56) / 4.06 V (#59), aux 14.5 V, SOH 100.0 %.
That is consistent with a near-full pack: displayed SOC sits above BMS SOC at the top end.

## Related
[[kona-electric-2020]] · [[kona-ev-battery-health]] · [[kona-ev-other-ecus]] · [[obd2-services]] ·
[[adapter-response-format]] · [[obdlink-ex]]
