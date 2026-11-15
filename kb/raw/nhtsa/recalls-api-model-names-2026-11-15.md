---
title: NHTSA recallsByVehicle — model-name probes and VIN endpoints
fetched: 2026-11-15
source_url: https://api.nhtsa.gov/recalls/recallsByVehicle
---

Probes run 2026-11-15 with curl (no API key). `Count` and campaign numbers returned:

| Query (make / model / modelYear) | Count | Campaigns |
|---|---|---|
| honda / clarity / 2018 | 0 | — |
| honda / clarity plug-in hybrid / 2018 | 1 | 23V858000 |
| honda / cr-v / 2004 | 13 | 19V182000 20V768000 19V501000 19V499000 18V268000 15V320000 14V700000 15V370000 14V353000 12V486000 12V136000 07E104000 04V255000 |
| honda / odyssey / 2016 | 6 | 18V170000 17V725000 16V932000 16V933000 16V417000 23V858000 |
| hyundai / kona electric / 2020 | 4 | 21V127000 20V630000 20V748000 26V728000 |
| hyundai / kona ev / 2020 | 0 | — |

`https://api.nhtsa.gov/products/vehicle/models?modelYear=2020&make=hyundai&issueType=r` lists NHTSA's
model names (e.g. `KONA`, `KONA ELECTRIC`).

VIN-specific endpoints tried: `https://api.nhtsa.gov/recalls/recallsByVin?vin=…` and
`https://api.nhtsa.gov/vehicles/byVin?vin=…` both answered HTTP 403 `{"message":"Missing Authentication Token"}`.

Result fields per recall: Manufacturer, NHTSACampaignNumber, parkIt, parkOutSide, overTheAirUpdate,
ReportReceivedDate (dd/mm/yyyy), Component, Summary, Consequence, Remedy, Notes, ModelYear, Make, Model.
In the 2020 Kona Electric reply, 21V127000 and 20V630000 have parkOutSide true.
