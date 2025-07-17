import socket
import struct
import sys

UDP_IP = "0.0.0.0"
UDP_PORT = 20777

'''
uint16_t  m_packetFormat;            // 2020
uint8_t   m_gameMajorVersion;        // Game major version - "X.00"
uint8_t   m_gameMinorVersion;        // Game minor version - "1.XX"
uint8_t   m_packetVersion;           // Version of this packet type, all start from 1
uint8_t   m_packetId;                // Identifier for the packet type, see below
uint64_t  m_sessionUID;              // Unique identifier for the session
float     m_sessionTime;             // Session timestamp
uint32_t  m_frameIdentifier;         // Identifier for the frame the data was retrieved on
uint8_t   m_playerCarIndex;          // Index of player's car in the array
uint8_t   m_secondaryPlayerCarIndex; // Index of secondary player's car in the array (splitscreen)
                                        // 255 if no second player
'''

header_fmt = "<HBBBBQfIBB"
header_size = struct.calcsize(header_fmt)

'''
struct LapData
{
    float       m_lastLapTime;               // Last lap time in seconds
    float       m_currentLapTime;            // Current time around the lap in seconds
    uint16_t    m_sector1TimeInMS;           // Sector 1 time in milliseconds
    uint16_t    m_sector2TimeInMS;           // Sector 2 time in milliseconds
    float       m_bestLapTime;               // Best lap time of the session in seconds
    uint8_t     m_bestLapNum;                // Lap number best time achieved on
    uint16_t    m_bestLapSector1TimeInMS;    // Sector 1 time of best lap in the session in milliseconds
    uint16_t    m_bestLapSector2TimeInMS;    // Sector 2 time of best lap in the session in milliseconds
    uint16_t    m_bestLapSector3TimeInMS;    // Sector 3 time of best lap in the session in milliseconds
    uint16_t    m_bestOverallSector1TimeInMS;// Best overall sector 1 time of the session in milliseconds
    uint8_t     m_bestOverallSector1LapNum;  // Lap number best overall sector 1 time achieved on
    uint16_t    m_bestOverallSector2TimeInMS;// Best overall sector 2 time of the session in milliseconds
    uint8_t     m_bestOverallSector2LapNum;  // Lap number best overall sector 2 time achieved on
    uint16_t    m_bestOverallSector3TimeInMS;// Best overall sector 3 time of the session in milliseconds
    uint8_t     m_bestOverallSector3LapNum;  // Lap number best overall sector 3 time achieved on
    float       m_lapDistance;               // Distance vehicle is around current lap in metres – could
                                             // be negative if line hasn’t been crossed yet
    float       m_totalDistance;             // Total distance travelled in session in metres – could
                                             // be negative if line hasn’t been crossed yet
    float       m_safetyCarDelta;            // Delta in seconds for safety car
    uint8_t     m_carPosition;               // Car race position
    uint8_t     m_currentLapNum;             // Current lap number
    uint8_t     m_pitStatus;                 // 0 = none, 1 = pitting, 2 = in pit area
    uint8_t     m_sector;                    // 0 = sector1, 1 = sector2, 2 = sector3
    uint8_t     m_currentLapInvalid;         // Current lap invalid - 0 = valid, 1 = invalid
    uint8_t     m_penalties;                 // Accumulated time penalties in seconds to be added
    uint8_t     m_gridPosition;              // Grid position the vehicle started the race in
    uint8_t     m_driverStatus;              // Status of driver - 0 = in garage, 1 = flying lap
                                             // 2 = in lap, 3 = out lap, 4 = on track
    uint8_t     m_resultStatus;              // Result status - 0 = invalid, 1 = inactive, 2 = active
                                             // 3 = finished, 4 = disqualified, 5 = not classified
                                             // 6 = retired
};
'''

lap_data_pkt_fmt = "<ffHHfBHHHHBHBHBfffBBBBBBBBB"
lap_data_pkt_size = struct.calcsize(lap_data_pkt_fmt)


'''
struct CarTelemetryData
{
    uint16_t  m_speed;                      // Speed of car in kilometres per hour
    float     m_throttle;                   // Amount of throttle applied (0.0 to 1.0)
    float     m_steer;                      // Steering (-1.0 (full lock left) to 1.0 (full lock right))
    float     m_brake;                      // Amount of brake applied (0.0 to 1.0)
    uint8_t   m_clutch;                     // Amount of clutch applied (0 to 100)
    int8_t    m_gear;                       // Gear selected (1-8, N=0, R=-1)
    uint16_t  m_engineRPM;                  // Engine RPM
    uint8_t   m_drs;                        // 0 = off, 1 = on
    uint8_t   m_revLightsPercent;           // Rev lights indicator (percentage)
    uint16_t  m_brakesTemperature[4];       // Brakes temperature (celsius)
    uint8_t   m_tyresSurfaceTemperature[4]; // Tyres surface temperature (celsius)
    uint8_t   m_tyresInnerTemperature[4];   // Tyres inner temperature (celsius)
    uint16_t  m_engineTemperature;          // Engine temperature (celsius)
    float     m_tyresPressure[4];           // Tyres pressure (PSI)
    uint8_t   m_surfaceType[4];             // Driving surface, see appendices
};
'''

car_tlm_pkt_fmt = "<HfffBbHBBHBBHfB"
car_tlm_pkt_size = struct.calcsize(car_tlm_pkt_fmt)

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

sock.bind((UDP_IP, UDP_PORT))

def car_data_pretty(car_data_packet):
    print(f"""
speed:    {car_data_packet[0]}
throttle: {car_data_packet[1]}
steer:    {car_data_packet[2]}
brake:    {car_data_packet[3]}
clutch:   {car_data_packet[4]}
gear:     {car_data_packet[5]}
Eng RPM:  {car_data_packet[6]}
""")

print("Listening...")

while True:
    data, addr = sock.recvfrom(2048)

    header_data = data[:header_size]
    unpacked_header = struct.unpack(header_fmt, header_data)

    packet_id = unpacked_header[4]

    print(f"Rx Data: {unpacked_header} from {addr}")

    match packet_id:
        case 2: 
            # print("\tLap Data Packet:")
            ldp_data =  data[header_size:header_size + lap_data_pkt_size]
            unpacked_ldp = struct.unpack(lap_data_pkt_fmt, ldp_data)
            # print(f"\t\t{unpacked_ldp}")
        case 6: 
            print("\tCar Telemetry")
            car_data = data[header_size:header_size + car_tlm_pkt_size]
            unpacked_car_data = struct.unpack(car_tlm_pkt_fmt, car_data)
            car_data_pretty(unpacked_car_data)
