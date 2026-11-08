---
title: OVMS v3 vehicle module for Kia e-Niro / Hyundai Kona EV (vehicle_kianiroev) — ECU addresses, poll list, session handling
source_url: https://github.com/openvehicles/Open-Vehicle-Monitoring-System-3/tree/2755ca51f2833af5093a4260ed74e11b2fd936db/vehicle/OVMS.V3/components
commit: 2755ca51f2833af5093a4260ed74e11b2fd936db
fetched: 2026-11-08
license: MIT (Open Vehicle Monitoring System v3)
---

The OVMS module for the Kia e-Niro also serves the Hyundai Kona Electric (its docs say so). Its
ECU names come from the shared Kia Soul EV header `kia_common.h`. The writes it performs (2F/30 IO
control in an extended session) are shown only because they prove those modules implement UDS
sessions; carlink never sends them (wiki page adapter-safety).

## vehicle_kianiroev/docs/index.rst (lines 1–8, verbatim)

```rst
============================================
Kia e-Niro / Hyundai Kona / Hyundai Ioniq FL
============================================

Vehicle Type: **KN**

The Kia e-Niro shares the same platform with the Hyundai Kona, Hyundai Ioniq (FL) and Kia Soul EV (2020-2024).

```

## vehicle_kiasoulev/src/kia_common.h (lines 276–292, verbatim)

```c
#define UDS_SID_IOCTRL_BY_ID 			0x2F		// InputOutputControlByCommonID
#define UDS_SID_IOCTRL_BY_LOC_ID	0x30 	// InputOutputControlByLocalID
#define UDS_SID_TESTER_PRESENT		0x3E 	// TesterPresent

#define UDS_DEFAULT_SESSION 									0x01
#define UDS_PROGRAMMING_SESSION 							0x02
#define UDS_EXTENDED_DIAGNOSTIC_SESSION 				0x03
#define UDS_SAFETY_SYSTEM_DIAGNOSTIC_SESSION 	0x04
#define KIA_90_DIAGNOSTIC_SESSION 				0x90

// ECUs
#define SMART_JUNCTION_BOX 		0x771
#define INTEGRATED_GATEWAY 		0x770
#define BODY_CONTROL_MODULE  	0x7A0
#define SMART_KEY_UNIT 				0x7A5
#define ABS_EBP_UNIT 					0x7A5
#define ON_BOARD_CHARGER_UNIT 0x794
```

## vehicle_kianiroev/src/vehicle_kianiroev.cpp — poll list (lines 80–118, verbatim)

```c
// Pollstate 2 - car is charging
static const OvmsPoller::poll_pid_t vehicle_kianiroev_polls[] =
  {
  		{ 0x7e2, 0x7ea, VEHICLE_POLL_TYPE_OBDII_1A, 				0x80, 			{       0,  120,	 120 }, 0, ISOTP_STD },  // VMCU - VIN

		{ 0x7e4, 0x7ec, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0x0101, 		{      9,    9,   9 }, 0, ISOTP_STD }, 	// BMC Diag page 01 - Must be called when off to detect when charging
		{ 0x7e4, 0x7ec, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0x0102, 		{       0,   59,   9 }, 0, ISOTP_STD }, 	// BMC Diag page 02
		{ 0x7e4, 0x7ec, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0x0103, 		{       0,   59,   9 }, 0, ISOTP_STD }, 	// BMC Diag page 03
		{ 0x7e4, 0x7ec, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0x0104, 		{       0,   59,   9 }, 0, ISOTP_STD }, 	// BMC Diag page 04
		{ 0x7e4, 0x7ec, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0x0105, 		{       0,   59,   9 }, 0, ISOTP_STD },		// BMC Diag page 05
		{ 0x7e4, 0x7ec, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0x0106, 		{       0,    9,   9 }, 0, ISOTP_STD },		// BMC Diag page 06

		{ 0x7a0, 0x7a8, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0xB00C, 		{       0,   29,  29 }, 0, ISOTP_STD },   // BCM Heated handle
		{ 0x7a0, 0x7a8, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0xB00E, 		{       0,   10,  10 }, 0, ISOTP_STD },   // BCM Chargeport ++

		{ 0x7a0, 0x7a8, VEHICLE_POLL_TYPE_OBDIIEXTENDED,   0xC002, 		{       0,   60,   0 }, 0, ISOTP_STD }, 	// TMPS - ID's
		{ 0x7a0, 0x7a8, VEHICLE_POLL_TYPE_OBDIIEXTENDED,   0xC00B, 		{       0,   13,   0 }, 0, ISOTP_STD }, 	// TMPS - Pressure and Temp

		{ 0x770, 0x778, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0xbc03, 		{      7,    7,   7 }, 0, ISOTP_STD },  // IGMP Door status + IGN1 & IGN2 - Detects when car is turned on
		{ 0x770, 0x778, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0xbc04, 		{       0,   11,  11 }, 0, ISOTP_STD },  // IGMP Door status
		{ 0x770, 0x778, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0xbc07, 		{       0,   13,  13 }, 0, ISOTP_STD },  // IGMP Rear/mirror defogger

		{ 0x7b3, 0x7bb, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0x0100, 		{       0,   10,  10 }, 0, ISOTP_STD },  // AirCon
		//{ 0x7b3, 0x7bb, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0x0102, 		{       0,   10,  10 } },  // AirCon - No usable values found yet
		{ 0x7c6, 0x7ce, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0xA020, 		{       0,   19, 120 }, 0, ISOTP_STD },  // Cluster. ODO
		{ 0x7c6, 0x7ce, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0xB002, 		{       0,   19, 120 }, 0, ISOTP_STD },  // Cluster. ODO

		{ 0x7d1, 0x7d9, VEHICLE_POLL_TYPE_OBDIIEXTENDED,  	0xc101, 		{       0,   27,  27 }, 0, ISOTP_STD },  // ABS/ESP - Emergency lights

		{ 0x7e5, 0x7ed, VEHICLE_POLL_TYPE_OBDIIGROUP,  		0x01, 			{       0,   58,  11 }, 0, ISOTP_STD }, 	// OBC - On board charger
    //{ 0x7e5, 0x7ed, VEHICLE_POLL_TYPE_OBDIIGROUP,  		0x02, 			{       0,   58,  11 } }, 	// OBC
		{ 0x7e5, 0x7ed, VEHICLE_POLL_TYPE_OBDIIGROUP,  		0x03, 			{       0,   58,  11 }, 0, ISOTP_STD }, 	// OBC

		{ 0x7e2, 0x7ea, VEHICLE_POLL_TYPE_OBDIIGROUP,  		0x01, 			{       0,    7,  19 }, 0, ISOTP_STD },  // VMCU - Shift position
		{ 0x7e2, 0x7ea, VEHICLE_POLL_TYPE_OBDIIGROUP,  		0x02, 			{     	  0,    7,  7 }, 0, ISOTP_STD },  // VMCU - Aux Battery data

		{ 0x7e3, 0x7eb, VEHICLE_POLL_TYPE_OBDIIGROUP,  		0x02, 			{       0,   11,  11 }, 0, ISOTP_STD },  // MCU

    POLL_LIST_END
```

## vehicle_kianiroev/src/kn_can_send.cpp (lines 152–260, verbatim)

```c
 * Send tester present messages for all ECUs
 */
void OvmsVehicleKiaNiroEv::SendTesterPresentMessages()
	{
	if( !kn_shift_bits.Park ){
		StopTesterPresentMessages();
	}
	if( igmp_tester_present_seconds>0 )
		{
		igmp_tester_present_seconds--;
		SendTesterPresent(INTEGRATED_GATEWAY, 2);
		}
	if( bcm_tester_present_seconds>0 )
		{
		bcm_tester_present_seconds--;
		SendTesterPresent(BODY_CONTROL_MODULE, 2);
		}

	if( smk_tester_present_seconds>0 )
		{
		smk_tester_present_seconds--;
		SendTesterPresent(SMART_KEY_UNIT, 2);
		}
	}

/**
 * Stop all tester present massages for all ECUs
 */
void OvmsVehicleKiaNiroEv::StopTesterPresentMessages()
	{
	igmp_tester_present_seconds = 0;
	bcm_tester_present_seconds = 0;
	smk_tester_present_seconds = 0;
	}


/**
 * Send a can message to ECU to notify that tester is present
 */
void OvmsVehicleKiaNiroEv::SendTesterPresent(uint16_t id, uint8_t length)
	{
	if(kia_enable_write)
		SendCanMessage(id, length,UDS_SID_TESTER_PRESENT, 0,0,0,0,0,0);
	}

/**
 * Send a can message to set ECU in a specific session mode
 */
bool OvmsVehicleKiaNiroEv::SetSessionMode(uint16_t id, uint8_t mode)
	{
	return SendCanMessage_sync(id, 2,VEHICLE_POLL_TYPE_OBDIISESSION, mode,0,0,0,0,0);
	}

/**
 * Put the car in proper session mode and then send the command
 */
bool OvmsVehicleKiaNiroEv::SendCommandInSessionMode(uint16_t id, uint8_t count, uint8_t serviceId, uint8_t b1, uint8_t b2, uint8_t b3, uint8_t b4, uint8_t b5, uint8_t b6, uint8_t mode )
	{
	if(!kia_enable_write) return false;

	bool result = false;
	SendTesterPresent(id,2);
	vTaskDelay( xDelay );
	if( SetSessionMode(id, mode ))
		{
		vTaskDelay( xDelay );
		SendTesterPresent(id,2);
		vTaskDelay( xDelay );
		result = SendCanMessage_sync(id, count, serviceId, b1, b2, b3, b4, b5, b6 );
		}
	SetSessionMode(id, UDS_DEFAULT_SESSION);
	return result;
	}

/**
 * Send command to IGMP
 * 770 04 2F [b1] [b2] [b3]
 */
bool OvmsVehicleKiaNiroEv::Send_IGMP_Command( uint8_t b1, uint8_t b2, uint8_t b3)
	{
	return SendCommandInSessionMode(INTEGRATED_GATEWAY, 4,UDS_SID_IOCTRL_BY_ID, b1, b2, b3, 0,0,0, UDS_EXTENDED_DIAGNOSTIC_SESSION );
	}

/**
 * Send command to Body Control Module
 * 7a0 04 2F [b1] [b2] [b3]
 */
bool OvmsVehicleKiaNiroEv::Send_BCM_Command( uint8_t b1, uint8_t b2, uint8_t b3)
	{
	return SendCommandInSessionMode(BODY_CONTROL_MODULE, 4,UDS_SID_IOCTRL_BY_ID, b1, b2, b3, 0,0,0, UDS_EXTENDED_DIAGNOSTIC_SESSION );
	}

/**
 * Send command to Smart Key Unit
 * 7a5 [b1] 2F [b2] [b3] [b4] [b5] [b6] [b7]
 */
bool OvmsVehicleKiaNiroEv::Send_SMK_Command( uint8_t b1, uint8_t b2, uint8_t b3, uint8_t b4, uint8_t b5, uint8_t b6, uint8_t b7)
	{
	return SendCommandInSessionMode(SMART_KEY_UNIT, b1,UDS_SID_IOCTRL_BY_ID, b2, b3, b4, b5, b6, b7, UDS_EXTENDED_DIAGNOSTIC_SESSION);
	}

/**
 * Send command to ABS & EBP Module
 * 7d5 03 30 [b1] [b2]
 */
bool OvmsVehicleKiaNiroEv::Send_EBP_Command( uint8_t b1, uint8_t b2, uint8_t mode)
	{
	return SendCommandInSessionMode(ABS_EBP_UNIT, 3,UDS_SID_IOCTRL_BY_LOC_ID, b1, b2, 0,0,0,0, mode );
	}
```
