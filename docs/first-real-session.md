# First real session on each car

Nothing in carlink has run on the real adapter or cars yet (DECISIONS O1). This is the checklist for the
first session on each car. Add `--trace <file>` to every command: a surprise then becomes data the
simulator can learn from. A trace holds the car's VIN, so keep traces out of the repository (or remove
the VIN first). `--trace` creates the file but not its folder, so the examples write to the home folder.

Do not run `clear` to test anything. Clear only a real code, after the person agreed (skill step 6).

## Before any car

1. `./install.sh` in the repository.
2. Plug the OBDLink EX into USB and run `carlink ports`. The adapter must appear as `selected`.

## 2004 Honda CR-V (`crv-2004`), ignition ON

1. `carlink --trace ~/carlink-crv-trace.txt probe`. Expect protocol ISO 9141-2; connecting can take
   about 10 s. The car reports no VIN; that check fails normally.
2. `carlink --trace ~/carlink-crv-trace.txt scan --vehicle crv-2004 --vin <VIN>`. Expect
   `scan.ecus` to be `["10"]` only, and `codes_complete: true`.
3. In the trace, find the reply to `03`. With no stored codes, J1979 lets this car answer
   `43 00 00 00 00 00 00` or stay silent (`NO DATA`). Record which on wiki `dtc-format` ("When an ECU
   has no codes"); DECISIONS D39 handles both.
4. `carlink --trace ~/carlink-crv-trace.txt tests --vehicle crv-2004`. Check that the Honda tests
   decode (wiki `honda-mode06`).

## 2016 Honda Odyssey (`odyssey-2016`), engine running

1. `carlink --trace ~/carlink-odyssey-trace.txt probe`. Record the protocol: 11-bit CAN (`ATDPN` A6)
   or 29-bit (A7). The wiki expects 11-bit, but 2011–2015 Odysseys answered on 29-bit (DECISIONS D32).
   Fix the "Protocol" row under "Diagnostic link" on wiki `odyssey-2016` either way.
2. `carlink --trace ~/carlink-odyssey-trace.txt scan --vehicle odyssey-2016`. The van should report its
   own VIN. Record `scan.ecus`: `7E8` alone is expected (no separate transmission computer), or
   `18DAF111` / `18DAF11D` on 29-bit.
3. `carlink --trace ~/carlink-odyssey-trace.txt tests --vehicle odyssey-2016`. Expect per-cylinder
   misfire counts (wiki `honda-mode06`).

## 2020 Hyundai Kona Electric (`kona-ev-2020`), in READY

1. `carlink --trace ~/carlink-kona-trace.txt probe`. Expect 11-bit CAN. The car reports no VIN.
2. `carlink --trace ~/carlink-kona-trace.txt scan --vehicle kona-ev-2020 --vin <VIN>`, then check:
   - which ECUs answered `0100` in the trace (7EB and 7EC are expected);
   - every entry of `modules[]` with an `error` or `refused`. The airbag module (`SRS`) is expected to
     be refused. For the brake module (`IEB`), note whether it answered at all (DECISIONS O7);
   - the `soc_display` and `odometer_mi` readings in `extended` against the dashboard.
3. Rear tire order (DECISIONS O1): carlink reports the rear tires as `tire_rear_a_psi` and
   `tire_rear_b_psi` because their left/right order is unknown. Let a few psi out of the rear left
   tire, drive briefly so the sensor reports, scan again, and record which of the two dropped on wiki
   `kona-ev-other-ecus`. Refill the tire.
4. The first real module clear (only for a real code, with the person's yes, and with `--trace` on that
   clear — it is never re-run): note whether it worked
   in READY or needed the retry in ON (DECISIONS O10). Then change skill step 6.2 to name only the state
   that worked.

## 2018 Honda Clarity Plug-in Hybrid (`clarity-phev-2018`), in READY

1. `carlink --trace ~/carlink-clarity-trace.txt probe`. Expect 29-bit CAN (`ATDPN` A7) and the VIN
   from the car (`JHMZC5F…J…`).
2. `carlink --trace ~/carlink-clarity-trace.txt scan --vehicle clarity-phev-2018`, then check:
   - `scan.ecus`: OBDb's captures had seven (`18DAF101`, `02`, `06`, `0E`, `15`, `16`, `2A`). Record the
     real list on wiki `clarity-phev-2018`, and which ECU answered `0902` (the VIN) in the trace;
   - `extended`: `odometer_mi` against the dashboard, `hv_soc` against the battery gauge, and
     `hv_capacity_ah` (55 Ah new; record it — wiki `clarity-phev-hv-battery`);
   - no "Vehicle-specific data partly unavailable" finding. If there is one, the trace shows which
     29-bit request was refused or unanswered.
3. Run the scan once more with the car ON but not READY (POWER pressed twice without the brake). Record
   whether the BMS (`01`, `16`) and the meter (`60`) still answer: the clear (skill step 6.2) is done in
   that state.
4. `carlink --trace ~/carlink-clarity-trace.txt tests --vehicle clarity-phev-2018` with the engine
   having run this trip. Record whether misfire counts come back (the simulator assumes standard MIDs).

## After each car

For each surprise: correct the wiki page it contradicts (and mark the contradiction, `kb/schema.md`),
add simulator data in `src/sim.rs` with a test (`CLAUDE.md`), and close the open decision in
`DECISIONS.md`.
