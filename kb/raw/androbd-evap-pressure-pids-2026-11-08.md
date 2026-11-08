---
title: AndrOBD — how it decodes service 01 PIDs 32, 53 and 54 (rows from pids.csv and conversions.csv)
source_url: https://github.com/fr3ts0n/AndrOBD/tree/28d1a09e1b085ce8d07f536d814ab96f6f21e272/library/src/main/java/com/fr3ts0n/ecu/prot/obd/res
commit: 28d1a09e1b085ce8d07f536d814ab96f6f21e272
fetched: 2026-11-08
license: GPL-3.0 project; three data rows quoted for the formula-disagreement record.
note: conversions are LINEAR value = raw * FACT / DIV + OFFS. The 0xFFFF mask and 0 offset mean PID 32 is decoded as an unsigned word.
---

pids.csv (tab-separated: svc, pid, ofs, len, bit_offset, bit_length, bit_mask, formula, format, …, mnemonic, label):

```
0x01,0x02	0x32	0	2	0	16	0xFFFF	PRESS_VAPOR	%.2f	pressure_vapor_evaporative_purge	Evap System Vapor Pressure
0x01,0x02	0x53	0	2	0	16	0xFFFF	PRESS_VAPOR_ABS	%.3f	pressure_vapor_evaporative_purge_absolute	Absolute Evap system Vapor Pressure
0x01,0x02	0x54	0	2	0	16	0xFFFF	PRESS_VAPOR_REL	%.0f	pressure_vapor_evaporative_purge_rel	Evap System Vapor Pressure
```

conversions.csv (CONVERSION_ID, TYPE, VARIANT, SYSTEM, FACT, DIV, OFFS, PhOf, UNIT):

```
PRESS_VAPOR	LINEAR	0	IMPERIAL	100	99635	0	0	in H2O
PRESS_VAPOR	LINEAR	0	METRIC	1	4	0	0	Pa
PRESS_VAPOR_ABS	LINEAR	0	METRIC	1	200	0	0	kPa
PRESS_VAPOR_REL	LINEAR	0	METRIC	1	1	-32767	0	Pa
```
