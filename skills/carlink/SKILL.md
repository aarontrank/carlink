---
name: carlink
description: Diagnose cars through an OBDLink EX adapter with the `carlink` command — read and explain trouble codes and warning lights, check live data, misfires and battery health, compare with earlier visits, clear codes after a repair and verify the fix, and walk a person through resets and relearns (oil life / Maintenance Minder, TPMS, idle learn). In-depth support for the 2004 Honda CR-V, 2016 Honda Odyssey and 2020 Hyundai Kona Electric; basic support for any other OBD-II car. Use when someone asks to scan or diagnose a car, explain a check-engine or other warning light or a trouble code, check the 12 V or EV battery, clear or reset codes, reset a maintenance or TPMS light, or check whether a repair worked.
---

# carlink — diagnose and reset cars with an OBDLink EX

`carlink` talks to a car through an **OBDLink EX**: a USB adapter that plugs into the car's diagnostic
(OBD) port. Every command prints **one JSON object**. You run the commands; the person you are helping
(a technician or the car's owner) does the physical steps and makes the decisions.

## The rules — never break these

1. **Use only the commands in this skill** (or `carlink --help`). Never talk to the adapter any other
   way and never run other OBD programs.
2. **Never clear codes without a clear "yes" from the person, in this conversation, for this car and
   for each system you will clear,** after you told them what clearing does (step 6).
3. **Safety first.** If any finding has `"severity": "critical"`, say it first, in one plain sentence.
   If nothing is critical, say "nothing critical was found" — do not promise it is "safe to drive".
4. **Report what the data shows; do not invent.** If carlink has no description for a code, say so.
   If carlink could not read something, say it could not — never "no faults".
5. **When a command fails, do what its `what_to_do` says.** Retry the same command at most twice
   (running it again after the person fixed what `what_to_do` asked for is not a retry).
   **Never run `carlink clear` again because something went wrong** — read `clear_status` (step 6).
6. **Simulator = practice.** If any output has `"simulated": true` or a headline starting
   `SIMULATOR —`, begin **every** message to the person — instructions like "turn the ignition on"
   included — with **"Practice data (simulator), not your car:"**.

## What do you want to do? Start here

| The person asks… | Go to |
|---|---|
| "Check engine light is on", "what's wrong with my car", "scan it" | steps 1 → 2 → 3 → 4 |
| "What does code P0xxx mean?" (no car needed) | `carlink dtc <CODE> --vehicle <id>` |
| "Is the battery OK?" (Kona: big battery or 12 V; others: 12 V) | steps 1 → 3, then read the findings and `extended` |
| "Is it misfiring / which cylinder?" | step 5, `carlink tests` |
| "Clear / reset the codes or the light" | step 6 (only after a repair, with a yes) |
| "Did the repair work?" / "Will it pass inspection?" | step 3, then `inspection` and step 6.4 |
| "Reset the oil light / Maintenance Minder / TPMS light", "idle relearn" | step 7 |
| "What changed since last time?" | the History section |

## Step 1 — Check the setup

Ask the person to plug the OBDLink EX into this computer's USB port and into the car's OBD port (under
the dashboard, driver's side), and to turn the ignition **ON**. For the Kona: press start with a foot
on the brake until **READY** shows. Then run:

```
carlink probe
```

- `"ok": true` → go on. Note `vin` and `vehicle.id` if they are there. A failed `vin` check under
  `"ok": true` only means the car does not report its VIN (normal on the CR-V and the Kona): ask for it
  in step 2.
- `"ok": false` → tell the person the `what_to_do` of the failed check, wait, run `carlink probe` again.

## Step 2 — Find out about the car and the problem

Ask, unless you already know:
- **Which car.** Profile ids: `crv-2004`, `odyssey-2016`, `kona-ev-2020`. Any other car: leave out
  `--vehicle` (generic OBD-II only).
- **The VIN**: 17 characters on the windshield (driver's side, bottom corner) or the driver's door
  jamb. carlink needs it to save history and to clear codes. The CR-V and the Kona do not report it
  themselves, so for them you **must** ask. If `probe` showed a `vin`, use that.
- **If the person's VIN and the car's `vin` differ, STOP.** The adapter is in a different car than
  they think. Tell them, and do not scan or clear anything until they confirm which car it is in.
- **The mileage** on the dashboard (if they don't know it, leave out `--odometer-mi`; the Kona reports its own).
- **The complaint**: what they noticed, which lights are on, when it happens.

## Step 3 — Scan

```
carlink scan --vehicle <id> --vin <VIN> --odometer-mi <miles> --note "<complaint in a few words>"
```

Leave out `--vehicle` for a car without a profile. Example:
`carlink scan --vehicle odyssey-2016 --vin 5FNRL5H60GB000001 --odometer-mi 98000 --note "CEL on, rough idle"`

Read the output **in this order**:

| field | what it tells you |
|---|---|
| `summary.headline` | one-line result. "code read INCOMPLETE" = some codes may be missing: scan again |
| `findings[]` | problems, most serious first: `severity`, `title`, `detail`, `recommendation` |
| `codes[]` | **engine/emissions codes only**: `code`, `description` (null = no description), `kinds` (`stored` = confirmed, `pending` = seen once, `permanent` = only clears itself after a fix), `vehicle_note` (usual cause on this car) |
| `codes_complete` | `false` = some codes could not be read: say the list may be incomplete and scan again |
| `inspection` | `likely_pass` and `reasons` for an emissions inspection (null on the Kona) |
| `tests.misfire[]` | misfire counts per cylinder (`current` = this drive, `average` = recent drives; `cylinder: 0` = all cylinders together). 0 is normal |
| `modules[]` | (Kona) **brake, airbag, body and other module codes — they are only here, never in `codes[]`**: `name`, `description`, `dtcs[]` with `state` (`active` = failing now, `stored`, `pending`, `history`); `refused` = carlink could not read that module |
| `extended` | (Kona) battery and car data: `soh`, `soc_display`, `cell_delta_v`, `aux_12v`, `tire_*_psi`, `odometer_mi`… (empty on other cars) |
| `vehicle.recalls`, `vehicle.notes` | recalls to check by VIN at nhtsa.gov/recalls, and what carlink cannot reach on this car |
| `session_id` | the saved scan (null = not saved, usually because there was no VIN) |

Severity: `critical` — unsafe, stop driving; `warning` — needs repair; `advice` — watch it or do it
soon; `info` — for the record.

## Step 4 — Explain it to the person

Most important first, plain words, US units (°F, miles, psi — carlink already gives °F and miles in
the finding text). Shape:

> **2016 Honda Odyssey — 2 problems to fix, 1 to watch.**
> 1. **Misfire on cylinder 3 (P0303)** — the engine skips on one cylinder; counts show cylinders 1–3
>    misfiring, the usual VCM spark-plug fouling on this van. Honda's warranty extension 25-061 covers
>    this repair for 10 years from the van's original purchase or 150,000 miles; if you don't know the
>    first-sale date, ask a Honda dealer to check the VIN before paying. Is the check-engine light
>    steady or flashing? If it flashes, stop driving.
> 2. …
>
> It would not pass an emissions inspection right now (check-engine light on). Recall to check:
> 23V858 (fuel pump).

- Explain codes from `description`, `vehicle_note` and the findings. If both are null: "this is a
  manufacturer-specific code; it needs the factory service information".
- carlink cannot see whether the check-engine light is **flashing**: with any misfire code, ask.
- Kona battery: report `soh` (state of health). `cell_delta_v` is only a health check when
  `soc_display` is 80 % or more; below that, say the cell check needs a charge to 80 % first.
- Never recommend clearing codes as a fix. Clearing only turns the light off.

## Step 5 — Dig deeper when needed

- **A code in detail:** `carlink dtc <CODE> --vehicle <id>` (read it character by character: the
  second character is always a digit 0–3, never the letter O).
- **Misfires and self-tests:** `carlink tests --vehicle <id>` → `misfire[]` per cylinder and
  `results[]` with `verdict` `pass` / `FAIL` / `not_run`. Read it **before** clearing (a clear zeroes
  it). On the CR-V there are no misfire counts, only Honda's raw tests.
- **Live data over time:** `carlink watch --vehicle <id> --keys <k1,k2,…> --samples <n> --interval-ms 500`
  → `stats.<key>` with `min`, `max`, `avg`, `last`. Duration = samples × interval (max 600 samples).
  - Before you judge a reading taken "at 2,500 rpm", check `stats.rpm`: if it never got near 2,500,
    the reading does not count — ask the person to hold the speed steady and take it again.
  - Lean/rich codes (P0171/P0174/P0172/P0175): `stft_b1,ltft_b1,stft_b2,ltft_b2,rpm` at idle, then
    while the person holds 2,500 rpm. High at idle but near 0 at 2,500 → vacuum leak. High under load →
    fuel delivery.
  - Charging: `module_voltage,rpm`, engine running, then with headlights and blower on: should reach
    13.5 V or more.
  - Catalyst (P0420): `o2_b1s2_v,rpm` while the person holds 2,500 rpm (engine warm): a steady
    downstream reading around 0.6–0.7 V is a good catalyst; swinging like the upstream sensor is a worn one.
  - Kona battery: `--vehicle kona-ev-2020` with no `--keys`.
  - A wrong key is refused with the list of valid keys.

## Step 6 — After the repair: clear the codes and check the fix

**If they want codes cleared before any repair:** tell them the light will come back if the fault is
still there, and that clearing erases information a technician needs. Clear only if they still say yes
after hearing that, and say so in `--note`.

**6.1 Ask first.** Tell the person: *"Clearing turns off the warning light and erases the stored
details of the fault. The car's self-tests restart, so it would fail an emissions inspection until it
has been driven normally for a few days. If the fault is still there, the light comes back."* For a
module (Kona), also name the system: *"…and it clears the brake system's stored faults."* Ask:
**"Do you want me to clear them now?"**

**6.2 Clear.** Before you type `--confirm-clear-codes`, you must already have sent the 6.1 message and
received a yes in this conversation. The person turns the engine **OFF** and the ignition **ON**.
**Kona:** it stays as it was for the scan — READY, in Park, parking brake on. If the clear comes back
`refused` or `not_acknowledged`, the one more try (6.3) is with the car ON but **not** READY: the person
switches the car off, then — foot **off** the brake — presses the POWER button twice (the dashboard
lights up; READY does not show).

```
carlink clear --vehicle <id> --vin <VIN> --confirm-clear-codes --note "<what was repaired>"
```

- By default it clears the **engine/emissions codes only**.
- To clear a module, name it: `--only IEB` or `--only engine,IEB` (names from `modules[].name`), or
  `--only all` (engine plus every module with codes) — only after the person agreed to each system.
- **Kona:** always name the module (`--only IEB` etc.): carlink never sends the engine clear there, so
  a plain `carlink clear` sends nothing. It never clears the `BMS`, `SRS` or `SMK` modules
  (battery-campaign evidence, airbags, smart key): if someone asks for those, do not run the command —
  tell them carlink will not, and that the dealer has to handle them.
- If the person asks to clear a module that has no codes in the scan, tell them there is nothing to
  clear there.
- If it says `nothing_to_clear`, nothing was sent; read `next_steps` (it lists modules with codes).

**6.3 Read `clear_status` and do exactly what `next_steps` says.**

| `clear_status` | meaning | what you do |
|---|---|---|
| `cleared` | checked by a fresh scan: no codes left (a `permanent` code may still be listed; that is normal) | tell the person; do every `next_steps` item (Hondas: idle learn) |
| `codes_remain` | codes came straight back from a module that accepted the clear | the fault is still there: go back to diagnosis. **Do not clear again** |
| `refused` / `not_acknowledged` | a module refused or did not take the clear | check engine off / ignition on / car stopped (Kona: switch to ON, not READY — see 6.2), then you may run the clear **once** more |
| `unverified` | sent, but the result could not be checked | **do not clear again.** Wait a minute, run `carlink scan` |

**6.4 Is it fixed?** Not right after clearing. For engine codes, it is fixed when, after normal
driving, a new `carlink scan` shows the self-test that `next_steps` names as complete and the code has
not come back as pending or stored. For module codes (Kona), it is fixed when, after a few days of
normal driving, the module's code has not come back in `modules[]`. Do not say "fixed" before that. `carlink history compare <before_id> <after_id>`
shows what changed.

## Step 7 — Resets and relearns

```
carlink resets --vehicle <id>
```

Each entry has `name`, `steps`, `notes` and `carlink_can_do`. Only code clearing is done by carlink
(step 6). Everything else — oil life / Maintenance Minder, TPMS light, idle learn, crank learn,
service reminder — **the person does on the car**: give the steps **one at a time** and wait for
"done". Tell the person anything in `notes` (e.g. that a step is unverified for this car, or needs a
dealer tool).

To follow an idle learn, check rpm and temperature now and then with
`carlink watch --vehicle <id> --keys rpm,coolant_temp --samples 60 --interval-ms 1000`. A `watch`
command runs until all samples are taken: keep each one under about 90 seconds (samples × interval)
and run it again if you need more.

## History

Stored per car, by VIN.

```
carlink history vehicles                       # every car seen: its key (VIN), profile, last visit
carlink history sessions --vehicle <VIN>       # visits, newest first, with session ids
carlink history dtcs <VIN>                     # each code: first/last seen, present_in_latest
carlink history compare <old_id> <new_id>      # new codes, gone codes, readings before/after
carlink history trend <VIN> <reading>          # e.g. battery_voltage, ltft_b1, misfire_cyl3_current, soh
carlink history keys <VIN>                     # which readings can be trended
carlink history show <session_id>              # one full saved scan
```

`present_in_latest` is true only for a code that is set now (a code left only as `permanent` is
waiting for the car to re-test). Trend points carry `kind` and `engine_running`: compare like with
like (12.6 V engine off and 14 V engine running are both normal).

## When a command fails

The JSON has `error`, `error_kind` and `what_to_do`. Tell the person `what_to_do` in plain words.

| `error_kind` | usual cause |
|---|---|
| `adapter_not_found`, `adapter_disconnected`, `probe_adapter` | adapter not plugged into USB, or the cable came loose |
| `adapter_busy` | another program or carlink command is using the adapter |
| `not_an_obd_adapter` | the wrong USB device was picked: `carlink ports`, then `--port <name>` |
| `adapter_timeout`, `adapter_rejected_command`, `adapter_addressing` | adapter glitch: unplug it from USB for 5 s, retry |
| `vehicle_not_responding`, `probe_vehicle` | ignition off, or the adapter not pushed fully into the car's port |
| `probe_power_at_obd_port` | adapter on USB but not in the car (or a blown fuse) |
| `vin_required` | ask for the VIN and pass `--vin` |
| `vehicle_mismatch` | the adapter is in a different car than `--vin`/`--vehicle` say: check which car |
| `engine_running` | engine off, ignition on, then retry |
| `engine_state_unknown` | engine off; for the Kona add `--vehicle kona-ev-2020` |
| `scan_incomplete` | codes could not all be read: run `carlink scan` again first |
| `clear_not_allowed` | carlink will not clear that on this car — do not try to get around it |
| `confirmation_required` | you ran `clear` without `--confirm-clear-codes`: ask the person first |
| `unknown_vehicle`, `bad_input`, `usage`, `request_not_allowed`, `not_found` | your command was wrong: check this skill or `carlink --help` |

## The three cars

- **2004 Honda CR-V (`crv-2004`)** — old K-line protocol: connecting takes up to ~10 s. No VIN from the
  car: pass `--vin`. Only the engine computer is reachable: **ABS and airbag codes need a Honda tool**
  (or the service manual's blink-code procedure). After clearing: idle learn.
- **2016 Honda Odyssey (`odyssey-2016`)** — known for misfires on cylinders 1–3 from VCM (warranty
  extension 25-061 for vans first sold after 2016-11-07) and torque-converter judder (bulletin
  17-043). `carlink tests` shows misfire counts per cylinder. **Airbag, ABS/VSA, steering and TPMS
  codes need a Honda tool.** After clearing: idle learn, then crank pattern learn.
- **2020 Hyundai Kona Electric (`kona-ev-2020`)** — electric: no engine data. carlink reads battery
  health (`soh`, 98 cell voltages, `cell_delta_v`), the 12 V battery, tire pressures and codes from
  14 modules. No VIN from the car: pass `--vehicle kona-ev-2020 --vin <VIN>`. The airbag module's
  codes are **not** readable by carlink. Check recall **21V-127** (battery pack replacement) was done.

## What carlink cannot do

Program or code modules, register new TPMS sensors or keys, open diagnostic sessions, reset airbag
crash data, read Honda ABS/airbag/steering/TPMS modules, read the Kona airbag module, or replace the
factory tool (Honda HDS / Hyundai GDS). Say so plainly when asked.

## Words you may need to explain

- **Check-engine light / MIL** — the engine warning light.
- **Trouble code (DTC)** — a code like P0303 that a computer stores when it detects a fault.
- **Pending** — seen once, not confirmed. **Stored** — confirmed. **Permanent** — stays until the car
  verifies the repair itself.
- **Freeze frame** — engine conditions when a code was stored.
- **Readiness monitors / self-tests** — the car's own tests; inspections need them complete.
- **Fuel trim** — how much fuel the computer adds (+) or removes (−); within ±10 % is normal.
- **Misfire count** — times a cylinder failed to fire; 0 is normal, hundreds in one drive is a fault.
- **SOH (Kona)** — battery state of health, 100 % = like new. **Cell spread** — highest minus lowest
  cell voltage; ≤ 0.02 V is healthy (measured at ≥ 80 % charge).

## Practice without a car

`--port sim:odyssey-2016` (also `sim:odyssey-2016:can29`, `sim:crv-2004`, `sim:kona-ev-2020`); add
`:engine-off`, `:ignition-off` or `:not-in-car` to rehearse those situations (they combine, e.g.
`sim:odyssey-2016:can29:engine-off`). Practice runs are saved
to a separate practice history. Each command starts the simulated car fresh, so codes you cleared are
back on the next command, and its engine speed never changes — that is the simulator, not the car. Never present simulator output as the
person's car (rule 6).

## If the real car disagrees with this skill

Do not edit carlink or its knowledge base during a job. Tell the person what differed (e.g. "the van
answered on a different address", "this code meant X here") so the maintainer can record it.
