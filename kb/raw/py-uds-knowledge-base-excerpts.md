---
title: py-uds knowledge base — DTC formats and status bits, S3 timing, services 0x14/0x19/0x3E, negative response codes
source_url: https://github.com/mdabrowski1990/uds/tree/69c90393968c026fd8eef1305e44bc9d95402be1/docs/source/pages/knowledge_base
commit: 69c90393968c026fd8eef1305e44bc9d95402be1
fetched: 2026-11-08
license: MIT (py-uds, Maciej Dąbrowski)
note: Rendered at https://uds.readthedocs.io/en/latest/pages/knowledge_base/ . A secondary summary of ISO 14229-1, used here where the standard itself was not available.
---

## dtc.rst (lines 15–148, verbatim)

```rst
DTC Presentation Formats
------------------------
Historically, DTCs were 2 bytes (16 bits) long.
Modern diagnostic protocols defined by ISO 14229 (UDS) and SAE J2012 use 3-byte (24-bit) DTCs,
which are now the industry standard.

+-------------------+-------------------+
| DTC in UDS Format | DTC in OBD format |
+===================+===================+
| 0x000000          | P0000-00          |
+-------------------+-------------------+
| 0x56789A          | C1678-9A          |
+-------------------+-------------------+
| 0xA5B6C7          | B25B6-C7          |
+-------------------+-------------------+
| 0xFFFFFF          | U3FFF-FF          |
+-------------------+-------------------+


.. _knowledge-base-dtc-obd-format:

OBD
```
The OBD-II format for DTCs is defined in
`SAE J2012 <https://www.sae.org/standards/j2012_202509-diagnostic-trouble-code-definitions>`_.

Example DTC values:

- P0123-45
- U2100-00
- B3FED-CB
- C10F0-FF

It consists of eight characters:

- **one letter** that identifies the system

  - `P` - Powertrain (bit pattern: 00)
  - `C` - Chassis (bit pattern: 01)
  - `B` - Body (bit pattern: 10)
  - `U` - Network/Communication (bit pattern: 11)

- **one numeric digit** which informs whether DTC is defined by a standard or vehicle manufacturer

  `0-3` (2 bits)

- **one hexadecimal digit** which indicates sub-system

  `0-F` (4 bits)

- **two hexadecimal digits** which identify the fault

  `00-FF` (8 bits)

- **dash**

  `-`

- **two hexadecimal digits** which identify the symptom of the fault

  `00-FF` (8 bits)


.. _knowledge-base-dtc-uds-format:

UDS
```
The UDS representation (defined by ISO 14229-1) encodes each DTC as a 24-bit value,
typically represented as a six-digit hexadecimal number.

Example DTC values:

- 0x012345
- 0xDCBA98
- 0xFFFFFF


.. _knowledge-base-dtc-status:

DTC Status
----------
Each DTC has an 8-bit status field that describes the current state of the associated fault and its history.

- **testFailed** (b[0] - LSB) - indicates the result of most recently performed diagnostic test for this DTC

  - 0 - the most recent test passed (no failure detected)
  - 1 - the most recent test failed (a failure detected)

- **testFailedThisOperationCycle** (b[1]) - indicates whether the DTC has reported *testFailed* at any time during
  the current cycle

  - 0 - no failure detected during the current operation cycle
  - 1 - at least one test failed during the current operation cycle

- **pendingDTC** (b[2]) - indicates whether DTC has reported *testFailed* at any time during the current
  or the previous operation cycle

  - 0 - no check failed during the current or the last completed operation cycle
  - 1 - at least one test failed during the current or the last completed operation cycle

- **confirmedDTC** (b[3]) - indicates whether DTC is desired to be stored in a long-term memory

  - 0 - DTC is not confirmed

    - cleared on :ref:`ClearDiagnosticInformation <knowledge-base-service-clear-diagnostic-information>` call
    - cleared when DTC Aging Counter threshold for *self-healing* is reached

  - 1 - DTC is confirmed

    - set whenever *testFailed* is set

- **testNotCompletedSinceLastClear** (b[4]) - indicates whether the DTC test has been completed since the last
  :ref:`ClearDiagnosticInformation <knowledge-base-service-clear-diagnostic-information>` call

  - 0 - at least one test (either passed or failed) was performed since the last clear
  - 1 - no test was performed since the last clear

- **testFailedSinceLastClear** (b[5]) - indicates whether at least one test failed since the last
  :ref:`ClearDiagnosticInformation <knowledge-base-service-clear-diagnostic-information>` call

  - 0 - no test failed since the last clear
  - 1 - at least one test failed since the last clear

- **testNotCompletedThisOperationCycle** (b[6]) - indicates whether DTC test was performed this operation cycle

  - 0 - at least one test (either passed or failed) was performed this operation cycle
  - 1 - no test was performed during the current operation cycle

- **warningIndicatorRequested** (b[7] - MSB) - indicates whether a warning indicator associated with DTC shall be
  activated

  - 0 - warning indicator not requested
  - 1 - warning indicator requested

```

## time_parameters.rst (lines 1–36, verbatim)

```rst
UDS Time Parameters
===================


Session Parameters
------------------


.. _knowledge-base-s3-client:

S3 :sub:`Client`
````````````````
On the client side, the S3\ :sub:`Client` parameter defines the time interval for sending periodic
:ref:`Tester Present <knowledge-base-service-tester-present>` request messages.
This ensures that the server(s) the client communicates with do not exit the current diagnostic session
due to inactivity.

Default value:
  2000ms

Minimum value:
  P2\ :sub:`Client`

Maximum value:
  S3\ :sub:`Server`


.. _knowledge-base-s3-server:

S3\ :sub:`Server`
`````````````````
On the server side, the S3\ :sub:`Server` parameter defines the inactivity time (i.e. the period without any
request messages), after which the server shall exit the current diagnostic session and return to the Default Session.

S3\ :sub:`Server` = 5000ms

```

## service.rst — ClearDiagnosticInformation (lines 275–340, verbatim)

```rst
ClearDiagnosticInformation (0x14)
---------------------------------
ClearDiagnosticInformation service is used by the client to clear Diagnostic Trouble Codes (DTCs) and related data
stored in one or more server memories.


Request Format
``````````````


ISO 14229-1:2020
''''''''''''''''
The optional *MemorySelection* field is introduced to allow clearing diagnostic information from
a specific DTC memory (e.g. one of the sub-systems).

+-------------------+------------+-------------------+----------------------------------+----------+
| Name              | Bit Length | Value             | Description                      | Present  |
+===================+============+===================+==================================+==========+
| *SID*             | 8          | 0x14              | ClearDiagnosticInformation       | Always   |
+-------------------+------------+-------------------+----------------------------------+----------+
| *groupOfDTC*      | 24         | 0x000000-0xFFFFFF | Group of DTCs to be cleared      | Always   |
|                   |            |                   |                                  |          |
|                   |            |                   | 0xFFFF33: Emissions-system group |          |
|                   |            |                   |                                  |          |
|                   |            |                   | 0xFFFFD0: Safety-system group    |          |
|                   |            |                   |                                  |          |
|                   |            |                   | 0xFFFFFE: VOBD system group      |          |
|                   |            |                   |                                  |          |
|                   |            |                   | 0xFFFFFF: all                    |          |
+-------------------+------------+-------------------+----------------------------------+----------+
| *MemorySelection* | 8          | 0x00-0xFF         | Specifies DTC memory to clear    | Optional |
+-------------------+------------+-------------------+----------------------------------+----------+


ISO 14229-1:2013
''''''''''''''''
+--------------+------------+-------------------+----------------------------------+---------+
| Name         | Bit Length | Value             | Description                      | Present |
+==============+============+===================+==================================+=========+
| *SID*        | 8          | 0x14              | ClearDiagnosticInformation       | Always  |
+--------------+------------+-------------------+----------------------------------+---------+
| *groupOfDTC* | 24         | 0x000000-0xFFFFFF | Group of DTCs to be cleared      | Always  |
|              |            |                   |                                  |         |
|              |            |                   | 0xFFFF33: Emissions-system group |         |
|              |            |                   |                                  |         |
|              |            |                   | 0xFFFFD0: Safety-system group    |         |
|              |            |                   |                                  |         |
|              |            |                   | 0xFFFFFE: VOBD system group      |         |
|              |            |                   |                                  |         |
|              |            |                   | 0xFFFFFF: all                    |         |
+--------------+------------+-------------------+----------------------------------+---------+


Positive Response Format
````````````````````````
+--------+------------+-------+----------------------------+---------+
| Name   | Bit Length | Value | Description                | Present |
+========+============+=======+============================+=========+
| *RSID* | 8          | 0x54  | ClearDiagnosticInformation | Always  |
+--------+------------+-------+----------------------------+---------+


.. _knowledge-base-service-read-dtc-information:

```

## service.rst — ReadDTCInformation reportDTCByStatusMask (lines 449–510, verbatim)

```rst
reportDTCByStatusMask (0x02)
````````````````````````````
This sub-function can be used by the client to request a list of all :ref:`DTCs <knowledge-base-dtc>` stored in
the server’s memory that match a specific :ref:`status <knowledge-base-dtc-status>` mask (*DTCStatusMask*).
A DTC is included in the response if :code:`(DTC Status & DTCStatusMask) != 0`.
This sub-function provides the client with both the DTC values and their corresponding status information for
all DTCs that satisfy the given mask.


Request Format
''''''''''''''
+--------------------------------------------------+----------------+-----------+--------------------------------------------------------------+---------+
| Name                                             | Bit Length     | Value     | Description                                                  | Present |
+==================================================+================+===========+==============================================================+=========+
| *SID*                                            | 8              | 0x19      | ReadDTCInformation                                           | Always  |
+---------------+----------------------------------+---+------------+-----------+--------------------------------------------------------------+---------+
| *SubFunction* | *suppressPosRspMsgIndicationBit* | 8 | 1 (b[7])   | 0x0-0x1   | Information for the server whether to send positive response | Always  |
|               |                                  |   |            |           |                                                              |         |
|               |                                  |   |            |           | 0: response required                                         |         |
|               |                                  |   |            |           |                                                              |         |
|               |                                  |   |            |           | 1: suppress positive response                                |         |
|               +----------------------------------+   +------------+-----------+--------------------------------------------------------------+         |
|               | *reportType*                     |   | 7 (b[6-0]) | 0x02      | reportDTCByStatusMask                                        |         |
+---------------+----------------------------------+---+------------+-----------+--------------------------------------------------------------+---------+
| *DTCStatusMask*                                  | 8              | 0x00-0xFF | DTC status mask to use for DTCs matching                     | Always  |
+--------------------------------------------------+----------------+-----------+--------------------------------------------------------------+---------+


Positive Response Format
''''''''''''''''''''''''
+--------------------------------------------------------+---------------------+-------------------+-----------------------------------------------------------------+------------------------------------------+
| Name                                                   | Bit Length          | Value             | Description                                                     | Present                                  |
+========================================================+=====================+===================+=================================================================+==========================================+
| *RSID*                                                 | 8                   | 0x59              | ReadDTCInformation                                              | Always                                   |
+---------------------+----------------------------------+--------+------------+-------------------+-----------------------------------------------------------------+------------------------------------------+
| *SubFunction*       | *suppressPosRspMsgIndicationBit* | 8      | 1 (b[7])   | 0x0-0x1           | Echo of *suppressPosRspMsgIndicationBit* value from the request | Always                                   |
|                     +----------------------------------+        +------------+-------------------+-----------------------------------------------------------------+                                          |
|                     | *reportType*                     |        | 7 (b[6-0]) | 0x02              | reportDTCByStatusMask                                           |                                          |
+---------------------+----------------------------------+--------+------------+-------------------+-----------------------------------------------------------------+------------------------------------------+
| *DTCStatusAvailabilityMask*                            | 8                   | 0x00-0xFF         | DTC Status bits supported by the ECU                            | Always                                   |
+---------------------+----------------------------------+--------+------------+-------------------+-----------------------------------------------------------------+------------------------------------------+
| *DTC and DTCStatus* | *DTC*                            | 32 * n | 24         | 0x000000-0xFFFFFF | DTC#1                                                           | If at least one DTC matches the criteria |
|                     +----------------------------------+        +------------+-------------------+-----------------------------------------------------------------+                                          |
|                     | *DTCStatus*                      |        | 8          | 0x00-0xFF         | Status of DTC#1                                                 |                                          |
|                     +----------------------------------+        +------------+-------------------+-----------------------------------------------------------------+------------------------------------------+
|                     | ...                              |        | ...                                                                                                                                         |
|                     +----------------------------------+        +------------+-------------------+-----------------------------------------------------------------+------------------------------------------+
|                     | *DTC*                            |        | 24         | 0x000000-0xFFFFFF | DTC#n                                                           | If at least n DTCs matches the criteria  |
|                     +----------------------------------+        +------------+-------------------+-----------------------------------------------------------------+                                          |
|                     | *DTCStatus*                      |        | 8          | 0x00-0xFF         | Status of DTC#n                                                 |                                          |
+---------------------+----------------------------------+--------+------------+-------------------+-----------------------------------------------------------------+------------------------------------------+


.. _knowledge-base-service-read-dtc-information-03:

reportDTCSnapshotIdentification (0x03)
``````````````````````````````````````
This sub-function can be used by the client to request identification numbers of all stored DTC snapshot records.


Request Format
''''''''''''''
```

## service.rst — TesterPresent (lines 4677–4730, verbatim)

```rst
TesterPresent (0x3E)
--------------------
TesterPresent service is used by the client to indicate to the server that the diagnostic tester is still
present on the communication bus.
Reception of this service prevents the server’s :ref:`S3Server <knowledge-base-s3-server>` timer from expiring,
thereby extending the active diagnostic session.

.. note:: From the server’s implementation perspective, TesterPresent does not perform any diagnostic action.
   Reception of **any** diagnostic request (TesterPresent or otherwise) resets/extends the S3\ :sub:`Server` timer.


Request Format
``````````````
+--------------------------------------------------+----------------+-----------+--------------------------------------------------------------+---------+
| Name                                             | Bit Length     | Value     | Description                                                  | Present |
+==================================================+================+===========+==============================================================+=========+
| *SID*                                            | 8              | 0x3E      | TesterPresent                                                | Always  |
+---------------+----------------------------------+---+------------+-----------+--------------------------------------------------------------+---------+
| *SubFunction* | *suppressPosRspMsgIndicationBit* | 8 | 1 (b[7])   | 0x0-0x1   | Information for the server whether to send positive response | Always  |
|               |                                  |   |            |           |                                                              |         |
|               |                                  |   |            |           | 0: response required                                         |         |
|               |                                  |   |            |           |                                                              |         |
|               |                                  |   |            |           | 1: suppress positive response                                |         |
|               +----------------------------------+   +------------+-----------+--------------------------------------------------------------+         |
|               | *zeroSubFunction*                |   | 7 (b[6-0]) | 0x00-0x7F | Currently carries no information                             |         |
|               |                                  |   |            |           |                                                              |         |
|               |                                  |   |            |           | 0x00: zeroSubFunction                                        |         |
|               |                                  |   |            |           |                                                              |         |
|               |                                  |   |            |           | 0x01-0x7F: reserved                                          |         |
+---------------+----------------------------------+---+------------+-----------+--------------------------------------------------------------+---------+


Positive Response Format
````````````````````````
+--------------------------------------------------+----------------+-----------+-----------------------------------------------------------------+---------+
| Name                                             | Bit Length     | Value     | Description                                                     | Present |
+==================================================+================+===========+=================================================================+=========+
| *RSID*                                           | 8              | 0x7E      | TesterPresent                                                   | Always  |
+---------------+----------------------------------+---+------------+-----------+-----------------------------------------------------------------+---------+
| *SubFunction* | *suppressPosRspMsgIndicationBit* | 8 | 1 (b[7])   | 0x0-0x1   | Echo of *suppressPosRspMsgIndicationBit* value from the request | Always  |
|               +----------------------------------+   +------------+-----------+-----------------------------------------------------------------+         |
|               | *zeroSubFunction*                |   | 7 (b[6-0]) | 0x00-0x7F | Echo of *zeroSubFunction* value from the request                |         |
+---------------+----------------------------------+---+------------+-----------+-----------------------------------------------------------------+---------+


.. _knowledge-base-service-access-timing-parameter:

AccessTimingParameter (0x83)
----------------------------
AccessTimingParameter service is used by the client to read or modify the timing parameters of
the diagnostic communication link.

.. warning:: Withdrawn in ISO 14229-1:2020

```

## diagnostic_message.rst — negative response codes used by carlink (verbatim)

```rst

- **0x11 - serviceNotSupported**: This NRC indicates that the requested action will not be taken because the
  server does not support the requested service.

- **0x12 - SubFunctionNotSupported**: This NRC indicates that the requested action will not be taken because the
  server does not support the service specific parameters of the request message.

- **0x13 - incorrectMessageLengthOrInvalidFormat**: This NRC indicates that the requested action will not be taken
  because the length of the received request message does not match the prescribed length for the specified service
  or the format of the parameters do not match the prescribed format for the specified service.

- **0x22 - conditionsNotCorrect**: This NRC indicates that the requested action will not be taken because the server
  prerequisite conditions are not met.

- **0x31 - requestOutOfRange**: This NRC indicates that the requested action will not be taken because the server has
  detected that the request message contains a parameter which attempts to substitute a value beyond its range of
  authority (e.g. attempting to substitute a data byte of 111 when the data is only defined to 100), or which attempts
  to access a DataIdentifier/RoutineIdentifer that is not supported or not supported in active session.
- **0x78 - requestCorrectlyReceived-ResponsePending**: This NRC indicates that the request message was received
  correctly, and that all parameters in the request message were valid (these checks can be delayed until after sending
  this NRC if executing the boot software), but the action to be performed is not yet completed and the server is not
  yet ready to receive another request. As soon as the requested service has been completed, the server shall send
  a positive response message or negative response message with a response code different from this.

- **0x79-0x7D - ISO Reserved**: This range of values is reserved for future definition by ISO 14229 Standard.

- **0x7E - SubFunctionNotSupportedInActiveSession**: This NRC indicates that the requested action will not be taken
  because the server does not support the requested SubFunction in the session currently active. This NRC shall only
  be used when the requested SubFunction is known to be supported in another session, otherwise response code
  SubFunctionNotSupported shall be used.

- **0x7F - serviceNotSupportedInActiveSession**: This NRC indicates that the requested action will not be taken because
  the server does not support the requested service in the session currently active. This NRC shall only be used when
  the requested service is known to be supported in another session, otherwise response code serviceNotSupported
  shall be used.

```
