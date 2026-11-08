# carlink

Rust CLI for the OBDLink EX; consumers are AI agents via `skills/carlink/SKILL.md`. Read README.md.

- Facts about adapters, protocols and vehicles live in the LLM wiki under `kb/` (pages in `kb/wiki/`,
  conventions in `kb/schema.md`, immutable sources in `kb/raw/`); cite the
  page in code when a byte offset or threshold comes from it, and update the page when the real car
  disagrees.
- `src/actions.rs` is the only place that sends anything beyond the read sweep. Keep `raw` an
  allowlist; never add AT/ST passthrough (wiki `adapter-safety` lists commands that brick or
  permanently reconfigure the adapter).
- Every vehicle quirk gets simulator data in `src/sim.rs` so tests cover it without hardware.
- TDD: write the failing test first; `cargo test` must pass before committing.
