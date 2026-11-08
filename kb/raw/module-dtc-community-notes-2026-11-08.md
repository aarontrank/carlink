---
title: Module-DTC research — community / forum / search-result notes (Honda 29-bit IDs, Honda DLC pins, Hyundai SRS, prior-art apps)
source_url: multiple (listed per item)
fetched: 2026-11-08
---

These are **search-result summaries and short quotes**, not verbatim page captures (several forum
pages sit behind bot walls). Each item names its page. Treat as `community` / `unverified` unless an
item says otherwise.

## Honda: generic scan tools list two 29-bit "modules", $18DAF111 and $18DAF11D

- OdyClub, "Trying to find out if my Ody had software update for TC - 2 modules show on my scanner.
  Which to review?" (https://www.odyclub.com/threads/trying-to-find-out-if-my-ody-had-software-update-for-tc-2-modules-show-on-my-scanner-which-to-review.362587/):
  a 2008 Odyssey EX-L RES owner checking TSB 09-053 (PCM update for lock-up clutch) wrote that the
  scan tool "shows 2 modules … $18DAF111 OR $18DAF11D". The excerpt does not say which is which.
- OdyClub, "How long until P0118 will reset itself?"
  (https://www.odyclub.com/threads/how-long-until-p0118-will-reset-itself.372388/): a scan showed
  "P0118 is found on Module $18DAF111 - Engine Coolant Temperature Sensor 1 Circuit High".
- JustAnswer, "Honda Pilot 2012 Engine Light Codes 18DAF11D and 18DAF111"
  (https://www.justanswer.com/honda/demi4-codes-18daf11d-18daf111-exact.html) and "Honda Pilot Engine
  Codes 18DAF11D and 18DAF111 Explained"
  (https://www.justanswer.com/honda/fqi1g-just-engine-codes-morning-2013-honda.html): for a 2013 Pilot
  the expert says these "are the module identification numbers", not trouble codes; also reported on a
  2010 Accord with a ScanGauge II and a Foxwell NT201. (A third JustAnswer page about a 2011 Accord
  treats them as fault codes; that reading is wrong.)
- Unofficial Honda FIT Forums (https://www.fitfreak.net/forums/fit-freak-newbie-faqs/81724-forum-newbie-here-hello-everyone.html):
  an ELM327/STN1100 log on a Honda Fit shows `AT SP 7`, `AT DP` → "ISO 15765-4 (CAN 29/500)", and the
  `0100` answer `18 DA F1 10 06 41 00 BE 3F A8 13 55` (about 12 years old).

## Honda DLC pins on pre-CAN cars

- OdyClub, "2002-2004 Odyssey ODB-II pinout"
  (https://www.odyclub.com/threads/2002-2004-odyssey-odb-ii-pinout-can-anyone-confirm-which-wires-you-have-connected.363846/),
  quoting the shop manual: "4 - Ground for PGM tester 5 - Ground for communication network 7 - K-line for
  off-board communication 9 - Circuit F28 [Service check signal] 14 - Circuit F29 [ABS/TCS and SRS control
  modules] 16 - Power for PGM tester B+". The same thread says the Odyssey did not get CAN until MY2007.
- Honda-Tech, "Honda HDS and cheap MVCI cables"
  (https://honda-tech.com/forums/engine-management-tuning-124/honda-hds-cheap-mvci-cables-3312290/): one
  user says shorting MVCI cable pins 7 (K-line) and 14 (CAN-L) lets HDS reach ABS/IMMO/SRS. Single
  unverified claim; vehicle not stated. Not something carlink can or should do.
- Typeaccord (7th-gen Accord) forum threads (https://typeaccord.co.uk/board/threads/diagnostic-scanners-for-7th-gen.23506/):
  "Only the engine ECU is fully OBD2 compliant… The MICU acts as a proprietary gateway to the other ECUs";
  HDS reads them over the K-line. Different car, same Honda era.

## Hyundai / Kia SRS (airbag) module

- diag.net, "Hyundai variant coding" (https://diag.net/msg/m3tay3xv76ortcppym39tq2py8): a technician
  writes that Hyundai and Kia SRS controllers are "a one time write on the variant coding" and that crash
  data "is stored in both MCU and eeprom".
- Airbag-module reset vendors (e.g. https://www.safetyrestore.com/airbag-module-reset-service/60-hyundai-srs-airbag-module-reset.html,
  https://www.myairbags.com/product/hyundai-airbag-control-module-reset-95910-k5200/) sell bench "crash
  data" resets for Hyundai SRS units; their existence implies crash records are not removed by a scan-tool
  DTC clear. Commercial pages, not Hyundai documents.
- No source found that names the Hyundai/Kia SRS diagnostic CAN ID. "7D2 → 7DA = airbag" is repeated in
  community lists and search-engine summaries without a citable origin.

## Hyundai / Kia E-GMP (2nd-gen platforms, for comparison only)

- EvmetricsOBD ECU database (https://www.evmetrics.co.uk/obd/ecus.html): for Hyundai/Kia E-GMP lists
  7A0 BCM (TPMS, locks, lights; "Confirmed"), 7C6 instrument cluster ("Confirmed"), 770 central gateway
  ("Mapped"), 7E4 battery management and 7E2 VMCU ("Research"); replies at request + 8.

## Prior art: phone apps that read module DTCs over an ELM327

- Yorudan210/obd-diag (https://github.com/Yorudan210/obd-diag, README in French): scans the 11-bit bus
  for responding ECUs, "reads each module's faults with UDS (19 02 FF) and automatically falls back to
  KWP2000 (18)", identifies modules with 22 F1xx, clears per module "with UDS 14 and falls back to KWP"
  after confirmation. README only; the fallback trigger is not described.
- Car Scanner ELM OBD2 (https://www.carscanner.info/) ships a "Hyundai Kona EV/Niro EV/Soul EV"
  connection profile; its traffic on a real Kona is in raw/projectgus-kona-ev-can-logs-diag.md.
