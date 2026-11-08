---
title: Snap-on Global OBD Vehicle Communication Software Manual (EAZ0025B43C Rev. A, Aug 2013) — service $05/$06 excerpt
source_url: https://www.snapon.com/Files/Diagnostics/UserManuals/GlobalOBDVehicleCommunicationSoftwareManual_EAZ0025B43.pdf
fetched: 2026-11-08
publisher: Snap-on
note: Excerpt (manual pp. 12–16) extracted with pypdf; figures lost.
---

For safety and/or technical design reasons, some ECMs may not respond to this service under all 
conditions.
NOTE:
i In most cases, once a MIL is deactivated, the original code will remain in memory until forty to 
eighty warm-up cycles are completed without the fault reappearing.
1.6.5  Service $05: Oxygen Sensor Monitoring Test Results
The purpose of this service is to allow access to the on-board oxygen sensor monitoring test 
results (if implemented).
NOTE:
i For CAN vehicles, Service $05 is not supported. Oxygen sensor test results are implemented in 
Service $06. For non-CAN vehicles, manufacturers can use Service $06 as an alternative to 
Service $05.
Different manufacturers may use many methods to calculate test results for this service. The scan 
tool converts test values and displays them in the standard units of measurement.
The ECM transmits the latest test results available from the system. Test results are retained by 
the ECM, even over multiple ignition OFF cycles, until replaced by the results of a more recent 
test. Test results are requested by Test ID. (See Test ID Table).
Test results can be reported either as a constant or as a calculated value, depending on the Test 
ID. Calculated values are reported with minimum and maximum limits.
NOTE:
i Depending on the manufacturer, calculated values may be rounded off, which means that a test 
may fail despite the values reading within minimum and maximum limits.
$03 Confirmed diagnostic trouble codes
$05 Oxygen sensor test data
$06 Specific on-board monitoring test results
$07 Pending diagnostic trouble codes
Table 1-4 Reset Information (sheet 2 of 2)
Service Items to reset
===== page 16 =====
12
Introduction Diagnostic Service Definitions
The scan tool checks if the readiness status of oxygen sensors is completed. If the test is not 
completed, the scan tool will display a warning.
Figure 1-4 Test ID Values Example
NOTE:
i Refer to the table below for an explanation of the individual Test ID numbers.
NOTE:
i Not all test values are applied on or supported by all vehicles. Only supported Test IDs will come 
up in the selection list.
$06 $05
$07
$08
$0A $09
$04
$03
$02 $01
Rich
Lean
Oxygen
Sensor
Output
Numbers Refer to Test ID
Table 1-5 Test ID Table
Test ID No. Description
$01 Rich to Lean sensor threshold voltage (constant)
$02 Lean to Rich sensor threshold voltage (constant)
$03 Low sensor voltage for switch time calculation (constant)
$04 High sensor voltage for switch time calculation (constant)
$05 Rich to Lean sensor switch time (calculated)
$06 Lean to Rich sensor switch time (calculated)
$07 Minimum sensor voltage for test cycle (calculated)
$08 Maximum sensor voltage for test cycle (calculated)
$09 Time between sensor transitions (calculated)
$0A Sensor Period (calculated)
$0B
EWMA (Exponential Weighted Moving Average) 
misfire counts for last 10 driving cycles (calculated)
Calculation: 0.1 * (current counts) + 0.9 * (previous 
average)
Initial value for (previous average) = 0
$0C Misfire counts for last/current driving cycles (calculated)
===== page 17 =====
13
Introduction Diagnostic Service Definitions

1.6.6  Service $06: Specific Monitored Systems
The purpose of this service is to allow access to the test results for on-board diagnostic monitoring 
tests of specific components or systems that are:
• Continuously monitored (CAN only)
• Not continuously monitored.
CAN Protocol
For CAN the Test ID Table from Service $05 is used. The request message for test values includes 
an OBDMID (On-Board Diagnostic Monitor IDentification) that indicates the information 
requested. Unit information is given with the values. Test values (results) are always reported with 
the minimum and maximum test limits.
IMPORTANT:
The Test ID Table below is for CAN Protocol only:
If an OBDMID has not been completed at least once since Service $04 or battery disconnect, then 
the following properties of the parameter are set to zero by the ECM:
• Test Value (Results)
• Minimum Test Limit
• Maximum Test Limit
Table 1-6 Test ID Table for CAN Protocol
Test ID No. Description
$01 Rich to Lean sensor threshold voltage, (Constant)
$02 Lean to Rich sensor threshold voltage, (Constant)
$03 Low sensor voltage for switch time calculation, (Constant)
$04 High sensor voltage for switch time calculation, (Constant)
$05 Rich to Lean sensor switch time, (Calculated)
$06 Lean to Rich sensor switch time, (Calculated)
$07 Minimum sensor voltage for test cycle, (Calculated)
$08 Maximum sensor voltage for test cycle, (Calculated)
$09 Time between sensor transitions, (Calculated)
$0A Sensor Period, (Calculated)
$0B
EWMA (Exponential Weighted Moving Average) 
misfire counts for last 10 driving cycles (calculated)
Calculation: 0.1 * (current counts) + 0.9 * (previous 
average)
Initial value for (previous average) = 0
$0C Misfire counts for last/current driving cycles (calculated)
===== page 20 =====
16
Introduction Diagnostic Service Definitions
Non CAN Protocol
The vehicle manufacturer is responsible for assigning Test IDs and Component IDs for tests of 
different systems and components. The latest test results are to be retained, even over multiple 
ignition OFF cycles, until replaced by more recent test results. Test results are requested by Test 
ID. Test results are reported only for supported combinations of test limit type and component ID 
and are reported as positive (unassigned) values. Service $06 test values and limits are raw 
values that not scaled.
To date, only some manufacturers have provided conversion factors for technicians to utilize this 
data. See the example below for a US Ford version of Service $06 Catalytic Converter test results.
Ford conversion factor: multiply the value by 0.0156 to get a value from 0 to 1.0.
This catalyst is about to fail. Normal readings for a good catalyst should be approximately 
0t o0 . 1( s w i t c h  r a t i o ) .
Service $06 Notes
Consider the following when performing this service:
• If in Service $01 (Readiness Test Status) one of the tests is not completed, a warning 
message displays indicating that the values may be invalid or not available.
• Not all test values are applicable or supported by all vehicles. Only supported Test IDs appear 
on the selection list. The last item of the selection list is Manual Selection of Test ID, which is 
used to select non-supported Test IDs.
• This service can be used as an alternative to Service $05 to report oxygen sensor test results 
The values are not converted and are displayed in standard units.
