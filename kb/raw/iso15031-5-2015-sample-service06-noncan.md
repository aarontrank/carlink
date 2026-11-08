---
title: ISO 15031-5:2015 (BS edition) publisher sample — service $06 test result/limit description and non-CAN message example (pp. 66–67)
source_url: https://www.normsplash.com/Samples/BSI/139381268/BS-ISO-15031-5-2015-en-2.pdf
fetched: 2026-11-08
publisher: BSI / ISO (free sample pages)
note: >
  Short excerpt of the free publisher sample, kept for reference; © ISO 2015. Text extracted with
  pypdf; tabs flattened. Subscripted "16" means hexadecimal (e.g. "8416" = 84 hex). This is the
  TID / TLTCID format used on ISO 9141-2, ISO 14230-4 and SAE J1850 (not CAN).
---


===== page 1 =====
BS ISO 15031-5:2015
 
ISO 15031-5:2015(E)
7.6.3.4 Test Result description
The Test Result represents the test result and is defined in Table 77.
Table 77 — Test Result description
Parameter Name # of Bytes Description
Test Result 2 
(High and 
Low Byte)
Test Result - this value shall be less than or equal to the test limit if MSB of Test 
Limit Type and Component ID byte is “0”, and shall be greater tha n or equal to 
the test limit if MSB of Test Limit Type and Component ID byte is “1”. The Test 
Value is a two-byte unsigned numeric value (0 – 65 535).
7.6.3.5 Test Limit description
The Test Limit is defined in Table 78.
Table 78 — Test Limit description
Parameter Name # of Bytes Description
Test Limit 2 
(High and 
Low Byte)
The Test Limit value is either a minimum or a maximum value to which the test 
results are compared. The Test Limit is a two-byte unsign ed numeric value 
(0 – 65 535).
7.6.4 Message example
Tables 79 to 81 below show how the “request on-board monitoring test results for specific monitored 
systems” service shall be implemented.
7.6.4.1  Step #1: Request on-board monitoring test results for specific monitored systems 
(request for supported Test IDs)
The external test equipment requests all supported Test IDs from t he vehicle. Refer to the example of 
Service 0116 for guidance on requesting supported PIDs (the same concept is used for sup ported TIDs).
As a result of the supported TID request, the external test equipment create s an internal list of 
supported TIDs for each ECU. ECU#1 (ECM) supports Test ID 02. ECU #2 (TCM) does not support any 
Test IDs.
7.6.4.2  Step #2: Request on-board monitoring test results for specific monitored systems
The external test equipment sends a “request on-board monito ring test results for specific monitored 
systems” request message with one (1) supported Test ID to the vehicle. The response messages indicate 
which Component IDs are supported. The request mess age includes the following Test ID:
Test ID 0216 - Lean to rich sensor threshold voltage (constant), (supported Component IDs: 0416, 1616).
In general, the external test equipment should read the test status of Servi ce 01 16 PID 01 16 prior 
to executing Service 06 16 with Test ID 01 16 and 06 16 to verify whether the tests are supported and 
completed. The test values reported may be invalid if the test is not complete d.
 
66 © ISO 2015 – All rights reserved
===== page 2 =====
BS ISO 15031-5:2015
 
ISO 15031-5:2015(E)
Table 79 — Request on-board monitoring test results  for specific monitored systems request 
message
Message Direction: External test equipment → All ECUs
Message Type: Request
Data Byte Description (all values are in hexadecimal) Byte Value Mnemonic
#1 Request on-board monitoring test results for specif ic monitored 
systems request SID
0616 SIDRQ
#2 TID Lean to rich sensor threshold voltage (constant) 02 16 TID
Table 80 — Request on-board monitoring test results  for specific monitored systems   
response message
Message Direction: ECU#1 → External test equipment
Message Type: Response
Data Byte Description (all values are in hexadecimal) Byte Value Mnemonic
#1 Request on-board monitoring test results for specif ic monitored 
systems response SID
4616 SIDPR
#2 TID Lean to rich sensor threshold voltage (constant) 02 16 TID
#3 Test Limit Type: test limit is minimum value; Component ID: 04 8416 TLTCID
#4 Test Value High Byte: test fails if test value is less than test limit 0016 TVHI
#5 Test Value Low Byte: test fails if test value is less than test limit 1016 TVLO
#6 Minimum Test Limit High Byte 0016 TLHI
#7 Minimum Test Limit Low Byte 0016 TLLO
NOTE ECU#2 does not support any Test IDs and therefore does not send a resp onse message.
Table 81 — Request on-board monitoring test results  for specific monitored systems   
response message
Message Direction: ECU#1 → External test equipment
Message Type: Response
Data Byte Description (all values are in hexadecimal) Byte Value Mnemonic
#1 Request on-board monitoring test results for specific monitored 
systems response SID
4616 SIDPR
#2 TID Lean to rich sensor threshold voltage (constant) 02 16 TID
#3 Test Limit Type: test limit is maximum value; Component ID: 16 16 1616 TLTCID
#4 Test Value High Byte: test fails if test value is greater than test 
limit
0016 TVHI
#5 Test Value Low Byte: test fails if test value is greater than test 
limit
3216 TVLO
#6 Maximum Test Limit High Byte 0016 TLHI
#7 Maximum Test Limit Low Byte 2016 TLLO
NOTE The above example shows that the test in ECU#1 for Test I D 0216 and Component ID 04 16 passed and 
that the test in ECU#1 for Test ID 02 16 and Component ID 1616 failed.
 
© ISO 2015 – All rights reserved 67
