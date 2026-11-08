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
