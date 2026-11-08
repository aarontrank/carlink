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
