---
title: Hyundai TSB 23-01-073H-1 — Kona EV DTC inspection and BMS update (Service Campaign 9A4)
source_url: https://static.nhtsa.gov/odi/tsbs/2023/MC-10244989-0001.pdf
fetched: 2026-11-08
publisher: Hyundai Motor America (TSB filed with NHTSA as MC-10244989-0001)
note: Text of PDF pages 1–5 extracted with pypdf (tables and figures lose their layout). Pages 6–10 are BMS ROM-ID tables, parts and warranty tables, and are omitted.
---

=====PAGE 1
Circulate To:  Service Manager, Warranty Manager, Service Advisors, Technicians, Fleet Repair 

Technical Service Bulletin 
GROUP NUMBER 
CAMPAIGN 23-01-073H-1 
DATE MODEL(S) 
October 2023 KONA ELECTRIC (OS EV) 
                     KONA EV DTC INSPECTION AND BMS UPDATE (SERVICE CAMPAIGN 9A4) 
This TSB supersedes TSB 23-01-073H with updated part tables for 19-20 MY vehicles.  

Description:  Certain 19-23MY Kona Electric (OS EV) vehicles may experience a condition of EV 
warning light ON with DTC P1AA6 to P1AAF found stored.  The Battery Management System (BMS) 
software update will be performed to enhance battery diagnostics logic.  
Flowchart summary of the service procedure is provided on page 2 summarized as follows:   
• Vehicles that do not have EV Warning Light On will only need the BMS software update.   
• Certain vehicles incurring EV Warning Light On and DTC P1AA6 found stored with Freeze Frame 
Data Information Code 1=128 will only require the BMS Update.   
• Certain other DTC P1AA6 Information Code 1-2 values or DTC P1AA7 to P1AAF will require the 
Battery System Assembly (BSA) to be replaced and then the BMS Update to be performed. 
Applicable Vehicles (Certain): 
• 2019-23MY Kona Electric (OS EV) produced from 8/28/2018 to 3/8/2023 

GDS Information: 
System Event # Description 
BMS 999* OS EV NEW LOGIC (RDV) BMS UPGRADE 
* or select the latest available Event # that displays in GDS. 

Warranty Information: 
MODEL OP CODE OP NAME CAUSAL PART OP TIME NATURE CAUSE 
Kona 
Electric  
(OS EV) 
30D055R1 
DTC (INFO CODE) CHECK AND 
BATTERY MANAGEMENT 
SYSTEM SOFTWARE UPDATE 
375A0-
K4000 0.4 M/H I3A ZZ3 
Refer to 
pages 9-10 

BSA REPLACE 
Refer to 
Pages 9-10 
Refer to 
pages 9-10 
NOTE 1: The BMS software update and if necessary BSA replace are to be submitted on the Claim 
Entry Screen as "Campaign" type.  
NOTE 2: If a part is found in need of replacement while performing this campaign and the affected 
part is still under warranty, submit a separate claim using the same repair order. If the affected part is 
out of warranty, submit a Prior Approval request for goodwill consideration prior to performing work. 
NOTE 3: This TSB includes Repair validation photos. Op times including VIN, Mileage, and Repair 
validation photos as outlined in the Digital Documentation Policy. 
NOTE 4: Each BSA replacement labor op code will include reimbursement for bolts, washers, and 
coolant. Please refer to TSB 23-EV-001H parts information chart for the applicable part numbers.' 

SUBJECT: 

Dealers must perform this Service Campaign on all affected vehicles prior to customer retail 
delivery and whenever an affected vehicle is in the shop for any maintenance or repair. 
Access the “Vehicle Information” screen via WEBDCS to identify open campaigns. 
 IMPORTANT

=====PAGE 2
KONA EV DTC INSPECTION AND BMS UPDATE (SERVICE CAMPAIGN 9A4)  
TSB #:  23-01-073H-1 Page 2 of 10 
 SUBJECT: 
SUBJECT  
Service Procedure: 

Table Of Contents: 
Section Pages Description 
A 4-5 DTC CHECK 
B 6-8 BMS SOFTWARE UPDATE   
C 8-10 BSA REPLACE (only when applies) 

Summary of Service Procedure Flow: 

This TSB includes Repair validation photos. Refer to the latest Warranty Digital Documentation 
Policy for requirements. 
STUI

=====PAGE 3
KONA EV DTC INSPECTION AND BMS UPDATE (SERVICE CAMPAIGN 9A4) 

TSB #:  23-01-073H-1 Page 3 of 10 
 SUBJECT: 
SUBJECT:    
A.  DTC CHECK   
A1. 

A2. 

A3. 

Perform BMS system Fault Search by GDS. 

Check for any of DTC P1AA6, P1AA7, P1AA8, P1AA9, P1AAA, P1AAB, P1AAC, P1AAD, 
P1AAE or P1AAF found stored as an active or history DTC? 

➢ No – (None of the above listed DTC are found stored): 
Go to section B - Perform BMS Software Update. 

➢ Yes – (DTC P1AA6 is found stored): 
Go to next step A3. - Check the DTC P1AA6 Freeze Frame Information Code 1-2 
values to determine next steps.  

➢ Yes – (Any of DTC P1AA7, P1AA8, P1AA9, P1AAA, P1AAB, P1AAC, P1AAD, P1AAE 
or P1AAF are found stored):   
Go to section C. - Take STUI photo of Freeze Frame Data, Replace BSA and perform 
BMS Update if needed. 

Check the DTC P1AA6 Freeze Frame Data and locate Information Code1-2:

=====PAGE 4
KONA EV DTC INSPECTION AND BMS UPDATE (SERVICE CAMPAIGN 9A4)  
TSB #:  23-01-073H-1 Page 4 of 10 
 SUBJECT: 
SUBJECT  
A4. 

Find the DTC in the chart below, 

….. then if DTC P1AA6 also check the Information Codes1-2 values, 

………. then find the corresponding value in the last column “BSA Replace?”: 

➢ “BSA Replace?” = NO – Go to section B. (BMS Update only) 

➢ “BSA Replace?” = YES – Go to section C. (BSA Replace and BMS Update) 

➢ “BSA Replace?” = Techline Review (Whether or not BSA is to be replaced will be 
decided after Techline review of the BMS Data with Engineering). 

Battery Incident 
 Condition 
Early BMS Software Later BMS Software BSA Replace? 
DTC Info  code 1 Info  code 2 DTC 
Cell voltage deviation err
or 
P1AA6
00 
1 - P1AA700 YES 
Insulation resistance erro
r 2 - P1AA800 YES 
Abnormal cell voltage ch
ange after charging 4~64 - P1AA900 YES 
Abnormal current change 
during charging (CV secti
on) 
128 
- P1AAA00 
P1AA600- NO 

P1AAA00 
*Techline review  
Overtemperature - 1 P1AAB00 YES 
Overvoltage - 2 P1AAC00 YES 
Low voltage - 4 P1AAD00 YES 
Battery temperature  
deviation error - 8 P1AAE00 YES 
MAVD - 16 P1AAF00 YES 
RDV - - - P1AB000 *Techline review 

NOTE: *Techline review with Engineering to decide if BSA replace based on BMS data

=====PAGE 5
KONA EV DTC INSPECTION AND BMS UPDATE (SERVICE CAMPAIGN 9A4) 

TSB #:  23-01-073H-1 Page 5 of 10 
 SUBJECT: 
SUBJECT:    
B.  BMS SOFTWARE UPDATE   
B1.  

B2. Perform the BMS Update in Auto Mode. 

Use the Auto Mode ID Check to verify the BMS 
ROM ID before updating the software. 

GDS Vehicle Battery Voltage: 
• If the voltage is below 12 volts as per the GDS warning, then select Back and run 
the vehicle at least 20 minutes to reach an adequate battery state of charge to 
prevent an ECU Update failure. Cycle the ignition OFF/ON before retrying the ECU 
update again.  

WARNING 
! 

GDS ECU Updates must be performed in Auto Mode. 
• If ECU Update starts but then fails in Auto Mode, perform update in Manual Mode 
to recover. 
• Turn off ignition for 30 seconds and try Manual Mode recovery. 
• If unsuccessful, re-attempt Manual Mode after disconnecting the cables from the 
12V battery terminals and performing cap discharge (connecting bar or heavy wire 
between “+” to “-“ cables). 
  Information 
 i
