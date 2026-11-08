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

### Open decisions (need Aaron)
- **O1. Real-car verification.** Nothing has run on the adapter or the cars. The first sessions should
  confirm, per car: protocol and responding ECUs, whether the VIN reads, the Kona module addresses and
  whether explicit flow control (`ATFCSM1`) is right, and the rear TPMS left/right order.
- **O2. Merge `hardening` into `main`.**
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
- **O4. Oxygen-sensor numbering for cars that use PID 1D** (four banks of two) instead of PID 13: carlink
  names sensors by the PID 13 layout. None of the three cars is known to use 1D.
