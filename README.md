# carlink

Diagnostics through an **OBDLink EX** (USB, STN chip) for AI agents: every command prints one JSON
object, and the agent skill in `skills/carlink/SKILL.md` tells an agent — even a weak one — how to
diagnose a car in a shop or at home, explain it to a person, clear codes after a repair, verify the
fix, and walk the person through resets.

In-depth support: **2004 Honda CR-V**, **2016 Honda Odyssey**, **2020 Hyundai Kona Electric** (battery
health with all 98 cells, 12 V, tires, and trouble codes from 10 other modules). Any OBD-II car gets
the generic diagnosis.

```
./install.sh                                   # build, install ~/.local/bin/carlink, link the skill
carlink probe                                  # setup checklist: adapter, power at the port, car, VIN
carlink scan --vehicle odyssey-2016 --vin <VIN> --odometer-mi 98000
carlink tests --vehicle odyssey-2016           # self-tests and per-cylinder misfire counts
carlink watch --keys stft_b1,ltft_b1,rpm --samples 20
carlink clear --vehicle odyssey-2016 --vin <VIN> --confirm-clear-codes
carlink history vehicles                       # then: history dtcs|trend|compare <key> …
carlink --port sim:kona-ev-2020 scan           # built-in simulator, no car needed
```

Every failure is JSON with `error`, `error_kind` and `what_to_do`. History is kept per VIN in
`~/.local/share/carlink/carlink.db` (simulator runs go to `simulator.db`).

## Knowledge base

`kb/` is an LLM wiki (`kb/schema.md`, pages in `kb/wiki/`, sources in `kb/raw/`): the adapter command
set, OBD-II and UDS decoding, service 06, live-data interpretation, per-car byte maps, recalls and reset
procedures. Code cites the page a byte offset or threshold comes from.

## Safety model

- Reads are fixed sweeps; `raw` accepts only read services (01 02 03 06 07 09 0A 18 19 21 22).
- The only writes are in `clear`: OBD service 04 for engine/emissions codes, UDS/KWP service 14 for a
  module that is named. `clear` needs `--confirm-clear-codes` and a VIN; it refuses when the car reports
  an engine running, when the connected car contradicts `--vin`/`--vehicle`, or when the code read was
  incomplete. It saves a scan first, sends each request once (never retried), and judges the outcome only
  from a fresh scan.
- No AT/ST adapter configuration is sent beyond the session setup; some of those commands permanently
  alter the adapter (wiki `adapter-safety`). A bare CR is never sent (it repeats the last command).

## Development

`cargo test` runs everything against the simulator (`src/sim.rs`), including the CLI end to end; the
simulator can inject adapter faults and odd replies. There is no CI: run `cargo test` and
`cargo clippy --all-targets` before committing. Decisions are logged in `DECISIONS.md`; the review
findings of the 2026-11-08 hardening pass are in `docs/review-2026-11-08.md`.
