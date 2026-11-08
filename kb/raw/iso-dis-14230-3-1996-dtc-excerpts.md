---
title: ISO/DIS 14230-3 (KWP2000 Part 3, draft of 16 Dec 1996) — response codes and DTC services 0x13/0x18
source_url: http://www.internetsomething.com/kwp/KWP2000%20ISO%2014230-3.pdf
fetched: 2026-11-08
note: Draft International Standard text extracted with pypdf (93 pages); tables flattened. The final ISO 14230-3:1999 was not obtained.
---

## 4.4 Response Code value summary table (verbatim, flattened)

```
Page: 15 of 93 
4.4 - Response Code value summary table 
The following table lists and assigns hex values for all response codes used in KWP 2000. 
The definition of each response code is described in the ISO 14229 Diagnostic Services  
Specification document. 
Hex Value Response Code 
10 generalReject 
11 serviceNotSupported 
12 subFunctionNotSupported-invalidFormat 
21 busy-RepeatRequest 
22 conditionsNotCorrect or requestSequenceError 
23 routineNotComplete 
31 requestOutOfRange 
33 securityAccessDenied 
35 invalidKey 
36 exceedNumberOfAttempts 
37 requiredTimeDelayNotExpired 
40 downloadNotAccepted 
41 improperDownloadType 
42 can'tDownloadToSpecifiedAddress 
43 can'tDownloadNumberOfBytesRequested 
50 uploadNotAccepted 
51 improperUploadType 
52 can'tUploadFromSpecifiedAddress 
53 can'tUploadNumberOfBytesRequested 
71 transferSuspended 
72 transferAborted 
74 illegalAddressInBlockTransfer 
75 illegalByteCountInBlockTransfer 
76 illegalBlockTransferType 
77 blockTransferDataChecksumError 
78 reqCorrectlyRcvd-RspPending (requestCorrectlyReceived-ResponsePending) 
79 incorrectByteCountDuringBlockTransfer 
80 - FF manufacturerSpecificCodes 
Table 4.4 - Response Code value summary table 
=====PAGE 16
ISO / DIS 14230 - 3 : KEYWORD PROTOCOL 2000 - PART 3: IMPLEMENTATION 
Page: 16 of 93 
4.5 - Response code handling 
The figure below specifies the server behaviour on a client request message. This figure shows the 
```

## Service identifier summary (lines containing DTC services, verbatim)

```
6.1 - StartDiagnosticSession service........................................................................................... ............................... 24 
6.2 - StopDiagnosticSession service............................................................................................ ............................... 26 
6.4 - TesterPresent service ......................................................................................................................................... 31 
8.1 - ReadDiagnosticTroubleCodes service.............................................................................................................. 54 
8.2 - ReadDiagnosticTroubleCodesByStatus service .............................................................................................. 55 
8.3 - ReadStatusOfDiagnosticTroubleCodes service............................................................................... ................ 57 
8.5 - ClearDiagnosticInformation service ................................................................................................................ 62 
startDiagnosticSession 10 50 
readDiagnosticTroubleCodes 13 53 
clearDiagnosticInformation 14 54 
readStatusOfDiagnosticTroubleCodes 17 57 
readDiagnosticTroubleCodesByStatus 18 58 
stopDiagnosticSession 20 60 
testerPresent 3E 7E 
• clearDiagnosticInformation, 
During the period of (a) negative response message(s) the testerPresent service shall be disabled 
• clearDiagnosticInformation 
startDiagnosticSession.Request[...] 
<stopDiagnosticSession> Request[] 
<startDiagnosticSession> Request[...] 
```

## 8.1 ReadDiagnosticTroubleCodes and 8.2 ReadDiagnosticTroubleCodesByStatus (verbatim, flattened)

```
=====PAGE 54
ISO / DIS 14230 - 3 : KEYWORD PROTOCOL 2000 - PART 3: IMPLEMENTATION 
Page: 54 of 93 
8.1 - ReadDiagnosticTroubleCodes service 
8.1.1 - Parameter definition 
• The parameter groupOfDTC is used by the ReadDTC, ReadDTCByStatus and 
ReadStatusOfDTC services to select to which functional group of DTC or to which specific DTC 
access is requested. Format and length of this parameter are vehicle manufacturer specific. 
Note: Standardised DTCs and DTCs formats are specified in SAE J2012 for passenger cars and 
in SAE J1587 and SAE J1939 for heavy duty vehicles. 
• The parameter numberOfDTC is used by the ReadNumberOfDTC positive response to 
indicate how many DTCs of the group specified in the request have been stored by the 
client(s). A specific standard value is noDTCStored. 
Hex Description 
00 noDTCStored 
This value indicates to the server that the client(s) doesn't (don't) have any DTC stored. 
01-FF numberOfDTCStored 
This range of values indicate to the server that the client(s) has (have) stored DTC(s). 
Table 8.1.1.2 - Definition of numberOfDTC values 
• The parameter listOfDTC is used by the server to report DTC(s) stored by the client(s). 
Note: Standardised DTCs and DTCs formats are specified in SAE J2012 for passenger cars and 
in SAE J1587 and SAE J1939 for heavy duty vehicles. 
8.1.2 - Message data bytes 
Data Byte Parameter Name Cvt Hex Value Mnemonic
1 readDiagnosticTroubleCodes Request Service Id M 13 RDDTC 
2 
: 
n 
groupOfDTC#1 
         : 
groupOfDTC#m 
U 
: 
U 
xx 
: 
xx 
GODTC 
Table 8.1.2.1 - ReadDiagnosticTroubleCodes Request Message 
Data Byte Parameter Name Cvt Hex Value Mnemonic
1 readDiagnosticTroubleCodes Positive Resp. Service Id M 53 RDDTCPR
2 numberOfDTC M xx #DTC 
3 
: 
: 
: 
: 
: 
: 
n 
listOfDTC=[ 
DTC#1 
      : 
DTC#1 
      : 
      : 
DTC#m 
      : 
DTC#m 
] 
C 
xx 
: 
xx 
: 
: 
xx 
: 
xx 
LSTOFDTC
Table 8.1.2.2 - ReadDiagnosticTroubleCodes Positive Response Message 
C = condition: listOfDTC is only present if numberOfDTC is > "$00". 
=====PAGE 55
ISO / DIS 14230 - 3 : KEYWORD PROTOCOL 2000 - PART 3: IMPLEMENTATION 
Page: 55 of 93 
Data Byte Parameter Name Cvt Hex Value Mnemonic
1 negativeResponse Service Id S 7F NACK 
2 readDiagnosticTroubleCodes Request Service Id M 13 RDDTC 
3 responseCode=[ 
KWP2000ResponseCode, 
manufacturerSpecific 
] 
M xx=[ 
00-7F, 
80-FF 
] 
RC 
Table 8.1.2.3 - ReadDiagnosticTroubleCodes Negative Response Message 
8.1.3 - Message description 
The readDiagnosticTroubleCodes service is used by the client to read diagnostic trouble codes 
from the server's memory. If the user optional parameter groupOfDTC is present in the request 
message then the server(s) shall only report DTC(s) based on the functional group selected by the 
client. 
If the server(s) doesn't (don't) have any DTC stored it (they) shall set the parameter 
numberOfDTCStored to "$00". This causes the server(s) not to include DTC(s) in the parameter 
listOfDTC in the positive response message. 
8.1.4 - Message flow example 
See message flow example of Physical Addressed Service in section 5.3.1.1 and Functional 
Addressed Service in section 5.3.2.1. 
8.2 - ReadDiagnosticTroubleCodesByStatus service 
8.2.1 - Parameter definition 
• The parameter statusOfDTC is used by ReadDiagnosticTroubleCodesByStatus and 
ReadStatusOfDiagnosticTroubleCodes service to inform the client about the status of each 
DTC stored in the server's memory. Format and length of this parameter are vehicle 
manufacturer specific. 
Note: Standardised DTCs and DTCs formats are specified in SAE J2012 for passenger cars and 
in SAE J1587 and SAE J1939 for heavy duty vehicles. 
• The parameter groupOfDTC is defined in section 8.1.1. 
• The parameter numberOfDTC is defined in section 8.1.1. 
• The parameter listOfDTCAndStatus is used by the server to report DTC(s) and their 
associated status. 
=====PAGE 56
ISO / DIS 14230 - 3 : KEYWORD PROTOCOL 2000 - PART 3: IMPLEMENTATION 
Page: 56 of 93 
8.2.2 - Message data bytes 
Data Byte Parameter Name Cvt Hex Value Mnemonic
1 ReadDiagnosticTroubleCodesByStatus Request SId M 18 RDDTCS
2 
: 
n 
statusOfDTC#1 
         : 
statusOfDTC#m 
M 
: 
U 
xx 
: 
xx 
STATDTC
n+1 
: 
n+m 
groupOfDTC#1 
         : 
groupOfDTC#m 
U 
: 
U 
xx 
: 
xx 
GODTC 
Table 8.2.2.1 - ReadDiagnosticTroubleCodesByStatus Request Message 
Data Byte Parameter Name Cvt Hex Value Mnemonic
1 ReadDiagnosticTroubleCodesByStatus Pos. Response SId M 58 RDDTCSPR
2 numberOfDTC M xx #DTC 
3 
: 
: 
: 
: 
: 
: 
: 
: 
: 
: 
: 
: 
n 
listOfDTCAndStatus=[ 
DTC#1 
      : 
DTC#1 
statusOfDTC#1 
      : 
statusOfDTC#1 
      : 
      : 
DTC#m 
      : 
DTC#m 
statusOfDTC#m 
      : 
statusOfDTC#m 
] 
C 
xx 
: 
xx 
xx 
: 
xx 
: 
: 
xx 
: 
xx 
xx 
: 
xx 
LSTDTCST
Table 8.2.2.2 - ReadDiagnosticTroubleCodesByStatus Positive Response Message 
C = condition: listOfDTCAndStatus is only present if numberOfDTC is > "$00". 
Data Byte Parameter Name Cvt Hex Value Mnemonic
1 negativeResponse Service Id S 7F NACK 
2 ReadDiagnosticTroubleCodesByStatus Request SId M 18 RDDTC 
3 responseCode=[ 
KWP2000ResponseCode, 
manufacturerSpecific 
] 
M xx=[ 
00-7F, 
80-FF 
] 
RC 
Table 8.2.2.3 - ReadDiagnosticTroubleCodesByStatus Negative Response Message 
8.2.3 - Message description 
The readDiagnosticTroubleCodesByStatus service is used by the client to read diagnostic trouble 
codes by status from the server's memory. If the user optional parameter groupOfDTC is present in 
the request message then the server(s) shall only report DTC(s) with status information based on 
the functional group selected by the client. 
=====PAGE 57
ISO / DIS 14230 - 3 : KEYWORD PROTOCOL 2000 - PART 3: IMPLEMENTATION 
Page: 57 of 93 
If the server(s) doesn't (don't) have any DTC with st atus information stored it (they) shall set the 
parameter numberOfDTCStored to "$00". This causes the server(s) not to include DTC(s) and 
status information in the parameter listOfDTC in the positive response message. 
8.2.4 - Message flow example 
See message flow example of Physical Addressed Service in section 5.3.1.1 and Functional 
Addressed Service in section 5.3.2.1. 
8.3 - ReadStatusOfDiagno sticTroubleCodes service 
8.3.1 - Parameter definition 
• The parameter groupOfDTC is defined in section 8.1.1. 
```
