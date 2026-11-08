---
title: Honda maintenance reminder reset (CR-V 2004 MAINT REQ'D, Odyssey 2016 Maintenance Minder)
type: procedure
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/charm-crv-2004/maintenance.txt
  - url:https://www.manualslib.com/manual/979848/Honda-2016-Odyssey.html?page=459
  - url:https://www.manualslib.com/manual/979848/Honda-2016-Odyssey.html?page=458
  - url:https://itstillruns.com/dashboard-indicator-light-honda-crv-2281417.html
  - url:https://www.mechanicwiki.com/2017/11/honda-odyssey-1999-2017-engine-oil-life.html
tags: [procedure, honda, maintenance, reset, crv-2004, odyssey-2016]
confidence: verified
---

# Honda maintenance reminder reset

**Generic OBD adapter: cannot do this on either car.** Both reminders live in the instrument
cluster / gauge control module, not in the emissions PCM. A Service 04 clear ([[clearing-codes]])
does not touch them. Honda's HDS can reset them; a plain ELM/STN adapter cannot. Do them from the
driver's seat with the steps below. `carlink` should print these steps and never claim to have
done the reset.

## 2004 CR-V: "MAINT REQ'D" indicator

The 2002–2006 CR-V has a simple distance-based **MAINT REQ'D** light, not the computed
Maintenance Minder. It comes on about every 10,000 mi after reset (community; the normal schedule
itself is 10,000 mi / 1 yr oil changes).

Steps from the service manual "Maintenance Indicator Reset". confidence: verified.

1. Turn the engine off.
2. Press and hold the **Select/Reset** button on the instrument panel (next to the odometer), then
   turn the ignition switch to **ON (II)**. Do not start the engine.
3. Keep holding for **at least 10 seconds**, until the indicator resets.

Gotcha (Honda Service News Feb 2003, written for 2002–03 CR-V and possibly applying to 2004): the
reset is refused while the headlights or parking lights are on, including aftermarket DRL modules
that turn them on at ignition ON. confidence: community.

## 2016 Odyssey: Maintenance Minder

The Minder computes oil life (%) from engine operating conditions and shows codes:

| Code | Meaning (owner's manual p. 457) |
|---|---|
| A | Replace engine oil |
| B | Oil + filter, inspect brakes/parking brake/steering/suspension/driveshaft boots/brake lines/fluids/exhaust/fuel lines |
| 1 | Rotate tires |
| 2 | Air cleaner element, dust & pollen filter, inspect drive belt |
| 3 | Replace transmission fluid (ATF DW-1) |
| 4 | Replace spark plugs, **replace timing belt**, inspect water pump, inspect valve clearance |
| 5 | Replace engine coolant |

The oil-life display appears with "due soon" items once oil life reaches **15 %**. Replace brake
fluid every 3 years regardless. Change oil at least once a year even if no SERVICE message appears.

### Reset (owner's manual p. 458, paraphrased). confidence: verified

1. Ignition **ON** (engine may be off). The vehicle must be stationary.
2. Press and hold the **Select/Reset knob** (on the gauge cluster; on trims with the
   steering-wheel MID it is the **SEL/RESET** / INFO control) for **10 seconds or more**. The
   information display enters the reset mode.
3. Select the item to reset (engine oil life, or the individual codes that were serviced).
4. Press and hold again for a few seconds to enter that item's reset mode, then hold **≥ 5 s** to
   confirm. The codes clear and oil life returns to **100 %**.

On trims with the i-MID / navigation screen, the same reset is also reachable through the
settings menu ("Vehicle Settings → Maintenance Info/Reset"). confidence: community.

Reset only the codes that were actually done. Resetting the whole Minder after an oil change
alone hides code 4 (timing belt), and a missed timing-belt service on the J35 is the expensive one.
