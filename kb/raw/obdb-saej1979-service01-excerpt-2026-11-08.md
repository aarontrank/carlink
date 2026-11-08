---
title: OBDb SAEJ1979 signalset — service 01 PID definitions (excerpt for the extended-PID research)
source_url: https://github.com/OBDb/SAEJ1979/blob/06690177d2d5e11dbf82566297128d673b5a03a7/signalsets/v3/default.json
commit: 06690177d2d5e11dbf82566297128d673b5a03a7
fetched: 2026-11-08
license: CC BY-SA 4.0
note: >
  Verbatim JSON of the selected commands (signals only). OBDb fmt: `bix` = bit index from the first
  data byte after `41 <PID>` (0 = MSB of A), `len` in bits, value = raw*mul/div + add, `sign: true`
  = two's complement. Many `description` strings read like J1979 text. The file defines service 01
  only (no service 06).
---

### 01 03

```json
[{"id": "FUELSYS1", "path": "Fuel.Generic", "name": "Fuel system 1 status", "fmt": {"len": 8, "map": {"0": {"description": "No status reported: this fuel system is not running or not present, e.g. the engine is off", "value": "OFF"}, "1": {"description": "Open loop - has not yet satisfied conditions to go closed loop", "value": "OL"}, "2": {"description": "Closed loop - using oxygen sensor(s) as feedback for fuel control", "value": "CL"}, "4": {"description": "Open loop due to driving conditions (e.g. power enrichment, deceleration enleanment)", "value": "OL-Drive"}, "8": {"description": "Open loop - due to detected system fault", "value": "OL-Fault"}, "16": {"description": "Closed loop, but fault with at least one oxygen sensor - may be using single oxygen sensor for fuel control", "value": "CL-Fault"}, "32": {"description": "Open loop - has not yet satisfied conditions to go closed loop (Bank 2)", "value": "OL B2"}, "64": {"description": "Open loop due to driving conditions (Bank 2) (e.g. power enrichment, deceleration enleanment, cylinder deactivation)", "value": "OL-Drive B2"}, "68": {"description": "Open loop due to driving conditions on bank 1 and bank 2 (e.g. power enrichment, deceleration enleanment, cylinder deactivation)", "value": "OL-Drive B1+B2"}, "128": {"description": "Open loop - due to detected system fault (Bank 2)", "value": "OL-Fault B2"}}}}, {"id": "FUELSYS2", "path": "Fuel.Generic", "name": "Fuel system 2 status", "fmt": {"bix": 8, "len": 8, "map": {"0": {"description": "No status reported: this fuel system is not running or not present, e.g. the engine is off", "value": "OFF"}, "1": {"description": "Open loop - has not yet satisfied conditions to go closed loop", "value": "OL"}, "2": {"description": "Closed loop - using oxygen sensor(s) as feedback for fuel control", "value": "CL"}, "4": {"description": "Open loop due to driving conditions (e.g. power enrichment, deceleration enleanment)", "value": "OL-Drive"}, "8": {"description": "Open loop - due to detected system fault", "value": "OL-Fault"}, "16": {"description": "Closed loop, but fault with at least one oxygen sensor - may be using single oxygen sensor for fuel control", "value": "CL-Fault"}, "32": {"description": "Open loop - has not yet satisfied conditions to go closed loop (Bank 2)", "value": "OL B2"}, "64": {"description": "Open loop due to driving conditions (Bank 2) (e.g. power enrichment, deceleration enleanment, cylinder deactivation)", "value": "OL-Drive B2"}, "68": {"description": "Open loop due to driving conditions on bank 1 and bank 2 (e.g. power enrichment, deceleration enleanment, cylinder deactivation)", "value": "OL-Drive B1+B2"}, "128": {"description": "Open loop - due to detected system fault (Bank 2)", "value": "OL-Fault B2"}}}}]
```

### 01 04

```json
[{"id": "LOAD_PCT", "path": "Engine.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Calculated engine load", "suggestedMetric": "engineLoad", "description": "Represents the amount of work the engine is doing. Expected to reach 100% at wide open throttle/wide open pedal at any altitude, temperature or rpm for both naturally aspirated and boosted engines. If engine load is limited for powertrain protection e.g. engine/turbocharger protection, this value may not reach 100%. For hybrid vehicles, indicates the torque produced only by the internal combustion engine, not the torque being delivered by the entire powertrain. For electric vehicles, the meaning of this parameter is undefined."}]
```

### 01 12

```json
[{"id": "AIR_STAT", "path": "Engine.Generic", "name": "Commanded secondary air status", "fmt": {"bix": 5, "len": 3, "map": {"1": {"description": "Upstream of first catalytic converter", "value": "UPS"}, "2": {"description": "Downstream of first catalytic converter inlet", "value": "DNS"}, "4": {"description": "Atmosphere / off", "value": "OFF"}, "8": {"description": "Pump commanded on for diagnostics", "value": "DIAG"}}}}]
```

### 01 13

```json
[{"id": "O2S24_EXISTS", "path": "Engine.Generic.OxygenSensors", "fmt": {"len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 2, Sensor 4 present"}, {"id": "O2S23_EXISTS", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 1, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 2, Sensor 3 present"}, {"id": "O2S22_EXISTS", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 2, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 2, Sensor 2 present"}, {"id": "O2S21_EXISTS", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 3, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 2, Sensor 1 present"}, {"id": "O2S14_EXISTS", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 4, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 1, Sensor 4 present"}, {"id": "O2S13_EXISTS", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 5, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 1, Sensor 3 present"}, {"id": "O2S12_EXISTS", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 6, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 1, Sensor 2 present"}, {"id": "O2S11_EXISTS", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 7, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 1, Sensor 1 present"}]
```

### 01 14

```json
[{"id": "O2S11", "path": "Engine.Generic.OxygenSensors", "fmt": {"len": 8, "max": 1.275, "div": 200, "unit": "volts"}, "name": "O2S Output Voltage Bank 1, Sensor 1"}, {"id": "SHRTFT11", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 8, "len": 8, "max": 99.22, "min": -100, "mul": 100, "div": 128, "add": -100, "nullmax": 99.2, "unit": "percent"}, "name": "SHRTFT associated with O2S11"}]
```

### 01 15

```json
[{"id": "O2S12", "path": "Engine.Generic.OxygenSensors", "fmt": {"len": 8, "max": 1.275, "div": 200, "unit": "volts"}, "name": "O2S Output Voltage Bank 1, Sensor 2"}, {"id": "SHRTFT12", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 8, "len": 8, "max": 99.22, "min": -100, "mul": 100, "div": 128, "add": -100, "nullmax": 99.2, "unit": "percent"}, "name": "SHRTFT associated with O2S12"}]
```

### 01 18

```json
[{"id": "O2S21", "path": "Engine.Generic.OxygenSensors", "fmt": {"len": 8, "max": 1.275, "div": 200, "unit": "volts"}, "name": "O2S Output Voltage Bank 2, Sensor 1"}, {"id": "SHRTFT21", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 8, "len": 8, "max": 99.22, "min": -100, "mul": 100, "div": 128, "add": -100, "nullmax": 99.2, "unit": "percent"}, "name": "SHRTFT associated with O2S21"}]
```

### 01 19

```json
[{"id": "O2S22", "path": "Engine.Generic.OxygenSensors", "fmt": {"len": 8, "max": 1.275, "div": 200, "unit": "volts"}, "name": "O2S Output Voltage Bank 2, Sensor 2"}, {"id": "SHRTFT22", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 8, "len": 8, "max": 99.22, "min": -100, "mul": 100, "div": 128, "add": -100, "nullmax": 99.2, "unit": "percent"}, "name": "SHRTFT associated with O2S22"}]
```

### 01 1C

```json
[{"id": "OBDSUP", "path": "OBD.Generic", "name": "OBD requirements to which vehicle is designed", "fmt": {"len": 8, "map": {"1": {"description": "OBD II (California ARB)", "value": "OBD II"}, "2": {"description": "OBD (Federal EPA)", "value": "OBD"}, "3": {"description": "OBD & OBD II", "value": "OBD & OBD II"}, "4": {"description": "OBD I", "value": "OBD I"}, "5": {"description": "Not OBD compliant", "value": "NO OBD"}, "6": {"description": "EOBD", "value": "EOBD"}, "7": {"description": "EOBD & OBD II", "value": "EOBD & OBD II"}, "8": {"description": "EOBD & OBD", "value": "EOBD & OBD"}, "9": {"description": "EOBD, OBD, OBD II", "value": "EOBD, OBD, OBD II"}, "10": {"description": "JOBD", "value": "JOBD"}, "11": {"description": "JOBD & OBD II", "value": "JOBD & OBD II"}, "12": {"description": "JOBD & EOBD", "value": "JOBD & EOBD"}, "13": {"description": "JOBD, EOBD, OBD II", "value": "JOBD, EOBD, OBD II"}, "14": {"description": "Heavy duty vehicles (EURO IV) B1", "value": "EURO IV B1"}, "15": {"description": "Heavy duty vehicles (EURO V) B2", "value": "EURO V B2"}, "16": {"description": "Heavy duty vehicles (EURO EEC) C (gas engines)", "value": "EURO C"}, "17": {"description": "Engine manufacturer diagnostics (EMD)", "value": "EMD"}, "18": {"description": "Engine Manufacturer Diagnostics Enhanced (EMD+)", "value": "EMD+"}, "19": {"description": "Heavy Duty On-Board Diagnostics (Child/Partial)", "value": "HD OBD-C"}, "20": {"description": "Heavy Duty On-Board Diagnostics", "value": "HD OBD"}, "21": {"description": "World Wide Harmonized OBD", "value": "WWH OBD"}, "23": {"description": "Heavy Duty Euro OBD Stage I without NOx Control", "value": "HD EOBD-I"}, "24": {"description": "Heavy Duty Euro OBD Stage I with NOx Control", "value": "HD EOBD-I N"}, "25": {"description": "Heavy Duty Euro OBD Stage II without NOx Control", "value": "HD EOBD-II"}, "26": {"description": "Heavy Duty Euro OBD Stage II with NOx Control", "value": "HD EOBD-II N"}, "27": {"description": "Heavy Duty ZEV", "value": "HD-ZEV"}, "28": {"description": "Brazil OBD Phase 1", "value": "OBDBr-1"}, "29": {"description": "Brazil OBD Phase 2 and Phase 2+", "value": "OBDBr-2"}, "30": {"description": "Korean OBD", "value": "KOBD"}, "31": {"description": "India BS4 OBD I", "value": "IOBD-I-BS4"}, "32": {"description": "India BS4 OBD II", "value": "IOBD-II-BS4"}, "33": {"description": "Euro VI", "value": "HD EOBD-VI"}, "34": {"description": "OBD, OBD II and HD OBD", "value": "OBD, OBD II and HD OBD"}, "35": {"description": "Brazil OBD Phase 3", "value": "OBDBr-3"}, "36": {"description": "Motorcycle, Euro OBD-I", "value": "MC EOBD-I"}, "37": {"description": "Motorcycle, Euro OBD-II", "value": "MC EOBD-II"}, "38": {"description": "Motorcycle, China OBD-I", "value": "MC COBD-I"}, "39": {"description": "Motorcycle, Taiwan OBD-I", "value": "MC TOBD-I"}, "40": {"description": "Motorcycle, Japan OBD-I", "value": "MC JOBD-I"}, "41": {"description": "China Nationwide Stage 6", "value": "CN-OBD-6"}, "42": {"description": "Brazil OBD Phase 7", "value": "OBDBr-P7"}, "43": {"description": "China Heavy Duty VI", "value": "CN-HDOBD-VI"}, "44": {"description": "India BS6 OBD I", "value": "IOBD-I-BS6"}, "45": {"description": "India BS6 OBD II", "value": "IOBD-II-BS6"}, "46": {"description": "India BSVI HD OBD", "value": "IHDOBD-BSVI"}, "47": {"description": "Brazil OBD Phase 8", "value": "OBDBr-P8"}, "48": {"description": "Japan Heavy Duty OBD-II", "value": "HD-JOBD-II"}, "49": {"description": "Korea Heavy Duty OBD-II", "value": "HD-KOBD-II"}, "50": {"description": "China Off-Road IV OBD", "value": "CN-OROBD-IV"}, "51": {"description": "Light Duty ZEV, ACC-II", "value": "CARB ACC-II"}, "52": {"description": "Motorcycle, Japan OBD-II", "value": "MC JOBD-II"}, "53": {"description": "Motorcycle, California (CARB) OBD", "value": "MC CARB OBD"}, "54": {"description": "Motorcycle, Federal (EPA) OBD", "value": "MC EPA OBD"}, "55": {"description": "Motorcycle, 50-State (CARB & EPA) OBD", "value": "MC CARB & EPA OBD"}, "56": {"description": "Heavy Duty ZEV, CARB ZEP", "value": "HD ZEV CARB ZEP"}, "57": {"description": "Light Duty ZEV, CARB ACC-II and EPA Tier 4 (GTR 22)", "value": "CARB ACC-II & EPA TIER4"}, "58": {"description": "Light Duty ZEV, EPA Tier 4 (GTR 22)", "value": "EPA TIER4"}, "59": {"description": "EPA HD OBD", "value": "EPA HD"}}}}]
```

### 01 1D

```json
[{"id": "O2S42_EXISTS", "path": "Engine.Generic", "fmt": {"len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 4, Sensor 2 present"}, {"id": "O2S41_EXISTS", "path": "Engine.Generic", "fmt": {"bix": 1, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 4, Sensor 1 present"}, {"id": "O2S32_EXISTS", "path": "Engine.Generic", "fmt": {"bix": 2, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 3, Sensor 2 present"}, {"id": "O2S31_EXISTS", "path": "Engine.Generic", "fmt": {"bix": 3, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 3, Sensor 1 present"}, {"id": "O2S22_EXISTS", "path": "Engine.Generic", "fmt": {"bix": 4, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 2, Sensor 2 present"}, {"id": "O2S21_EXISTS", "path": "Engine.Generic", "fmt": {"bix": 5, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 2, Sensor 1 present"}, {"id": "O2S12_EXISTS", "path": "Engine.Generic", "fmt": {"bix": 6, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 1, Sensor 2 present"}, {"id": "O2S11_EXISTS", "path": "Engine.Generic", "fmt": {"bix": 7, "len": 1, "max": 1, "unit": "noyes"}, "name": "02S Bank 1, Sensor 1 present"}]
```

### 01 24

```json
[{"id": "LAMBDA11_VOLT", "path": "Engine.Generic.OxygenSensors", "fmt": {"len": 16, "max": 1.99, "div": 32768, "unit": "scalar"}, "name": "Lambda value, Equivalence Ratio Bank 1, Sensor 1", "suggestedMetric": "o2Lambda"}, {"id": "O2S11_VOLT", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 16, "len": 16, "max": 7.999, "div": 8192, "unit": "volts"}, "name": "Wide Range O2S Voltage, O2 Sensor Bank 1, Sensor 1"}]
```

### 01 28

```json
[{"id": "LAMBDA21_VOLT", "path": "Engine.Generic.OxygenSensors", "fmt": {"len": 16, "max": 1.99, "div": 32768, "unit": "scalar"}, "name": "Lambda value, Equivalence Ratio Bank 2, Sensor 1", "description": "Bank 2, sensor 1 on engines that report sensor locations with 01 13; bank 3, sensor 1 with 01 1D."}, {"id": "O2S21_VOLT", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 16, "len": 16, "max": 7.999, "div": 8192, "unit": "volts"}, "name": "Wide Range O2S Voltage, O2 Sensor Bank 2, Sensor 1"}]
```

### 01 2C

```json
[{"id": "EGR_PCT", "path": "Engine.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Commanded EGR"}]
```

### 01 2D

```json
[{"id": "EGR_ERR", "path": "Engine.Generic", "fmt": {"len": 8, "max": 99.22, "min": -100, "mul": 100, "div": 128, "add": -100, "unit": "percent"}, "name": "EGR error"}]
```

### 01 2E

```json
[{"id": "EVAP_PCT", "path": "Engine.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Commanded evaporative purge"}]
```

### 01 32

```json
[{"id": "EVAP_VP", "path": "Engine.Generic", "fmt": {"len": 16, "max": 8191.75, "min": -8192, "div": 4000, "sign": true, "unit": "kilopascal"}, "name": "Evap system vapor pressure", "description": "The pressure signal is normally obtained from a sensor located in the fuel tank or a sensor in an evaporative system vapor line."}]
```

### 01 34

```json
[{"id": "LAMBDA11_CURRENT", "path": "Engine.Generic.OxygenSensors", "fmt": {"len": 16, "max": 1.99, "div": 32768, "unit": "scalar"}, "name": "Lambda value, Equivalence Ratio Bank 1, Sensor 1", "suggestedMetric": "o2Lambda"}, {"id": "O2S11_CURRENT", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 16, "len": 16, "max": 127.996, "min": -128, "div": 256, "add": -128, "unit": "milliamps"}, "name": "Wide Range O2S Current, O2 Sensor Bank 1, Sensor 1"}]
```

### 01 38

```json
[{"id": "LAMBDA21_CURRENT", "path": "Engine.Generic.OxygenSensors", "fmt": {"len": 16, "max": 1.99, "div": 32768, "unit": "scalar"}, "name": "Lambda value, Equivalence Ratio Bank 2, Sensor 1", "description": "Bank 2, sensor 1 on engines that report sensor locations with 01 13; bank 3, sensor 1 with 01 1D."}, {"id": "O2S21_CURRENT", "path": "Engine.Generic.OxygenSensors", "fmt": {"bix": 16, "len": 16, "max": 127.996, "min": -128, "div": 256, "add": -128, "unit": "milliamps"}, "name": "Wide Range O2S Current, O2 Sensor Bank 2, Sensor 1"}]
```

### 01 3C

```json
[{"id": "CATEMP11", "path": "Engine.Generic", "fmt": {"len": 16, "max": 6513.5, "min": -40, "div": 10, "add": -40, "unit": "celsius"}, "name": "Catalyst temperature bank 1, sensor 1"}]
```

### 01 3D

```json
[{"id": "CATEMP21", "path": "Engine.Generic", "fmt": {"len": 16, "max": 6513.5, "min": -40, "div": 10, "add": -40, "unit": "celsius"}, "name": "Catalyst temperature bank 2, sensor 1"}]
```

### 01 3E

```json
[{"id": "CATEMP12", "path": "Engine.Generic", "fmt": {"len": 16, "max": 6513.5, "min": -40, "div": 10, "add": -40, "unit": "celsius"}, "name": "Catalyst temperature bank 1, sensor 2"}]
```

### 01 3F

```json
[{"id": "CATEMP22", "path": "Engine.Generic", "fmt": {"len": 16, "max": 6513.5, "min": -40, "div": 10, "add": -40, "unit": "celsius"}, "name": "Catalyst temperature bank 2, sensor 2"}]
```

### 01 43

```json
[{"id": "LOAD_ABS", "path": "Engine.Generic", "fmt": {"len": 16, "max": 25700, "mul": 100, "div": 255, "unit": "percent"}, "name": "Absolute load value", "suggestedMetric": "absoluteEngineLoad"}]
```

### 01 44

```json
[{"id": "EQ_RAT", "path": "Engine.Generic", "fmt": {"len": 16, "max": 1.99, "div": 32768, "unit": "scalar"}, "name": "Commanded equivalence ratio", "suggestedMetric": "commandedLambda", "description": "Commanded fuel/air equivalence ratio (lambda): actual A/F ratio over stoichiometric A/F ratio, so above 1 is lean and below 1 is rich. Multiply by the fuel's stoichiometric A/F ratio for the commanded A/F ratio (gasoline: 14.64 x 0.95 = 13.9). Systems with conventional oxygen sensors show the commanded value in open loop and 1.000 in closed loop; systems with wide-range (linear) sensors show it in both. On engines whose banks are controlled independently, this is bank 1."}]
```

### 01 45

```json
[{"id": "TP_R", "path": "Control.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Relative throttle position", "description": "Relative or 'learned' throttle position."}]
```

### 01 47

```json
[{"id": "TP_B", "path": "Control.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Absolute throttle position B", "suggestedMetric": "throttlePosition"}]
```

### 01 48

```json
[{"id": "TP_C", "path": "Control.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Absolute throttle position C", "suggestedMetric": "throttlePosition"}]
```

### 01 49

```json
[{"id": "APP_D", "path": "Control.Pedals.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Accelerator pedal position D"}]
```

### 01 4A

```json
[{"id": "APP_E", "path": "Control.Pedals.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Accelerator pedal position E"}]
```

### 01 4B

```json
[{"id": "APP_F", "path": "Control.Pedals.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Accelerator pedal position F"}]
```

### 01 4C

```json
[{"id": "TAC_PCT", "path": "Engine.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Commanded throttle actuator control"}]
```

### 01 51

```json
[{"id": "FUEL_TYP", "path": "Engine.Generic", "name": "Type of fuel currently being used by the vehicle", "fmt": {"len": 8, "map": {"1": {"description": "Gasoline/petrol", "value": "GAS"}, "2": {"description": "Methanol", "value": "METH"}, "3": {"description": "Ethanol", "value": "ETH"}, "4": {"description": "Diesel", "value": "DSL"}, "5": {"description": "Liquefied petroleum gas", "value": "LPG"}, "6": {"description": "Compressed natural gas", "value": "CNG"}, "7": {"description": "Propane", "value": "PROP"}, "8": {"description": "Battery/electric", "value": "ELEC"}, "9": {"description": "Bi-fuel vehicle using gasoline/petrol", "value": "BI_GAS"}, "10": {"description": "Bi-fuel vehicle using methanol", "value": "BI_METH"}, "11": {"description": "Bi-fuel vehicle using ethanol", "value": "BI_ETH"}, "12": {"description": "Bi-fuel vehicle using LPG", "value": "BI_LPG"}, "13": {"description": "Bi-fuel vehicle using CNG", "value": "BI_CNG"}, "14": {"description": "Bi-fuel vehicle using propane", "value": "BI_PROP"}, "15": {"description": "Bi-fuel vehicle using battery", "value": "BI_ELEC"}, "16": {"description": "Bi-fuel vehicle using battery and combustion engine for propulsion", "value": "BI_MIX"}, "17": {"description": "Hybrid vehicle using gasoline engine for propulsion", "value": "HYB_GAS"}, "18": {"description": "Hybrid vehicle using gasoline engine on ethanol for propulsion", "value": "HYB_ETH"}, "19": {"description": "Hybrid vehicle using diesel engine for propulsion", "value": "HYB_DSL"}, "20": {"description": "Hybrid vehicle using battery for propulsion", "value": "HYB_ELEC"}, "21": {"description": "Hybrid vehicle using battery and combustion engine for propulsion", "value": "HYB_MIX"}, "22": {"description": "Hybrid vehicle in regeneration mode", "value": "HYB_REG"}, "23": {"description": "Bi-fuel vehicle using diesel", "value": "BI_DSL"}, "24": {"description": "Bi-fuel vehicle using natural gas", "value": "BI_NG"}, "25": {"description": "Bi-fuel vehicle using diesel", "value": "BI_DSL"}, "26": {"description": "Natural gas", "value": "NG"}, "27": {"description": "Dual fuel vehicle using diesel and CNG", "value": "DSL_CNG"}, "28": {"description": "Dual fuel vehicle using diesel and LNG", "value": "DSL_LNG"}, "29": {"description": "Fuel cell utilizing hydrogen", "value": "FC_H2"}, "30": {"description": "Hydrogen Internal Combustion Engine", "value": "HICE_HHO"}, "31": {"description": "Kerosene", "value": "KERO"}, "32": {"description": "Heavy Fuel Oil", "value": "HFO"}}}}]
```

### 01 52

```json
[{"id": "ALCH_PCT", "path": "Fuel.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Alcohol fuel percentage", "description": "Indicates the percentage of alcohol contained in ethanol or methanol fuels, if used. For example, ethanol fuel (E85) normally contains 85% ethanol, in which case this parameter will display 85%"}]
```

### 01 53

```json
[{"id": "EVAP_VPA", "path": "Engine.Generic", "fmt": {"len": 16, "max": 327.675, "div": 200, "unit": "kilopascal"}, "name": "Absolute evap system vapor pressure", "description": "Normally obtained from a sensor located in the fuel tank or a sensor in an evaporative system vapor line."}]
```

### 01 54

```json
[{"id": "EVAP_VP_WIDE", "path": "Engine.Generic", "fmt": {"len": 16, "max": 32768, "min": -32767, "div": 1000, "sign": true, "unit": "kilopascal"}, "name": "Evap system vapor pressure (wide)", "description": "The pressure signal is normally obtained from a sensor located in the fuel tank or a sensor in an evaporative system vapor line."}]
```

### 01 5A

```json
[{"id": "APP_R", "path": "Control.Pedals.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Accelerator pedal position (relative)"}]
```

### 01 5B

```json
[{"id": "BAT_SOC", "path": "Battery.Generic", "fmt": {"len": 8, "max": 100, "mul": 100, "div": 255, "unit": "percent"}, "name": "Hybrid battery pack remaining charge", "suggestedMetric": "stateOfCharge", "description": "The percent remaining level of charge for a battery pack used for propulsion, expressed as a percentage of total useable battery energy, commonly referred to as State Of Charge (SOC)."}]
```

### 01 61

```json
[{"id": "TQ_DD", "path": "Engine.Generic", "fmt": {"len": 8, "max": 130, "min": -125, "add": -125, "unit": "percent"}, "name": "Driver's demand engine torque"}]
```

### 01 62

```json
[{"id": "TQ_ACT", "path": "Engine.Generic", "fmt": {"len": 8, "max": 130, "min": -125, "add": -125, "unit": "percent"}, "name": "Actual engine torque"}]
```

### 01 63

```json
[{"id": "TQ_REF", "path": "Engine.Generic", "fmt": {"len": 16, "max": 65535, "unit": "newtonMeters"}, "name": "Engine reference torque"}]
```

