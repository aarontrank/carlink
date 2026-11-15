---
title: InsideEVs Forum (Clarity section) — battery capacity, warranty and failure threads
source_url: https://www.insideevsforum.com/community/threads/battery-warranty.8264/ ; https://www.insideevsforum.com/community/threads/battery-capacity-test-reports-ongoing-data-for-warranty.8317/ ; https://www.insideevsforum.com/community/threads/budget-battery-capacity-readout.10531/page-19 ; https://www.insideevsforum.com/community/threads/2018-honda-clarity-plug-in-hybrid-battery-failure-at-108k-miles-%E2%80%93-10-000-repair-quote-honda-customer-care-doesnt-care.20886/page-2
fetched: 2026-11-13
---

(clarityforum.com blocks automated fetches via a TollBit paywall/redirect; its content below comes only
from web-search snippets and is marked as such.)

## battery-warranty.8264 (Feb 2020) — verbatim
- insightman, Feb 12 2020: "The Clarity Plug-In Hybrid Pre-Delivery Inspection document (17-093) describes how
  bad the battery has to be to qualify for a replacement." / "A new Clarity's Battery Pack Capacity is typically
  55 Ah or slightly better."
- KentuckyKen, Feb 12 2020: "Note that California (PZEV states?) gets a 10 yr warranty while the rest of us get
  an 8 yr warranty." / "This equates to about a 1/3 loss of capacity or range."
- glonroy, Feb 12 2020: "The warranty guide states that the "gradual capacity loss of the high voltage battery is
  expected and not covered under warranty.""

## battery-capacity-test-reports.8317 (2020) — verbatim
- Cash Traylor, Feb 18 2020, 2018 Touring: "Battery Pack Capacity 53.8 Ah (~98%)" (12/2018, 3,750 mi);
  "Battery Pack Capacity 49.6 Ah (~90%)" (2/2020, 13,325 mi).
- Rangavasu, Mar 12 2020: "March 12, 2020 - 14205 miles - 2018 Touring - Battery Pack Capacity 50.9 Ah"
- Clarity Dave, Mar 14 2020: "Had the capacity checked on 10/24/2019 with 18,321 miles on the car, result was 51.7 Ah."
- Steven B, Mar 15 2020 (mfg 10/2017): "Test performed at dealer: 52.9 Ah." (~19k mi)

## budget-battery-capacity-readout.10531 (2022) — verbatim
- PDX Clarity, Aug 3 2022: "2019 model; 11,050 miles; Battery Capacity (Total) = 53.72Ah" (Car Scanner)
- coutinpe, Aug 7 2022: "52.5 Ah at 3.5 years and 28800 miles (approx.)" (Vgate BLE + Car Scanner)
- KeeFeeRe, Aug 2 2022: "capacity 47.88Ah, mileage 96640km, year of production 11/17" (Car Scanner)
- MrFixit, Aug 2 2022: Torque Pro fails because "the offset required to obtain the Battery Capacity is too large
  for the App to handle."
- Search snippet (same thread, other page): "BattCap 48.44 ... 2018 build 1/18 Purchased 11/18 96,800 miles"

## battery-failure-at-108k.20886 page 2 (2026) — verbatim
- Landshark, Mar 13 2026: "Each has a capacity of 27.5Ah When connected in parallel the capacity is doubled to 55Ah."
  / "Is it possible that the dealer has measured, or provided, the capacity of half of the battery?"
- Lonewarrior, Mar 13 2026: "I have yet to come across a warranty replacement that was approved solely under the
  36.6Ah policy" / "The P0A7F and P1DC7 combination appears to be the primary basis" [for approved replacements]
- AaD, Mar 13 2026: "battery replacement under warranty was initiated by the check engine & power system warning lights"
- AHolbro1, Mar 28 2026 (2019 Touring, ~180k mi): "my battery capacity had gradually degraded to something
  (unremembered) just north of 38 AH" / "battery capacity was reset to 55 AH and EV range on the GoM instantly jumped
  from 23" [after clearing codes] / "Battery Capacity has decayed to 50.54 AH since last code-clear." / "I encountered
  the codes in HV and Sport mode, about 30-40 minutes into a 70+mph hwy transit." / repair "did come in at $10,688.39"
- ajh (original post, as quoted by aneiy Nov 4 2026), 2018 at 108k mi: codes "P0A7F – Hybrid battery deterioration",
  "P1DC7 – Battery pack deterioration", "U0296 – Lost communication with battery charge control module",
  "P0AE1 – Pre-charge contactor circuit issue"; messages "Plug In Charging System Problem. Range Limited"; car
  "would no longer enter Eco mode, and would no longer initiate charging from the wall."; "Battery pack: ~$8,700",
  "Total repair estimate: ~$10,700"; "the vehicle being just outside the 100K battery warranty"
- aneiy, Nov 4 2026: "i live in California which is 10 year or 150k miles and I was at 111k miles" / "i paid $0 out of
  pocket for those codes"

## clarityforum.com (search snippets only — not fetched)
- "My 2018 Clarity has 17,000 miles and when I recently checked my Battery Capacity it is 53.6."
- 7-year-old 2018 readings of 43.9 Ah and 42.92 Ah (thread "2018 Honda Clarity PHEV Battery Capacity at 42.92 Ah—call
  for concern?"); a reply called it "a normal rate of degradation for a 7 year old PHEV lithium ion battery".
- One owner: 41.1 Ah at 59,000 miles on a 2018.
- HDS readout: dealer "used the Honda Diagnostic System (Ver 1.006.013) to confirm my battery capacity supported 55.0Ah".

## Summary
- New ≈ 55 Ah (total, A+B parallel; 27.5 Ah per sub-pack). Typical: ~50–54 Ah in years 1–2, ~48 Ah near 60–97k mi,
  ~42–44 Ah at ~7 years on some 2018s; the user's own ~40 Ah fits the low end of the 7–8-year spread.
- Clearing DTCs (or 12 V disconnect) can reset the stored capacity estimate to 55 Ah, after which it relearns
  downward — a reading right after a code clear is not a state-of-health measurement.
- Approved warranty pack replacements reported with P0A7F + P1DC7 (+ sometimes U0296/P0AE1), not on Ah alone.
---
title: clarity-phev/Battery-Capacity-Read (GitHub README)
source_url: https://github.com/clarity-phev/Battery-Capacity-Read
fetched: 2026-11-13
---

## Verbatim excerpts (README)
- "The nominal battery capacity for a new vehicle is 55 Ah"
- "Honda defines the eligibility for warranty replacement as being 2/3 of the nominal capacity, or 36.6 Ah."
- "For the Clarity, this warranty is 8 years, 100K miles in most states."
- "It increases to 10 years, 150K miles" (in CARB/ZEV states — sentence truncated in fetch)
- "the specifics of the HV battery warranty are based on a capacity measurement of the battery expressed in ampere-hours (Ah)."
- "Option 2 - A Proven 'good' ELM327 OBDII adapter with the Car Scanner App"
- "Honda equips dealer service departments with a Honda-specific system called "i-HDS,""

The README links PDFs (AP200_Instructions.pdf, ELM_CarScanner_Instructions.pdf, ELM_Adapter_Specifics.pdf)
that I did not download; no PID/header/offset is in the README text itself.

## Summary
Community instructions repo. Agrees with Honda S/B 17-093 on 55 Ah new / 36.6 Ah warranty line; its
"2/3 of nominal" gloss is the repo's own inference (2/3 × 55 = 36.67).
---
title: ClarityForum.com threads — web-search snippets only (site blocks automated fetch via TollBit)
source_url: https://www.clarityforum.com/threads/fuel-pump-recall.2354/ ; https://www.clarityforum.com/threads/fuel-pump-recall-woes.2606/ ; https://www.clarityforum.com/threads/plug-in-charging-system-problem-range-limited.2287/ ; https://www.clarityforum.com/threads/coolant-pump-not-working-p0c4a.1862/ ; https://www.clarityforum.com/threads/engine-maintenance-mode.1463/ ; https://www.clarityforum.com/threads/best-12v-replacement-battery-for-2018-clarity-phev.2343/
fetched: 2026-11-13
---
**Not fetched directly** — `curl`/WebFetch get a TollBit redirect/202 challenge. Text below is what web-search result summaries
quoted; treat as unverified paraphrase-level evidence.

- fuel-pump-recall.2354: after recall pump replacement, "numerous check engine codes...p0454, p0455, p0456....paid for the capless
  fuel system ($430)"; codes returned; dealer then proposed a new tank (~$2,500). Another owner: "Tighten Fuel Cap" then
  "Emissions System Problem" two days after a tank replacement.
- fuel-pump-recall-woes.2606: "Tighten Fuel Cap" followed in a day by "Emission System Problem"; dealer replaced fuel cap, then fuel
  pump again + smoke test; later "fuel tank and fuel pump module have both been replaced as well as the 2 new seals".
- plug-in-charging-system-problem-range-limited.2287: P0D27 ("Battery Charger Input Voltage Too Low") → dealer replaced battery
  charger; also P0D57 (charger cable proximity detection circuit range/performance) with charge inlet cable 1F410-5WJ-A00.
- coolant-pump-not-working-p0c4a.1862: P0C4A (coolant pump) after traction-battery replacement — possibly connector not seated;
  another owner P04CA [sic, as quoted] while charging on 240 V fixed by replacing the ES water pump.
- engine-maintenance-mode.1463: owners went ~a month without the engine starting; "Some people say a month and half or so" before it
  starts by itself.
- best-12v-replacement-battery.2343: OEM battery ~310 CCA; any Group 51R works.
