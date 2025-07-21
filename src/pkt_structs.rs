use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use chrono::{Utc, DateTime};

#[derive(Debug)]
#[repr(C)]
pub struct Header {
    pub time:                DateTime<Utc>,
    pub packet_format:       i16,
    pub maj_version:         i8,
    pub min_version:         i8, 
    pub packet_version:      i8,
    pub packet_id:           i8,
    pub session_uid:         i64, 
    pub session_time:        f32,
    pub frame_id:            i32, 
    pub player_car_idx:      i8, 
    pub sec_player_car_id:   i8,
}

impl Header {
    pub fn decode_header(packet: &[u8]) -> Self {
        let mut rdr = Cursor::new(packet);

        Self {
            time:               Utc::now(),
            packet_format:      rdr.read_i16::<LittleEndian>().unwrap(),
            maj_version:        rdr.read_i8().unwrap(),
            min_version:        rdr.read_i8().unwrap(),
            packet_version:     rdr.read_i8().unwrap(),
            packet_id:          rdr.read_i8().unwrap(),
            session_uid:        rdr.read_i64::<LittleEndian>().unwrap(),
            session_time:       rdr.read_f32::<LittleEndian>().unwrap(),
            frame_id:           rdr.read_i32::<LittleEndian>().unwrap(),
            player_car_idx:     rdr.read_i8().unwrap(),
            sec_player_car_id:  rdr.read_i8().unwrap(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
#[repr(C)]
pub struct LapDataPacket{
    pub time:                DateTime<Utc>,
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
            time:                           Utc::now(),
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
    pub time:          DateTime<Utc>,
    pub speed_kph:         i16,
    pub throttle:          f32,
    pub steer:             f32, 
    pub brake:             f32,
    pub clutch:            i8,
    pub gear:              i8,
    pub eng_rpm:           i16,
    pub drs:               i8,
    pub rev_light_percent: i8,
    pub brake_1_temp:      i16,
    pub brake_2_temp:      i16,
    pub brake_3_temp:      i16,
    pub brake_4_temp:      i16,
    pub tyre_1_surface_temp: i8,
    pub tyre_2_surface_temp: i8,
    pub tyre_3_surface_temp: i8,
    pub tyre_4_surface_temp: i8,
    pub tyre_1_inner_temp:   i8, 
    pub tyre_2_inner_temp:   i8, 
    pub tyre_3_inner_temp:   i8, 
    pub tyre_4_inner_temp:   i8,
    pub engine_temp:         i16,
    pub tyre_1_pressure:     f32, 
    pub tyre_2_pressure:     f32, 
    pub tyre_3_pressure:     f32, 
    pub tyre_4_pressure:     f32,
    pub tyre_1_surf_type:    i8, 
    pub tyre_2_surf_type:    i8, 
    pub tyre_3_surf_type:    i8, 
    pub tyre_4_surf_type:    i8, 
}

impl CarTelemetryPacket{
   pub fn decode_car_tlm_packet(packet_chunk: &[u8]) -> Self {
        let mut rdr = Cursor::new(packet_chunk);

        Self {
            time:               Utc::now(),
            speed_kph:          rdr.read_i16::<LittleEndian>().unwrap(), 
            throttle:           rdr.read_f32::<LittleEndian>().unwrap(),
            steer:              rdr.read_f32::<LittleEndian>().unwrap(),
            brake:              rdr.read_f32::<LittleEndian>().unwrap(),
            clutch:             rdr.read_i8().unwrap(),
            gear:               rdr.read_i8().unwrap(),
            eng_rpm:            rdr.read_i16::<LittleEndian>().unwrap(),
            drs:                rdr.read_i8().unwrap(),
            rev_light_percent:  rdr.read_i8().unwrap(),
            brake_1_temp:       rdr.read_i16::<LittleEndian>().unwrap(),
            brake_2_temp:       rdr.read_i16::<LittleEndian>().unwrap(),
            brake_3_temp:       rdr.read_i16::<LittleEndian>().unwrap(),
            brake_4_temp:       rdr.read_i16::<LittleEndian>().unwrap(),
            tyre_1_surface_temp: rdr.read_i8().unwrap(),
            tyre_2_surface_temp: rdr.read_i8().unwrap(),
            tyre_3_surface_temp: rdr.read_i8().unwrap(),
            tyre_4_surface_temp: rdr.read_i8().unwrap(),
            tyre_1_inner_temp:   rdr.read_i8().unwrap(),  
            tyre_2_inner_temp:   rdr.read_i8().unwrap(),  
            tyre_3_inner_temp:   rdr.read_i8().unwrap(),  
            tyre_4_inner_temp:   rdr.read_i8().unwrap(), 
            engine_temp:         rdr.read_i16::<LittleEndian>().unwrap(),
            tyre_1_pressure:     rdr.read_f32::<LittleEndian>().unwrap(), 
            tyre_2_pressure:     rdr.read_f32::<LittleEndian>().unwrap(), 
            tyre_3_pressure:     rdr.read_f32::<LittleEndian>().unwrap(), 
            tyre_4_pressure:     rdr.read_f32::<LittleEndian>().unwrap(),
            tyre_1_surf_type:    rdr.read_i8().unwrap(), 
            tyre_2_surf_type:    rdr.read_i8().unwrap(), 
            tyre_3_surf_type:    rdr.read_i8().unwrap(), 
            tyre_4_surf_type:    rdr.read_i8().unwrap(), 
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