# Decision log

Decisions made while building and hardening carlink, newest last, with the reason for each.
**Open decisions** that need a human are collected at the end.

## 2026-11-08 — autonomous hardening pass (started 00:11 PST, 8-hour budget)

Brief from Aaron: adversarial review of the whole codebase, list every problem, fix them all test-first,
and end with a version an AI agent can use in an auto shop on the Hondas or the Hyundai to diagnose
problems and reset errors. The skill must work even for less capable agents.

### Process
- **D1. Work on branch `hardening`, commit per phase, do not merge to `main`.** Merging stays with Aaron;
  each commit has passed `cargo test`, clippy, `code-review` and `comment-review`. `main` stays at
  `00b510c` until he merges.
- **D2. The review findings list lives in `docs/review-2026-11-08.md`.** Every problem gets an id, and the
  fix commit or the reason it was not fixed is recorded next to it.

### Protocol layer
- **D3. A lost CAN frame is an error, never shorter data.** Consecutive frames are sequence-checked and a
  message shorter than its declared length is rejected. Silent short data shifted every Kona battery
  offset (C1); an error is retried once and otherwise reported.
- **D4. After a timeout, interrupt the adapter with a space.** Any byte stops a command in progress; an idle
  ELM/STN ignores spaces, so it cannot become part of the next command. A bare CR is never sent (it repeats
  the previous command, which could be a clear).
- **D5. The adapter's addressing state lives in one place (`Adapter`).** It records when it may be pointed at a
  single module; every later request first restores normal addressing and refuses to run if it cannot. The
  clear path additionally re-asserts it right before service 04. This replaces three separate patches from
  the earlier review rounds (R2–R4, R6).
- **D6. Reads are retried once after a transient adapter error; a clear is never retried.** The set of
  read-only services is one list (`adapter::READ_SERVICES`) used both for the retry rule and for `raw`.
- **D7. The device must identify as ELM327/STN/OBDLink (`ATI`) before anything else is sent**, and every
  setup command must answer `OK`. A shop has other USB-serial equipment; carlink must not reset it.

### Data and history
- **D8. History is keyed by VIN (read from the car, else `--vin`), the profile id only as a last resort.**
  This reverses yesterday's profile-id key: in a shop, many cars of the same model pass through and must not
  share one history. A scan with no VIN gets an Info finding telling the agent to pass `--vin`.
- **D9. Every session records `codes_complete`.** "Is this code still present?" is answered only from the
  latest session whose code reads all succeeded, so a failed read can never look like "codes gone" (R9).
- **D10. Schema versioning via `PRAGMA user_version`**, with an in-place migration from the first schema.

### Clearing codes (the re-scope proposed after the 2026-11-07 review stopped)
- **D11. The outcome of a clear comes only from a fresh scan, never from the reply to service 04.**
  `clear_status` is one of `cleared` (verified: no stored/pending codes left), `codes_remain`,
  `not_acknowledged` (nobody answered and the codes are still there — safe to try once more), or
  `unverified` (could not re-scan — never clear again blindly). Once 04 has been sent the command never
  returns an error, so an agent cannot mistake a sent clear for an unsent one.
- **D12. A refused clear records nothing.** The engine-off check runs before the pre-clear scan is saved.
- **D13. An unknown vehicle is treated as having an engine**: it must positively report 0 rpm before a clear.
  An EV must be identified (VIN or `--vehicle`) to skip the rpm check.

### CLI contract for agents
- **D14. Every failure is JSON with `error`, `error_kind` (stable) and `what_to_do`**, including command-line
  usage mistakes (exit 2). A weak agent can act on `what_to_do` without interpreting the message.
- **D15. JSON keeps field order (`serde_json/preserve_order`)** so `summary` and `findings` come first and the
  raw data (cell voltages, live PIDs) last.
- **D16. The simulator takes fault modes (`sim:<car>:engine-off`, `:ignition-off`)** so an agent can rehearse
  the clear and error paths without a car.

### Phase 2 — after the independent data/clear/skill review
- **D17. `clear` clears engine/emissions codes by default; a module is cleared only when named
  (`--only IEB`) or with `--only all`.** This reverses my first design (clear every module with codes):
  the person must agree to each system by name, and some are safety systems (brakes, steering). When
  modules have codes and none were named, the output lists them and says how to clear them.
- **D18. History requires a VIN.** A scan without one (from the car or `--vin`) is reported but not
  saved, and `clear` refuses. Different cars of one model (or different unidentified cars) must never
  share a history. `--vin` must pass the North-American check digit unless `--vin-no-check` is given.
- **D19. `clear` refuses when the connected car contradicts `--vin`/`--vehicle`** (`vehicle_mismatch`):
  in a shop the adapter can be in the wrong car.
- **D20. "Codes came back" is said only about a module that confirmed the clear.** Codes still present
  in a module that stayed silent or refused are `not_acknowledged` / `refused` (one retry allowed).
- **D21. A code list is complete only if every CAN OBD ECU answered service 03 and no ECU reports more
  confirmed codes (or a lit MIL) than were read.** A `cleared` verdict is therefore never based on a lost
  reply.
- **D22. Vehicle notes are appended after safety advice, never replace it** (stop driving, flashing MIL).
- **D23. The Kona profile needs VIN position 8 = `G`** (battery-electric per NHTSA vPIC): a gasoline 2020
  Kona must not be treated as electric. The engine-off check also always asks the car for rpm.
- **D24. Simulator output is marked (`summary.simulated`, headline `SIMULATOR —`) and is saved to a
  separate `simulator.db` unless `--db` is given.**
- **D25. `present_in_latest` means set (stored/pending/active) in the latest complete session**; a code
  left only as permanent is not "present". A clear and the scan right before it count as one visit.
- **D26. Readiness verdict follows US I/M rules of thumb**: continuous monitors must be ready, at most one
  other monitor incomplete for MY2001+ (two for 1996–2000), permanent codes noted. Some states are
  stricter; the output says so.
- **D27. After a verified clear, the output names the self-test (monitor) that must complete before a
  repair may be called confirmed.**

### Phase 3 — research-backed modules, service 06, skill
- **D28. Stopping rule triggered after code-review round 3** (rounds 2 and 3 both found problems my own
  fixes had introduced). Response: the remaining items were resolved by removing logic rather than adding
  it — the "MIL lit with no stored code" heuristic and the visit de-duplication were deleted, the Kona
  12 V rule became a physical threshold (below a rested battery while the HV system is on), and
  `history compare` refuses instead of guessing. Later review rounds were limited to new code.
- **D29. Module reads follow the observed Kona behaviour, not the ISO ideal:** `19 02 08` first (accepted
  by every Kona module), then only the supported failing/pending bits (`avail & 0xAF`, never 0x10/0x40);
  KWP modules are read with `18 00 FF 00` (Hyundai rejects `18 02`), after a UDS refusal of 0x11 or 0x12;
  Hyundai's KWP status bits (0x40 failing now, 0x20 stored, 0x80 this drive), not DaimlerChrysler's.
- **D30. The Kona never gets OBD service 04 from carlink, and its BMS, airbag (SRS) and smart-key (SMK)
  modules are read-only.** On a bench Kona only the BMS acknowledged 04, and the BMS's codes are what
  Hyundai's battery campaign 9A4 uses to decide on a pack replacement. Other Kona modules are cleared one
  at a time by physical address. Service 04 stays the clear for the Hondas (it is the standard there and
  nothing beyond the PCM is reachable).
- **D31. Whatever carlink could not read is said explicitly** (module refused, module silent, airbag
  module needing a session): the skill forbids "no faults" for anything not read.
- **D32. The engine ECU is `7E8`, `18DAF110`, `18DAF111` or K-line `10`.** The 2011–2015 Odysseys answer
  generic OBD on 29-bit (`18DAF111` engine, `18DAF11D` transmission); the van's protocol is unverified,
  so both are supported and simulated (`sim:odyssey-2016:can29`).
- **D33. Service 06 is read in every CAN scan** (misfire counts must be captured before any clear, and
  are stored for trends); the K-line version (CR-V) only by `carlink tests`, since it has no misfire data.
- **D34. No carlink session ever opens a diagnostic session (10 xx), security access or routine.** The
  airbag module and possibly the IEB need one for reads; that stays the factory tool's job.
- **D35. The skill no longer tells a shop agent to edit or commit the knowledge base** during a job; it
  reports discrepancies to the person instead (review D24).

- **D36. The skill was tested by giving it to a weaker model (Haiku) on three simulator jobs** (Odyssey
  shop job, Kona battery + ABS, CR-V with the adapter not yet in the car). Every confusion it reported
  became a skill or CLI fix (review T1–T7). Agents' own mistakes that a clearer rule could prevent were
  treated as skill defects.
- **D37. Each simulator command starts the car fresh** (state is not carried between processes). Keeping
  state would need a state file; the skill says so instead.

- **D38. Review rounds stop here.** Five code-review rounds ran on this branch; after the stopping rule
  (D28), later rounds only fixed problems in new code, and the last fixes got a low-effort,
  high-confidence-only review. Residual items are recorded as open decisions O8 and O9 rather than
  patched further.

### Phase 4 — final pass on what agents read
- **D39. A K-line ECU's silence to service 03 counts as "no codes" only when its own monitor status
  (PID 01) counts zero confirmed codes** (extends D21). An ISO 9141 ECU may answer `43 00 00 …` or stay silent, and
  requiring an answer outright would mark every CR-V scan incomplete — and block its clears — if the real
  car stays silent. On CAN, silence is still always a lost reply. Basis: J1979 makes a no-code reply
  optional on ISO 9141-2/J1850 and required on CAN and KWP (wiki `dtc-format`), and the real Kona captures
  show every CAN ECU answering `43 00`. Failure direction: a K-line ECU that answers `0100` but reports no
  PID 01 count and stays silent makes every scan incomplete (no clear) — loud, never a silent "no codes".
  The rule covers every non-CAN protocol, KWP included: KWP must reply, but when the ECU's own count is
  zero, a lost reply hides no confirmed code either.
- **D40. Every `carlink --help` argument must have a description, enforced by a test.** Rule 1 of the skill
  sends agents to the help text when the skill does not cover something.
- **D41. A lost adapter link ends the command with its error; nothing partial is reported or saved.** Once
  the adapter is disconnected, or two exchanges in a row time out (each timeout is already followed by an
  interrupt and drain, so the second one — a retry or the next command — means the adapter is not
  answering), nothing more is sent,
  and `scan`, `tests` and `clear`'s scans return `adapter_disconnected` / `adapter_timeout` (with its
  `what_to_do`). Before, every remaining request waited out its own timeout — about 17 s each, over ten
  minutes for a scan, far past an agent's tool timeout — and the half-empty result was saved. A clear
  that was already sent still ends `unverified` (D11), never retried.
- **D42. Review rounds on the lost-link latch stop at round 8 (stopping rule).** Round 7's KWP finding came
  from my round-6 wiki change and round 8's K39 from my round-7 `with_header` fix: two consecutive rounds
  with fix-induced findings. The residual items (review K37–K42) are recorded, not patched. None of them
  makes carlink send a second clear or present a lost link as a clean result; at worst an error message
  names the wrong step, or the lost link is reported one request later in the same command.

### Phase 5 — rehearsal and the Kona clear
- **D43. On the Kona, codes are cleared with the car ON but not READY** (POWER pressed twice from off,
  foot off the brake): the usual "ignition on, engine off" of a DTC erase. Scanning stays in READY.
  Unverified: the module captures come from a damaged car and a bench rig, so no source shows which
  state the modules need for a clear (O10). A wrong state shows as `refused` / `not_acknowledged`, never
  as a silent clear. **Superseded by D45.**
- **D44. Simulator faults combine** (`sim:odyssey-2016:can29:engine-off`), so every clear path can be
  rehearsed. `sim::FAULTS` lists them; tests check that every listed fault is accepted and named in the
  `--port` help and in SKILL.md.
- **D45. Re-scope of D43 after the stopping rule (rounds 10 and 11 both had findings caused by my own
  fixes): the Kona is cleared in the state it was scanned in** (READY, Park, parking brake on) — the one
  state in which its modules have just answered — and only the single allowed retry after `refused` /
  `not_acknowledged` uses ON-not-READY. This removes the unverified firm rule instead of patching it:
  no power change before the first try, no dead end if one state is wrong, and the 12 V battery is
  charged during the first try. The remaining round-11 items are recorded, not patched (review K53–K57).

### Phase 6 — skill trial round 3
- **D46. Trial round 3** gave the skill to a weaker model (Haiku) on three new jobs: a Kona shop job
  (airbag and ABS lights), a 29-bit Odyssey repair check over two visits, and a CR-V owner wanting the
  light cleared before an inspection. All three cleared only after the 6.1 message and a yes, never
  touched the airbag/battery/smart-key modules, labelled practice data, and never called a repair fixed.
  Every confusion they reported became a skill or simulator fix (review T10–T17).
- **D47. Review rounds on phase 6 stop at round 14 (stopping rule).** Rounds 13 and 14 both found problems
  caused by my own fixes (K65; K67). The last of them — splitting continuous from waiting self-tests in
  carlink's confirmation step — was re-scoped by removal: the step now always requires several days of
  normal driving with none of the codes back, which is right for every kind of code. The re-scope is
  pinned by a test and a mutation run instead of a fifteenth round; the residuals are review K68.

### Phase 7 — skill trial round 4
- **D48. Trial round 4** re-ran the Kona shop job and the CR-V inspection job on the revised skill and
  added a Kona owner job (battery health, TPMS light). The round-3 fixes held in every run: practice
  prefix from the first message, the VIN read off the car, the airbag safety line, "a clear will not
  get you through the inspection", and the post-clear message. The remaining confusions were wording
  (review T18–T20), so the trials stop here.

### Phase 8 — ready for the first real session
- **D49. `--trace <file>` records the raw adapter traffic** (every request, its raw reply or error, and
  how long it took), as a transport wrapper: no protocol code changes, and a trace that cannot be
  written never stops a diagnosis. The skill tells agents to re-run a surprising command (never `clear`)
  with it. `docs/first-real-session.md` turns O1, O7 and O10 into per-car steps.
- **D50. Review rounds on `--trace` stop at round 17 (stopping rule).** Rounds 16 and 17 both found
  problems caused by my own fixes to how the trace file is opened. Re-scoped by removal: a new file is
  created `0600` in one step, an existing file is never chmod-ed, and a file that cannot be opened is
  skipped with a stderr warning — the same "a trace never stops a diagnosis" rule as a failed write.
  Residuals: review K76.

## 2026-11-13 — 2018 Honda Clarity Plug-in Hybrid (autonomous, branch `clarity-phev-2018`)

Brief from Aaron: confirm carlink works with his 2018 Honda Clarity Touring, fill any gaps, open a PR,
work independently, and log every decision for review. Nothing below has run on the real car (O11).

### What the car is
- **D51. "2018 Clarity Touring" is the Plug-in Hybrid.** The Electric and the Fuel Cell were lease-only
  and had no Touring trim; NHTSA vPIC decodes `JHMZC5F3*JC` as "2018 Clarity TOURING, PHEV" and the
  Electric as `JHMZC6F`. The profile `clarity-phev-2018` matches `JHMZC5F` + year `J` (both trims).
- **D52. The evidence is OBDb's real 2018 captures** (`OBDb/Honda-Clarity` at `20139af`,
  `kb/raw/obdb-honda-clarity-2026-11-13.md`), plus Honda service bulletin 17-093, the owner's manual and
  NHTSA data. Every byte offset carlink uses was decoded from the captures and compared with OBDb's own
  expected values (0 mismatches over 17,256 cell values and the odometer).

### Gaps found in carlink, and the fixes
- **D53. Gap: carlink would not have recognised the Clarity's engine computer.** Seven ECUs answer
  generic OBD on 29-bit CAN; the one with the emissions monitors is `18DAF10E`, which was not in
  `ENGINE_ECUS`. Without it, readings were attributed to whichever module answered first and none got
  the engine's history keys. Added after `18DAF111` (so the Odyssey and other cars are unaffected).
- **D54. The engine-off check before a clear now looks at every module's rpm, not the first reply.**
  The doc comment already promised "any car reporting rpm above 0 is refused"; with seven modules
  answering 010C, the old code checked only whichever reply came first. Fails closed.
- **D55. Physical addressing works on 29-bit CAN.** `with_header` accepts `18DAxxF1` on a 29-bit car
  (reply filter `18DAF1xx`, flow control to the module) and refuses a header of the other size before
  anything is sent. `reset_addressing` on 29-bit now sends `ATFCSM0 ATAR ATSH18DB33F1` (it was a no-op),
  so the clear path also re-asserts the functional header on the Odyssey's 29-bit variant — the safer
  direction. All commands are volatile and documented in the OBDLink FRPM; none is on the never-send list.
- **D56. Clarity extended read = BMS `01` (`22 202A` summary, `22 2028` cells), BMS `16` (same), meter
  `60` (`22 7022` odometer).** Read-only service 22, as on the Kona. The shared request helper
  (`payloads`) moved from `kona.rs` to `vehicles/mod.rs` so both cars use it.
- **D57. Cell spread is computed within each 84-cell string, the worse of the two**, never across the
  two strings: cells in series carry the same current, while the strings are read a moment apart. All
  168 cells must read 2.5–4.3 V or no spread is reported.
- **D58. New powertrain `plug_in_hybrid`.** For the clear it is treated like an engine car (it must
  positively report 0 rpm) — the engine can start by itself in READY.

### Battery health and clearing
- **D59. Capacity is judged against Honda's own numbers:** 55 Ah new and the 36.6 Ah
  warranty-replacement line (S/B 17-093, which also says the car sets no code at that level — so this
  reading is the owner's only early warning). Warning below 36.6 Ah, Advice below 44 Ah (80 %). Not
  invented: the Kona's 90/80 % SOH ladder was not reused.
- **D60. Exactly 55.0 Ah gets an Info finding, not a health verdict.** One owner reports the reading
  jumping back to 55 Ah after a code clear, and 95 of OBDb's captures sit at exactly 55.0 Ah; a new car
  also reads 55, so carlink says "not a health result yet" rather than "healthy".
- **D61. Cell-spread findings only near rest (|current| ≤ 5 A): Advice ≥ 50 mV, Warning ≥ 100 mV.**
  No Honda threshold was found; real Claritys showed up to 46 mV at unknown load, so the Advice line
  sits above anything observed. Unverified (O12).
- **D62. carlink refuses the OBD clear while P0A7F, P1DC7 or P144A is stored** (new profile field
  `kept_codes`, gated on exact codes). Owners report Honda approving warranty pack replacements on
  P0A7F + P1DC7, and the clear also resets the capacity estimate: an innocent "clear the light" could
  destroy a warranty claim, and a 2018 is at the end of its 8-year term in 2026. Same principle as the
  Kona's BMS (D30). It also refuses when the code read was incomplete (it cannot rule the codes out),
  and ignores a code left only as permanent (service 04 cannot clear those). Any other clear still works; the skill now tells the person, before a Clarity
  clear, that the capacity reading resets and the current value is saved in history.
- **D63. Clarity clears are done in ON, not READY** (POWER twice without the brake): in READY the
  engine starts by itself (owner's manual). No idle learn is listed for the Clarity — none is sourced
  and the driver cannot hold engine rpm.

### Simulator, tests and docs
- **D64. Simulator `sim:clarity-phev-2018`:** 29-bit, the seven ECUs mirroring common PIDs, the engine
  computer with the full set; BMS and meter replies are real OBDb captures chosen so their voltages
  agree. Assumptions, marked in the code: the VIN comes from `18DAF10E`, the misfire MIDs are standard,
  the car is READY with the engine running, and one pending P0456 (EVAP; matches NHTSA complaints about
  "Emission System Problem" after the fuel-pump recall) is there to practise on. New fault
  `:battery-worn` (35.2 Ah, P0A7F + P1DC7) so the refusal and the warranty finding can be rehearsed.
- **D65. The skill gained Clarity lines rather than a new section**: profile id, READY/ON instructions,
  the capacity explanation, the clear clause and refusal, practice ports.
- **D66. Skill hardening: two rounds on Haiku** (`claude -p --safe-mode`, which hid the global
  CLAUDE.md, rules and memory on Claude Code 2.1.325 — checked; every trial's `init` line showed Haiku, four
  tools, no MCP servers and no installed carlink skill). Round 1: battery-health question (owner), clear
  after a gas-cap repair (shop), "just clear it before my trade-in" with worn-battery codes (trap).
  Round 2: the three again, plus a new TPMS-after-rotation and cell-balance job. It stopped after round 2:
  the re-runs passed the items that had failed, everything new was low, and round 2 had a new job.
  Findings (T = trial):
  - T1 high: the scan printed a second, per-string capacity (20.5 Ah) that two agents set against the
    36.6 Ah pack line — one nearly told an owner the battery qualified. Fixed in the tool: carlink no
    longer reads or prints it (wiki `clarity-phev-hv-battery`). Held in round 2.
  - T2 medium: the Clarity reports its own odometer but the skill said "leave out `--odometer-mi`" only
    for the Kona. Fixed in the skill. Held.
  - T5 medium: an agent said the pack "qualifies" for replacement. The skill now says "may be eligible if
    still within the warranty; Honda decides after a dealer check; never 'qualifies'". Held.
  - T6 medium: refusing the trade-in clear, an agent invented a reason. The skill now says to give the
    error's reason and that the light returns, and nothing of its own. Held.
  - T9 medium (read-through): the glossary's "cell spread ≤ 0.02 V is healthy" was the Kona's rule and
    would have judged the Clarity's normal spread. Split per car. Held (round 2 new job).
  - Low, fixed without a re-run: P0A7F/P1DC7 note names the POWER SYSTEM light (T7); ask the first-sale
    date and state before talking coverage (T8; this one was re-run and held); say plainly when the
    warranty has ended; skip the 6.1 question when a kept code makes carlink refuse; the Clarity's TPMS
    is indirect, so carlink has nothing to read. Not changed: READY cannot be confirmed from carlink's
    output (T3), a trial calling 14.1 V "fine" unprompted (T4).
  - Not reached by any job: the Maintenance Minder and 12 V reset steps, `watch`, history trends, and the
    exactly-55.0 Ah finding (unit-tested only).
- **D67. Code review (high) of the branch: ten findings, seven fixed.** Fixed test-first: the battery-code
  guard failed open on an incomplete scan with `--only engine` and blocked clears over a permanent-only
  code (D62); an unset capacity (`FFFF` = 655.35 Ah) would have been reported — capacities outside
  0–60 Ah are dropped; the generic "engine not warming up" finding is dropped on the Clarity (its engine
  does not run whenever the car is on) and the charging findings name the DC-DC converter instead of an
  alternator; the guard's advice no longer names Honda; `raw --header` takes 29-bit headers, and a
  header of the wrong size is `bad_input` with a `what_to_do`; the first-session checklist no longer
  names the removed per-string capacity. Not fixed: carlink cannot tell READY from ON on the Clarity
  (no documented reading; the skill and profile carry it, as on the Kona); the vehicle helpers
  (`r`, `u16be`, `get`, `finding`) duplicated between `kona.rs` and `clarity.rs`, and per-vehicle hooks
  chosen by id string in `extended`/`annotate` — both existing patterns, left for a refactor of their own.
- **D67a. Review stopped at round 3 (stopping rule), re-scoped by removal.** Round 2 found two problems
  caused by my round-1 fixes (a lowercase 29-bit `--header` refused; the DC-DC rewording missed the
  engine-off case). I fixed those, plus a padding guard for the BMS summary and a rewording of the
  resting-battery findings. Round 3 then found medium problems in those round-2 additions (the guard
  half-worked and mislabelled its error; its range checks had no test that could fail; the new advice
  contradicted the generic "at rest" detail). Two rounds in a row of fix-caused findings: stopped, and
  removed both round-2 additions instead of patching them. Kept from round 2: uppercasing `--header`,
  the "stored or pending" wording (with is/are), and the integration test tying the Clarity rewording to
  the generic finding titles. Residuals (not fixed, low or out of scope): a 12 V reading that is low
  because the DC-DC converter failed in READY with the engine off is reported with the generic "12 V
  battery partly discharged / weak at rest" advice — the profile notes say 14 V in READY is normal and
  what the charging light means; a partly FF-padded BMS reply would be decoded as-is (no capture shows
  one); `7DF`/`18DAF1F1` accepted as physical headers; header validation in two places; the guard only
  for the 2018 profile (O14).
- **D68. `skill-shootout` was not run.** It needs Aaron's go-ahead on its plan and cost ceiling before
  anything runs (≈ $5 per arm, three arms, plus judging), and its arms would need `Bash` with the
  branch's `carlink` build on PATH. Offered in the PR; verdict: not run.

### Open decisions (Clarity)
- **O11. First session on the Clarity** (`docs/first-real-session.md`): confirm the seven ECUs, which
  one answers `0902`, that the BMS and meter answer 29-bit physical reads in READY and in ON, and the
  odometer against the dashboard.
- **O12. Cell-spread thresholds** (D61) need a Honda source or real data from Aaron's car.
- **O14. Battery-evidence guard beyond the 2018.** P0A7F/P1DC7/P144A are protected only when the car is
  identified as `clarity-phev-2018`; a 2019–2021 Clarity (`JHMZC5F…K/L`) or an unidentified car gets
  them cleared like any code. Widening it (to those years, or to every hybrid) is a scope decision for
  Aaron.
- **O13. Aaron's battery warranty.** A 2018 bought new in 2018 reaches 8 years / 100,000 miles during
  2026 (10 years / 150,000 miles in CARB states). Read `hv_capacity_ah` soon: below 36.6 Ah inside the
  term is a warranty replacement.

### Open decisions (need Aaron)
- **O1. Real-car verification.** Nothing has run on the adapter or the cars. The first sessions should
  confirm, per car: protocol and responding ECUs, whether the VIN reads, the Kona module addresses and
  whether explicit flow control (`ATFCSM1`) is right, and the rear TPMS left/right order.
- **O2. Merge `hardening` into `main`.** Closed 2026-11-08: Aaron asked for the merge, and `main` was
  fast-forwarded to `hardening`.
- **O3. Shop data.** The history DB now holds customer VINs. Decide on retention, export and deletion
  before using it in a shop (`history` has no delete).
- **O5. Kona modules not in carlink's table.** The captures show unnamed UDS modules (725, 733, 776,
  780, 783, 796, 7B7) and an SCC radar (7D0) on some trims. They are left out until named; adding them is
  a table edit in `src/vehicles/kona.rs`.
- **O6. Honda modules beyond the PCM.** No documented access on these two cars with an OBDLink EX
  (an MX+ reaches Honda's other network). Revisit only with evidence from the real van.
- **O7. Whether the Kona IEB answers a physical read in the default session** (the only logged read
  followed a session change). If it does not, carlink reports it as "did not answer" — never as clean.
- **O8. Engine-ECU choice on unknown 29-bit cars.** carlink treats one ECU per scan as the engine
  (7E8, then 18DAF110, then Honda's 18DAF111, then K-line 10). On a car where both 29-bit addresses
  answer and one later drops out, a history key could move between them. Fine for the three cars; revisit
  if carlink is used on other 29-bit cars.
- **O9. KWP2000 status byte meanings.** Hyundai's (0x40 failing now, 0x20 stored, 0x80 this drive) are
  applied to every KWP module; only the Kona has KWP modules today. A profile for another make that adds
  KWP modules needs its own mapping.
- **O10. The Kona's power state for a module clear.** The skill clears in READY and retries once in ON
  (D45). The first real clear should record which state the module acknowledged in, and skill step 6.2
  should then name only that one.
- **O4. Oxygen-sensor numbering for cars that use PID 1D** (four banks of two) instead of PID 13: carlink
  names sensors by the PID 13 layout. None of the three cars is known to use 1D.
