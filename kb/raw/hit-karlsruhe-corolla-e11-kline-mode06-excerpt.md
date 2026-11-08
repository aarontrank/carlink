---
title: HIT Karlsruhe — Toyota Corolla E11 diagnosis over OBD (ISO 9141-2, K-line) — service 06 excerpt
source_url: http://www.hit-karlsruhe.de/aol2mime/toyota_corolla_e11_diagnose.htm
fetched: 2026-11-08
language: German (translation notes in brackets)
note: Excerpts of a hobbyist page logging real ISO 9141-2 traffic (10400 baud, 5-baud init). Text extracted from HTML (ISO-8859-1 → UTF-8).
---


Genauso wie die Verfügbarkeit von Mode-1-Standard-PIDs abgefragt wird (01 00) 
kann auch die Verfügbarkeit von den Enhanced PIDs abgefragt werden, z.B. beim 
ZZE111:

 01 00 : PIDs 01..20

 01 20 : PIDs 21..40

 06 00 -> 46 00 FF 80 00 00 00 : Mode 6 TID 01 existiert

 01 B0 -> 41 B0 00 01 : PID 'C0' existiert
…
Mode 06

 06 00 => 06 00 FF 80 00 00 00 [= X:FD51++]

 06 01 bis 06 0A : ?

Mode 6 TID 1 - Rich to lean sensor threshold voltage (constant)

 mit 01 01 vorher prüfen, ob Test abgeschlossen ist

 es können mehrere Antworten folgen für min, max und Components

 [0000..00FF] = 0V..1,275V / 5mV/Bit

 46 01 aa bb cc dd ee

 aa.7: Test limit Type (0=max,1=min)

 aa.6:0: Component ID (herstellerdefiniert für z.B. Vor- und 
 Nach-Kat-Lambdasonde)

 bbcc: Test Value

 ddee: Minimum/Maximum Test Limit (Prüfgrenzwert)

 z.B. 46 01 05 00 01 00 73

…
 > 01 01

 < 41 01 01 07 61 00 (Monitor status since DTCs cleared)

 > 06 00

 < 46 00 ff 80 00 00 00 (Test results, other component/system monitoring 
 PID support)

…

 > 68 6a f0 01 01

 < 48 6b 10 41 01 00 07 61 61

 > 68 6a f0 06 00

 < 48 6b 10 46 00 ff 80 00 00 00


[Translation notes: "Mode 6 TID 01 existiert" = "mode 6 TID 01 exists"; "mit 01 01 vorher prüfen, ob Test abgeschlossen ist" = "check with 01 01 first whether the test has completed"; "es können mehrere Antworten folgen für min, max und Components" = "several answers may follow, for min, max and components"; "herstellerdefiniert für z.B. Vor- und Nach-Kat-Lambdasonde" = "manufacturer-defined, e.g. pre- and post-cat O2 sensor"; "Prüfgrenzwert" = "test limit".]
