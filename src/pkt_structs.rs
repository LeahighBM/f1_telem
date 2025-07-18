use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug)]
#[repr(C)]
pub struct Header {
    packet_format:    u16,
    maj_version:      u8,
    min_version:      u8, 
    packet_version:    u8,
    pub packet_id:         u8,
    session_uid:       u64, 
    session_time:      f32,
    frame_id:          u32, 
    player_car_idx:    u8, 
    sec_player_car_id: u8,
}

impl Header {
    pub fn decode_header(packet: &[u8]) -> Self {
        let mut rdr = Cursor::new(packet);

        Self {
            packet_format:      rdr.read_u16::<LittleEndian>().unwrap(),
            maj_version:        rdr.read_u8().unwrap(),
            min_version:        rdr.read_u8().unwrap(),
            packet_version:     rdr.read_u8().unwrap(),
            packet_id:          rdr.read_u8().unwrap(),
            session_uid:        rdr.read_u64::<LittleEndian>().unwrap(),
            session_time:       rdr.read_f32::<LittleEndian>().unwrap(),
            frame_id:           rdr.read_u32::<LittleEndian>().unwrap(),
            player_car_idx:     rdr.read_u8().unwrap(),
            sec_player_car_id:  rdr.read_u8().unwrap(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
#[repr(C)]
pub struct LapDataPacket{
    last_lap_time:     f32,        
    current_lap_time:  f32,   
    sector1TimeInMS:   u16,        
    sector2TimeInMS:   u16,        
    bestLapTime:       f32,        
    bestLapNum:        u8,        
    bestLapSector1TimeInMS:     u16,
    bestLapSector2TimeInMS:     u16,
    bestLapSector3TimeInMS:     u16,
    bestOverallSector1TimeInMS: u16,
    bestOverallSector1LapNum:   u8,
    bestOverallSector2TimeInMS: u16,
    bestOverallSector2LapNum:   u8,
    bestOverallSector3TimeInMS: u16,
    bestOverallSector3LapNum:   u8,
    lapDistance:       f32,                                         
    totalDistance:     f32,                                    
    safetyCarDelta:    f32,       
    carPosition:       u8,        
    currentLapNum:     u8,         
    pitStatus:         u8,         
    sector:            u8,         
    currentLapInvalid: u8,         
    penalties:         u8,        
    gridPosition:      u8,        
    driverStatus:      u8,                                   
    resultStatus:      u8,        
}

impl LapDataPacket {
    pub fn decode_lap_data_packet(packet_chunk: &[u8]) -> Self {
        let mut rdr = Cursor::new(packet_chunk);

        Self {
            last_lap_time:                  rdr.read_f32::<LittleEndian>().unwrap(),
            current_lap_time:               rdr.read_f32::<LittleEndian>().unwrap(),
            sector1TimeInMS:                rdr.read_u16::<LittleEndian>().unwrap(),  
            sector2TimeInMS:                rdr.read_u16::<LittleEndian>().unwrap(),
            bestLapTime:                    rdr.read_f32::<LittleEndian>().unwrap(),
            bestLapNum:                     rdr.read_u8().unwrap(),
            bestLapSector1TimeInMS:         rdr.read_u16::<LittleEndian>().unwrap(), 
            bestLapSector2TimeInMS:         rdr.read_u16::<LittleEndian>().unwrap(),
            bestLapSector3TimeInMS:         rdr.read_u16::<LittleEndian>().unwrap(),
            bestOverallSector1TimeInMS:     rdr.read_u16::<LittleEndian>().unwrap(),
            bestOverallSector1LapNum:       rdr.read_u8().unwrap(),
            bestOverallSector2TimeInMS:     rdr.read_u16::<LittleEndian>().unwrap(),
            bestOverallSector2LapNum:       rdr.read_u8().unwrap(),
            bestOverallSector3TimeInMS:     rdr.read_u16::<LittleEndian>().unwrap(),
            bestOverallSector3LapNum:       rdr.read_u8().unwrap(),
            lapDistance:                    rdr.read_f32::<LittleEndian>().unwrap(),
            totalDistance:                  rdr.read_f32::<LittleEndian>().unwrap(),
            safetyCarDelta:                 rdr.read_f32::<LittleEndian>().unwrap(),
            carPosition:                    rdr.read_u8().unwrap(),
            currentLapNum:                  rdr.read_u8().unwrap(),
            pitStatus:                      rdr.read_u8().unwrap(), 
            sector:                         rdr.read_u8().unwrap(), 
            currentLapInvalid:              rdr.read_u8().unwrap(), 
            penalties:                      rdr.read_u8().unwrap(), 
            gridPosition:                   rdr.read_u8().unwrap(), 
            driverStatus:                   rdr.read_u8().unwrap(), 
            resultStatus:                   rdr.read_u8().unwrap(), 
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CarTelemetryPacket{
    speed_kph:             u16,
    throttle:          f32,
    steer:             f32, 
    brake:             f32,
    clutch:            u8,
    gear:              i8,
    eng_rpm:           u16,
    drs:               u8,
    rev_light_percent: u8,
    brake_1_temp:      u16,
    brake_2_temp:      u16,
    brake_3_temp:      u16,
    brake_4_temp:      u16,
    tyre_1_surface_temp: u8,
    tyre_2_surface_temp: u8,
    tyre_3_surface_temp: u8,
    tyre_4_surface_temp: u8,
    tyre_1_inner_temp:   u8, 
    tyre_2_inner_temp:   u8, 
    tyre_3_inner_temp:   u8, 
    tyre_4_inner_temp:   u8,
    engine_temp:         u16,
    tyre_1_pressure:     f32, 
    tyre_2_pressure:     f32, 
    tyre_3_pressure:     f32, 
    tyre_4_pressure:     f32,
    tyre_1_surf_type:    u8, 
    tyre_2_surf_type:    u8, 
    tyre_3_surf_type:    u8, 
    tyre_4_surf_type:    u8, 
}

impl CarTelemetryPacket{
   pub fn decode_car_tlm_packet(packet_chunk: &[u8]) -> Self {
        let mut rdr = Cursor::new(packet_chunk);

        Self {
            speed_kph:          rdr.read_u16::<LittleEndian>().unwrap(), 
            throttle:           rdr.read_f32::<LittleEndian>().unwrap(),
            steer:              rdr.read_f32::<LittleEndian>().unwrap(),
            brake:              rdr.read_f32::<LittleEndian>().unwrap(),
            clutch:             rdr.read_u8().unwrap(),
            gear:               rdr.read_i8().unwrap(),
            eng_rpm:            rdr.read_u16::<LittleEndian>().unwrap(),
            drs:                rdr.read_u8().unwrap(),
            rev_light_percent:  rdr.read_u8().unwrap(),
            brake_1_temp:       rdr.read_u16::<LittleEndian>().unwrap(),
            brake_2_temp:       rdr.read_u16::<LittleEndian>().unwrap(),
            brake_3_temp:       rdr.read_u16::<LittleEndian>().unwrap(),
            brake_4_temp:       rdr.read_u16::<LittleEndian>().unwrap(),
            tyre_1_surface_temp: rdr.read_u8().unwrap(),
            tyre_2_surface_temp: rdr.read_u8().unwrap(),
            tyre_3_surface_temp: rdr.read_u8().unwrap(),
            tyre_4_surface_temp: rdr.read_u8().unwrap(),
            tyre_1_inner_temp:   rdr.read_u8().unwrap(),  
            tyre_2_inner_temp:   rdr.read_u8().unwrap(),  
            tyre_3_inner_temp:   rdr.read_u8().unwrap(),  
            tyre_4_inner_temp:   rdr.read_u8().unwrap(), 
            engine_temp:         rdr.read_u16::<LittleEndian>().unwrap(),
            tyre_1_pressure:     rdr.read_f32::<LittleEndian>().unwrap(), 
            tyre_2_pressure:     rdr.read_f32::<LittleEndian>().unwrap(), 
            tyre_3_pressure:     rdr.read_f32::<LittleEndian>().unwrap(), 
            tyre_4_pressure:     rdr.read_f32::<LittleEndian>().unwrap(),
            tyre_1_surf_type:    rdr.read_u8().unwrap(), 
            tyre_2_surf_type:    rdr.read_u8().unwrap(), 
            tyre_3_surf_type:    rdr.read_u8().unwrap(), 
            tyre_4_surf_type:    rdr.read_u8().unwrap(), 
        }
    }
}

// pub struct Test{
//     packet_format:    u16, // size evals to 2 (good)
//     maj_version:       u8, // size evals to 4 (bad)
//     min_version:       u8, // size evals to 4 (good)
//     foo:               u8, // size evals to 6 (bad)     
//     bar:               u8, // size evals to 6 (good)

// }