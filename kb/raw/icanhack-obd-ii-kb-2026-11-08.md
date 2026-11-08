---
title: I CAN Hack Knowledge Base — On-board diagnostics (OBD-II) chapter (services 05 and 06 excerpt)
source_url: https://icanhack.nl/knowledge-base/diagnostics/obd-ii/
fetched: 2026-11-08
publisher: I CAN Hack (Gouda, NL)
note: Text extracted from the HTML page; navigation removed. Byte-layout diagrams flatten to runs such as "Tester46OBDMIDTIDUAS ID2 bytes2 bytes2 bytes".
---


Signal Protocols
The OBD-II standard allows for implementing one or more of the following signaling protocols. Since 2008, CAN is mandatory in the United States.
SAE J1850 PWM: Two-wire PWM-based signal running at 41.6 kbit/s. Most commonly used in older Ford vehicles. [2]
SAE J1850 VPW: One-wire VPW-based signal. After every bit the level on the bus changes. The length of the high pulse determines the value of the bit: 64 µs for a logical 1, 128 µs for a logical 0. Due to the variable length of each bit there is no fixed data rate. Most commonly used in older General Motors vehicles.
ISO 9141 (K-Line): UART-based signal running at 10.4 kbit/s. Used in EU, Chrysler, and Asian cars from 2000 until 2004. [3]
ISO 14230 (KWP2000): Same physical layer as ISO 9141. Common after 2003. [4]
ISO 15765 (CAN): Mandatory in the United States since 2008. Uses ISO-TP as a transport layer. [5]
Service IDs (SIDs)
The protocol itself was standardized as SAE J1979 [6] (also published as ISO 15031-5 [7]) and defines a set of services that must be supported. These "services" used to be called "modes". A successful response starts with the SID ORed with 0x40 (e.g. SID 0x01 becomes a response of 0x41). On error, the reply starts with 0x7F, followed by the SID and a UDS-style negative response code.
The PID, OBDMID, TID and INFOTYPE values referenced below are catalogued in the SAE J1979-DA digital annex [8].
0x01: Show current data
Reads a live emissions-related parameter. The request carries a 1-byte PID; the response echoes the PID followed by 1 to 4 data bytes (named A, B, C, D). PID 0x00 is special: it reports a bitmap of which PIDs in the range 0x01 to 0x20 the ECU supports. The same trick (PID 0x20, 0x40, 0x60, …) walks the rest of the table.
…

0x05: Oxygen sensor monitoring test results (non-CAN only)
Reports oxygen sensor test results addressed by a Test ID. Defined for ISO 9141-2 [9], ISO 14230-4 [4] and SAE J1850 [2] only; on CAN this functionality is folded into Service 0x06.
0x05 — Request oxygen sensor monitoring test results
Tester
ECU05TIDO2 sensor
Field | Mnemonic | Value | Description | 
Service ID | SIDRQ | 05 | Request oxygen sensor monitoring test results SID. | 
testIdentifier | TID | TID | Test ID identifying the oxygen sensor parameter. | 
o2SensorId | O2S | O2 sensor | Oxygen sensor identifier (bank/sensor). Optional depending on Test ID. | 
ECU
Tester45TIDO2 sensortest value bytes
Field | Mnemonic | Value | Description | 
Response SID | SIDPR | 45 | Request SID ORed with 0x40. | 
testIdentifier | TID | TID | Echoed Test ID. | 
o2SensorId | O2S | O2 sensor | Echoed sensor identifier. | 
dataRecord | DREC | test value bytes | Test result, format defined per Test ID. | 

0x06: On-board monitoring test results
Reports test values, minimum and maximum limits for on-board monitors (catalyst, EVAP, misfire, oxygen sensors on CAN, …). Each monitor is addressed by an OBDMID; each monitor can have multiple Test IDs. The Unit and Scaling ID tells the tester how to interpret the 16-bit value and limits.
0x06 — Request on-board monitoring test results
Tester
ECU06OBDMID
Field | Mnemonic | Value | Description | 
Service ID | SIDRQ | 06 | Request on-board monitoring test results SID. | 
monitorIdentifier | OBDMID | OBDMID | On-Board Diagnostic Monitor ID. | 
ECU
Tester46OBDMIDTIDUAS ID2 bytes2 bytes2 bytes
Field | Mnemonic | Value | Description | 
Response SID | SIDPR | 46 | Request SID ORed with 0x40. | 
monitorIdentifier | OBDMID | OBDMID | Echoed OBDMID. | 
testIdentifier | TID | TID | Standardized or manufacturer-defined Test ID. | 
unitAndScalingId | UASID | UAS ID | Encodes the unit and scaling for the test value and limits. | 
testValue | TV | 2 bytes | Test result, 16-bit big-endian. | 
minTestLimit | MINTL | 2 bytes | Minimum test limit, 16-bit big-endian. | 
…
[5]↑abISO. Road vehicles, Diagnostic communication over Controller Area Network (DoCAN), Part 4: Requirements for emissions-related systems, ISO 15765-4:2016.
[6]↑SAE. E/E Diagnostic Test Modes, SAE J1979.
[7]↑ISO. Road vehicles, Communication between vehicle and external equipment for emissions-related diagnostics, Part 5: Emissions-related diagnostic services, ISO 15031-5.
[8]↑abcSAE. J1979-DA, Digital Annex of E/E Diagnostic Test Modes, SAE J1979-DA.
[9]↑ISO. Road vehicles, Diagnostic systems, Part 2: CARB requirements for interchange of digital information, ISO 9141-2:1994.

Quick openCtrl KOn this page
11.1History
11.2SAE J1962 Connector
