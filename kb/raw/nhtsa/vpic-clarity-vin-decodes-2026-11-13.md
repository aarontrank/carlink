---
title: NHTSA vPIC decodes of Honda Clarity VIN patterns (2017-2020)
source_url: https://vpic.nhtsa.dot.gov/api/vehicles/DecodeVinValues/<VIN>?format=json
fetched: 2026-11-13
---

# NHTSA vPIC decodes — Honda Clarity

Raw JSON saved under (a session-scratchpad copy, not kept; see source_url), (a session-scratchpad copy, not kept; see source_url), (a session-scratchpad copy, not kept; see source_url) (one file per query).

## Partial-VIN decodes (8 chars + `*` + year + plant), verbatim fields

| Query | Model | ModelYear | Trim | FuelTypePrimary | FuelTypeSecondary | ElectrificationLevel | DisplacementL |
|---|---|---|---|---|---|---|---|
| `JHMZC5F1*JC` | Clarity | 2018 | PHEV | Electric | Gasoline | PHEV (Plug-in Hybrid Electric Vehicle) | 1.5 |
| `JHMZC5F3*JC` | Clarity | 2018 | TOURING | Electric | Gasoline | PHEV (Plug-in Hybrid Electric Vehicle) | 1.5 |
| `JHMZC5F1*KC` | Clarity | 2019 | PLUG-IN HYBRID PHEV | Electric | Gasoline | PHEV | 1.5 |
| `JHMZC5F3*KC` | Clarity | 2019 | PLUG-IN HYBRID TOURING | Electric | Gasoline | PHEV | 1.5 |
| `JHMZC5F1*LC` | Clarity | 2020 | PHEV | Electric | Gasoline | PHEV | 1.5 |
| `JHMZC5F3*LC` | Clarity | 2020 | Touring | Electric | Gasoline | PHEV | 1.5 |
| `JHMZC6F3*HC` | Clarity | 2017 | Electric Touring | Electric | (blank) | BEV (Battery Electric Vehicle) | (blank) |
| `JHMZC6F3*JC` | Clarity | 2018 | Electric Touring | Electric | (blank) | BEV (Battery Electric Vehicle) | (blank) |
| `JHMZC6F3*KC` | Clarity | 2019 | Electric Touring | Electric | (blank) | BEV | (blank) |
| `JHMZC6F3*LC` | Clarity | 2020 | ELECTRIC Touring | Electric | (blank) | BEV | (blank) |

Queries that returned no model (`ErrorCode 6,14 – "Unable to provide information for some of the
characters in the VIN, based on the manufacturer submission"`): `JHMZC5F2*JC`, `JHMZC4F1*JC`,
`JHMZC4F3*JC`, `JHMZC4F5*JC`, every `JHMZC{1,2,3,4,6}F{1,2,4,5}*{H,J,K,L}C` except those above, and
every `JHMZC{1..9}{A..N excl F}{1,3,5}*JC`.
**No Clarity Fuel Cell pattern was found in vPIC** with these probes.

## Full-VIN decodes (real auction VINs, check digit verified by vPIC)

`JHMZC5F3XJC******` (bid.cars lists as 2018 Clarity Plug-In Hybrid Touring) — verbatim non-empty fields:
```
'ErrorText': '0 - VIN decoded clean. Check Digit (9th position) is correct'
'Model': 'Clarity', 'ModelYear': '2018', 'Trim': 'TOURING', 'VehicleDescriptor': 'JHMZC5F3*JC'
'FuelTypePrimary': 'Electric', 'FuelTypeSecondary': 'Gasoline',
'ElectrificationLevel': 'PHEV (Plug-in Hybrid Electric Vehicle)'
'EngineModel': 'LEB3(H4)', 'DisplacementL': '1.5', 'EngineCylinders': '4', 'EngineHP': '103'
'TPMS': 'Indirect'
'TransmissionStyle': 'Continuously Variable Transmission (CVT)'
'PlantCity': 'SAYAMA', 'PlantState': 'SAITAMA', 'PlantCountry': 'JAPAN'
'Note': 'Curb Weight: 4,052 to 4,059 lbs'
```
`JHMZC5F37JC******` — same fields, Trim 'TOURING', VehicleDescriptor 'JHMZC5F3*JC', TPMS 'Indirect'.
`JHMZC5F11JC******` — Trim 'PHEV', VehicleDescriptor 'JHMZC5F1*JC', TPMS 'Indirect', 'DriveType': '4x2'.

## Summary
- 2018 Clarity PHEV = `JHMZC5F` + trim digit (pos 8) + check digit (pos 9) + `J` (2018) + `C` (Sayama plant).
- Position 8: `1` = base ("PHEV"), `3` = Touring. Model year letter J/K/L = 2018/2019/2020.
- Clarity Electric (BEV) = `JHMZC6F3`; it does not collide with `JHMZC5`.
- Fuel Cell pattern not found in vPIC (see honda-erg file: Honda's own ERG says ZC5 = PHEV).
- vPIC lists TPMS = Indirect for both trims.
