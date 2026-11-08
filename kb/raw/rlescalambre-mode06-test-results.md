---
title: RLEscalambre — "$06 Non-Continuous/Continuous Monitor Test Results"
source_url: https://www.rlescalambre.net/06-non-continuous-continuous-monitor-test-results
fetched: 2026-11-08
note: Training-site article; text extracted from HTML, charts (images) not included.
---

The ECM compares test results to the limits and reports a Pass or Fail result to the scan tool for each monitored system and component. This MODE will report results in one trip if the monitor runs. They’re stored and are not live values. To review a test result, you must exit the MODE and reselect it to update the reading. A PC-based scan tool, like ESCAN and Silver Scan Tools, constantly queries MODE $06 for the latest changes. In most cases, it is not necessary to constantly check the test results, as they change only once per trip. This constant MODE $06 query can help monitor misfires during the current drive cycle. 
How can it help diagnose vehicle emission-related concerns? MODE $06 monitor test results can help confirm the success of repairs for a non-continuous system. MODE $06 test values, even for a pending DTC, are available to the technician on a two-trip monitor’s first trip. MODE $06 test results can indicate if a monitored system (component) is close to failing a monitor test. It is important to note that a confirmed DTC should be present before replacing any parts for the monitor test in question, and that further testing is required.
If a TSB indicates that a reflash is available to fix a monitor-related issue, checking MODE $06 before and after the reflash can show the change in the Pass/Fail limits. Here are two examples where the limit was changed during the reprogramming process:
Before Reflash:
After Reflash:
Before Reflash:
After Reflash:
MODE $06 and non-Continuous Monitor Tests
MODE $06 for CAN-equipped systems includes continuous monitor test results for the Misfire and Fuel System monitor. 
The Misfire Monitor includes two test results per cylinder. MID A2 represents Cylinder #1, and MID A7 represents Cylinder #6.
Misfire average for the last test drive cycles (MID A2 - $OB).
Misfires for the current key cycle (MID A2 - $OC)
Note: MID A1 OC is the cumulative total of misfires for all cylinders for the current drive cycle.
This can be invaluable for intermittent misfires because it reports them before they become severe enough to trigger the MIL. This will be covered later in this section.
The Fuel System Monitor includes test results per engine bank.  TID $81 represents Cylinder #1. This will be covered later in this section.
MODE $06 Terminology
Pre-CAN Systems: 
TID = Test Identification – The system (EVAP, CAT, O2, EGR, etc.) being tested. 
CID = Component Identification – The component or area of the system being tested. It is no longer used for CAN MODE $06.
The problem with pre-CAN MODE $06 was that TID/CID was not standardized. The following chart shows the TID/CID for a Toyota/Lexus. These numbers will be completely different when looking at other manufacturers.
This is how GM reported pre-CAN MODE $06 information for the Catalytic Converter (Click here to link to GM 2 MODE $06):
CAN "C" Equipped Systems: 
MID = Monitor Identification: The non-continuous system (EVAP, CAT, O2, EGR, VVT, AIR) or continuous system (Misfire, Fuel system, etc.) being tested. 
TID = Test Identification – Specific monitor test being tested.
The chart shown below identifies MIDS for CAN "C" systems. Regardless of the manufacturer, these MIDS are standardized for all vehicles. Some scan tools will report them by MID in Hex, rather than by the actual Monitor ID name. Some scan tools will report them using the MID and the Monitor name. Regardless, this chart applies to all CAN "C" vehicles. 
RELATED TERMINOLOGY: 
Test Limit Type: To pass a test, the value must be either a minimum or maximum value (or between a min/max value) 
Hexadecimal ($): Numeric/Alpha unit that indicates a specific TID/CID or test value (Example: $02) 
Raw Data: Test data in a decimal number indicates the actual test results. 
Manufacturer’s Conversion: A value supplied by the manufacturer to convert this data to values that can be used to diagnose a system (volts, Ohms, amps, inches of mercury, etc.). 
Test Value: = Actual test results. 
Test Result: = Indicates whether the system/component passed or failed a test.
MODE $06 Reporting Formats
Pre-CAN systems: Here is how the PCM and Scan Tool report the Test Results. 
Step 1: The PCM communicates in Hexadecimal, a combination of numbers and letters.
Step 2:  The Scan Tool reports the results in Decimal format. Global OBDII does not show the test's name or the actual result values. Viewing MODE $06 through the enhanced (OEM) side of the scan tool might provide the actual name of the monitor test.
Step 3:  The scan tool reports the monitor test name and unit value. Before CAN, this required using a manufacturer’s scan tool because unit conversion numbers differed. There was no way for an aftermarket scan tool manufacturer to keep up with the changes.
When working with Pre-CAN MODE $06, there are things to be aware of to avoid being fooled. The things to look out for are:
There was no standardization for the TIDs, MIDs, and CIDs. Example: Toyota/Lexus identified the Bank 1 Catalytic Converter as TID $01, Ford identified it as TID $10, Honda/Acura identified it as TID $63.
After a MODE $04, Memory Clear, the manufacturer could display values that were not real, and that can be misleading to the technician.
Ford was known for displaying a value that appeared believable, but it could be misleading if the technician were not aware of the issue.
Note: In the above capture, the readiness status for the Catalytic Converter was Incomplete. This means the monitor tests have not run. The customer was advised to replace both Catalytic Converters and decided to replace the B1 CAT.  When the Catalytic Converter Monitor Tests ran and were completed, the actual MODE $06 values are shown on the right. Question: Did one or both of the Catalytic Converters need to be replaced?  The answer is NO! Both the replacement CAT and the original CAT were passed by a lot during the actual Monitor Tests. The technician was unaware of Ford's pre-CAN MODE $06 default values.
GM and Ford were known to display a Failed Test Result even though the monitor test had not run. This could be identified by the lack of a Pending or Confirmed DTC. In the capture below, no Pending or Confirmed DTC was present.
CAN Equipped Systems:
Monitor Identification (MIDs) are standardized across all platforms. Example: All manufacturers must identify the Bank 1 Catalytic Converter as MID $21.
There is no standardization of the Test Identification (TID), which defines how each specific monitor test is to be accomplished. 
After a MODE $04, Memory Clear, the default for the Minn/Max/Values are all set to zero. When this happens, the Test Result will display Incomplete. This means the specific monitor test has not run or was not completed. 
