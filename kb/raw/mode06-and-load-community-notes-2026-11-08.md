---
title: Service 06 / misfire / engine-load notes gathered by web search and page extraction (pages that could not be saved verbatim)
source_url: multiple (listed per item)
fetched: 2026-11-08
note: >
  These are search-result summaries or WebFetch extractions, not verbatim page captures. OdyClub
  threads redirect bots to a paid gateway (HTTP 402), so only search-engine summaries were available.
  Treat every item as `community` / `unverified` unless it says otherwise.
---

## OdyClub (Honda Odyssey forum) — service 06 misfire readouts

- "Sudden Misfires Cylinders 4-5-6" (https://www.odyclub.com/threads/sudden-misfires-cylinders-4-5-6.367601/,
  ~early 2022; model year not shown in the summary; 195k+ miles on the original timing belt, so probably a
  timing-belt J35 of an earlier generation). Owner's codes: P0300, P0304, P0305, P0306; limp mode on bank 2.
  Posted mode 6 data: "A5 0B 59 counts, A5 0C 96 counts, A6 0B 02 counts, A6 0C 0 counts, A7 0B 31 counts,
  A7 0C 0 counts. A2 through A4 zero counts." Replies: timing would hit every cylinder, so look for a bank-2
  cause; one cylinder with low compression can make the whole bank misfire because trims are per bank;
  one reply says the PCM shuts the misfiring cylinders off until the next restart. Resolution not shown.
- "Chasing cause of intermittent misfires" (https://www.odyclub.com/threads/chasing-cause-of-intermittent-misfires.368976/),
  2015 Odyssey, ~135k km: mode 6 showed "a moving misfire, occurring with low frequency on all cylinders"
  (a handful of counts on cylinders 1–6 over a few drives), never set a code. Replies split between VCM
  activation and PCM/fuel causes.
- "2014 Odyssey Misfires right after 105K service" (https://www.odyclub.com/threads/2014-odyssey-misfires-right-after-105k-service.363911/):
  failed seal on piston 1 let oil in and fouled the plug; a reply's rule of thumb: misfires on cylinders
  1–4 are usually VCM-related, misfires on all cylinders suggest PCM, crank/cam sensor or grounds.

## Torque Pro readouts posted on other forums (format reference)

- MyG37 "Misfire reading with torque pro app" (https://www.myg37.com/forums/engine-drivetrain-and-forced-induction/294576-misfire-reading-with-torque-pro-app-2.html):
  sample "MID:$21 TID:$80 - Catalyst Monitor Bank 1 Max: 255 Min: 10 Test result value: 25 PASS" and
  "MID:$21 TID:$82 - Catalyst Monitor Bank 1 Test incomplete or dependant test failed"; misfire MID $A2
  TID $0B (EWMA of last 10 drive cycles) and $0C (last/current cycle).
- Tacoma World "Strange Torque App information regarding O2 sensor and misfires?"
  (https://www.tacomaworld.com/threads/strange-torque-app-information-regarding-o2-sensor-and-misfires.373529/):
  "MID:$a1 TID:$0b EWMA Misfire counts (average) for last 10 drive cycles - Misfire Monitor General Data
  Max: 65,535Count Min: 0Count Test result value: 0Count PASS".

## Manufacturer documents seen only through search summaries

- Toyota 2GR-FE repair manual, SFI system (https://share.qclt.com/%E4%B8%B0%E7%94%B0Toyota%20RAV4%20Repair%20Manual/2GR-FE%20Engine%20Control%20System/SFI%20System/006004.pdf):
  MID $A1 = all cylinders, $A2 = cylinder 1, …; TID $0B = EWMA, "Total misfire counts for last driving
  cycle × 0.1 + Last …" (rest cut off); MID $A1 TID $0C: engine running → total misfire counts for the
  current driving cycle; ignition ON engine off → the last driving cycle's total. Scaling "multiply by 1".
- GM "mode $06 data definitions for GM vehicles using GMLAN" (https://gsitlc.ext.gm.com/gmspo/mode6/pdf/GM%20CAN%20mode%20$06%20data%20final_dm.pdf,
  now behind a GM sign-in; Wayback blocked automated fetches): rows such as "A2 0C 24 Misfire counts for
  the last / current driving cycles 0 to 65535 counts 1 count / bit", "21 A0 05 Catalyst Test Bank 1
  (using OSC normalized ratio units) 0 to 1.999 unitless", a UASID 0A "Maximum Sensor Voltage Achieved
  0.0000 to 7.9900 V at 0.122 mv / bit"; scaling exceptions, e.g. "OBDMID $01 TID $88 test limits and value
  should be divided by 1000"; for some OBDMID/TID pairs "the test value could equal the test limit when the
  test has failed".
- SGMW (Baojun) OBD system declaration (https://s7abe6bda7f063e33.jimcontent.com/download/version/1694851052/module/13375256349/name/Baojun%20Trouble%20Shooting%20&%20Error%20Codes%20DTC.pdf):
  TIDs $0B and $0C listed with min 0 counts / max 65535 counts; "$80–$FE" = manufacturer-defined TID range.
- Ford OBD Operations Summary PDFs (fordservicecontent.com; Akamai blocks automated download): MIDs A2–AD
  with $0B "EWMA misfire counts for last 10 driving cycles" and $0C "Misfire counts for last/current driving
  cycle" (events); MID A1 listed with Ford TIDs $80–$82 (total misfire rate vs catalyst-damage / emission
  thresholds).

## J1979 rules quoted in search excerpts of the standard (copies of SAE J1979 found on forums were not opened)

- Service $06 on CAN: if an unsupported OBDMID is requested the ECU does not respond; if a supported OBDMID
  has no stored data the ECU must still answer positively with test value, min and max set to $00.
- PID $13 shall only be supported if PID $1D is not supported, and vice versa.

## J1979 / ISO 15031-5 definitions of the two engine-load PIDs

From Todd Erickson, "What is 'Calculated Load'?" (https://www.linkedin.com/pulse/what-calculated-load-todd-erickson),
which cites ISO 15031-5 (WebFetch extraction, quotes as returned):

- PID $04: "LOAD_PCT = [current airflow] / [(peak airflow at WOT@STP as a function of rpm) * (BARO/29.92) *
  SQRT(298/(AAT+273))]"; STP = "25 °C, 29.92 in Hg"; "Reaches 100% at WOT at any altitude, temperature or
  rpm for both naturally aspirated and boosted engines"; "Indicates the percent of peak available torque";
  diesels use fuel flow in place of airflow.
- PID $43: "LOAD_ABS = [air mass (g / intake stroke)] / [1.184 (g / litre) * cylinder displacement
  (litres / intake stroke)]"; "The constant is 1.184 (g/liter) * cylinder displacement (liters/intake
  stroke) based on air density at STP"; "Ranges from 0% to approximately 95% for naturally aspirated
  engines, 0% – 400% for boosted engines"; linearly correlated with indicated and brake torque; peak value
  correlates with volumetric efficiency at WOT.
- Search summaries of the FORScan forum thread https://forscan.org/forum/viewtopic.php?t=5289 (HTTP 403 to
  fetchers) quote the same definitions, adding "At engine off and ignition on the LOAD_ABS = 0 %" and that
  LOAD_PCT is "linearly correlated with engine vacuum".

## ScannerDanner forum — 2004 Honda CR-V A/F sensor on a scan tool

- "2004 Honda CR-V - P0134" (https://scannerdanner.com/forum/post-your-repair-questions-here/3325-2004-honda-cr-v-p0134.html):
  in closed loop the tool showed "AF Sensor (mA) = -127.5", "AF FB AVE (LT Fuel) = 1.00",
  "AF FB CMD (A/F) = 14.56", "AF LAMBDA = 2.00", identical key-on engine-off. A reply: -127 mA is a full-lean
  reading; check whether it persists with the sensor unplugged and test the signal wires for shorts to
  ground. (Whether those items were generic PIDs or Honda-enhanced data is not stated.)

## 2002–2006 CR-V (K24A1) air metering — MAP only

- CR-V Owners Club, "Is there a MAF Sensor on a 2003 Honda CR-V?"
  (https://www.crvownersclub.com/threads/is-there-a-maf-sensor-on-a-2003-honda-cr-v.189337/): "There is ONLY
  MAP for 2003 Honda CRV, no MAF"; another member says MAF sensors by the air box only arrived with the
  third generation (unchecked). Go-Parts MAP guide (https://www.go-parts.com/garage/ps-1997-2006-honda-cr-v-map-sensor):
  1997–2006 CR-V uses a MAP sensor and no MAF. Forum MAP voltages: KOEO at sea level ≈ 2.86 V; warm idle
  ≤ 1.00 V (0.98 typical). Search summaries only.

## 2011–2017 Odyssey (J35Z8) air metering — has a MAF sensor

- Parts listings (search summaries): eBay used OEM sensor "37980RADL11 … ODYSSEY 07-17 (3.5L)"
  (https://www.ebay.com/itm/127551174545); Amazon aftermarket kit for "Honda Odyssey 2011-2017 3.5L V6"
  cross-referencing 37980-RNA-A01, 37980-RAD-L11, 37980-RV0-A01
  (https://www.amazon.com/PUAMRPTS-Sensor-Compatible-Odyssey-2011-2017/dp/B0HC9Q692G). Go-Parts P1101
  page (https://www.go-parts.com/garage/obd-p1101-honda-various-2005-2018): expected warm-idle MAF voltage
  0.4–2.2 V on these Hondas. Whether the PCM also uses its MAP sensor for load is not stated.

## 2014–2017 Odyssey upstream sensors are wideband A/F sensors

- PartsGeek (https://www.partsgeek.com/mmparts/air_fuel_ratio_sensor/honda/odyssey.html), search summary:
  NGK air/fuel sensors for the 2014–2017 Odyssey 3.5L V6, "5-Wire A/F Sensor, Sensor Type: Wide-Band", in
  two positions, "Upstream Front" and "Upstream Rear". An eBay Denso-branded listing
  (https://www.ebay.com/itm/164761023288) fits every 2016 trim with the 3.5L SOHC V6 as "Upstream".
