# dtc-generic.csv — source and license

- **Source:** OBDex, https://github.com/foerbsnavi/obdex
  (bundle fetched from https://foerbsnavi.github.io/OBDex/generic.min.json on 2026-11-07;
  repository HEAD at the time: `9ba1ffa095f3140d9e21b985b322ddea3134bb60`)
- **License of the data:** CC0 1.0 Universal (public domain dedication), per the repository's
  `LICENSE-DATA`: https://creativecommons.org/publicdomain/zero/1.0/legalcode
  Attribution is not required; it is given here anyway. (OBDex's tooling is MIT, `LICENSE-CODE`;
  none of its code is included here.)
- **What was taken:** `code` and the English `title` (`title.en`) of each of the 9,533 generic
  SAE J2012 codes (P0, P2, P3, U0, U3, C0, B0). Whitespace collapsed; rows sorted by code.
  Nothing else (German text, causes, symptoms, sources) was copied.
- **Caveats:** a generic title is the J2012 meaning. Some manufacturers reuse generic-range
  codes with their own meaning (e.g. older GM numbers wheel-speed circuits C0035–C0050 in its
  own scheme), and P1/B1/C1/U1 etc. are manufacturer-specific and are not in this file.
- **Not used:** python-OBD's code table (GPL-2.0). mytrile/obd-trouble-codes (MIT, 2014) was
  checked and rejected: older wording and some wrong entries (e.g. P0442 given as "Incorrect
  Purge Flow"; J2012 has it as EVAP small leak).
