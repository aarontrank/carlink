---
title: DaimlerChrysler Keyword Protocol 2000 Requirements Definition, Release 2.2 (2002) — services $14 and $18, Status Of DTC, session table
source_url: https://andrewrevill.co.uk/ReferenceLibrary/OBDII%20Specifications%20-%20KWP2000%20DaimlerChrysler%202002.pdf
fetched: 2026-11-08
publisher: DaimlerChrysler (Auburn Hills / Stuttgart), 05 August 2002
note: Text extracted with pypdf (122 pages); tables are flattened. One OEM's KWP2000 implementation, not the ISO text. Hyundai's KWP-style ECUs follow its $18 $00 request format but NOT its Status Of DTC bit meanings (see raw/projectgus-kona-ev-can-logs-diag.md).
---

## Table 3.2.1-1 Service Identifier Session Support (verbatim, flattened)

```
Stand By Session ($89), Extended Diagnostic Session ($92). 
9. If an ECU is functionally addressed with a Start Diagnostic Session ($10) request mesage, there shall be 
no response (positive or negative) 
=====PAGE 16
KEYWORD PROTOCOL 2000 REQUIREMENTS DEFINITION Service ID $10 –  Start Diagnostic Session 
Vehicle Diagnostics,CoC (DCA) & Diagnostic Standards (DCS) 
Copyright ©2000-2002 DaimlerChrysler  8 of 114  Release Version: 2.2 Release Date: 05 August 2002 
Table 3.2.1-1 lists all of the diagnostic service identifiers and the Diagnostic Sessions  supported by this 
document.  The ‘X’ indicates that a service identifier may be supported while the ECU is running in the specified 
Diagnostic Session.  Each column indicates the available set of services that can be supported under each 
diagnostic session.  Not all services are required under the selected session but should be implemented in order 
to achieve the desired functionality of the diagnostic session. 
SID 
[1] 
Service Identifier Description 
[2] 
Normal / 
Default 
Session 
($10 $81) 
[3] 
ECU Flash 
Reprogramming 
Session 
($10 $85) 
[4] 
Stand 
By 
Session 
($10 
$89) 
[5] 
ECU 
Passive 
Session 
($10 $90) 
[6] 
Extended 
Diagnostic 
Session 
($10 $92) 
[7] 
Page 
# 
[8] 
$01-
$09 
SAE J1979 / ISO 15031-5 OBD II Test 
Sessions 
X    X - 
$10 Start Diagnostic Session X X X X X 7 
$11 ECU Reset  X X  X 11 
$14 Clear Diagnostic Information     X 12 
$17 Read Status Of Diagnostic Trouble Codes X    X 14 
$18 Read Diagnostic Trouble Codes By Status X    X 17 
$1A Read ECU Identification X X   X 23 
$21 Read Data By Local Identifier X X X  X 25 
$22 Read Data By Identifier X X X  X 27 
$23 Read Memory By Address     X 29 
$27 Security Access  X X  X 31 
$28 Disable Normal Message Transmission     X 35 
$29 Enable Normal Message Transmission     X 37 
$2C Dynamically Define Local Identifier     X 38 
$2E Write Data By Identifier  X X  X 44 
$30 Input Output Control By Local Identifier   X  X 46 
$31 Start Routine By Local Identifier  X X  X 51 
$32 Stop Routine By Local Identifier  X X  X 53 
$33 Request Routine Results By Local 
Identifier 
 X X  X 55 
$34 Request Download  X   X 56 
$35 Request Upload  X   X 57 
$36 Transfer Data  X   X 58 
$37 Request Transfer Exit  X   X 60 
$3B Write Data By Local Identifier  X X  X 64 
$3D Write Memory By Address     X 67 
$3E Tester Present X X X  X 69 
$85 Control DTC Setting  X X X X 70 
$86 Response On Event X    X 72 
$BA - 
$BF 
System Supplier Specific (X) (X) (X) (X) (X) n/a 
Table 3.2.1-1 Service Identifier Session Support 
=====PAGE 17
KEYWORD PROTOCOL 2000 REQUIREMENTS DEFINITION Service ID $10 –  Start Diagnostic Session 
Vehicle Diagnostics,CoC (DCA) & Diagnostic Standards (DCS) 
Copyright ©2000-2002 DaimlerChrysler  9 of 114  Release Version: 2.2 Release Date: 05 August 2002 
COLUMN DEFINITIONS FOR TABLE 3.2.1-1 
C o l u m n  D e s c r i p t i o n            
[1]  SID – This is the hexadecimal data value assigned to each service ID. 
[2] Service Identifier Description – The column is used for the qualitative description of the 
```

## 3.4 Service ID $14 – Clear Diagnostic Information (verbatim)

```
3.4 S ERVICE ID $14 –  CLEAR DIAGNOSTIC INFORMATION 
3.4.1 M ESSAGE DESCRIPTION 
PURPOSE 
The service, Clear Diagnostic Information ($14), is used by the diagnostic tool to clear Diagnostic Trouble 
Codes (DTC) and associated diagnostic information in the ECU’s memory. 
Note for Implementation at DCA: A complete list of Diagnostic Trouble Codes for Powertrain (P), Body (B), 
Chassis (C), and Network (U) will be provided by Vehicle Diagnostics, CoC. 
REQUIREMENTS 
1. If no DTC and/or diagnostic information was stored, the ECU shall return the Clear Diagnostic 
Information Positive Response ($54)  and the Group of DTC parameters after receiving the Clear 
Diagnostic Information Request ($14) service  
The diagnostic tool shall send the Group Of DTC parameter in the Clear Diagnostic Information Request 
($14) service to specify a single DTC or a group of DTC’s to be cleared.  A value other than $0000, $4000, 
$8000, $C000, or $FF00 implies a single DTC. 
A successful completion of Clear Diagnostic Information shall reset all DTC related information in the ECU's 
memory specified by the Group Of DTC parameter.  (Note:  For DCA, this excludes data stored in the Fault 
Historical Record / Diagnostic Tell – Tale Retention Stack.) 
DCA Only: All DTC values used in the Group Of DTC parameter must be coordinated with the Vehicle 
Diagnostics, CoC group. 
3.4.2 R EQUEST MESSAGE FORMAT 
Data 
Byte # Data Value Parameter Description 
Message 
Usage 
Reference 
Page 
0 $14 Clear Diagnostic Information Request Service ID Mandatory  
1 $XX Group Of DTC (High Byte) Mandatory 82 
2 $XX Group Of DTC (Low Byte) Mandatory 82 
 $0000 All Powertrain DTC’s Optional 82 
 $0001 - 
$3FFF 
Powertrain DTC Optional 83 
 $4000 All Chassis DTC’s Optional 82 
 $4001 - 
$7FFF 
Chassis DTC Optional 83 
 $8000 All Body DTC’s Optional 82 
 $8001 - 
$BFFF 
Body DTC Optional 83 
 $C000 All Network DTC’s Optional 82 
 $C001 - 
$FEFF 
Network DTC Optional 83 
 $FF00 All DTC’s Mandatory 83 
Table 3.4.2-1 Clear Diagnostic Information Request Message Format 
3.4.3 P OSITIVE RESPONSE MESSAGE FORMAT 
Data 
Byte # Data Value Parameter Description 
Message 
Usage 
Reference 
Page 
0 $54 Clear Diagnostic Information Positive Response Service ID Mandatory  
1 $XX Group Of DTC ( (High Byte) Mandatory 82 
2 $XX Group Of DTC (Low Byte) Mandatory 82 
  The Group Of DTC parameters must be identical to the Group Of DTC 
parameters sent in the request message.  Refer to Table 3.4.2-1 
Table 3.4.3-1 Clear Diagnostic Information Positive Response Message Format 
=====PAGE 21
KEYWORD PROTOCOL 2000 REQUIREMENTS DEFINITION Service ID $14 –  Clear Diagnostic Information 
Vehicle Diagnostics,CoC (DCA) & Diagnostic Standards (DCS) 
Copyright ©2000-2002 DaimlerChrysler  13 of 114  Release Version: 2.2 Release Date: 05 August 2002 
3.4.4 N EGATIVE RESPONSE MESSAGE FORMAT 
Data 
Byte # Data Value Parameter Description 
Message 
Usage 
Reference 
Page 
0 $7F Negative Response Mandatory  
1 $14 Clear Diagnostic Information Request Service Mandatory  
2 $XX Negative Response Code Mandatory  
  Refer to Section 5 “ Negative Response Codes” for global and diagnostic 
service specific negative response codes. 
Table 3.4.4-1 Clear Diagnostic Information Negative Response Message 
3.4.5 I MPLEMENTATION EXAMPLE OF CLEAR DIAGNOSTIC INFORMATION (FUNCTION GROUP) 
```

## 3.6.2 Read Diagnostic Trouble Codes By Status — request format (verbatim)

```
5. DCA Only:  All DTC values used in the Group Of DTC parameter must be coordinated with the Vehicle 
Diagnostics,CoC group. 
6. All DTC’s returned in the positive response shall be returned in chronolgical order from newest (most 
recent) to oldest. 
7. When using the Request Extended Number of Supported DTCs ($E0), the service Request Supported 
DTC and Status ($18 $01 / $18 $03)  have to be repeated within a certain time window (e.g. P3-max). 
After the window has elapsed the ECU starts to report the DTCs from the beginning again, if the service 
Request Supported DTC and Status ($18 $01 / $18 $03) is requested again. 
3.6.2 R EQUEST MESSAGE FORMAT 
Data 
Byte # Data Value Parameter Description 
Message 
Usage 
Reference 
Page 
0 $18 Read Diagnostic Trouble Codes By Status Request Service ID Mandatory  
1 $XX Status Of DTC Mandatory 83 
 $00 Request Identified DTC and Status (SAE J2012 / ISO 15031-6 format) Conditional 1 83 
 $01 Request Supported DTC and Status (SAE J2012 / ISO 15031-6 format) Conditional 3 83 
 $02 Request Identified 2 Byte Hex DTC and Status Conditional 2 83 
 $03 Request Supported 2 Byte Hex DTC and Status Conditional 3 84 
 $04 Request Most Recent DTC Optional 84 
 $E0 Request Extended Number of Supported DTCs Conditional 4 84 
2 $XX Group Of DTC = Request By Functional Group (High Byte) Mandatory 82 
3 $XX Group Of DTC = Request By Functional Group (Low Byte) Mandatory 82 
 $0000 All Powertrain DTC’s Optional 82 
 $4000 All Chassis DTC’s Optional 82 
 $8000 All Body DTC’s Optional 82 
 $C000 All Network DTC’s Optional 82 
 $FF00 All DTC’s Mandatory 83 
Table 3.6.2-1  Read Diagnostic Codes By Status Request Message Format 
Condition1: This sub-function shall be supported by all DCA and MMC ECUs. 
Condition2: This sub-function shall be supported by all DCS-ECUs. 
Condition3: This sub-function shall be supported if the DTC Readiness Flag information shall be supported. 
Condition4: This sub-function shall be supported if the number of supported DTCs exceeds 255 or if the ECU 
requires segmentation on application layer to transmit all supported DTCs. 
=====PAGE 26
```

## Status Of DTC values used in the request, and 4.3.6 Status Of DTC in the response (verbatim)

```
4.3.5 S TATUS OF DTC (AS USED BY THE DIAGNOSTIC TOOL REQUEST MESSAGE) 
The Status Of DTC, used with the diagnostic tool request message, is used to specify the DTC’s that are to be 
returned. 
REQUEST IDENTIFIED DTC AND STATUS (SAE J2012 / ISO 15031-6 FORMAT) ($00) 
This value is used by the diagnostic tool to indicate to the ECU that the ECU shall respond with all 
diagnostic trouble codes (DTC’s) in the format defined by SAE J2012 / ISO 15031-6 which have been 
identified to cause a problem at the time of the request. 
REQUEST SUPPORTED DTC AND STATUS (SAE J2012 / ISO 15031-6 FORMAT) ($01) 
This value is used by the diagnostic tool to indicate to the ECU that the ECU shall respond with all 
supported diagnostic trouble codes (DTC’s) in the format defined by SAE J2012 / ISO 15031 -6 regardless of 
their status.  Even if no DTC is stored the ECU shall report all DTC’s which are supported. This allows the 
diagnostic tool to test the DTC Readiness Flag in the Status Of DTC parameter of each supported DTC. 
REQUEST IDENTIFIED 2 BYTE HEX DTC AND STATUS ($02) 
This value is used by the diagnostic tool to indicate to the ECU that the ECU shall respond with all 
diagnostic trouble codes (DTC’s) in 2 byte hex format which have been identified to cause a problem at the 
time of the request. The DTCs shall be reported by the ECU in the same sequence as they have been 
detected. Each DTC shall be reported by the ECU uniquely. (if an occurrence count is necessary it shall be 
reported in the corresponding environment / system supplier data of service $17. 
=====PAGE 92
KEYWORD PROTOCOL 2000 REQUIREMENTS DEFINITION Clear DTC Information, Read Status of DTC, and Read DTC By  Status 
Vehicle Diagnostics,CoC (DCA) & Diagnostic Standards (DCS) 
Copyright ©2000-2002 DaimlerChrysler  84 of 114  Release Version: 2.2 Release Date: 05 August 2002 
This subfunction shall be supported for ECUs used for DCS 
REQUEST SUPPORTED 2 BYTE HEX DTC AND STATUS ($03) 
This value is used by the diagnostic tool to indicate to the ECU that the ECU shall respond with all 
supported diagnostic trouble codes (DTC’s) in 2 byte hex format regardless of their status .  Even if no DTC 
is stored the ECU shall report all DTC’s which are supported. This allows the diagnostic tool to test the DTC 
Readiness Flag in the Status Of DTC parameter of each supported DTC. 
REQUEST MOST RECENT 2 BYTE HEX DTC AND STATUS ($04) 
This value is used by the diagnostic tool to indicate to the ECU that the ECU shall respond with the most 
recently detected diagnostic trouble code (DTC) in 2 byte hex format and its status. Only one DTC is 
reported. 
REQUEST EXTENDED NUMBER OF SUPPORTED DTCS 
This value is used by the diagnostic tool to indicate to the ECU that the ECU shall respond with the total 
number of all supported diagnostic trouble codes (DTC’s) in 2 byte hex format regardless of their status .  
4.3.6 S TATUS OF DTC (AS USED BY THE ECU POSITIVE RESPONSE MESSAGE) 
The Status Of DTC, used with the ECU positive response message, is used to report status information about 
each DTC, which has been identified at the time of the request, independent of their status. (i.e. stored, active, 
pending.)  Table 4.3.6-1 shows the bit configuration for the Status Of DTC response. 
Bit Description of Status Of DTC-Response 
3,2,1,0 Reserved.  Set to all zero’s.  (i.e. 0000b) 
4 DTC Readiness Flag 
This bit indicates whether the diagnostic trouble code test conditions have been met since the last successful Clear Diagnostic 
Information service. This bit allows for one bit to be available for service which indicates that the diagnostic test for a DTC has 
been completed, since DTC information was cleared. 
'0' = “Test Complete for this DTC” 
'1' = “Test Not Complete for this DTC” 
E.g. The DTC Readiness Flag of a kickdown switch of an Automatic Transmission Controller will only be set to the  Test 
Complete status if the kickdown switch is activated (full throttle) and if all other DTC diagnostic test criteria have been met during 
the driving cycle. The other Status Of DTC parameter values shall only be updated by the software of the ECU if the DTC 
Readiness Flag is set to Test Complete or if the diagnostic tool has successfully performed a Clear Diagnostic Information 
service. After a Clear Diagnostic Information service, the DTC Readiness Flag shall be set to Test Not Complete. 
6,5 DTC Storage State 
This bit field is state encoded and represents different diagnostic trouble code states.  The state diagram of the DTC Storage 
State parameter in Figure 5 shall provide more detail about the possible DTC Storage State conditions in the ECU's memory. 
'00' = “No DTC Detected at time of request”; 
no DTC is stored in non volatile memory; 
'01' = “DTC Not Present at time of request (Stored DTC)”; 
DTC is no longer present;  
DTC is stored in non volatile memory; 
'10' = “DTC Maturing-Pending at time of request” 
DTC has not met maturing criteria to become an active/stored DTC but failure has been detected; 
'11' = “DTC Present at time of request (Active DTC)”; 
DTC is present;  
DTC is stored in non volatile memory; 
7 Warning Indicator Request State 
This bit shall report the status of any warning indicators associated with a particular DTC.  Warning outputs may consist of 
indicator lamp(s), displayed text information, etc.  If no warning indicators exist for a given system or particular DTC, this status 
shall default to a logic ”0” state.  If the warning output is on for a given DTC, the stored status flag shall also be true.  
Reset to a logical ‘0’ after a call to Clear Diagnostic Information. Additional reset conditions defined by vehicle manufacturer / 
implementation. 
1 = Warning indicator requested to be ON. 
0 = Warning indicator not requested to be ON 
Table 4.3.6-1 Definition of Status Of DTC values 
```
