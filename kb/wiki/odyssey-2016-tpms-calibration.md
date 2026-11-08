---
title: 2016 Odyssey TPMS — warning reset and sensor registration
type: procedure
created: 2026-11-07
updated: 2026-11-07
sources:
  - raw/honda/2016-odyssey-touring-tpms-quickguide.pdf
  - url:https://www.manualslib.com/manual/979848/Honda-2016-Odyssey.html?page=429
  - url:https://mitchell1.com/shopconnection/featured-tpms-tip-2011-2019-honda-odyssey/
  - url:https://crmshonda.my.salesforce-sites.com/hondaknowledge/articles/Knowledge/What-is-Honda-s-Tire-Pressure-Monitoring-System-TPMS-w-Tire-Fill-Assist
  - url:https://www.hondainfocenter.com/2018/Odyssey/Feature-Guide/Safety-Features/Tire-Pressure-Monitoring-System/
tags: [procedure, honda, odyssey-2016, tpms]
confidence: community
---

# 2016 Odyssey TPMS: warning reset and sensor registration

**Generic OBD adapter: cannot do this.** Registering TPMS sensor IDs needs Honda HDS, or an
aftermarket TPMS tool with Honda "OBD relearn", plus a 315 MHz sensor trigger tool. The
[[obdlink-ex]] has neither. A Service 04 clear ([[clearing-codes]]) does not turn off a
low-pressure warning.

## Which system the 2016 has

- **Direct TPMS** (a pressure sensor/transmitter at each valve stem). Mitchell1's 2011–2019
  Odyssey tip describes valve-stem sensors and an HDS + ATEQ VT55 ID-memorisation procedure for the
  pre-2018 vans.
- Honda's 2016 Odyssey TPMS quick guide only says to check every tire with a gauge and inflate to
  the door-jamb label. It has **no "calibrate" step**, and the cluster does not say which tire is low.
- The menu-driven **"TPMS Calibration"** and **Tire Fill Assist** belong to the **2018+**
  Odyssey. Honda's knowledge base lists Tire Fill Assist on 2018–2020 Odyssey, not 2016, and the
  2018 Feature Guide introduces per-tire display as new. Web pages telling 2016 owners to "Settings
  → TPMS Calibration" are mixing up generations; one owner comment on such a page reports the menu
  item does not exist on their 2016.

confidence: community. The owner's manual TPMS section (p. 428) is the authoritative check. If the
van's menu does show a TPMS Calibration item, record that here; it would mean an indirect system.

## Low-pressure light on (no sensor work done)

1. With the tires cold, set all four (and check the spare) to the door-jamb pressure.
2. Drive. The light should go out by itself once the sensors report normal pressure. Mitchell1
   quotes a verification drive at **≥ 28 mph for ≥ 1 minute**.
3. If the light **blinks for about a minute and then stays on**, that is a **system fault**
   (dead sensor battery, unregistered sensor), not low pressure. It needs a TPMS tool.

## After new sensors or wheels (dealer or tire shop with a TPMS tool)

Summarised from the Mitchell1 HDS procedure (2011–2017):

1. Ignition off. Connect HDS to the DLC, then ignition ON, and confirm it talks to the TPMS unit.
2. Choose ID memorisation in HDS. Hold the trigger tool (ATEQ VT55) at each wheel in the order the
   HDS prompts until all four IDs are stored. Do not switch the ignition off mid-way, because that
   cancels the process.
3. Drive at ≥ 28 mph for ≥ 1 minute and confirm the indicator does not blink.
4. Optional verification: drop one tire below ~28 psi and confirm the light comes on, then refill.

Tire **rotation** on the 2011–2017 van: Mitchell1 describes a separate sensor-location step with
the trigger tool. Whether the 2016 shows per-tire positions at all is unconfirmed (the quick guide
suggests it does not), so a rotation probably needs no action. confidence: unverified.

## 2004 CR-V

It has no TPMS. See [[crv-2004]].
