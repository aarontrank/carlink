---
title: American Honda Job Aid 06932 v3 (June 2017) — DLC Circuit Troubleshooting (HDS-to-vehicle F-CAN / K-Line communication)
source_url: https://static.nhtsa.gov/odi/tsbs/2017/MC-10116735-9999.pdf
fetched: 2026-11-08
publisher: American Honda Motor Co. (filed with NHTSA as MC-10116735-9999)
note: Full text of the 3-page PDF extracted with pypdf; figure-free, layout flattened.
---

=====PAGE 1
Job Aid
 2017 American Honda Motor Co., Inc. – All Rights Reserved 06932 Version 3 1 of 3
June 2017Applies To: See VEHICLES AFFECTED
DLC Circuit Troubleshooting
(Supersedes DLC Circuit Troubleshooting, dated March 2014, to revise the information marked by the
black bars)
REVISION SUMMARY
Under K-Line Only System Troubleshooting
Procedure, step 4 was changed.
BACKGROUND
Use this information to diagnose HDS-to-vehicle
communication issues involving vehicles equipped with
F-CAN or K-Line communication circuits.
NOTE: Under VEHICLES AFFECTED, all models are
equipped with a K-Line to DLC circuit, and some are
equipped with an additional F-CAN to DLC circuit.
See the information below to determine which
troubleshooting procedure to use.
VEHICLES AFFECTED
Equipped with both F-CAN and K-Line:
2008–14 Accord – ALL
2014 Accord Hybrid – ALL
2014 Accord Plug-In – ALL
2010–14 Crosstour – ALL
2006–14 Civic – ALL (includes Natural Gas and Si)
2006–14 Civic Hybrid – ALL
2007–14 CR-V – ALL
2011–14 CR-Z – ALL
2007–11 Element – ALL
2007–13 Fit – ALL
2013–14 Fit EV
2010–14 Insight – ALL
2007–14 Odyssey – ALL
2009–14 Pilot – ALL
2009–14 Ridgeline – ALL
2006–09 S2000 – ALL
Equipped with K-Line Only:
1998–07 Accord – ALL
2005–07 Accord Hybrid – ALL
2001–05 Civic – ALL (includes GX and Si)
2003–05 Civic Hybrid – ALL
2003–06 Element – ALL
2000–06 Insight – ALL
1999–06 Odyssey – ALL
2003–08 Pilot – ALL
2006–08 Ridgeline – ALL
2000–05 S2000 – ALL
F-CAN System Troubleshooting Procedure
NOTE: This procedure requires advanced
troubleshooting skills, including the ability to check
circuit power, continuity, and data network
communication.
1. Check the condition of the 12-volt battery and its
state-of-charge before proceeding. See Service
Bulletin 88-023, Battery Testing and Replacement.
2. Connect the HDS to the vehicle, and check for
DTCs.
Do you see HDS has been unable to confirm
communication to vehicle on the screen?
Yes – Go to step 3.
No   – If DTCs are set, continue with normal system
troubleshooting.
3. Connect the HDS to a known-good-vehicle, and
check for DTCs.
Does the HDS communicate with the vehicle?
Yes – Go to step 4.
No   – Check the HDS and MVCI for damage or
failure.
4. Use the electronic service manual to check if the
subject vehicle monitors DTC P0641 (sensor
reference voltage A malfunction) or P0651 (sensor
reference voltage B malfunction).
• If the subject vehicle monitors P0641 or P0651,
go to step 6.
• If the subject vehicle does not monitor P0641 or
P0651, go to step 5.
5. Disconnect the MAP sensor connector, then
measure the voltage on the VCC circuit.
• If the voltage is between 4.5 and 5.5 V, go to
step 6.
• If the voltage is under 4.5 V or over 5.5 V, go to
step 13.
6. Use the electronic service manual to identify all of
the control modules on the F-CAN circuit.
7. Disconnect each control module on the F-CAN
circuit, one at a time, and check if the HDS can
communicate with the vehicle, but do not
disconnect the PCM and gauge control module.
• If communication is restored when a control
module is disconnected, go to step 8.

=====PAGE 2
2 of 3
• If there is no communication when all of the
control modules are disconnected, except the
PCM and gauge control module, go to step 9.
8. Check the power circuit for battery voltage and
ground circuit for continuity on the control module
that interrupted communication. Do a voltage drop
test to check for excessive circuit resistance.
Does the control module have power and ground?
Yes – Replace the control module.
No   – Repair the faulty circuit.
9. Check the F-CAN H and F-CAN L circuits for
continuity between the PCM and the gauge control
module.
Do both circuits have continuity?
Yes – Go to step 10.
No   – Repair the faulty circuit.
10. Check the F-CAN H and F-CAN L circuits for
continuity to body ground.
Does either circuit have continuity to ground?
Yes – Repair the short to ground in the affected
circuit.
No   – Go to step 11.
11. Replace the PCM with a known-good one.
Is communication restored?
Yes – Replace the PCM.
No   – Go to step 12.
12. Replace the gauge control module with a
known-good one.
Is communication restored?
Yes – Replace the gauge control module.
No   – Check the gauge control module power circuit
for battery voltage and ground circuits for
continuity. Do a voltage drop test to check for
excessive circuit resistance.
13. Use the electronic service manual to identify all of
the sensors using the 5-volt VCC circuit.
Disconnect each sensor, one at a time, and check
for HDS-to-vehicle communication.
• If communication is restored when a sensor is
disconnected, go to step 14.
• If there is no communication when all of the
sensors in the circuit are disconnected, go to
step 15.
14. Replace the sensor that interrupted
communication.
15. Check the signal wire on any disconnected sensor
connector for continuity to body ground.
Is there continuity on any signal circuit?
Yes – Repair the short to ground.
No   – Go to step 16.
16. Measure the IG1 circuit voltage at the PCM
connector. Check the 12-volt battery state-of-
charge again before doing this step.
• If the reading is less than 11.5 V, repair the PCM
IG1 circuit.
• If the reading is 11.5 V or more, go to step 17.
17. Check for continuity to body ground at each PCM
ground terminal.
Do all ground terminals have continuity?
Yes – Replace the PCM.
No   – Repair the affected ground circuit.
K-Line Only System Troubleshooting Procedure
NOTE: This procedure requires advanced
troubleshooting skills, including the ability to confirm
circuit power, continuity, and data network
communication.
1. Check the condition of the 12-volt battery and its
state-of-charge before proceeding. See Service
Bulletin 88-023, Battery Testing and Replacement.
2. Connect the HDS to the vehicle, and check for
DTCs.
Do you see HDS has been unable to confirm
communication to vehicle on the screen?
Yes – Go to step 3.
No   – If DTCs are set, continue with normal system
troubleshooting.
3. Connect the HDS to a known good-vehicle.
Does the HDS communicate with the vehicle?
Yes – Go to step 4.
No   – Check the HDS and MVCI for damage or
failure.
4. Measure the voltage on the MAP sensor VCC
circuit.
• If the voltage is between 4.5 and 5.5 V, go to
step 5.
•If the voltage is under 4.5 V or over 5.5 V, go to
step 11.
5. Use the electronic service manual to identify all of
the control modules on the K-Line circuit.
6. Disconnect each control module on the K-Line
circuit, one at a time, and check if the HDS can
communicate with the vehicle, but do not
disconnect the PCM.
• Check for communication after each module is
disconnected. If communication is restored, go to
step 7.
• If there is no communication with the PCM
connected, go to step 8.
=====PAGE 3
3 of 3
7. Check the power circuit for battery voltage and
ground circuits for continuity on the control module
that interrupted communication. Do a voltage drop
test to check for excessive circuit resistance.
Does the control module have power and ground?
Yes – Replace the control module.
No   – Repair the faulty power or ground circuit to the
control module.
8. Check the K-Line circuit for continuity between the
PCM and gauge control module.
Does the circuit have continuity?
Yes – Go to step 9.
No   – Repair the open in the affected circuit.
9. Check the K-Line circuit for continuity to body
ground.
Does the circuit have continuity to ground?
Yes – Repair the short to ground in the affected
circuit.
No   – Go to step 10.
10. Replace the PCM with a known-good one.
Is communication restored?
Yes – Replace the PCM.
No   – Go to step 11.
11. Disconnect all of the sensors using the 5-volt VCC
circuit, one at a time.
• After you disconnect each sensor, check for
communication. If communication is restored,
go to step 12. If there is no communication,
disconnect the next sensor.
• If there is no communication when all of the
sensors are disconnected, go to step 13.
12. Replace the sensor that interrupted
communication.
13. Check the signal wire on any disconnected sensor
connector for continuity to body ground.
NOTE: Make sure the PCM and gauge control
module are connected during this step.
Is there continuity on any signal circuit?
Yes – Repair the short to ground.
No   – Go to step 14.
14. Measure the IG1 circuit voltage at the PCM
connector. Check the 12-volt battery state-of-
charge again before doing this step.
• If the reading is 11.4 V or less, repair the PCM
IG1 circuit.
• If the reading is 11.5 V or more, go to step 15.
15. Check for continuity to body ground for each PCM
ground terminal.
Do all ground terminals have continuity?
Yes – Replace the PCM.
No   – Repair the affected ground circuit.