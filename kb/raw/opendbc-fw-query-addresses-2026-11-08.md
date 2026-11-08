---
title: commaai/opendbc — Hyundai and Honda firmware-query addresses (fingerprints.py, values.py)
source_url: https://github.com/commaai/opendbc/tree/27fa1c5d8eed3238a0a78c52ff5abb6cdc807bd2/opendbc/car
commit: 27fa1c5d8eed3238a0a78c52ff5abb6cdc807bd2
fetched: 2026-11-08
license: MIT (opendbc)
---

openpilot identifies a car by asking each diagnostic address for its firmware version and matching
the answers against `fingerprints.py`. Every `(Ecu.x, address, sub)` key below therefore answered a
read on real cars of that platform. Caveat for carlink: openpilot sends these queries through its
own harness at the camera connector as well as through the OBD port (`bus=` / `obd_multiplexing`
in the request list), so an address here proves the module exists on the car, not that it is
reachable through the DLC. Responses are at request + 8 for 11-bit (Hyundai) and at `18DAF1xx` for
29-bit (Honda).

## opendbc/car/hyundai/values.py (verbatim excerpts)

```python
  HYUNDAI_KONA_EV = HyundaiPlatformConfig(
    [HyundaiCarDocs("Hyundai Kona Electric 2018-21", car_parts=CarParts.common([CarHarness.hyundai_g]))],
    CarSpecs(wheelbase=2.6, steerRatio=13.42, tireStiffnessFactor=0.385),
    flags=HyundaiFlags.EV | HyundaiFlags.ALT_LIMITS,
  )
  HYUNDAI_KONA_EV_2022 = HyundaiPlatformConfig(
    [HyundaiCarDocs("Hyundai Kona Electric 2022-23", car_parts=CarParts.common([CarHarness.hyundai_o]))],
    CarSpecs(wheelbase=2.6, steerRatio=13.42, tireStiffnessFactor=0.385),
    flags=HyundaiFlags.CAMERA_SCC | HyundaiFlags.EV | HyundaiFlags.ALT_LIMITS,
  )
  HYUNDAI_KONA_EV_2ND_GEN = HyundaiCanFDPlatformConfig(
    [HyundaiCarDocs("Hyundai Kona Electric (with HDA II, Korea only) 2023", video="https://www.youtube.com/watch?v=U2fOCmcQ8hw",
                    car_parts=CarParts.common([CarHarness.hyundai_r]))],
...
HYUNDAI_VERSION_REQUEST_LONG = bytes([uds.SERVICE_TYPE.READ_DATA_BY_IDENTIFIER]) + \
  p16(0xf100)  # Long description

HYUNDAI_VERSION_REQUEST_ALT = bytes([uds.SERVICE_TYPE.READ_DATA_BY_IDENTIFIER]) + \
  p16(0xf110)  # Alt long description

HYUNDAI_ECU_MANUFACTURING_DATE = bytes([uds.SERVICE_TYPE.READ_DATA_BY_IDENTIFIER]) + \
  p16(uds.DATA_IDENTIFIER_TYPE.ECU_MANUFACTURING_DATE)

HYUNDAI_VERSION_RESPONSE = bytes([uds.SERVICE_TYPE.READ_DATA_BY_IDENTIFIER + 0x40])

# Regex patterns for parsing platform code, FW date, and part number from FW versions
...
FW_QUERY_CONFIG = FwQueryConfig(
  fw_version_regex=br"\xf1\x00[\x00-\xff]{19,56}",
  requests=[
    # TODO: add back whitelists
    # CAN queries (OBD-II port)
    Request(
      [HYUNDAI_VERSION_REQUEST_LONG],
      [HYUNDAI_VERSION_RESPONSE],
    ),

    # CAN & CAN-FD queries (from camera)
    Request(
      [HYUNDAI_VERSION_REQUEST_LONG],
      [HYUNDAI_VERSION_RESPONSE],
      bus=0,
    ),
    Request(
      [HYUNDAI_VERSION_REQUEST_LONG],
      [HYUNDAI_VERSION_RESPONSE],
      bus=1,
      obd_multiplexing=False,
    ),

    # CAN & CAN FD query to understand the three digit date code
    # LKA steering cars usually use 6 digit date codes, so skip bus 1
    Request(
      [HYUNDAI_ECU_MANUFACTURING_DATE],
      [HYUNDAI_VERSION_RESPONSE],
      bus=0,
      logging=True,
    ),

    # CAN-FD alt request logging queries for hvac and parkingAdas
    Request(
      [HYUNDAI_VERSION_REQUEST_ALT],
      [HYUNDAI_VERSION_RESPONSE],
      bus=0,
      logging=True,
    ),
    Request(
      [HYUNDAI_VERSION_REQUEST_ALT],
      [HYUNDAI_VERSION_RESPONSE],
      bus=1,
      logging=True,
      obd_multiplexing=False,
    ),
  ],
  # We lose these ECUs without the comma power on these cars.
  # Note that we still attempt to match with them when they are present
  non_essential_ecus={
    Ecu.abs: [CAR.HYUNDAI_PALISADE, CAR.HYUNDAI_SONATA, CAR.HYUNDAI_SANTA_FE_2022, CAR.KIA_K5_2021, CAR.HYUNDAI_ELANTRA_2021,
              CAR.HYUNDAI_SANTA_FE, CAR.HYUNDAI_KONA_EV_2022, CAR.HYUNDAI_KONA_EV, CAR.HYUNDAI_CUSTIN_1ST_GEN, CAR.KIA_SORENTO,
              CAR.KIA_CEED, CAR.KIA_SELTOS],
  },
  extra_ecus=[
    (Ecu.adas, 0x730, None),              # ADAS Driving ECU on platforms with LKA steering
    (Ecu.parkingAdas, 0x7b1, None),       # ADAS Parking ECU (may exist on all platforms)
    (Ecu.hvac, 0x7b3, None),              # HVAC Control Assembly
    (Ecu.cornerRadar, 0x7b7, None),
    (Ecu.combinationMeter, 0x7c6, None),  # CAN FD Instrument cluster
```

## opendbc/car/hyundai/fingerprints.py (verbatim excerpts)

```python
  CAR.HYUNDAI_IONIQ_EV_2020: {
    (Ecu.fwdRadar, 0x7d0, None): [
      b'\xf1\x00AEev SCC F-CUP      1.00 1.00 99110-G7200         ',
      b'\xf1\x00AEev SCC F-CUP      1.00 1.00 99110-G7500         ',
      b'\xf1\x00AEev SCC F-CUP      1.00 1.01 99110-G7000         ',
      b'\xf1\x00AEev SCC F-CUP      1.00 1.01 99110-G7100         ',
      b'\xf1\x00AEev SCC FHCUP      1.00 1.01 99110-G7100         ',
    ],
    (Ecu.eps, 0x7d4, None): [
      b'\xf1\x00AE  MDPS C 1.00 1.01 56310/G7310 4APEC101',
      b'\xf1\x00AE  MDPS C 1.00 1.01 56310/G7510 4APEC101',
      b'\xf1\x00AE  MDPS C 1.00 1.01 56310/G7560 4APEC101',
    ],
    (Ecu.fwdCamera, 0x7c4, None): [
      b'\xf1\x00AEE MFC  AT EUR LHD 1.00 1.00 95740-G2600 190730',
      b'\xf1\x00AEE MFC  AT EUR LHD 1.00 1.00 95740-G2700 201027',
      b'\xf1\x00AEE MFC  AT EUR LHD 1.00 1.01 95740-G2600 190819',
      b'\xf1\x00AEE MFC  AT EUR LHD 1.00 1.03 95740-G2500 190516',
      b'\xf1\x00AEE MFC  AT EUR RHD 1.00 1.01 95740-G2600 190819',
      b'\xf1\x00AEE MFC  AT USA LHD 1.00 1.01 95740-G2600 190819',
    ],
  },
  CAR.HYUNDAI_IONIQ_EV_LTD: {
    (Ecu.fwdRadar, 0x7d0, None): [
      b'\xf1\x00AEev SCC F-CUP      1.00 1.00 96400-G7000         ',
      b'\xf1\x00AEev SCC F-CUP      1.00 1.00 96400-G7100         ',
      b'\xf1\x00AEev SCC F-CUP      1.00 1.01 96400-G7000         ',
    ],
    (Ecu.eps, 0x7d4, None): [
      b'\xf1\x00AE  MDPS C 1.00 1.02 56310G7300\x00 4AEEC102',
      b'\xf1\x00AE  MDPS C 1.00 1.03 56310/G7300 4AEEC103',
      b'\xf1\x00AE  MDPS C 1.00 1.03 56310G7300\x00 4AEEC103',
      b'\xf1\x00AE  MDPS C 1.00 1.04 56310/G7301 4AEEC104',
      b'\xf1\x00AE  MDPS C 1.00 1.04 56310/G7501 4AEEC104',
      b'\xf1\x00AE  MDPS C 1.00 1.04 56310/G7551 4AEEC104',
    ],
    (Ecu.fwdCamera, 0x7c4, None): [
      b'\xf1\x00AEE MFC  AT EUR LHD 1.00 1.00 95740-G2300 170703',
      b'\xf1\x00AEE MFC  AT EUR LHD 1.00 1.00 95740-G2400 180222',
      b'\xf1\x00AEE MFC  AT EUR LHD 1.00 1.00 95740-G7200 160418',
      b'\xf1\x00AEE MFC  AT EUR RHD 1.00 1.00 95740-G2400 180222',
      b'\xf1\x00AEE MFC  AT USA LHD 1.00 1.00 95740-G2400 180222',
    ],
...
  CAR.HYUNDAI_KONA_EV: {
    (Ecu.abs, 0x7d1, None): [
      b'\xf1\x00OS IEB \x01 212 \x11\x13 58520-K4000',
      b'\xf1\x00OS IEB \x02 210 \x02\x14 58520-K4000',
      b'\xf1\x00OS IEB \x02 212 \x11\x13 58520-K4000',
      b'\xf1\x00OS IEB \x03 210 \x02\x14 58520-K4000',
      b'\xf1\x00OS IEB \x03 211 \x04\x02 58520-K4000',
      b'\xf1\x00OS IEB \x03 212 \x11\x13 58520-K4000',
      b'\xf1\x00OS IEB \x04 212 \x11\x13 58520-K4000',
      b'\xf1\x00OS IEB \r 105\x18\t\x18 58520-K4000',
    ],
    (Ecu.fwdCamera, 0x7c4, None): [
      b'\xf1\x00OE2 LKAS AT EUR LHD 1.00 1.00 95740-K4200 200',
      b'\xf1\x00OSE LKAS AT AUS RHD 1.00 1.00 95740-K4300 W50',
      b'\xf1\x00OSE LKAS AT EUR LHD 1.00 1.00 95740-K4100 W40',
      b'\xf1\x00OSE LKAS AT EUR RHD 1.00 1.00 95740-K4100 W40',
      b'\xf1\x00OSE LKAS AT KOR LHD 1.00 1.00 95740-K4100 W40',
      b'\xf1\x00OSE LKAS AT KOR LHD 1.00 1.00 95740-K4300 W50',
      b'\xf1\x00OSE LKAS AT USA LHD 1.00 1.00 95740-K4100 W40',
      b'\xf1\x00OSE LKAS AT USA LHD 1.00 1.00 95740-K4300 W50',
    ],
    (Ecu.eps, 0x7d4, None): [
      b'\xf1\x00OS  MDPS C 1.00 1.03 56310/K4550 4OEDC103',
      b'\xf1\x00OS  MDPS C 1.00 1.04 56310-XX000 4OEDC104',
      b'\xf1\x00OS  MDPS C 1.00 1.04 56310/K4550 4OEDC104',
      b'\xf1\x00OS  MDPS C 1.00 1.04 56310K4000\x00 4OEDC104',
      b'\xf1\x00OS  MDPS C 1.00 1.04 56310K4050\x00 4OEDC104',
      b'\xf1\x00OS  MDPS C 1.00 1.05 56310K4000\x00 4OEDC105',
    ],
    (Ecu.fwdRadar, 0x7d0, None): [
      b'\xf1\x00OSev SCC F-CUP      1.00 1.00 99110-K4000         ',
      b'\xf1\x00OSev SCC F-CUP      1.00 1.00 99110-K4100         ',
      b'\xf1\x00OSev SCC F-CUP      1.00 1.01 99110-K4000         ',
      b'\xf1\x00OSev SCC FNCUP      1.00 1.01 99110-K4000         ',
    ],
  },
  CAR.HYUNDAI_KONA_EV_2022: {
    (Ecu.abs, 0x7d1, None): [
      b'\xf1\x00OS IEB \x02 101 \x11\x13 58520-K4010',
      b'\xf1\x00OS IEB \x02 102"\x05\x16 58520-K4010',
      b'\xf1\x00OS IEB \x03 101 \x11\x13 58520-K4010',
      b'\xf1\x00OS IEB \x03 102"\x05\x16 58520-K4010',
      b'\xf1\x00OS IEB \r 102"\x05\x16 58520-K4010',
    ],
    (Ecu.fwdCamera, 0x7c4, None): [
      b'\xf1\x00OSP LKA  AT AUS RHD 1.00 1.04 99211-J9200 904',
      b'\xf1\x00OSP LKA  AT CND LHD 1.00 1.02 99211-J9110 802',
      b'\xf1\x00OSP LKA  AT EUR LHD 1.00 1.04 99211-J9200 904',
      b'\xf1\x00OSP LKA  AT EUR RHD 1.00 1.02 99211-J9110 802',
      b'\xf1\x00OSP LKA  AT EUR RHD 1.00 1.04 99211-J9200 904',
      b'\xf1\x00OSP LKA  AT USA LHD 1.00 1.04 99211-J9200 904',
    ],
    (Ecu.eps, 0x7d4, None): [
      b'\xf1\x00OSP MDPS C 1.00 1.02 56310-K4271 4OEPC102',
      b'\xf1\x00OSP MDPS C 1.00 1.02 56310/K4261 4OEPC102',
      b'\xf1\x00OSP MDPS C 1.00 1.02 56310/K4271 4OEPC102',
      b'\xf1\x00OSP MDPS C 1.00 1.02 56310/K4970 4OEPC102',
      b'\xf1\x00OSP MDPS C 1.00 1.02 56310/K4971 4OEPC102',
      b'\xf1\x00OSP MDPS C 1.00 1.02 56310K4260\x00 4OEPC102',
      b'\xf1\x00OSP MDPS C 1.00 1.02 56310K4261\x00 4OEPC102',
      b'\xf1\x00OSP MDPS C 1.00 1.02 56310K4971\x00 4OEPC102',
    ],
    (Ecu.fwdRadar, 0x7d0, None): [
      b'\xf1\x00YB__ FCA -----      1.00 1.01 99110-K4500      \x00\x00\x00',
    ],
  },
  CAR.HYUNDAI_KONA_EV_2ND_GEN: {
    (Ecu.fwdRadar, 0x7d0, None): [
      b'\xf1\x00SXev RDR -----      1.00 1.00 99110-BF000         ',
    ],
    (Ecu.fwdCamera, 0x7c4, None): [
      b'\xf1\x00SX2EMFC  AT KOR LHD 1.00 1.00 99211-BF000 230410',
    ],
  },
  CAR.KIA_NIRO_EV: {
    (Ecu.fwdRadar, 0x7d0, None): [
      b'\xf1\x00DEev SCC F-CUP      1.00 1.00 99110-Q4000         ',
      b'\xf1\x00DEev SCC F-CUP      1.00 1.00 99110-Q4100         ',
      b'\xf1\x00DEev SCC F-CUP      1.00 1.00 99110-Q4500         ',
      b'\xf1\x00DEev SCC F-CUP      1.00 1.00 99110-Q4600         ',
      b'\xf1\x00DEev SCC F-CUP      1.00 1.02 96400-Q4000         ',
      b'\xf1\x00DEev SCC F-CUP      1.00 1.02 96400-Q4100         ',
      b'\xf1\x00DEev SCC F-CUP      1.00 1.03 96400-Q4100         ',
      b'\xf1\x00DEev SCC FHCUP      1.00 1.00 99110-Q4600         ',
      b'\xf1\x00DEev SCC FHCUP      1.00 1.03 96400-Q4000         ',
      b'\xf1\x00DEev SCC FNCUP      1.00 1.00 99110-Q4600         ',
    ],
    (Ecu.eps, 0x7d4, None): [
      b'\xf1\x00DE  MDPS C 1.00 1.04 56310Q4100\x00 4DEEC104',
      b'\xf1\x00DE  MDPS C 1.00 1.05 56310Q4000\x00 4DEEC105',
      b'\xf1\x00DE  MDPS C 1.00 1.05 56310Q4100\x00 4DEEC105',
      b'\xf1\x00DE  MDPS C 1.00 1.05 56310Q4150\x00 4DEEC105',
      b'\xf1\x00DE  MDPS C 1.00 1.05 56310Q4200\x00 4DEEC105',
    ],
    (Ecu.fwdCamera, 0x7c4, None): [
      b'\xf1\x00DEE MFC  AT EUR LHD 1.00 1.00 99211-Q4000 191211',
      b'\xf1\x00DEE MFC  AT EUR LHD 1.00 1.00 99211-Q4100 200706',
      b'\xf1\x00DEE MFC  AT EUR LHD 1.00 1.03 95740-Q4000 180821',
      b'\xf1\x00DEE MFC  AT EUR RHD 1.00 1.00 99211-Q4000 191211',
      b'\xf1\x00DEE MFC  AT KOR LHD 1.00 1.02 95740-Q4000 180705',
      b'\xf1\x00DEE MFC  AT KOR LHD 1.00 1.03 95740-Q4000 180821',
      b'\xf1\x00DEE MFC  AT USA LHD 1.00 1.00 99211-Q4000 191211',
      b'\xf1\x00DEE MFC  AT USA LHD 1.00 1.01 99211-Q4500 210428',
```

## opendbc/car/honda/values.py (verbatim excerpts)

```python
    [HondaCarDocs("Honda Pilot 2023-25", "All")],
    [HondaCarDocs("Honda Pilot 2026", "All")],
    [HondaCarDocs("Honda Odyssey 2021-26", "All", min_steer_speed=70. * CV.KPH_TO_MS)],
    [HondaCarDocs("Honda Odyssey 2018-20")],
      HondaCarDocs("Honda Odyssey (Taiwan) 2018-19"),
      HondaCarDocs("Honda Odyssey (Singapore) 2021")
      HondaCarDocs("Honda Pilot 2016-22", min_steer_speed=12. * CV.MPH_TO_MS),
...
HONDA_ALT_VERSION_REQUEST = bytes([uds.SERVICE_TYPE.READ_DATA_BY_IDENTIFIER]) + \
  p16(0xF112)
HONDA_ALT_VERSION_RESPONSE = bytes([uds.SERVICE_TYPE.READ_DATA_BY_IDENTIFIER + 0x40]) + \
  p16(0xF112)


FW_QUERY_CONFIG = FwQueryConfig(
  fw_version_regex=br"[A-Z0-9]{5}-[A-Z0-9]{3}(-|,)[A-Z0-9]{4}(\x00){2}$",
  requests=[
    # Currently used to fingerprint
    Request(
      [StdQueries.UDS_VERSION_REQUEST],
      [StdQueries.UDS_VERSION_RESPONSE],
      bus=1,
    ),

    # Data collection requests:
    # Log manufacturer-specific identifier for current ECUs
    Request(
      [HONDA_ALT_VERSION_REQUEST],
      [HONDA_ALT_VERSION_RESPONSE],
      bus=1,
      logging=True,
    ),
    # Nidec PT bus
    Request(
      [StdQueries.UDS_VERSION_REQUEST],
      [StdQueries.UDS_VERSION_RESPONSE],
      bus=0,
    ),
    # Bosch PT bus
    Request(
      [StdQueries.UDS_VERSION_REQUEST],
      [StdQueries.UDS_VERSION_RESPONSE],
      bus=1,
      obd_multiplexing=False,
    ),
  ],
  # We lose these ECUs without the comma power on these cars.
  # Note that we still attempt to match with them when they are present
  # This is or'd with (ALL_ECUS - ESSENTIAL_ECUS) from fw_versions.py
  non_essential_ecus={
    Ecu.eps: [CAR.ACURA_RDX_3G, CAR.HONDA_ACCORD, CAR.HONDA_E, CAR.HONDA_E_ADVANCE, CAR.ACURA_MDX_4G,
              *[c for c in CAR if c.config.flags & (HondaFlags.BOSCH_ALT_RADAR | HondaFlags.BOSCH_RADARLESS | HondaFlags.BOSCH_CANFD)]],
    Ecu.vsa: [CAR.ACURA_RDX_3G, CAR.HONDA_ACCORD, CAR.HONDA_CIVIC, CAR.HONDA_CIVIC_BOSCH, CAR.HONDA_CRV_5G, CAR.HONDA_CRV_HYBRID, CAR.HONDA_E,
              CAR.HONDA_E_ADVANCE, CAR.HONDA_INSIGHT, CAR.HONDA_NBOX_2G, CAR.ACURA_MDX_4G,
              *[c for c in CAR if c.config.flags & (HondaFlags.BOSCH_ALT_RADAR | HondaFlags.BOSCH_RADARLESS | HondaFlags.BOSCH_CANFD)]],
  },
  extra_ecus=[
    (Ecu.combinationMeter, 0x18da60f1, None),
    (Ecu.programmedFuelInjection, 0x18da10f1, None),
    # The only other ECU on PT bus accessible by camera on radarless Civic
    # This is likely a manufacturer-specific sub-address implementation: the camera responds to this and 0x18dab0f1
    # Unclear what the part number refers to: 8S103 is 'Camera Set Mono', while 36160 is 'Camera Monocular - Honda'
    # TODO: add query back, camera does not support querying both in parallel and 0x18dab0f1 often fails to respond
    # (Ecu.unknown, 0x18DAB3F1, None),
  ],
)
```

## opendbc/car/honda/fingerprints.py — ECU keys per platform (extracted, not verbatim)

- `CAR.HONDA_ODYSSEY`: (Ecu.eps, 0x18da30f1, None) (Ecu.fwdRadar, 0x18dab0f1, None) (Ecu.gateway, 0x18daeff1, None) (Ecu.shiftByWire, 0x18da0bf1, None) (Ecu.srs, 0x18da53f1, None) (Ecu.transmission, 0x18da1ef1, None) (Ecu.vsa, 0x18da28f1, None) 
- `CAR.HONDA_PILOT`: (Ecu.eps, 0x18da30f1, None) (Ecu.fwdRadar, 0x18dab0f1, None) (Ecu.gateway, 0x18daeff1, None) (Ecu.shiftByWire, 0x18da0bf1, None) (Ecu.srs, 0x18da53f1, None) (Ecu.transmission, 0x18da1ef1, None) (Ecu.vsa, 0x18da28f1, None) 
- `CAR.HONDA_ACCORD`: (Ecu.electricBrakeBooster, 0x18da2bf1, None) (Ecu.eps, 0x18da30f1, None) (Ecu.fwdCamera, 0x18dab5f1, None) (Ecu.fwdRadar, 0x18dab0f1, None) (Ecu.gateway, 0x18daeff1, None) (Ecu.hud, 0x18da61f1, None) (Ecu.shiftByWire, 0x18da0bf1, None) (Ecu.srs, 0x18da53f1, None) (Ecu.transmission, 0x18da1ef1, None) (Ecu.vsa, 0x18da28f1, None) 
- `CAR.HONDA_CIVIC`: (Ecu.eps, 0x18da30f1, None) (Ecu.fwdRadar, 0x18dab0f1, None) (Ecu.gateway, 0x18daeff1, None) (Ecu.srs, 0x18da53f1, None) (Ecu.transmission, 0x18da1ef1, None) (Ecu.vsa, 0x18da28f1, None) 
- `CAR.HONDA_CRV_5G`: (Ecu.electricBrakeBooster, 0x18da2bf1, None) (Ecu.eps, 0x18da30f1, None) (Ecu.fwdCamera, 0x18dab5f1, None) (Ecu.fwdRadar, 0x18dab0f1, None) (Ecu.gateway, 0x18daeff1, None) (Ecu.srs, 0x18da53f1, None) (Ecu.transmission, 0x18da1ef1, None) (Ecu.vsa, 0x18da28f1, None) 
