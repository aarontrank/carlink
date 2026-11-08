---
title: AA1Car — "OBD II Mode $06 Diagnostics" (Larry Carley)
source_url: https://www.aa1car.com/library/2005/us010516.htm
fetched: 2026-11-08
publisher: AA1Car Automotive Diagnostic & Repair Help
note: Article body extracted from HTML (site navigation removed). It links the Honda MY2005 mode $06 PDF saved as raw/honda/honda-mode06-2005-all-models.pdf.
---

by Larry Carley copyright AA1Car.com
Mode $06 is an advanced diagnostic mode within the onboard diagnostic (OBD II) system on 1996 and newer vehicles that keeps watch on how all of the sensors and other emission control components are functioning. It can only be accessed with a professional grade scan tool, and requires scan tool software that can convert the hexadecimal (16-bit computer code) into ordinary numbers and values. In other words, it is an advanced diagnostic mode that can reveal the inner workings of the OBD II system, and can tell you when a code is going to set even before it sets the code and turns on the Check Engine light. An inexpensive code reader or basic scan tool can't access or read mode $06 data.
Normally, most technicians ignore mode $06. On older scan tools, it was very difficult to access. And if they found it, the hexadecimal code looked like gibberish. You had to use conversion tables to translate the code into ordinary numbers, and you had to find the vehicle manufacturer's reference codes to decipher what each line meant. So it was a pain to use, and many technicians didn't even know it existed.
As time went on, scan tool manufacturers began to add software to their tools that made Mode $06 easier to find a use. Many professional grade scan tools now display all of the mode $06 data in plain English with units of measure for various types of data (pressure, temperature, etc.). Some will even flag in red values that are out of range, making it much easier to spot problems that may set a code.
The greatest value of mode $06 from a diagnostic standpoint is that it can verify repairs have fixed a problem without having to wait days for certain OBD II self-tests to run.
 To make sure a vehicle's emissions do not exceed federal limits, the OBD II system runs various self-checks (called "monitors") while the vehicle is being driven. Some of these monitors are "continuous" and monitor certain sensors and systems all the time. Other monitors (called "non-continuous" monitors) only run after certain operating conditions have been met. Consequently, non-continuous monitors may be slow to complete their self-checks, and may not set a fault code for several days or even weeks after a fault occurs. So being able to verify a repair has fixed a problem without having to wait for the Check Engine light to maybe come back on is a god way for repair shops to reduce comebacks and unhappy customers.
Mode $06 is most useful for checking misfires on Fords, and for checking the catalytic converter and evaporative emissions (EVAP) system on most cars. The latter two are non-continuous monitors and can be very slow to complete depending on driving conditions.
Using Mode $06 to Diagnose Misfires
Every time a cylinder misfires, OBD II adds one number to a running count of misfires for that cylinder. When the rate of misfire in any cylinder exceeds a certain threshold (typically around two percent), it will usually set a misfire code for that cylinder and turn on or flash the Check Engine light. If the misfire rate is less than about two percent, however, it usually will NOT set a code. Even so, the misfire may still be noticeable enough that you can feel it when the engine is under load or accelerating.
 By using your scan tool to access Mode $06 data, you can read the actual misfire counts that have been recorded for each cylinder. Note: This may require looking up the Mode $06 test reference code for the cylinder misfire data unless your scan tool translates that information for you. For example, on a Ford, the cylinder misfire data may be listed under TID $053. Each cylinder is identified as Component $01, $02, $03, etc., where the $ number corresponds to the cylinder number in the engine's firing order.
Normally, the misfire count should be zero or very close to zero for each cylinder if there are no problems. If there have only been a few occasional misfires (say less than 100 over a given time period), that is usually acceptable. But if you see a relatively high count (say more than several hundred or a thousand misfires for a given cylinder), it would tell you the cylinder is experiencing an abnormal misfire rate and there is a problem with ignition, fuel or compression in that cylinder.
For additional information on analyzing misfires, see the following:
Misfire Analysis
Misfire Diagnosis
Misfires (causes of P0300 random misfire code)
Misfire Diagnosis Chrysler 3.5L V6
How Mode $06 Data is Displayed
On older professional grade scan tools, all of the mode $06 data is in hex code with no units of measure defined. On newer professional grade scan tools, the software will often translate the Mode $06 data into familiar terms and units of measure. The data is listed in three columns:
* "TID" stands for Test identification. This is the sensor or component that is being monitored on that line.
* "CID" stands for Component identification. This is the test result for that sensor or component.
* PASS or FAIL in the last column indicates if the value is within range or out of range (less than the minimum acceptable value or greater than the maximum acceptable value for that sensor or component).
 Translating Mode $06 Hex Code
If a scan tool does not translate the mode $06 hex code, tables that list the TID and CID definitions must be used to make sense of the data. Not all vehicle manufacturers publish their mode $06 information. Ford and GM's mode $06 information can be found on the vehicle manufacturer's service information websites, and on the International Automotive Technicians Network website (www.iatn.net). 
For GM Mode 06 charts, Click Here.
Additional Mode 06 Diagnostic Information:
Mode 06 parameters for GM 4.6L engine (PDF file)
 Honda Mode 06 info for MY 2005 (PDF file)
Maximizing Your Scan Tool with Mode 06 (Vehicle Service Pros)
 More OBD II Diagnostic Articles:
