---
title: CARB 13 CCR §1968.2 (OBD II, 2006 amendments as filed) — excerpts on misfire definition and service $06 test results
source_url: https://ww2.arb.ca.gov/sites/default/files/barcu/regact/obdii06/1968_2oal.pdf
fetched: 2026-11-08
publisher: California Air Resources Board
note: >
  Text extracted with pypdf from the OAL-filed amendment document, so strike-through/underline
  edit marks are lost and some old/new words run together (e.g. "(f)(g)(3.4)", "systemstest").
  Later amendments apply to a 2016 vehicle; these provisions are the ones the 2016 Odyssey's
  service $06 behaviour derives from. Line breaks as extracted.
---

## Definition (c)(11) — engine misfire

(11)“Engine misfire”“Engine misfire” means lack of combustion in the cylinder 
due to absence of spark, poor fuel metering, poor compression, or any other cause. 
This does not include lack of combustion events in non-active cylinders due to 
default fuel shut-off or cylinder deactivation strategies. 

## (e)(3.2) — misfire malfunction criteria (opening lines)

(3.2) Malfunction Criteria: The OBD II system shall detect a misfire malfunction 
pursuant to the following: 
(3.2.1) Misfire causing catalyst damage: 
(A) Manufacturers shall determine the percentage of misfire evaluated in 200 
revolution increments for each engine speed and load condition that 
would result in a temperature that causes catalyst damage. The 
manufacturer shall submit documentation to support this percentage of 
30 
…
(3.2.2) Misfire causing emissions to exceed 1.5 times the FTP standards: 
(A) Manufacturers shall determine the percentage of misfire evaluated in 
1000 revolution increments that would cause emissions from an emission 
durability demonstration vehicle to exceed 1.5 times any of the applicable 
FTP standards if the percentage of misfire were present from the 
beginning of the test. To establish this percentage of misfire, the 
manufacturer shall utilize misfire events occurring at equally spaced, 
complete engine cycle intervals, across randomly selected cylinders 
throughout each 1000-revolution increment. If this percentage of misfire 
is determined to be lower than one percent, the manufacturer may set the 
malfunction criteria at one percent. 

## (g)(4.5) Test Results

(4.5) Test Results 
(4.5.1) For all monitored components and systems for gasoline engine vehicles 
identified in sections (e)(1) through (e)(8) except misfire detection, and 
fuel system monitoring, and oxygen sensor circuit and out-of-range 
monitoring, and for all monitored components and systems for diesel 
engine vehicles identified in sections (f)(1) through (f)(9) except those 
required to be monitored continuously, results of the most recent 
monitoring of the components and systems and the test limits established 
for monitoring the respective components and systems shall be stored 
and available through the data link in accordance with SAE J1979 
specifications. 
(4.5.2) The test results shall be reported such that properly functioning 
components and systems (e.g., “passing” systems) do not store test 
values outside of the established test limits. 
(4.5.3) The test results shall be stored until updated by a more recent valid test 
result or the fault memory of the OBD II system computer is cleared. 
Upon fault memory being cleared, test results reported for monitors that 
have not yet completed since the last time the fault memory was cleared 
shall report values that do not indicate a failure (i.e., a test value which is 
outside of the test limits). 
(4.5.4) Additionally, for vehicles using ISO 15765-4 (see section (f)(g)(3.4)) as the 
communication protocol: 
(A) The test results and limits shall be made available in the standardized 
format specified in SAE J1979 for the ISO 15765-4 protocol. Test results 
using vehicle manufacturer-defined monitor identifications (i.e., SAE 
J1979 OBDMIDs in the range of $E1-$FF) may not be used. 
(B) Test limits shall include both minimum and maximum acceptable values 
and shall be reported for all monitored components and systemstest 
results required identified in section (g)(4.5.1)s (e)(1) through (e)(8) , 
except fuel system monitoring. The test limits shall be defined so that a 
VVT systems, VVT monitoring test results and limits shall be stored and 
available in the standardized format specified in SAE J1979. 
(D) Monitors that have not yet completed since the last time the fault memory 
was cleared shall report values of zero for the test result and test limits. 
(E) All test results and test limits shall always be reported and the test results 
shall be stored until updated by a more recent valid test result or the fault 
memory of the OBD II system computer is cleared. For monitors with 
multiple pass/fail criteria (e.g., a purge flow diagnostic that can pass upon 
seeing a rich shift, lean shift, or engine speed change), on 25 percent of 
2009, 50 percent of 2010, and 100 percent of 2011 and subsequent 
model year vehicles, only the test results used in the most recent decision 
shall be reported with valid results and limits while test results not used in 
the most recent decision shall report values of zero for the test results and 
limits (e.g., a purge flow monitoring event that passed based on seeing a 
rich shift shall report the results and the limits of the rich shift test and 
shall report values of zero for the results and limits of the lean shift and 
engine speed change tests). 
(F) The OBD II system shall store and report unique test results for each 
separate diagnostic (e.g., an OBD II system with individual evaporative 
system diagnostics for 0.040 inch and 0.020 inch leaks shall separately 
report 0.040 inch and 0.020 inch test results). 
(4.6) Software Calibration Identification: 
