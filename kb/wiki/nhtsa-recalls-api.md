---
title: NHTSA recalls API (what `carlink recalls` asks)
type: concept
created: 2026-11-15
updated: 2026-11-15
sources:
  - raw/nhtsa/recalls-api-model-names-2026-11-15.md
  - raw/nhtsa/recalls-hyundai-kona-electric-2020-2026-11-15.json
  - raw/nhtsa/recalls-honda-odyssey-2016.json
tags: [recalls, nhtsa, api]
confidence: verified
---

# NHTSA recalls API

`carlink recalls --vehicle <id>` asks NHTSA's public, keyless endpoint
`https://api.nhtsa.gov/recalls/recallsByVehicle?make=<make>&model=<model>&modelYear=<year>` and merges
the reply with the profile's own recall list ([[carlink-cli]]).

## What it answers, and what it does not

- It lists the recalls filed for a **make, model and model year**. It does **not** say whether a recall
  applies to one VIN (build-date ranges are only in the `Summary` text) or whether the repair was done.
  That status comes from the manufacturer: nhtsa.gov/recalls (VIN search, web only) or a dealer.
  ([[kona-ev-recalls]] § How to check a specific VIN.)
- The VIN endpoints (`recalls/recallsByVin`, `vehicles/byVin`) answer HTTP 403 "Missing
  Authentication Token" without a key (probed 2026-11-15). carlink never sends the VIN.

## The model name must be NHTSA's

A model name NHTSA does not use returns `Count: 0` with "Results returned successfully", the same
reply as a car with no recalls. Probed 2026-11-15: `clarity` → 0, `clarity plug-in hybrid` → 1;
`kona ev` → 0, `kona electric` → 4. So each profile pins NHTSA's exact name (`Profile::nhtsa`), and
`carlink recalls` warns when a campaign in carlink's own list is missing from the reply (`carlink_only`).
NHTSA's names for a make and year: `api.nhtsa.gov/products/vehicle/models?modelYear=<y>&make=<m>&issueType=r`.

| Profile | make / model / year |
|---|---|
| crv-2004 | honda / cr-v / 2004 (13 recalls, 9 of them Takata campaigns carlink lists only generically) |
| odyssey-2016 | honda / odyssey / 2016 (6) |
| clarity-phev-2018 | honda / clarity plug-in hybrid / 2018 (1) |
| kona-ev-2020 | hyundai / kona electric / 2020 (4) |

## Fields carlink uses

`NHTSACampaignNumber` (`21V127000`, shown as `21V-127`), `ReportReceivedDate` (dd/mm/yyyy, shown as
yyyy-mm-dd), `Component`, `Summary`, `Consequence`, `Remedy`, and the flags `parkIt` (do not drive),
`parkOutSide` (park outside, away from buildings) and `overTheAirUpdate`. In the 2020 Kona Electric
reply, 21V-127 and 20V-630 carry `parkOutSide: true`; carlink turns each such flag into a warning.

## Related
[[kona-ev-recalls]] · [[carlink-cli]] · [[crv-2004]] · [[clarity-phev-2018]]
