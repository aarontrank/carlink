---
title: JejuSoul OBD-PIDs-for-HKMC-EVs — Kona EV & Niro EV extendedpids (Torque Pro CSVs)
source_url: https://github.com/JejuSoul/OBD-PIDs-for-HKMC-EVs/tree/493b335a52a2b9d7a2aa3fa747c1d56e94f575ee/Hyundai%20Kona%20EV%20%26%20Kia%20Niro%20EV
commit: 493b335a52a2b9d7a2aa3fa747c1d56e94f575ee
fetched: 2026-11-07
---

Verbatim copies of the README and the six CSV files. CSV columns (Torque Pro import format):
Name, ShortName, ModeAndPID, Equation, Min, Max, Units, Header. Equation letters follow Torque's scheme:
A = first data byte after the positive-response header (62 + 2-byte DID, or 61 + 1-byte local id),
B = next, ... Z, AA, AB, ...; `{j:7}` = bit 7 of byte J; `a<8` = a shifted left 8.

## README.md

```
# OBD PIDs for the Hyundai Kona EV and Kia Niro EV

## How to use:

These codes are primarily for use with Torque Pro which runs on Android phones and tablets.
To input the codes to the phone : [How to use Torque with a Hyundai / Kia EV](https://jejusoul.github.io/OBD-PIDs-for-HKMC-EVs/)

For some info on using Torque Pro. [Tips for using Torque Pro](https://jejusoul.github.io/OBD-PIDs-for-HKMC-EVs/tips.html)

## List of files: 

OBD PIDs in the extendedpids folder 

(To be updated)

## TODO:
Most of the PIDs from the Kia Soul EV are working for Ioniq and Niro. Some values are based on Soul EV and need to be adjusted to the Niro specs.

Investigation spreadsheet is at [Kona PIDs](https://docs.google.com/spreadsheets/d/1-9jZafV9eZeBUnPQo7qQHbX2-_4qZfWfRVpidoF1owA/edit?usp=sharing)

## References:
All merits for this PIDs are from the original dev from https://github.com/JejuSoul

```

## 001_KonaandNiro_EV_Aux_Battery.csv

```
000_Auxillary Battery SOC,Aux Batt SOC,2102,X,0,100,%,7E2
000_Auxillary Battery Current,Aux Batt CURR,2102,(SIGNED(W)*256+V)/1000,-100,100,A,7E2
000_Niro_Auxillary Battery Voltage,Aux Batt Volts,2102,(SIGNED(U)*256+T)/1000,11,14.6,V,7E2

```

## 002_KonaandNiro_EV_HVAC.csv

```
001_Outdoor Temperature,Outdoor Temperature,220100,(G/2)-40,-50,50,degC,7B3
001_Indoor Temperature,Indoor Temperature,220100,(F/2)-40,-50,50,degC,7B3

```

## 003_KonaandNiro_EV_BMS.csv

```
000_Airbag H/wire Duty,Airbag,0x220105,w,50,100,,7E4
000_Auxillary Battery Voltage,Aux Batt Volts,0x220101,ad*0.1,11,14.6,V,7E4
000_Available Charge Power,Max REGEN,0x220101,((f<8)+g)/100,0,98,kW,7E4
000_Available Discharge Power,Max POWER,0x220101,((h<8)+i)/100,0,98,kW,7E4
000_Battery Cell Voltage Deviation,V Diff,0x220105,u/50,0,0.5,V,7E4
000_Battery Current,Batt Current,0x220101,((Signed(K)*256)+L)/10,-230,230,A,7E4
000_Battery DC Voltage,Batt Volts,0x220101,((m<8)+n)/10,268.8,403.2,V,7E4
000_Battery Fan Feedback,Batt Fan SPD,0x220101,ac,0,120,Hz,7E4
000_Battery Fan Status,Batt Fan MOD,0x220101,ab,0,9,,7E4
000_Battery Heater 1 Temperature,Heater Temp1,0x220105,Signed(X),0,30,C,7E4
000_Battery Heater 2 Temperature,Heater Temp2,0x220105,Signed(Y),0,30,C,7E4
000_Battery Inlet Temperature,Batt InletT,0x220101,Signed(W),-40,80,C,7E4
000_Battery Max Temperature,Batt MaxT,0x220101,Signed(O),-40,80,C,7E4
000_Battery Min Temperature,Batt MinT,0x220101,Signed(P),-40,80,C,7E4
000_Battery Module 01 Temperature,Batt Temp01,0x220101,Signed(Q),-40,80,C,7E4
000_Battery Module 02 Temperature,Batt Temp02,0x220101,Signed(R),-40,80,C,7E4
000_Battery Module 03 Temperature,Batt Temp03,0x220101,Signed(S),-40,80,C,7E4
000_Battery Module 04 Temperature,Batt Temp04,0x220101,Signed(T),-40,80,C,7E4
000_Battery Power,Energy Draw,0x220101,val{000_Battery Current}*val{000_Battery DC Voltage}/1000,-90,90,kW,7E4
000_BMS Ignition,BMS Ignit.,0x220101,{ay:2},0,1,,7E4
000_BMS Main Relay,BMS Relay,0x220101,{j:0},0,1,,7E4
000_Cumulative Charge Current,CCC,0x220101,((ae<24)+(af<16)+(ag<8)+ah)/10,0,1000000,Ah,7E4
000_Cumulative Discharge Current,CDC,0x220101,((ai<24)+(aj<16)+(ak<8)+al)/10,0,1000000,Ah,7E4
000_Cumulative Energy Charged,CEC,0x220101,((am<24)+(an<16)+(ao<8)+ap)/10,0,1000000,kWh,7E4
000_Cumulative Energy Discharged,CED,0x220101,((aq<24)+(ar<16)+(as<8)+at)/10,0,1000000,kWh,7E4
000_Drive Motor Speed 1,Motor RPM 1,0x220101,(Signed(BB)*256)+BC,-10100,10100,rpm,7E4
000_Drive Motor Speed 2,Motor RPM 2,0x220101,(Signed(BD)*256))+BE,-10100,10100,rpm,7E4
000_HV_Charging,Charging,0x220101,{j:7},0,1,,7E4
000_Inverter Capacitor Voltage,BMS Capacitor,0x220101,((az<8)+ba),0,500,V,7E4
000_Isolation Resistance,Surge Resistor,0x220101,((bf<8)+bg),0,1000,kOhm,7E4
000_Maximum Cell Voltage,Max Cell V,0x220101,x/50,2.8,4.2,V,7E4
000_Maximum Cell Voltage No.,Max Cell V  No.,0x220101,y,0,98,,7E4
000_Maximum Deterioration Cell No.,Max Det Cell No.,0x220105,ab,0,98,,7E4
000_Minimum Cell Voltage,Min Cell V,0x220101,z/50,2.8,4.2,V,7E4
000_Minimum Cell Voltage No.,Min Cell V  No.,0x220101,aa,0,98,,7E4
000_Minimum Deterioration,Min Det,0x220105,((ac<8)+ad)/10,0,100,%,7E4
000_Minimum Deterioration Cell No.,Min Det Cell No.,0x220105,ae,0,98,,7E4
000_Normal Charge Port,J1772 Plug,0x220101,{j:5},0,1,,7E4
000_Operating Time,OpTime,0x220101,((au<24)+(av<16)+(aw<8)+ax)/3600,0,1000000,hours,7E4
000_Rapid Charge Port,Chademo Plug,0x220101,{j:6},0,1,,7E4
000_State of Charge BMS,SOC BMS,0x220101,e/2,0,100,%,7E4
000_State of Charge Display,SOC Display,0x220105,af/2,0,100,%,7E4
000_State of Health,SOH,0x220105,((z<8)+aa)/10,0,100,%,7E4

```

## 004_KonaandNiro_EV_BMS_cell_data.csv

```
000_Cell Voltage 01,Cell 01,0x220102,e/50,2.80,4.20,V,7E4
000_Cell Voltage 02,Cell 02,0x220102,f/50,2.80,4.20,V,7E4
000_Cell Voltage 03,Cell 03,0x220102,g/50,2.80,4.20,V,7E4
000_Cell Voltage 04,Cell 04,0x220102,h/50,2.80,4.20,V,7E4
000_Cell Voltage 05,Cell 05,0x220102,i/50,2.80,4.20,V,7E4
000_Cell Voltage 06,Cell 06,0x220102,j/50,2.80,4.20,V,7E4
000_Cell Voltage 07,Cell 07,0x220102,k/50,2.80,4.20,V,7E4
000_Cell Voltage 08,Cell 08,0x220102,l/50,2.80,4.20,V,7E4
000_Cell Voltage 09,Cell 09,0x220102,m/50,2.80,4.20,V,7E4
000_Cell Voltage 10,Cell 10,0x220102,n/50,2.80,4.20,V,7E4
000_Cell Voltage 11,Cell 11,0x220102,o/50,2.80,4.20,V,7E4
000_Cell Voltage 12,Cell 12,0x220102,p/50,2.80,4.20,V,7E4
000_Cell Voltage 13,Cell 13,0x220102,q/50,2.80,4.20,V,7E4
000_Cell Voltage 14,Cell 14,0x220102,r/50,2.80,4.20,V,7E4
000_Cell Voltage 15,Cell 15,0x220102,s/50,2.80,4.20,V,7E4
000_Cell Voltage 16,Cell 16,0x220102,t/50,2.80,4.20,V,7E4
000_Cell Voltage 17,Cell 17,0x220102,u/50,2.80,4.20,V,7E4
000_Cell Voltage 18,Cell 18,0x220102,v/50,2.80,4.20,V,7E4
000_Cell Voltage 19,Cell 19,0x220102,w/50,2.80,4.20,V,7E4
000_Cell Voltage 20,Cell 20,0x220102,x/50,2.80,4.20,V,7E4
000_Cell Voltage 21,Cell 21,0x220102,y/50,2.80,4.20,V,7E4
000_Cell Voltage 22,Cell 22,0x220102,z/50,2.80,4.20,V,7E4
000_Cell Voltage 23,Cell 23,0x220102,aa/50,2.80,4.20,V,7E4
000_Cell Voltage 24,Cell 24,0x220102,ab/50,2.80,4.20,V,7E4
000_Cell Voltage 25,Cell 25,0x220102,ac/50,2.80,4.20,V,7E4
000_Cell Voltage 26,Cell 26,0x220102,ad/50,2.80,4.20,V,7E4
000_Cell Voltage 27,Cell 27,0x220102,ae/50,2.80,4.20,V,7E4
000_Cell Voltage 28,Cell 28,0x220102,af/50,2.80,4.20,V,7E4
000_Cell Voltage 29,Cell 29,0x220102,ag/50,2.80,4.20,V,7E4
000_Cell Voltage 30,Cell 30,0x220102,ah/50,2.80,4.20,V,7E4
000_Cell Voltage 31,Cell 31,0x220102,ai/50,2.80,4.20,V,7E4
000_Cell Voltage 32,Cell 32,0x220102,aj/50,2.80,4.20,V,7E4
000_Cell Voltage 33,Cell 33,0x220103,e/50,2.80,4.20,V,7E4
000_Cell Voltage 34,Cell 34,0x220103,f/50,2.80,4.20,V,7E4
000_Cell Voltage 35,Cell 35,0x220103,g/50,2.80,4.20,V,7E4
000_Cell Voltage 36,Cell 36,0x220103,h/50,2.80,4.20,V,7E4
000_Cell Voltage 37,Cell 37,0x220103,i/50,2.80,4.20,V,7E4
000_Cell Voltage 38,Cell 38,0x220103,j/50,2.80,4.20,V,7E4
000_Cell Voltage 39,Cell 39,0x220103,k/50,2.80,4.20,V,7E4
000_Cell Voltage 40,Cell 40,0x220103,l/50,2.80,4.20,V,7E4
000_Cell Voltage 41,Cell 41,0x220103,m/50,2.80,4.20,V,7E4
000_Cell Voltage 42,Cell 42,0x220103,n/50,2.80,4.20,V,7E4
000_Cell Voltage 43,Cell 43,0x220103,o/50,2.80,4.20,V,7E4
000_Cell Voltage 44,Cell 44,0x220103,p/50,2.80,4.20,V,7E4
000_Cell Voltage 45,Cell 45,0x220103,q/50,2.80,4.20,V,7E4
000_Cell Voltage 46,Cell 46,0x220103,r/50,2.80,4.20,V,7E4
000_Cell Voltage 47,Cell 47,0x220103,s/50,2.80,4.20,V,7E4
000_Cell Voltage 48,Cell 48,0x220103,t/50,2.80,4.20,V,7E4
000_Cell Voltage 49,Cell 49,0x220103,u/50,2.80,4.20,V,7E4
000_Cell Voltage 50,Cell 50,0x220103,v/50,2.80,4.20,V,7E4
000_Cell Voltage 51,Cell 51,0x220103,w/50,2.80,4.20,V,7E4
000_Cell Voltage 52,Cell 52,0x220103,x/50,2.80,4.20,V,7E4
000_Cell Voltage 53,Cell 53,0x220103,y/50,2.80,4.20,V,7E4
000_Cell Voltage 54,Cell 54,0x220103,z/50,2.80,4.20,V,7E4
000_Cell Voltage 55,Cell 55,0x220103,aa/50,2.80,4.20,V,7E4
000_Cell Voltage 56,Cell 56,0x220103,ab/50,2.80,4.20,V,7E4
000_Cell Voltage 57,Cell 57,0x220103,ac/50,2.80,4.20,V,7E4
000_Cell Voltage 58,Cell 58,0x220103,ad/50,2.80,4.20,V,7E4
000_Cell Voltage 59,Cell 59,0x220103,ae/50,2.80,4.20,V,7E4
000_Cell Voltage 60,Cell 60,0x220103,af/50,2.80,4.20,V,7E4
000_Cell Voltage 61,Cell 61,0x220103,ag/50,2.80,4.20,V,7E4
000_Cell Voltage 62,Cell 62,0x220103,ah/50,2.80,4.20,V,7E4
000_Cell Voltage 63,Cell 63,0x220103,ai/50,2.80,4.20,V,7E4
000_Cell Voltage 64,Cell 64,0x220103,aj/50,2.80,4.20,V,7E4
000_Cell Voltage 65,Cell 65,0x220104,e/50,2.80,4.20,V,7E4
000_Cell Voltage 66,Cell 66,0x220104,f/50,2.80,4.20,V,7E4
000_Cell Voltage 67,Cell 67,0x220104,g/50,2.80,4.20,V,7E4
000_Cell Voltage 68,Cell 68,0x220104,h/50,2.80,4.20,V,7E4
000_Cell Voltage 69,Cell 69,0x220104,i/50,2.80,4.20,V,7E4
000_Cell Voltage 70,Cell 70,0x220104,j/50,2.80,4.20,V,7E4
000_Cell Voltage 71,Cell 71,0x220104,k/50,2.80,4.20,V,7E4
000_Cell Voltage 72,Cell 72,0x220104,l/50,2.80,4.20,V,7E4
000_Cell Voltage 73,Cell 73,0x220104,m/50,2.80,4.20,V,7E4
000_Cell Voltage 74,Cell 74,0x220104,n/50,2.80,4.20,V,7E4
000_Cell Voltage 75,Cell 75,0x220104,o/50,2.80,4.20,V,7E4
000_Cell Voltage 76,Cell 76,0x220104,p/50,2.80,4.20,V,7E4
000_Cell Voltage 77,Cell 77,0x220104,q/50,2.80,4.20,V,7E4
000_Cell Voltage 78,Cell 78,0x220104,r/50,2.80,4.20,V,7E4
000_Cell Voltage 79,Cell 79,0x220104,s/50,2.80,4.20,V,7E4
000_Cell Voltage 80,Cell 80,0x220104,t/50,2.80,4.20,V,7E4
000_Cell Voltage 81,Cell 81,0x220104,u/50,2.80,4.20,V,7E4
000_Cell Voltage 82,Cell 82,0x220104,v/50,2.80,4.20,V,7E4
000_Cell Voltage 83,Cell 83,0x220104,w/50,2.80,4.20,V,7E4
000_Cell Voltage 84,Cell 84,0x220104,x/50,2.80,4.20,V,7E4
000_Cell Voltage 85,Cell 85,0x220104,y/50,2.80,4.20,V,7E4
000_Cell Voltage 86,Cell 86,0x220104,z/50,2.80,4.20,V,7E4
000_Cell Voltage 87,Cell 87,0x220104,aa/50,2.80,4.20,V,7E4
000_Cell Voltage 88,Cell 88,0x220104,ab/50,2.80,4.20,V,7E4
000_Cell Voltage 89,Cell 89,0x220104,ac/50,2.80,4.20,V,7E4
000_Cell Voltage 90,Cell 90,0x220104,ad/50,2.80,4.20,V,7E4
000_Cell Voltage 91,Cell 91,0x220104,ae/50,2.80,4.20,V,7E4
000_Cell Voltage 92,Cell 92,0x220104,af/50,2.80,4.20,V,7E4
000_Cell Voltage 93,Cell 93,0x220104,ag/50,2.80,4.20,V,7E4
000_Cell Voltage 94,Cell 94,0x220104,ah/50,2.80,4.20,V,7E4
000_Cell Voltage 95,Cell 95,0x220104,ai/50,2.80,4.20,V,7E4
000_Cell Voltage 96,Cell 96,0x220104,aj/50,2.80,4.20,V,7E4
000_Cell Voltage 97,Cell 97,0x220105,ai/50,2.80,4.20,V,7E4
000_Cell Voltage 98,Cell 98,0x220105,aj/50,2.80,4.20,V,7E4

```

## 005_KonaandNiro_EV_Extra_gauges.csv

```
004_CALC Average Battery Module Temperature, Batt AvgT,0x220101,(val{000_Battery Module 01 Temperature}+val{000_Battery Module 02 Temperature}+val{000_Battery Module 03 Temperature}+val{000_Battery Module 04 Temperature})/4.0,-40,80,C,7E4
004_CALC Average Cell Voltage,Avg Cell V,0x220101,val{000_Battery DC Voltage}/98,2.8,4.2,V,7E4
004_CALC Estimated time 45kW 80% Charge,45kW 80%C,0x220105,(MAX(0:64*(0.8-(af/200)))/45)*60,0,100,minutes,7E4

```

## ABRP_KonaandNiro_PIDs.csv

```
Name,ShortName,ModeAndPID,Equation,Min Value,Max Value,Units,Header
~Modifications by Jason Rapp for ABRP data contribution,List Credit,,,,,,
!_ABRP_Battery Current,Batt Current,,val{000_Battery Current},-230,230,A,
!_ABRP_Battery DC Voltage,Batt Volts,,val{000_Battery DC Voltage},0,406,V,
!_ABRP_Battery Power,Energy Draw,,val{000_Battery Current}*val{000_Battery DC Voltage}/1000,-90,90,kW,
!_ABRP_Fast_Charging,Charging,,val{000_Rapid Charge Port},0,1,,
!_ABRP_State of Charge BMS,SOC BMS,,val{000_State of Charge BMS},0,100,%,
!_ABRP_State of Charge Display,SOC Display,,val{000_State of Charge Display},0,100,%,
!_ABRP_Max Deterioration,SOH,,val{000_Maximum Deterioration},0,100,%,
```

