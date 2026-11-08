---
title: 2004 Honda CR-V service manual (Operation CHARM) — reading ABS and SRS trouble codes; DLC connector view
source_url: https://charm.li/Honda%20Truck/2004/CR-V%204WD%20L4-2.4L/Repair%20and%20Diagnosis/
fetched: 2026-11-08
note: Page text extracted from the HTML; section paths given per page. The DLC connector view is an image (saved as raw/assets/crv-2004-dlc-connector-view.png) and is transcribed below.
---

## Brakes and Traction Control/Antilock Brakes / Traction Control Systems/Testing and Inspection/Reading and Clearing Diagnostic Trouble Codes/Reading Diagnostic Trouble Codes/With Generic Scan Tool/

```
With Generic Scan Tool Honda does not provide instructions for retrieving ABS DTCs with a generic scan tool. Refer to the scan tool's operation manual. 
```

## Brakes and Traction Control/Antilock Brakes / Traction Control Systems/Testing and Inspection/Reading and Clearing Diagnostic Trouble Codes/Reading Diagnostic Trouble Codes/With Manufacturer's Scan Tool/

```
With Manufacturer's Scan Tool HOW TO RETRIEVE ABS DTCs 
 Honda PGM Tester Method: 
 1. With the ignition switch OFF, connect the Honda PGM Tester (A) to the 16P Data Link Connector (DLC) (B) under the driver's side of the dashboard next to the accelerator pedal.
 2. Turn the ignition switch ON (II), and follow the prompts on the PGM Tester to display the DTC(s) on the screen. After determining the DTC, refer to the DTC Troubleshooting Index.
 NOTE: See the Honda PGM Tester user's manual for specific instructions.
 Service Check Signal (SCS) Circuit Method: 
 1. With the ignition switch OFF, connect the Honda PGM Tester (A) to the 16P Data Link Connector (DLC) (B) under the driver's side of the dashboard next to the accelerator pedal.
 2. Short the SCS circuit to body ground using the Honda PGM Tester.
 3. Turn the ignition switch ON (II) without the brake pedal pressed.
 NOTE: If the brake pedal is pressed when turning the ignition switch ON (II), the system shifts to the DTC clearing mode.
 4. The blinking frequency indicates the DTC. DTCs are indicated by a series of long and short blinks. One long blink equals 10 short blinks. Add the long and short blinks together to determine the DTC. After determining the DTC, refer to the DTC Troubleshooting Index.
 NOTE: 
 - If the DTC is not memorized, the ABS indicator will go off for 3.6 seconds , and then come back on.
 - If the ABS indicator stays on, troubleshoot for "ABS indicator does not go off".
 The system will not indicate the DTC unless these conditions are met:
 - The brake pedal is not pressed.
 - The ignition switch is turned ON (II).
 - The SCS circuit is shorted to body ground before the ignition switch is turned ON (II).
 5. Turn the ignition switch OFF.
 6. Disconnect the Honda PGM Tester from the DLC.
```

## Brakes and Traction Control/Antilock Brakes / Traction Control Systems/Testing and Inspection/Reading and Clearing Diagnostic Trouble Codes/Reading Diagnostic Trouble Codes/Without Scan Tool/

```
Without Scan Tool HONDA does not provide a procedure for reading diagnostic trouble codes without the use of the designated scan tool. 
```

## Restraints and Safety Systems/Air Bag Systems/Testing and Inspection/Reading and Clearing Diagnostic Trouble Codes/Reading Diagnostic Trouble Codes/With Manufacturer's Scan Tool/

```
With Manufacturer's Scan Tool NOTE: This is the only infomation provided by the manufacturer for reading diagnostic trouble codes.
 Reading the DTC 
When the SRS indicator is on, read the DTC using either of the following methods:
 PGM Tester "SRS" Menu Method: 
Connect the Honda PGM Tester (A) to the 1 6P Data Link Connector (DLC)(1), and follow the Tester's prompts in the "SRS" menu.
If the Tester indicates no DTC, no communication, DTC 3-6 to 3-10, DTC 4-6 to 4-10, DTC 9-1, or DTC 9-2, double-check by using the "SCS" menu method.
 PGM Tester "SCS" Menu Method (retrieving the flash codes): 
The SRS indicator (A) indicates the DTC by the number of blinks when the Honda PGM Tester (B) is connected to the DLC (data link connector) (16P) (1).
 1. Make sure the ignition switch is OFF.
 2. Connect the Honda PGM Tester (B) to the DLC (1), and follow the Tester's prompts in the "SCS" menu to ground the SCS line (see the Honda PGM Tester Operator's Manual).
 3. Turn the ignition switch ON (II). The SRS indicator comes on for about 6 seconds and then goes off. Then it will blink to indicate the DTC (see the table).
 4. Read the DTC.
 5. Turn the ignition switch OFF, and wait for 10 seconds .
 6. Disconnect the Honda PGM Tester from the DLC.
 7. Proceed with the troubleshooting procedure for the DTC.
Patterns Of DTC Indications: 
Reading The Main Code / Sub Code: 
Examples of DTC Indications 1, 2 and 3: 
 - Including the most recent problem, up to three different DTCs can be indicated (see example 1 shown).
 - In case of a continuous failure, the DTC will be indicated repeatedly (see example 1 shown).
 - In case of an intermittent failure, the SRS indicator will indicate the DTCs one time, then it will stay on (see example 2 shown).
 - If both a continuous and an intermittent failure occur, both DTCs will be indicated as continuous failures.
 - When the system is normal (no DTCs), the SRS indicator will stay on (see example 3).
 - If the SRS indicator comes on continuously without a DTC, there may be a problem with the system.
 - If the SRS indicator does not come on as indicated above, always check for an open or a short to ground in the SCS circuit before troubleshooting the system. 
```

## Restraints and Safety Systems/Air Bag Systems/Testing and Inspection/Reading and Clearing Diagnostic Trouble Codes/Reading Diagnostic Trouble Codes/With Generic Scan Tool/

```
With Generic Scan Tool NOTE : Manufacturer does not supply information for Reading Codes with a generic scan tool. 
```

## Diagrams/Connector Views/Data Link Connector/ — image transcription

Figure "215. DLC — Gray — Under left side of dash — On ECM/PCM wire harness. View from terminal side."
Terminal list as printed:

```
1  —
2  —
3  —
4  BLK (G451) (Ground for PGM tester)
5  BRN/YEL (Circuit Z20) (Signal ground for communication network)
6  —
7  LT BLU (DLC and MIL circuits) (K line for off-board communication)
8  —
9  BRN (Circuit F28)
10 —
11 —
12 RED/WHT (Write enable)
13 —
14 GRY (Circuit F55)
15 —
16 WHT/RED (Fuse 9 – Under-hood) (Power for PGM tester (B+, 4 amps max))
NOTE: Data Link Connector (DLC) terminals are numbered according to SAE standard J1962, not the
Honda standard. The numbers of the four end terminals are molded into the corners of the connector
face. This view of the DLC is the same as if you were looking at it installed in the vehicle.
```
