---
title: Kona Electric battery health — SOH, deterioration and cell imbalance
type: analysis
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/kona-ev-community-notes-2026-11-07.md
  - raw/obdb-hyundai-kona-electric.md
  - raw/evdash-carkiaeniro-src.md
  - raw/nhtsa-recalls-hyundai-kona-electric.md
tags: [kona-electric, soh, cell-balance, battery-health]
confidence: community
---

# Kona Electric battery health

How to read the BMS values from [[kona-ev-bms-pids]] on the [[kona-electric-2020]]. None of the
thresholds below come from Hyundai. They are owner-forum consensus, and each one is marked as such.

## Pack facts

- 64 kWh nominal, **98 series groups**, each group 3 cells in parallel (1st gen). The BMS reports
  voltage per group, which the tools call "cells". (`community`, speakev / insideevsforum)
- One bad cell inside a 3-parallel group is partly masked by its two neighbours. Small voltage
  deviations therefore matter more than the same number would on a 1-cell-per-group pack.
- Cell voltage resolution is **0.02 V** (`u8 / 50`). A deviation reading of 0.02 V can be anything
  from about 0 to 0.039 V. (`verified` from the encoding itself)
- Average cell voltage ≈ pack V / 98 (JejuSoul calc PID).

## SOH (220105 p25–26, `u16/10`)

- In practice it is **capped at 100.0 %**. A new pack reads 100.0 for a long time; one owner's
  Sept-2018 pack read 100 % until it was replaced four years later. No source reports a value above
  100 %. If `carlink` ever sees > 1000 raw, treat that as a decode error. (`community`)
- Early wear is therefore hidden. **Minimum deterioration** (220105 p28–29 `/10` %, plus the cell
  numbers at p27 / p30) moves first: one owner saw it rise 0.7 → 1.3 % while SOH stayed at 100, and
  later read SOH 97.5 % with min-deterioration 5.8 %. OBDb's real MY2020 capture shows SOH 99.2 % with
  min-deterioration 4.9 %. (`community`)
- **A BMS software update can reset SOH to 100 %.** One owner reported 91 % → 100 % after an update.
  An SOH value is only comparable to readings taken under the same BMS software. Recall 196/200 and
  the 2026 BMS recall both reflash the BMS (see [[kona-ev-recalls]]). (`community`)
- A replacement pack, as fitted under Recall 200, restarts SOH, CEC/CED and CCC/CDC history. A
  cumulative-energy figure much lower than mileage would suggest (≈ 0.15 kWh/mi × miles) points to a
  replaced pack. (`unverified`, inference)
- Capacity cross-check independent of SOH: track ΔCED against ΔSOC(BMS) over a long discharge.
  usable kWh ≈ ΔCED / (ΔSOC/100). (`unverified`, method; evDash records CEC/CED at
  each 10 % SOC step for exactly this, see `soc10ced` in its source)

## SOC: BMS vs displayed

- 220101 p4 is the BMS (true) SOC; 220105 p31 is the cluster-displayed SOC. They differ: in evDash's
  near-full sample BMS reads 89.5 % against 93.5 % displayed, and in OBDb's capture 36.5 % against
  37.5 %. Report both, and do not treat the gap as a fault. (`community`)

## Cell imbalance (max − min cell voltage)

Use 220101 p23/p25 (max/min, with cell numbers at p24/p26), or compute it from all 98 cells
(220102–220105). 220105 p20 is the BMS's own deviation byte.

| Δ (max − min) | Interpretation | Basis |
|---|---|---|
| 0.00–0.02 V | Normal / balanced | forum consensus (speakev, insideevsforum) |
| 0.04 V | Worth watching. Re-measure at ≥ 80 % SOC after a 100 % AC charge | KiwiME, insideevsforum |
| ≥ 0.05 V | Concerning; forum guess for where a dash warning ought to trigger | speakev |
| ≥ 0.10 V | Pack-replacement territory. BMS limits charging | hyundaikonaforum |

Practice notes (`community`):

- Measure at rest and at high SOC (≥ 80 %). Imbalance shows most at the ends of the SOC curve, and
  current flow skews the readings.
- The fix for a small imbalance is a **100 % AC charge** with the car left plugged in. 90 % does not
  balance. Owners recommend doing this monthly.
- A *specific* cell number that is always the minimum and never catches up after balancing is the
  classic sign of a self-discharging cell. Track min-cell number over time, not just Δ.
- The 2026 global BMS recall (KBA 16242R / Hyundai 61D004) exists to "detect voltage differences
  between the individual cells sooner". In other words, Hyundai itself treats cell-voltage spread
  as the early fire-risk signal. See [[kona-ev-recalls]].

Suggested `carlink` flags. These are *proposals* derived from the table above, not sourced limits:
Δ ≤ 0.02 V OK; 0.04 V WATCH; ≥ 0.06 V WARN. Also WARN when any module temperature differs from the
others by more than about 5 °C at rest, or when isolation resistance is unusually low compared with
the 1000–3000 kΩ seen in samples. (`unverified`; no source gives an isolation threshold.)

## Temperatures and charge-rate gates (evDash comment, `community`)

The BMS DC charge-current limit follows the minimum module temperature: ≥ 35 °C → 180 A; ≥ 25 °C →
200 A (no limit); ≥ 15 °C → 120 A; ≥ 5 °C → 90 A; ≥ 1 °C → 60 A; ≤ 0 °C → 40 A. More than 43 kW needs
at least 15 °C, and more than 58 kW needs at least 25 °C. This explains "slow DC charging in winter"
without any fault.

## Related
[[kona-ev-bms-pids]] · [[kona-ev-recalls]] · [[kona-electric-2020]]
