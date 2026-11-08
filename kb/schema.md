# Schema

## Domain
Diagnosing Aaron's vehicles through an **OBDLink EX** (ScanTool.net, STN-chip USB OBD-II
adapter) and the `carlink` CLI (`~/carlink`). Covers the adapter and its ELM327/STN command
sets, the OBD-II/UDS protocols, and the three vehicles: 2004 Honda CR-V, 2016 Honda Odyssey,
2020 Hyundai Kona Electric. Agents read this wiki when interpreting a `carlink` scan.

This wiki lives inside the carlink repo (`kb/`), so it is versioned with the code it describes:
changes are committed in `~/carlink`, not in the `~/wikikb` home.

## Page types

| Type | Purpose | Example |
|------|---------|---------|
| source-summary | Summary of a single raw source | One per saved datasheet / spec page |
| entity | A device or vehicle | `obdlink-ex`, `kona-electric-2020` |
| concept | A protocol, command set, data item or procedure | `elm327-at-commands`, `readiness-monitors` |
| procedure | A step-by-step reset/relearn or diagnostic procedure | `odyssey-2016-tpms-calibration` |
| comparison | Side-by-side analysis | |
| analysis | Deep dive or synthesis on a question, incl. findings from a scan | |
| overview / index / log / stub | As in the wikikb skill | |

## Frontmatter fields

Required on every page: title, type, created, updated.
Optional: sources (`raw/…` or `url:…`), tags, confidence (`verified` = from an
authoritative doc or observed on the car; `community` = forum/GitHub consensus;
`unverified` = single source or inference).

## Index categories

- Devices
- Vehicles
- Protocols & commands
- Procedures
- Sources
- Analyses

## Conventions

- Use [[wikilinks]]; page names unique across `wiki/`.
- Every byte offset, PID formula or procedure step cites its source and carries a `confidence`.
- Anything observed on the real cars via `carlink` beats a web source; record it and flag
  the contradiction rather than overwriting.
- Never write procedures that write to an ECU (coding, flashing, routine control) as
  something `carlink` does; document them as dealer/HDS/GDS tasks.
