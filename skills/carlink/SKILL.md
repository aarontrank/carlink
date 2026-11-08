---
name: carlink
description: Run vehicle diagnostics through Aaron's OBDLink EX with the `carlink` CLI, then summarize findings and recommend next steps. Use when asked to scan, diagnose or check a car (2004 Honda CR-V, 2016 Honda Odyssey, 2020 Hyundai Kona Electric), explain a check-engine light or trouble code, check the 12 V or EV battery, compare against earlier scans, clear codes, or reset/relearn something (TPMS, oil life, idle).
---

# carlink — car diagnostics for an agent

`carlink` (source `~/carlink`, binary on `PATH` after `~/carlink/install.sh`) talks to an
**OBDLink EX** USB adapter and prints **JSON only**. It is built for you, not for Aaron to run.
The knowledge base behind it is the wiki at `~/wikikb/vehicle-diagnostics/` (start at
`wiki/overview.md`; per-car pages `crv-2004.md`, `odyssey-2016.md`, `kona-electric-2020.md`).

## Before connecting

1. Ask Aaron which car, and to: plug the EX into the car's OBD port (under the dash, driver side)
   and into this Mac; turn the ignition **ON** (engine running for charging/fuel-trim checks, or
   engine off + ignition ON for clearing codes; Kona: press start until **READY**).
2. `carlink ports` — `selected` must be a `/dev/cu.usbserial-…` device. None → adapter not plugged in.
3. `carlink probe` — confirms adapter, protocol, voltage and VIN. On error, read the message: it
   says what to fix (ignition off, wrong port). Do not retry blindly more than twice.

## Diagnose

```
carlink scan --vehicle <id> --odometer-mi <miles>   # saves to history; ask Aaron for the odometer
```

Vehicle ids: `crv-2004`, `odyssey-2016`, `kona-ev-2020` (`carlink vehicles`). The CR-V likely
reports no VIN, so always pass `--vehicle`. The scan returns:

- `summary.headline`, `findings[]` (sorted critical → info, each with `recommendation`)
- `scan.dtcs[]` (`stored` = confirmed, `pending` = seen once, `permanent` = survives clearing)
- `scan.monitors` (readiness), `scan.freeze_frames` (conditions when the code set),
  `scan.live` (snapshot), `extended` (vehicle-specific, e.g. Kona battery SOH and cell voltages)
- `session_id` — cite it so Aaron can come back to it.

Then **interpret, don't just relay**:
- For each code: `carlink dtc <code> --vehicle <id>` for the vehicle-specific note, and the wiki
  page for that car. Say what it means, how urgent it is (drive / drive gently / stop), the likely
  causes in order, and what to check first. Say when something needs a shop.
- Convert units for Aaron: °C→°F, km→mi, kPa→psi.
- Compare with history (below) and say what changed since the last scan.
- If a reading is outside what the wiki says is normal but no rule fired, say so.
- Distinguish what the data shows from what you infer.

## History and trends

```
carlink history sessions [--vehicle <key>]      # newest first; key = profile id (e.g. odyssey-2016)
carlink history show <session_id>               # full saved report
carlink history keys <key>                      # which readings exist
carlink history trend <key> <reading>           # e.g. battery_voltage, ltft_b1, soh (Kona)
carlink history dtcs <key>                      # each code's first/last seen, still present?
```

Use these for "is it getting worse", recurring codes, 12 V battery decline, EV battery SOH over time.
For a visual history, read the JSON and build a chart artifact — the CLI deliberately has no UI.

## Clearing codes — only with Aaron's explicit yes

`carlink clear --vehicle <id> --confirm-clear-codes [--note "<why>"]`

Before running it, tell Aaron: it turns off the check-engine light, **erases freeze-frame data and
resets readiness monitors** (the car fails an emissions inspection until they complete), and does
not fix anything — the code returns if the fault remains. Permanent codes stay until the car
self-verifies the fix. Ask for a yes in this conversation; an earlier yes does not carry over.
Always pass `--vehicle`. The command saves a full scan first, and on a gasoline car refuses unless it can
confirm 0 rpm (engine off, ignition ON). Read `clear_status` (`cleared` or `unknown`) and `problems[]`: once the request has been sent,
**never clear again** on the strength of an error — run `carlink scan` a minute later to see the state.

## Resets and relearns

`carlink resets --vehicle <id>` lists every reset/relearn procedure for that car with
`carlink_can_do`. Only clearing codes is done over OBD by carlink. Oil-life/Maintenance-Minder,
TPMS calibration and idle relearn are done **by Aaron on the car** — walk him through the steps it
returns, one at a time. Anything marked as needing a dealer tool (HDS/GDS), say so; do not attempt it.

## Advanced: raw reads

`carlink raw <hex> [--header 7E4]` sends one read-only request (services 01 02 03 06 07 09 0A 22)
and returns the frames. Everything else is refused by design — never try to get around that (no
writing to the serial port directly, no AT/ST commands: some permanently alter the adapter).

## Testing without the car

`--port sim:<vehicle-id>` runs against a built-in simulator. Use it to try commands; never present
simulator output as Aaron's car.

## Record what you learn

If a scan reveals something about the car the wiki does not have (a PID that does not answer, a
normal value range, a fix that worked), add it to the car's wiki page per the `wikikb` skill.
