# carlink

Diagnostics for an **OBDLink EX** (USB, STN chip) built for AI agents: every command prints JSON,
and the agent skill in `skills/carlink/SKILL.md` tells an agent how to run a scan, interpret it,
compare it with history and walk the owner through resets.

Vehicles with in-depth profiles: 2004 Honda CR-V, 2016 Honda Odyssey, 2020 Hyundai Kona Electric
(EV battery SOH, all 98 cell voltages, 12 V, TPMS, odometer). Any OBD-II car gets the generic scan.

```
./install.sh                         # cargo build --release, ~/.local/bin/carlink, skill symlink
carlink ports                        # find the adapter
carlink scan --vehicle odyssey-2016  # full diagnostic, saved to ~/.local/share/carlink/carlink.db
carlink history trend odyssey-2016 battery_voltage
carlink --port sim:kona-ev-2020 scan # built-in simulator, no car needed
```

Knowledge base: `~/wikikb/vehicle-diagnostics/` (adapter command set, OBD-II decoding, per-car
byte maps, recalls, reset procedures, with sources).

## Safety model

- Reads are a fixed sweep; `raw` accepts only read services (01 02 03 06 07 09 0A 22).
- The only write is `clear` (service 04): it needs `--confirm-clear-codes`, refuses while the engine
  runs, and saves a full scan first.
- No AT/ST adapter configuration is ever sent beyond the session init; some of those commands
  permanently alter the adapter (wiki: `adapter-safety`).
- Empty commands are refused: a bare CR makes the adapter repeat its previous command.

## Development

`cargo test` runs everything against the simulator (`src/sim.rs`), including the CLI end to end.
There is no CI; run `cargo test` before committing.
