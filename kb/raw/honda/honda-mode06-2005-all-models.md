---
title: "Honda — SAE J1979 Mode $06 Test Information: All 2005 Hondas (TID/CID tables and conversions)"
source_url: https://www.aa1car.com/library/honda_mode06.pdf
linked_from: https://www.aa1car.com/library/2005/us010516.htm ("Honda Mode 06 info for MY 2005 (PDF file)")
fetched: 2026-11-08
publisher: American Honda (document as hosted by AA1Car)
pdf_copy: raw/honda/honda-mode06-2005-all-models.pdf
note: >
  Pre-CAN (non-CAN) service $06 definitions for every 2005 Honda. Text extracted from the PDF with
  pypdf; the "X" marks in the model columns lose their alignment in plain text, so the section
  "Column mapping" below re-derives which model each X belongs to from per-character x positions
  (pdfminer.six). That mapping is mechanical, not interpretation. Group 2 holds the CR-V and Odyssey
  (2005 model year: the CR-V is the same K24A1 generation as Aaron's 2004; the 2005 Odyssey is the
  previous, pre-CAN generation, not the 2016 van).
---

## Column mapping (derived from character positions)

X marks sit at fixed x positions per column. Group 2 columns, left to right: Civic Hybrid (except PZEV),
Civic Hybrid (PZEV), S2000, Pilot, CR-V, Odyssey. Rows below are TID / Test-Limit-Type-and-Component-ID
pairs with the models marked X.

```
p4 y=599: label="" TID="$01" CID="$80" models=['Civic Hybrid (exc. PZEV)', 'Civic Hybrid PZEV']
p4 y=581: label="" TID="$03" CID="$81" models=['Pilot', 'Odyssey']
p4 y=563: label="" TID="$49" CID="$00" models=['Pilot', 'Odyssey']
p4 y=545: label="" TID="$4A/$4B" CID="$80/$00" models=['Pilot', 'Odyssey']
p4 y=509: label="Catalyst (Bank 1)" TID="$05/$06" CID="$81/$81" models=['Civic Hybrid (exc. PZEV)', 'Civic Hybrid PZEV', 'S2000']
p4 y=491: label="" TID="$62" CID="$04" models=['Pilot', 'CR-V', 'Odyssey']
p4 y=473: label="Catalyst (Bank 2)" TID="$64" CID="$04" models=['Pilot', 'Odyssey']
p4 y=455: label="" TID="$76" CID="$80" models=['Pilot', 'CR-V', 'Odyssey']
p4 y=437: label="A/F Sensor (Bank 1)" TID="$78" CID="$80" models=['CR-V']
p4 y=419: label="" TID="$79" CID="$01" models=['Pilot', 'Odyssey']
p4 y=401: label="" TID="$77" CID="$80" models=['Pilot', 'Odyssey']
p4 y=383: label="" TID="$7A" CID="$01" models=['Pilot', 'Odyssey']
p4 y=359: label="A/F Sensor Heater (Bank 1)" TID="" CID="" models=['CR-V']
p4 y=300: label="" TID="$10/$11" CID="$87/$07" models=['S2000']
p4 y=228: label="SHO2S Heater (Bank 1)" TID="$18/$19" CID="$87/$07" models=['Civic Hybrid (exc. PZEV)']
p4 y=210: label="" TID="$18/$19" CID="$88/$08" models=['S2000']
p4 y=192: label="" TID="$18/$19" CID="$89/$09" models=['Civic Hybrid PZEV', 'CR-V']
p4 y=174: label="THO2S Heater" TID="$14/$15" CID="$87/$07" models=['Civic Hybrid (exc. PZEV)']
p5 y=599: label="" TID="$48" CID="$00" models=['Civic Hybrid (exc. PZEV)', 'Civic Hybrid PZEV', 'S2000']
p5 y=581: label="" TID="$48" CID="$01" models=['CR-V']
p5 y=528: label="Thermostat" TID="" CID="" models=['CR-V']
p5 y=499: label="" TID="" CID="" models=['Civic Hybrid (exc. PZEV)', 'Civic Hybrid PZEV', 'S2000']
p5 y=470: label="Secondary Air System" TID="" CID="" models=['S2000']
p5 y=447: label="Lean NOx CAT" TID="$6A" CID="$01" models=['Civic Hybrid (exc. PZEV)']
p5 y=289: label="" TID="" CID="" models=['Civic Hybrid (exc. PZEV)', 'S2000']
p5 y=231: label="" TID="" CID="" models=['Civic Hybrid (exc. PZEV)', 'S2000']
p5 y=202: label="" TID="" CID="" models=['Civic Hybrid (exc. PZEV)', 'S2000']
p5 y=179: label="" TID="$3E" CID="$83" models=['Civic Hybrid (exc. PZEV)', 'S2000']
p5 y=161: label="" TID="$81" CID="$80" models=['Civic Hybrid PZEV', 'Pilot', 'CR-V', 'Odyssey']
p5 y=143: label="" TID="$82/$8F" CID="$80/$80" models=['Civic Hybrid PZEV', 'Pilot', 'CR-V', 'Odyssey']
p5 y=125: label="" TID="$88" CID="$00" models=['Civic Hybrid PZEV', 'CR-V']
p5 y=107: label="" TID="$8B/$8D" CID="$00/$00" models=['Civic Hybrid PZEV', 'CR-V']
p6 y=593: label="" TID="" CID="" models=['Civic Hybrid PZEV', 'CR-V']
p6 y=564: label="" TID="" CID="" models=['Civic Hybrid PZEV', 'CR-V']
p6 y=530: label="" TID="$9C/$9D/" CID="$00/$00/" models=['Pilot', 'Odyssey']
p6 y=501: label="" TID="$B0" CID="$00" models=['Pilot', 'Odyssey']
p6 y=483: label="" TID="$B2" CID="$00" models=['Pilot', 'Odyssey']
p6 y=465: label="" TID="$B5" CID="$00" models=['Pilot', 'Odyssey']
```

(Rows whose X sits between two text rows, e.g. the Thermostat X at y=528, belong to the nearest
TID/CID pair: $4D/$4E/$4F/$50 with $81/$81/$81/$01.)

## Full text (pypdf extraction)


===== page 1 =====
Mode $06 Test ID by Model: Group 1
Monitor Test ID
Test Limit 
Type and 
Comp. ID
Accord 
L4
Accord 
V6
Civic
(D17A1 
Engine)
Civic
(D17A2 
Engine)
Civic
(D17A6 
Engine)
Civic
Hatch-
back
EGR Flow
$01 $80 X X X
$03 $81 X X
$49 $00 X X
$4A/$4B $80/$00 X X
Catalyst (Bank 1)
$04 $02 X
$05/$06 $81/$81 X X X
$62 $04 X X
Catalyst (Bank 2) $64 $04 X
A/F Sensor (Bank 1)
$76 $80 X
$78 $80 X X X X
$79 $01 X
A/F Sensor (Bank 2)
$77 $80
$7A $01 X
A/F Sensor Heater (Bank 1) $70/$71/ $72 $80/$00/ $00
PHO2S Heater (Bank 1)
$10/$11 $85/$05
$10/$11 $86/$06
$10/$11 $87/$07
$10/$11 $08
SHO2S Heater (Bank 1)
$18/$19 $85/$05 X
$18/$19 $86/$06 X X
$18/$19 $87/$07 X
$18/$19 $88/$08
$18/$19 $89/$09
THO2S Heater $14/$15 $87/$07 X
PHO2S Heater (Bank 2)
$12/$13 $85/$05
$12/$13 $08
SHO2S Heater (Bank 2)
$1A/$1B $86/$06
$1A/$1B $89/$09
SAE J1979 Mode $06 Test Information: All 2005 Hondas
===== page 2 =====
Mode $06 Test ID by Model: Group 1 (contd)
Monitor Test ID
Test Limit 
Type and 
Comp. ID
Accord 
L4
Accord 
V6
Civic
(D17A1 
Engine)
Civic
(D17A2 
Engine)
Civic
(D17A6 
Engine)
Civic
Hatch-
back
PCV
$48 $00 X X X X
$48 $01
Thermostat
$4D/$4E/
$4F/$50
$80/$80/
$80/$00
$4D/$4E/
$4F/$50
$81/$81/
$81/$01 X
$5C/$5D/
$5E/$5F
$00/$00/
$00/$00 XXXX
Secondary Air System $58/$59/ $5A $80/$00/ $80
Lean NOx CAT $6A $01 X
EVAP System
$21/$26/
$27/$38
$81/$81/
$81/$01
$29/$2D/
$2E/$2F
$81/$01/
$81/$81
$21/$26/
$27/$38
$82/$82/
$82/$02
$29/$2D/
$2E/$2F
$82/$02/
$82/$82
$3E $82
$21/$26/
$27/$38
$83/$83/
$83/$03 XXXX
$3A/$3B/
$3C
$82/$82/
$02
$3A/$3B/
$3C
$83/$83/
$03 XXXX
$29/$2D/
$2E/$2F
$83/$03/
$83/$83 XXXX
$3E $83 X X X X
$81 $80 X X
$82/$8F $80/$80 X X
$88 $00
$8B/$8D $00/$00
===== page 3 =====
Mode $06 Test ID by Model: Group 1 (contd)
Monitor Test ID
Test Limit 
Type and 
Comp. ID
Accord 
L4
Accord 
V6
Civic
(D17A1 
Engine)
Civic
(D17A2 
Engine)
Civic
(D17A6 
Engine)
Civic
Hatch-
back
EVAP System
$90/$91/
$92/$93
$00/$80/
$80/$00
$94/$95/
$96/$97
$00/$80/
$00/$00
$9A/$9B/
$9C/$9D/
$9E
$00/$00/
$00/$00/
$00
XX
$B0 $00 X
$B2 $00 X
$B5 $00 X
$B1 $00 X
$B3 $00 X
$B6 $00 X
$B4 $00 X
$B7 $00 X
===== page 4 =====
Mode $06 Test ID by Model: Group 2
Monitor Test ID
Test Limit 
Type and 
Comp. ID
Civic 
Hybrid
(Except 
PZEV)
Civic 
Hybrid
(PZEV)
S2000 Pilot CR-V Odyssey
EGR Flow
$01 $80 X X
$03 $81 X X
$49 $00 X X
$4A/$4B $80/$00 X X
Catalyst (Bank 1)
$04 $02
$05/$06 $81/$81 X X X
$62 $04 X X X
Catalyst (Bank 2) $64 $04 X X
A/F Sensor (Bank 1)
$76 $80 X X X
$78 $80 X
$79 $01 X X
A/F Sensor (Bank 2)
$77 $80 X X
$7A $01 X X
A/F Sensor Heater (Bank 1) $70/$71/ $72 $80/$00/ $00 X
PHO2S Heater (Bank 1)
$10/$11 $85/$05
$10/$11 $86/$06
$10/$11 $87/$07 X
$10/$11 $08
SHO2S Heater (Bank 1)
$18/$19 $85/$05
$18/$19 $86/$06
$18/$19 $87/$07 X
$18/$19 $88/$08 X
$18/$19 $89/$09 X X
THO2S Heater $14/$15 $87/$07 X
PHO2S Heater (Bank 2)
$12/$13 $85/$05
$12/$13 $08
SHO2S Heater (Bank 2)
$1A/$1B $86/$06
$1A/$1B $89/$09
===== page 5 =====
Mode $06 Test ID by Model: Group 2 (contd)
Monitor Test ID
Test Limit 
Type and 
Comp. ID
Civic 
Hybrid
(Except 
PZEV)
Civic 
Hybrid
(PZEV)
S2000 Pilot CR-V Odyssey
PCV
$48 $00 X X X
$48 $01 X
Thermostat
$4D/$4E/
$4F/$50
$80/$80/
$80/$00
$4D/$4E/
$4F/$50
$81/$81/
$81/$01 X
$5C/$5D/
$5E/$5F
$00/$00/
$00/$00 XXX
Secondary Air System $58/$59/ $5A $80/$00/ $80 X
Lean NOx CAT $6A $01 X
EVAP System
$21/$26/
$27/$38
$81/$81/
$81/$01
$29/$2D/
$2E/$2F
$81/$01/
$81/$81
$21/$26/
$27/$38
$82/$82/
$82/$02
$29/$2D/
$2E/$2F
$82/$02/
$82/$82
$3E $82
$21/$26/
$27/$38
$83/$83/
$83/$03 XX
$3A/$3B/
$3C
$82/$82/
$02
$3A/$3B/
$3C
$83/$83/
$03 XX
$29/$2D/
$2E/$2F
$83/$03/
$83/$83 XX
$3E $83 X X
$81 $80 X X X X
$82/$8F $80/$80 X X X X
$88 $00 X X
$8B/$8D $00/$00 X X
===== page 6 =====
Mode $06 Test ID by Model: Group 2 (contd)
Monitor Test ID
Test Limit 
Type and 
Comp. ID
Civic 
Hybrid
(Except 
PZEV)
Civic 
Hybrid
(PZEV)
S2000 Pilot CR-V Odyssey
EVAP System
$90/$91/
$92/$93
$00/$80/
$80/$00 XX
$94/$95/
$96/$97
$00/$80/
$00/$00 XX
$9A/$9B/
$9C/$9D/
$9E
$00/$00/
$00/$00/
$00
XX
$B0 $00 X X
$B2 $00 X X
$B5 $00 X X
$B1 $00
$B3 $00
$B6 $00
$B4 $00
$B7 $00
===== page 7 =====
Mode $06 Test ID by Model: Group 3
Monitor Test ID
Test Limit 
Type and 
Comp. ID
Civic
GX
Accord 
Hybrid Element Insight (M/T) Insight (CVT)
EGR Flow
$01 $80 X X
$03 $81 X
$49 $00 X
$4A/$4B $80/$00 X
Catalyst (Bank 1)
$04 $02 X
$05/$06 $81/$81 X X
$62 $04 X X
Catalyst (Bank 2) $64 $04 X
A/F Sensor (Bank 1)
$76 $80 X
$78 $80 X
$79 $01 X
A/F Sensor (Bank 2)
$77 $80 X
$7A $01 X
A/F Sensor Heater (Bank 1) $70/$71/ $72 $80/$00/ $00 X
PHO2S Heater (Bank 1)
$10/$11 $85/$05 X
$10/$11 $86/$06
$10/$11 $87/$07
$10/$11 $08
SHO2S Heater (Bank 1)
$18/$19 $85/$05 X
$18/$19 $86/$06 X
$18/$19 $87/$07 X
$18/$19 $88/$08
$18/$19 $89/$09 X
THO2S Heater $14/$15 $87/$07 X
PHO2S Heater (Bank 2)
$12/$13 $85/$05
$12/$13 $08
SHO2S Heater (Bank 2)
$1A/$1B $86/$06
$1A/$1B $89/$09
===== page 8 =====
Mode $06 Test ID by Model: Group 3 (contd)
Monitor Test ID
Test Limit 
Type and 
Comp. ID
Civic
GX
Accord 
Hybrid Element Insight (M/T) Insight (CVT)
PCV
$48 $00 X
$48 $01 X
Thermostat
$4D/$4E/
$4F/$50
$80/$80/
$80/$00
$4D/$4E/
$4F/$50
$81/$81/
$81/$01 X
$5C/$5D/
$5E/$5F
$00/$00/
$00/$00 XX X
Secondary Air System $58/$59/ $5A $80/$00/ $80
Lean NOx CAT $6A $01 X
EVAP System
$21/$26/
$27/$38
$81/$81/
$81/$01
$29/$2D/
$2E/$2F
$81/$01/
$81/$81
$21/$26/
$27/$38
$82/$82/
$82/$02
$29/$2D/
$2E/$2F
$82/$02/
$82/$82
$3E $82
$21/$26/
$27/$38
$83/$83/
$83/$03 XX
$3A/$3B/
$3C
$82/$82/
$02
$3A/$3B/
$3C
$83/$83/
$03 XX
$29/$2D/
$2E/$2F
$83/$03/
$83/$83 XX
$3E $83 X X
$81 $80 X X
$82/$8F $80/$80 X X
$88 $00 X
$8B/$8D $00/$00 X
===== page 9 =====
Mode $06 Test ID by Model: Group 3 (contd)
Monitor Test ID
Test Limit 
Type and 
Comp. ID
Civic
GX
Accord 
Hybrid Element Insight (M/T) Insight (CVT)
EVAP System
$90/$91/
$92/$93
$00/$80/
$80/$00 X
$94/$95/
$96/$97
$00/$80/
$00/$00 X
$9A/$9B/
$9C/$9D/
$9E
$00/$00/
$00/$00/
$00
X
$B0 $00 X
$B2 $00 X
$B5 $00 X
$B1 $00
$B3 $00
$B6 $00
$B4 $00
$B7 $00
===== page 10 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $01
Test Limit Type 
and Component 
ID
$80 DTC P0401
Test Description Monitoring the fluctuation of MAP sensor output when EGR valve is operated from full close to full open during fuel cut condition in deceleration.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 1.34 (mmHg)
The lowest limit value: Output value (Decimal) x 1.34 (mmHg)
The highest limit value: Nothing
Test ID $03
Test Limit Type 
and Component 
ID
$81 DTC P0401
Test Description Monitoring the fluctuation of engine load when EGR valve is operated from full close to full open during fuel cut condition in declaration.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (%)
The lowest limit value: Output value (Decimal) x 0.1 (%)
The highest limit value: Nothing
Test ID $04
Test Limit Type 
and Component 
ID
$02 DTC P0420
Test Description Monitoring the valve calculated by averaging the variance of secondary oxygen sensor signal.
Conversion to Engineering Units
Measured value: No unit
The lowest limit value: Nothing
The highest limit value: No unit
Test ID $05
Test Limit Type 
and Component 
ID
$81 DTC P0420
Test Description
Monitoring the moving average value of the OSC index. OSC index is calculated 
by the switching cycle of secondary heated oxygen sensor output during 
feedback control from secondary heated oxygen sensor output and the amount 
of fuel injected.
Conversion to Engineering Units
Measured value: No unit
The lowest limit value: No unit
The highest limit value: Nothing
===== page 11 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $06
Test Limit Type 
and Component 
ID
$81 DTC P0420
Test Description Monitoring the switching cycle of secondary heated oxygen sensor output during feedback control from secondary heated oxygen sensor output.
Conversion to Engineering Units
Measured value: Output value x 10 (msec.)
The lowest limit value: Output value x 10 (msec.)
The highest limit value: Nothing
Test ID $07
Test Limit Type 
and Component 
ID
$80 DTC P0430
Test Description Monitoring the switching cycle of secondary heated oxygen sensor output during feedback control from secondary heated oxygen sensor output.
Conversion to Engineering Units
Measured value: Output value x 10 (msec.)
The lowest limit value: Output value x 10 (msec.)
The highest limit value: Nothing
Test ID $07 Component ID $81 DTC P0420
Test Description Monitoring the switching period of secondary heated oxygen sensor output during feedback control based on secondary heated oxygen sensor output.
Conversion to Engineering Units
Measured value: Output value x 10 (msec.)
The lowest limit value: Output value x 10 (msec.)
The highest limit value: Nothing
Test ID $0A
Test Limit Type 
and Component 
ID
$81 DTC P0430
Test Description Monitoring the valve calculated by averaging the variance of secondary oxygen sensor signed.
Conversion to Engineering Units
Measured value: No unit
The lowest limit value: No unit
The highest limit value: Nothing
===== page 12 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $10
Test Limit Type 
and Component 
ID
$85 DTC P0135
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 26.7 (mA)
The lowest limit value: Output value x 26.7 (mA)
The highest limit value: Nothing
Test ID $10
Test Limit Type 
and Component 
ID
$87 DTC P0135
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 16.552 (mA)
The lowest limit value: Output value x 16.552 (mA)
The highest limit value: Nothing
Test ID $11
Test Limit Type 
and Component 
ID
$05 DTC P0135
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 26.7 (mA)
The lowest limit value: Nothing
The highest limit value: Output value x 26.7 (mA)
Test ID $11
Test Limit Type 
and Component 
ID
$07 DTC P0135
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 16.552 (mA)
The lowest limit value: Nothing
The highest limit value: Output value x 16.552 (mA)
===== page 13 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $12
Test Limit Type 
and Component 
ID
$85 DTC P0155
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 26.7 (mA)
The lowest limit value: Output value x 26.7 (mA)
The highest limit value: Nothing
Test ID $13
Test Limit Type 
and Component 
ID
$05 DTC P0155
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 26.7 (mA)
The lowest limit value: Nothing
The highest limit value: Output value x 26.7 (mA)
Test ID $14
Test Limit Type 
and Component 
ID
$87 DTC P0147
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 16.552 (mA)
The lowest limit value: Output value (Decimal) x 16.552 (mA)
The highest limit value: Nothing
Test ID $15
Test Limit Type 
and Component 
ID
$07 DTC P0147
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 16.552 (mA)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 16.552 (mA)
===== page 14 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $18
Test Limit Type 
and Component 
ID
$85 DTC P0141
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 26.7 (mA)
The lowest limit value: Output value x 26.7 (mA)
The highest limit value: Nothing
Test ID $18
Test Limit Type 
and Component 
ID
$86 DTC P0141
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 7.23 (mA)
The lowest limit value: Output value x 7.23 (mA)
The highest limit value: Nothing
Test ID $18
Test Limit Type 
and Component 
ID
$87 DTC P0141
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 16.552 (mA)
The lowest limit value: Output value x 16.552 (mA)
The highest limit value: Nothing
Test ID $18
Test Limit Type 
and Component 
ID
$88 DTC P0141
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 9.04 (A)
The lowest limit value: Output value x 9.04 (A)
The highest limit value: Nothing
===== page 15 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $18
Test Limit Type 
and Component 
ID
$89 DTC P0141
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.01 (A)
The lowest limit value: Output value (Decimal) x 0.01 (A)
The highest limit value: Nothing
Test ID $19
Test Limit Type 
and Component 
ID
$05 DTC P0141
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 26.7 (mA)
The lowest limit value: Nothing
The highest limit value: Output value x 26.7 (mA)
Test ID $19
Test Limit Type 
and Component 
ID
$06 DTC P0141
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 7.23 (mA)
The lowest limit value: Nothing
The highest limit value: Output value x 7.23 (mA)
Test ID $19
Test Limit Type 
and Component 
ID
$07 DTC P0141
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 16.552 (mA)
The lowest limit value: Nothing
The highest limit value: Output value x 16.552 (mA)
===== page 16 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $19
Test Limit Type 
and Component 
ID
$08 DTC P0141
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 9.04 (mA)
The lowest limit value: Nothing
The highest limit value: Output value x 9.04 (mA)
Test ID $19
Test Limit Type 
and Component 
ID
$09 DTC P0141
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.01 (A)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.01 (A)
Test ID $1A
Test Limit Type 
and Component 
ID
$86 DTC P0161
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 7.23 (mA)
The lowest limit value: Output value x 7.23 (mA)
The highest limit value: Nothing
Test ID $1B Component ID $06 DTC P0161
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value x 7.23 (mA)
The lowest limit value: Nothing
The highest limit value: Output value x 7.23 (mA)
===== page 17 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $21
Test Limit Type 
and Component 
ID
$83 DTC P1456
Test Description Monitoring the fluctuation of fuel tank pressure sensor output before and after EVAP bypass solenoid valve is opened after engine starting with cold condition.
Conversion to Engineering Units
Measured value: Output value x 0.488281 (mmHg)
The lowest limit value: Output value x 0.488281 (mmHg)
The highest limit value: Nothing
Test ID $26
Test Limit Type 
and Component 
ID
$83 DTC P1456
Test Description
Monitoring either of:
1) The difference between maximum and minimum of fuel tank pressure sensor 
output in predetermined time after engine starting with cold condition, or
2) The difference between the sensor output when EVAP control canister vent 
shut and EVAP bypass solenoid valves are opened before performing tank leak 
check, and the sensor output during tank leak check.
Conversion to Engineering Units
Measured value: Output value x 0.488281 (mmHg)
The lowest limit value: Output value x 0.488281 (mmHg)
The highest limit value: Nothing
Test ID $27
Test Limit Type 
and Component 
ID
$83 DTC P1456
Test Description
Monitoring the fluctuation of fuel tank pressure sensor output before and after the 
EVAP bypass solenoid valve is opened, after fuel tank pressure check is 
completed in predetermined time after engine starting with cold condition
Conversion to Engineering Units
Measured value: Output value x 0.488281 (mmHg)
The lowest limit value: Output value x 0.488281 (mmHg)
The highest limit value: Nothing
Test ID $29 Component ID $83 DTC P1457
Test Description Monitoring the fuel tank pressure sensor output when EVAP control canister vent shut valve is closed after releasing the pressure in EVAP vapor line.
Conversion to Engineering Units
Measured value: Output value x 0.488281 - 62.500000 (mmHg)
The lowest limit value: Output value x 0.488281 - 62.500000 (mmHg)
The highest limit value: Nothing
===== page 18 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $2D
Test Limit Type 
and Component 
ID
$03 DTC P0497
Test Description Monitoring the fuel tank pressure sensor during decompressing pressure in EVAP vapor line.
Conversion to Engineering Units
Measured value: Output value x 0.488281 - 62.500000 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value x 0.488281 - 62.500000 (mmHg)
Test ID $2E
Test Limit Type 
and Component 
ID
$83 DTC P1457
Test Description
Monitoring the fluctuation of fuel tank pressure sensor output after opening EVAP 
bypass solenoid valve while maintaining decompressed pressure in EVAP vapor 
line.
Conversion to Engineering Units
Measured value: Output value x 0.488281 (mmHg)
The lowest limit value: Output value x 0.488281 (mmHg)
The highest limit value: Nothing
Test ID $2F
Test Limit Type 
and Component 
ID
$83 DTC P1457
Test Description
Monitoring the fuel tank pressure sensor output when EVAP canister purge valve 
is closed. and EVAP bypass solenoid and EVAP canister vent shut valves are 
opened before decompressing pressure in EVAP vapor line.
Conversion to Engineering Units
Measured value: Output value x 0.488281 - 62.500000 (mmHg)
The lowest limit value: Output value x 0.488281 - 62.500000 (mmHg)
The highest limit value: Nothing
Test ID $38
Test Limit Type 
and Component 
ID
$03 DTC P1456
Test Description Monitoring the difference of fuel tank pressure sensor output in pressure retention mode after decompressing the fuel tank.
Conversion to Engineering Units
Measured value: Output value x 0.488281 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value x 0.488281 (mmHg)
===== page 19 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $3A
Test Limit Type 
and Component 
ID
$83 DTC P1456
Test Description Monitoring the fluctuation of fuel tank pressure sensor before and after the EVAP bypass solenoid valve is opened after engine starting with cold condition.
Conversion to Engineering Units
Measured value: Output value x 0.488281 (mmHg)
The lowest limit value: Output value x 0.488281 (mmHg)
The highest limit value: Nothing
Test ID $3B
Test Limit Type 
and Component 
ID
$83 DTC P1456
Test Description
Monitoring the difference between the sensor output when EVAP control canister 
vent shut and EVAP bypass solenoid valves are opened before performing tank 
leak check, and the sensor output during tank leak check.
Conversion to Engineering Units
Measured value: Output value x 0.488281 (mmHg)
The lowest limit value: Output value x 0.488281 (mmHg)
The highest limit value: Nothing
Test ID $3C
Test Limit Type 
and Component 
ID
$03 DTC P1456
Test Description
Monitoring the valve calculated by dividing the difference of fuel tank pressure 
sensor output in pressure retention mode after decompressing the fuel tank by 
EVAP leak monitoring time.
Conversion to Engineering Units
Measured value: No unit
The lowest limit value: Nothing
The highest limit value: No unit
Test ID $3E
Test Limit Type 
and Component 
ID
$83 DTC P1457
Test Description
Monitoring the fluctuation of fuel tank pressure sensor output after opening EVAP 
bypass solenoid valve while maintaining decompressing pressure in EVAP vapor 
line.
Conversion to Engineering Units
Measured value: Output value x 0.488281 (mmHg)
The lowest limit value: Output value x 0.488281 (mmHg)
The highest limit value: Nothing
===== page 20 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $48
Test Limit Type 
and Component 
ID
00$ DTC P1505
Test Description
Monitoring the difference of flow rate between estimated intake air to the 
manifold and passage air through the idle air control valve during the engine 
idling.
Conversion to Engineering Units
Measured value: Output value x 0.03662 (l/min.)
The lowest limit value: Nothing
The highest limit value: Output value x 0.03662 (l/min.)
Test ID $48
Test Limit Type 
and Component 
ID
$01 DTC P2279
Test Description Monitoring the difference of flow rate between estimated intake air to the manifold during engine idling and passage air through the idle air control valve.
Conversion to Engineering Units
Measured value: Output value x 0.10 (l/min.)
The lowest limit value: Nothing
The highest limit value: Output value x 0.10 (l/min.)
Test ID $49
Test Limit Type 
and Component 
ID
$00 DTC P0404
Test Description Difference of the target and actual lift quantities of the EGR valve during the EGR valve oven control.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.01 (mm)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.01 (mm)
Test ID $4A
Test Limit Type 
and Component 
ID
$80 DTC P2413
Test Description The EGR valve lift quantity when the EGR valve is controlled at open side.
Conversion to Engineering Units
Measured value lift: Output value (Decimal) x 0.01 (mm)
The lowest limit value lift: Output value (Decimal) x 0.01 (mm)
The highest limit value lift: Nothing
===== page 21 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $4B
Test Limit Type 
and Component 
ID
$00 DTC P2413
Test Description The EGR valve lift quantity when the EGR valve is controled at close side.
Conversion to Engineering Units
Measured value lift: Output value (Decimal) x 0.01 (mm)
The lowest limit value lift: Nothing
The highest limit value lift: Output value (Decimal) x 0.01 (mm)
Test ID $4D
Test Limit Type 
and Component 
ID
$80 DTC P0128, P1486
Test Description Monitoring engine coolant temperature when the average driving speed is higher than a certain level.
Conversion to Engineering Units
Measured value: Output value - 48 (”C)
The lowest limit value: Output value - 48 (”C)
The highest limit value: Nothing
Test ID $4D
Test Limit Type 
and Component 
ID
$81 DTC P0128
Test Description Monitoring engine coolant temperature when average vehicle speed is more than certain level.
Conversion to Engineering Units
Measured value: Output value (Decimal) - 40 (”C)
The lowest limit value: Output value (Decimal) - 40 (”C)
The highest limit value: Nothing
Test ID $4E
Test Limit Type 
and Component 
ID
$80 DTC P0128, P1486
Test Description Monitoring engine coolant temperature when estimated engine coolant temperature calculated based on driving condition is lower than a certain level.
Conversion to Engineering Units
Measured value: Output value - 48 (”C)
The lowest limit value: Output value - 48 (”C)
The highest limit value: Nothing
===== page 22 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $4E
Test Limit Type 
and Component 
ID
$81 DTC P0128
Test Description Monitoring engine coolant temperature when the estimated temperature calculated from vehicle driving condition is lower than certain level.
Conversion to Engineering Units
Measured value: Output value (Decimal) - 40 (”C)
The lowest limit value: Output value (Decimal) - 40 (”C)
The highest limit value: Nothing
Test ID $4F
Test Limit Type 
and Component 
ID
$80 DTC P0128, P1486
Test Description Monitoring engine coolant temperature when estimated temperature calculated based on driving condition reaches a certain level.
Conversion to Engineering Units
Measured value: Output value - 48 (”C)
The lowest limit value: Output value - 48 (”C)
The highest limit value: Nothing
Test ID $4F
Test Limit Type 
and Component 
ID
$81 DTC P0128
Test Description Monitoring engine coolant temperature when the estimated temperature calculated from vehicle driving condition reaches certain level.
Conversion to Engineering Units
Measured value: Output value (Decimal) - 40 (”C)
The lowest limit value: Output value (Decimal) - 40 (”C)
The highest limit value: Nothing
Test ID $50
Test Limit Type 
and Component 
ID
$00 DTC P0128, P1486
Test Description Monitoring the valve calculated by subtracting actual engine coolant temperature from estimated temperature calculated based on driving condition.
Conversion to Engineering Units
Measured value: Output value x 1 (”C)
The lowest limit value: Nothing
The highest limit value: Output value x 1 (”C)
===== page 23 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $50
Test Limit Type 
and Component 
ID
$01 DTC P0128
Test Description Monitoring the difference subtracted actual engine coolant temperature from the estimated temperature calculated from vehicle driving condition.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 1 (”C)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 1 (”C)
Test ID $58
Test Limit Type 
and Component 
ID
$80 DTC P0410
Test Description Monitoring the fluctuation of the air pump electric current sensor output before and after air pump is operated.
Conversion to Engineering Units
Measured value: Output value x 0.2930 (A)
The lowest limit value: Output value x 0.2930 (A)
The highest limit value: Nothing
Test ID $59
Test Limit Type 
and Component 
ID
$00 DTC P1410
Test Description Monitoring the fluctuation of the air pump electric current sensor output before and after air pump is operated.
Conversion to Engineering Units
Measured value: Output value x 0.2930 (A)
The lowest limit value: Nothing
The highest limit value: Output value x 0.2930 (A)
Test ID $5A
Test Limit Type 
and Component 
ID
$80 DTC P0411
Test Description Monitoring the fluctuation of the air pump electric current sensor output before and after secondary air valve is opened.
Conversion to Engineering Units
Measured value: Output value x 0.2930 (A)
The lowest limit value: Output value x 0.2930 (A)
The highest limit value: Nothing
===== page 24 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $5B
Test Limit Type 
and Component 
ID
$00 DTC P1419
Test Description Monitor the difference between banks of A/F compensation coefficience due to secondary air introduction from the air pump when idle
Conversion to Engineering Units
Measured value: Output value x 2^ - 15
The lowest limit value: Output value x 2^ - 15
The highest limit value: Nothing
Test ID $5C
Test Limit Type 
and Component 
ID
$00 DTC P0128, P1486
Test Description Monitoring the voltage of engine coolant temperature sensor when the average driving cycle is higher than a certain level.
Conversion to Engineering Units
Measured value: Output value x 0.0195 (V)
The lowest limit value: Nothing
The highest limit value: Output value x 0.0195 (V)
Test ID $5D
Test Limit Type 
and Component 
ID
$00 DTC P0128, P1486
Test Description
Monitoring the voltage of engine coolant temperature sensor when estimated 
engine coolant temperature calculated based on driving condition is lower than a 
certain level.
Conversion to Engineering Units
Measured value: Output value x 0.0195 (V)
The lowest limit value: Nothing
The highest limit value: Output value x 0.0195 (V)
Test ID $5E
Test Limit Type 
and Component 
ID
$00 DTC P0128, P1486
Test Description
Monitoring the voltage of engine coolant temperature sensor when estimated 
engine coolant temperature calculated based on driving condition reaches a 
certain level.
Conversion to Engineering Units
Measured value: Output value x 0.0195 (V)
The lowest limit value: Nothing
The highest limit value: Output value x 0.0195 (V)
===== page 25 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $5F
Test Limit Type 
and Component 
ID
$00 DTC P0128, P1486
Test Description Monitoring the valve calculated by subtracting actual engine coolant temperature from estimated temperature calculated based on driving condition.
Conversion to Engineering Units
Measured value: Output value x 1 (”C)
The lowest limit value: Nothing
The highest limit value: Output value x 1 (”C)
Test ID $62
Test Limit Type 
and Component 
ID
$04 DTC P0420
Test Description Monitoring moving average value calculated from the standard value of oxygen sensor output after passing through catalyst.
Conversion to Engineering Units
Measured value: No unit
The lowest limit value: Nothing
The highest limit value: No unit
Test ID $64
Test Limit Type 
and Component 
ID
$04 DTC P0430
Test Description Monitoring moving average value calculated from the standard value of oxygen sensor output after passing through catalyst.
Conversion to Engineering Units
Measured value: No unit
The lowest limit value: Nothing
The highest limit value: No unit
Test ID $6A
Test Limit Type 
and Component 
ID
$01 DTC P1420
Test Description
Monitoring the voltage of third heater oxygen sensor output after the 
predetermined time from the switching period of second heated oxygen sensor 
output after target A/F is changed lean to rich in order to radiate adsorbed Nox.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.020 (V)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.020 (V)
===== page 26 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $70
Test Limit Type 
and Component 
ID
$80 DTC P0135
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.01 (A)
The lowest limit value: Output value (Decimal) x 0.01 (A)
The highest limit value: Nothing
Test ID $71
Test Limit Type 
and Component 
ID
$00 DTC P0135
Test Description Monitoring the electric current value of sensor heater during turning heater on.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.01 (A)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.01 (A)
Test ID $72
Test Limit Type 
and Component 
ID
$00 DTC P0135
Test Description Monitoring the electric current va lue of sensor heater during turning heater off.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.01 (A)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.01 (A)
Test ID $76
Test Limit Type 
and Component 
ID
$80 DTC P0133
Test Description Integration value at the specified time according to the response performance measurement of the A/F sensor.
Conversion to Engineering Units
Measured value: No unit
The lowest limit value: Nothing
The highest limit value: No unit
===== page 27 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $77
Test Limit Type 
and Component 
ID
$80 DTC P0153
Test Description Integration value at the specified time according to the response performance measurement of the A/F sensor.
Conversion to Engineering Units
Measured value: No unit
The lowest limit value: Nothing
The highest limit value: No unit
Test ID $78
Test Limit Type 
and Component 
ID
$80 DTC P1172
Test Description The A/F sensor output value during A/F control.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.00390625 - 128 (mA)
The lowest limit value: Output value (Decimal) x 0.00390625 - 128 (mA)
The highest limit value: Nothing
Test ID $79
Test Limit Type 
and Component 
ID
$01 DTC P1172
Test Description The A/F sensor output value during A/F control.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.00390625 - 128 (mA)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.00390625 - 128 (mA)
Test ID $7A
Test Limit Type 
and Component 
ID
$01 DTC P1174
Test Description The A/F sensor output value during A/F control.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.00390625 - 128 (mA)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.00390625 - 128 (mA)
===== page 28 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $81
Test Limit Type 
and Component 
ID
$80 DTC P0496
Test Description Monitoring fuel tank pressure sensor output when vent shut valve is closed in prescribed time until purge starts after engine starts with cold condition.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The highest limit value: Nothing
Test ID $82
Test Limit Type 
and Component 
ID
$80 DTC P2422
Test Description Monitoring fuel tank pressure sensor output while purge is normally controlled.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The highest limit value: Nothing
Test ID $88
Test Limit Type 
and Component 
ID
$00 DTC P0497
Test Description Monitoring fuel tank pressure sensor value in failing to make the tank decompress to certain level after engine starts with cold condition.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
Test ID $8B
Test Limit Type 
and Component 
ID
$00 DTC P0457
Test Description Monitoring fuel tank pressure sensor value in failing to make the tank decompress to certain level.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
===== page 29 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $8D
Test Limit Type 
and Component 
ID
$00 DTC P0457
Test Description
Monitoring the difference of fuel tank pressure sensor output just before purge is 
paused after engine starts with cold condition and then the tank is decompressed 
to certain level, and after leak check mode starts and passes certain time.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
Test ID $8F
Test Limit Type 
and Component 
ID
$80 DTC P2422
Test Description Monitoring fuel tank pressure sensor output when engine starts.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The highest limit value: Nothing
Test ID $90
Test Limit Type 
and Component 
ID
$00 DTC P0442
Test Description Monitoring fuel tank pressure sensor output until purge starts after engine starts with cold condition and then PCS opening malfunction monitoring is finished.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
Test ID $91
Test Limit Type 
and Component 
ID
$80 DTC P0442
Test Description
Monitoring the difference between maximum and minimum of prescribed fuel 
tank pressure sensor output in prescribed time after engine start switch cold 
condition.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The highest limit value: Nothing
===== page 30 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $92
Test Limit Type 
and Component 
ID
$80 DTC P0442
Test Description
Monitoring the difference of fuel tank pressure sensor output just before purge is 
paused after engine starts with cold condition and then the tank is decompressed 
to certain level, and after leak check mode starts and passes certain time.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The highest limit value: Nothing
Test ID $93
Test Limit Type 
and Component 
ID
$00 DTC P0442
Test Description Monitoring fluctuation in fuel tank divided the fluctuation during leak check mode by leak check time after fuel tank is decompressed to certain level.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg/min.)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg/min.)
Test ID $94
Test Limit Type 
and Component 
ID
$00 DTC P0456
Test Description Monitoring fuel tank pressure sensor output in prescribed time after engine starts with cold condition.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
Test ID $95
Test Limit Type 
and Component 
ID
$80 DTC P0456
Test Description
Monitoring the difference of fuel tank pressure sensor output just before purge is 
paused after engine starts with cold condition and then the tank is decompressed 
to certain level, and after leak check mode starts and passes certain time.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The highest limit value: Nothing
===== page 31 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $96
Test Limit Type 
and Component 
ID
$00 DTC P0456
Test Description Monitoring fluctuation in fuel tank divided the fluctuation during leak check mode by leak check mode time after fuel tank is decompressed to certain level.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg/min.)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg/min.)
Test ID $97
Test Limit Type 
and Component 
ID
$00 DTC P0456
Test Description Monitoring fluctuation in fuel tank divided the fluctuation during leak check mode by leak check mode time after fuel tank is decompressed to certain level.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg/min.)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg/min.)
Test ID $9A
Test Limit Type 
and Component 
ID
$00 DTC P0456
Test Description Fuel tank pressure incr ease changing degree after the engine stop.
Conversion to Engineering Units
Measured value lift: Output value (Decimal) / 1024 - 32
The lowest limit value lift: Nothing
The highest limit value lift: Output value (Decimal) / 1024 - 32
Test ID $9B
Test Limit Type 
and Component 
ID
$00 DTC P0456
Test Description Fuel tank pressure incr ease changing degree after the engine stop.
Conversion to Engineering Units
Measured value lift: Output value (Decimal) / 32 - 1024
The lowest limit value lift: Nothing
The highest limit value lift: Output value (Decimal) / 32 - 1024
===== page 32 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $9C
Test Limit Type 
and Component 
ID
$00 DTC P0456
Test Description Fuel tank pres sure atmospheric pressure stagnation time after the engine stop.
Conversion to Engineering Units
Measured value lift: Output value (Decimal) x 1 (sec.)
The lowest limit value lift: Nothing
The highest limit value lift: Output value (Decimal) x 1 (sec.)
Test ID $9D Component ID $00 DTC P0456
Test Description Fuel tank pressure when tank air opened after the engine stop.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.001 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.001 (mmHg)
Test ID $9E
Test Limit Type 
and Component 
ID
$00 DTC P0456
Test Description Sensor flickering integrated time during waiting for fuel tank pressure sensor stability after the engine stop.
Conversion to Engineering Units
Measured value lift: Output value (Decimal) x 0.08 (sec.)
The lowest limit value lift: Nothing
The highest limit value lift: Output value (Decimal) x 0.08 (sec.)
Test ID $B0
Test Limit Type 
and Component 
ID
$00 DTC P0497
Test Description
Difference of the fuel tank pressure sensor value and when opened to air, when 
the purge flow integrataed value reached above the specified value at the time 
when decreasing the fuel tank pressure to the specified negative pressure after 
the cold start.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
===== page 33 =====
SAE J1979 Mode $06 Information by Test ID
Test ID $B1
Test Limit Type 
and Component 
ID
$00 DTC P0497
Test Description
Difference of the fuel tank pressure sensor value and when opened to air, when 
the purge flow integrataed value reached above the specified value at the time 
when decreasing the fuel tank pressure to the specified negative pressure after 
the cold start.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
Test ID $B2
Test Limit Type 
and Component 
ID
$00 DTC P0457
Test Description Fuel tank pressure sensor value when tried to decrease pressure to the specified negative pressure but cannot decrease the pressure after the cold start.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
Test ID $B3
Test Limit Type 
and Component 
ID
$00 DTC P0457
Test Description Fuel tank pressure sensor value when tried to decrease pressure to the specified negative pressure but cannot decrease the pressure after the cold start.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg)
Test ID $B4
Test Limit Type 
and Component 
ID
$00 DTC P0457
Test Description Fuel tank pressure sensor value when tried to decrease pressure to the specified negative pressure but cannot decrease the pressure after the cold start.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
===== page 34 =====
SAE J1979 Mode $06 Information by Test ID
2005-Mode06-00Honda
Test ID $B5
Test Limit Type 
and Component 
ID
$00 DTC P0455
Test Description
Fuel tank pressure changed quantity per unit time dividing the fuel tank pressure 
changed quantity with the leak check mode time during leak check mode is 
performed, after the fuel tank pressure is decreased to the specified negative 
pressure.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
Test ID $B6
Test Limit Type 
and Component 
ID
$00 DTC P0455
Test Description
Fuel tank pressure changed quantity per unit time dividing the fuel tank pressure 
changed quantity with the leak check mode time during leak check mode is 
performed, after the fuel tank pressure is decreased to the specified negative 
pressure.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg)
Test ID $B7
Test Limit Type 
and Component 
ID
$00 DTC P0455
Test Description
Fuel tank pressure changed quantity per unit time dividing the fuel tank pressure 
changed quantity with the leak check mode time during leak check mode is 
performed, after the fuel tank pressure is decreased to the specified negative 
pressure.
Conversion to Engineering Units
Measured value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)
The lowest limit value: Nothing
The highest limit value: Output value (Decimal) x 0.1 (mmHg) - 3276.8 (mmHg)