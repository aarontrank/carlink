---
title: ISO 14229-1 (UDS) — publisher sample pages covering ClearDiagnosticInformation (0x14) and ReadDTCInformation (0x19)
source_url: https://www.normsplash.com/Samples/BSI/198624878/BS-ISO-14229-1-2020-en-2.pdf
source_url_2: https://www.normsplash.com/Samples/ISO/912557935/ISO-14229-1-2013-en-2.pdf
fetched: 2026-11-08
publisher: ISO (sample pages distributed by the publisher's reseller; the full standards were not obtained)
note: Text extracted with pypdf; table layouts are flattened and pypdf inserts stray spaces inside words. Only the passages carlink relies on are kept.
---

## ISO 14229-1:2020 sample (printed pp. 227–230)

```
=====PAGE 1
ISO 14229-1:2020(E) 
© ISO 2020 – All rights reserved 
227 
 
Table 300 — ClearDiagnosticInformation request message flow example #1 
Message direction client → server 
Message type Request 
A_Data byte Description (all values are in hexadecimal) Byte value Mnemonic 
#1 ClearDiagnosticInformation Request SID 1416 CDTCI 
#2 groupOfDTC [ DTCHighByte ] (“Emissions-related systems”) FF16 DTCHB 
#3 groupOfDTC [ DTCMiddleByte ] FF16 DTCMB 
#4 groupOfDTC [ DTCLowByte ] 3316 DTCLB 
Table 301 specifies the ClearDiagnosticInformation positive response message flow example #1. 
Table 301 — ClearDiagnosticInformation positive response message flow example #1 
Message direction server → client 
Message type Response 
A_Data byte Description (all values are in hexadecimal) Byte value Mnemonic 
#1 ClearDiagnosticInformation Response SID 5416 CDTCIPR 
12.3 ReadDTCInformation (1916) service 
12.3.1 Service description 
12.3.1.1 General description 
This service allows a client to read the status of server resident Diagnostic Trouble Code (DTC) 
information from any server, or group of servers wi thin a vehicle. Unless otherwise required by the 
particularSubFunction, the server shall return all DTC information (e.g.  emissions- related and non 
emissions-related). This service allows the client to do the following: 
— retrieve the number of DTCs matching a client defined DTC status mask; 
— retrieve the list of all DTCs matching a client defined DTC status mask; 
— retrieve the list of DTCs within a particular  functional group matching a client defined DTC stat us 
mask; 
[… list of further 0x19 capabilities omitted …]
subclauses. 
This service makes use of the following terms: 
— Enable criteria:  Server/vehicle manufacturer/system supplier specifi c criteria used to control 
when the server actually performs a particular internal diagnostic. 
— Test p ass criteria: Server/vehicle manufacturer/system supplier specifi c conditions, that define, 
whether a  system being diagnosed is functioning properly with in normal, acceptable operating 
ranges (e.g. no failures exist and the diagnosed system is classified as “OK”). 
— Test failure criteria: Server/vehicle manufacturer/system supplier specific failure conditions that 
define, whether a system being diagnosed has failed the test. 
— Confirmed failure criteria:  Server/vehicle manufacturer/system supplier specifi c failure 
conditions that define whether the system being dia gnosed is definitively problematic (confirmed), 
warranting storage of the DTC record in long term memory. 
— Occurrence counter:  A counter maintained by certain servers that record s the number of 
instances in which a given DTC test reported a unique occurrence of a test failure. 
— Aging: A process whereby certain servers evaluate past results of ea ch internal diagnostic to 
determine if a confirmed DTC can be cleared from lo ng-term memory, for example in the event of a 
calibrated number of failure free cycles. 
A given DTC value (e.g.  080511 16) shall never be reported more than once in a positive resp onse to 
readDTCInformation with the exception of reading DT CSnapshotRecords, where the response may 
contain multiple DTCSnapshotRecords for the same DTC. 
When using paged-buffer- handling to read DTCs (espe cially for SubFunction = 
reportDTCByStatusMask), it is possible that the num ber of DTCs can decrease while creating the 
response. In such a case the response shall be fill ed up with DTC 000000 16 and DTC status 00 16. The 
client shall treat these DTCs as not present in the response message. 
IMPORTANT — The server and the client shall meet the request an d response message behaviour as 
specified in 8.7. 
12.3.1.2 Retrieving the number of DTCs that match a client defined status mask (SubFunction = 
 01 16 reportNumberOfDTCByStatusMask) 
A client can retrieve a count of the number of DTCs  matching a client defined status mask by sending a  
request for this service with the SubFunction set to reportNumberOfDTCByStatusMask. The response to 
this request contains the DTCStatusAvail abilityMask, which provides an indication of DTC st atus bits 
that are supported by the server for masking purpos es. Following the DTCStatusAvailabilityMask the 
response contains the DTCFormatIdentifier which rep orts information about the DTC formatting and  
encoding. The DTCFormatIdentifier is followed by th e DTCCount parameter which is a 2- byte unsigned 
numeric number containing the number of DTCs availa ble in the server's memory based on the status 
mask provided by the client. 
12.3.1.3 Retrieving the list of DTCs that match a client defined status mask (SubFunction = 0216 
    reportDTCByStatusMask) 
The client can retrieve a list of DTCs, which satis fy a client defined status mask by sending a reques t 
with the SubFunction byte set to reportDTCByStatusMask. This SubFunction allows the client to request 
the server to report all DTCs that are “testFailed” OR “confirmed” OR “etc.”. 
BS ISO 14229‑1:2020
=====PAGE 4
ISO 14229-1:2020(E) 
230 © ISO 2020 – All rights reserved 
 
The evaluation shall be done as follows: The server  shall perform a bit- wise logical AND-ing operation 
between the mask specified in the client’s request and th e actual status associated with each DTC 
supported by the server. In addition to the DTCStat usAvailabilityMask, the server shall return all DTC s 
for which the result of the AND-ing operation is no n-zero (i.e. (statusOfDTC & DTCStatusMask) != 0). If 
the client specifies a status mask that contains bi ts that the server does not support, then the serve r 
shall process the DTC information using only the bi ts that it does support. If no DTCs within the serv er 
match the masking criteria specified in the client’s r equest, no DTC or status information shall be 
provided following the DTCStatusAvailabilityMask byte in the positive response message. 
DTC status information shall be cleared upon a succ essful ClearDiagnosticInformation request from the 
client (see DTC status bit definitions in D.2  for further descriptions on the DTC status bit hand ling in 
case of a ClearDiagnosticInformation service request reception in the server). 
12.3.1.4 Retrieving DTCSnapshot record identification (SubFunction = 0316      
    reportDTCSnapshotIdentification) 
A client can retrieve DTCSnapshot record identifica tion information for all captured DTCSnapshot 
records by sending a request for this service with the SubFunction set to 
```

## ISO 14229-1:2013 sample — Table 272, positive response to reportDTCByStatusMask (printed pp. 200–201)

```
Table 272 defines the positive response message format of the sub-function parameter. 
Table 272 — Response message definition - sub-function = reportDTCByStatusMask, 
reportSupportedDTCs, reportFirstTestFailedDTC, reportFirstConfirmedDTC, 
reportMostRecentTestFailedDTC, reportMostRecentConfirmedDTC, 
reportMirrorMemoryDTCByStatusMask, reportEmissionsOBDDTCByStatusMask, 
reportDTCWithPermanentStatus 
A_Data byte Parameter Name Cvt Byte Value Mnemonic 
#1 ReadDTCInformation Response SID M 0x59 RDTCIPR 
#2 reportType = [  
   reportDTCByStatusMask 
   reportSupportedDTCs 
   reportFirstTestFailedDTC 
   reportFirstConfirmedDTC 
   reportMostRecentTestFailedDTC 
   reportMostRecentConfirmedDTC 
   reportMirrorMemoryDTCByStatusMask 
   reportEmissionsOBDDTCByStatusMask 
   reportDTCWithPermanentStatus ] 
M
0x02 
0x0A 
0x0B 
0x0C 
0x0D 
0x0E 
0x0F 
0x13 
0x15 
LEV_
RDTCBSM 
RSUPDTC
RFTFDTC 
RFCDTC 
RMRTFDTC 
RMRCDTC 
RMMDTCBSM 
ROBDDTCBSM 
RDTCWPS 
#3 DTCStatusAvailabilityMask M 0x00 – 0xFF DTCSAM 
#4
#5
#6
#7
#8
#9
#10
#11
:
#n-3
#n-2
#n-1
#n
DTCAndStatusRecord[] = [ 
      DTCHighByte#1 
      DTCMiddleByte#1 
      DTCLowByte#1 
      statusOfDTC#1 
      DTCHighByte#2 
      DTCMiddleByte#2 
      DTCLowByte#2 
      statusOfDTC#2 
       : 
      DTCHighByte#m 
      DTCMiddleByte#m 
      DTCLowByte#m 
      statusOfDTC#m ] 
C1
C1
C1
C1
C2
C2
C2
C2
:
C2
C2
C2
C2
0x00 – 0xFF 
0x00 – 0xFF 
0x00 – 0xFF 
0x00 – 0xFF 
0x00 – 0xFF 
0x00 – 0xFF 
0x00 – 0xFF 
0x00 – 0xFF 
:
0x00 – 0xFF 
0x00 – 0xFF 
0x00 – 0xFF 
0x00 – 0xFF 
DTCASR_ 
DTCHB 
DTCMB 
DTCLB 
SODTC
DTCHB 
DTCMB 
DTCLB 
SODTC
:
DTCHB 
DTCMB 
DTCLB 
SODTC
=====PAGE 9
ISO 14229-1:2013(E) 
© ISO 2013 – All rights reserved 201
Table 272 — (continued)
A_Data byte Parameter Name Cvt Byte Value Mnemonic 
C1: This parameter is only present if DTC information  is available to be reported. 
C2: This parameter is only present if reportType = re portSupportedDTCs, reportDTCByStatusMask,  
reportMirrorMemoryDTCByStatusMask, reportEmissionsO BDDTCByStatusMask, reportDTCWithPermanentStatus and  
more than one DTC information is available to be reported. 
Table 273 defines the positive response message format of the sub-function parameter. 
```
