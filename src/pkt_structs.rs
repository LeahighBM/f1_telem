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

#[derive(Debug)]
#[repr(C)]
pub struct MotionDataPacket{
    pub time:               DateTime<Utc>,
    pub pos_x:              f32,
    pub pos_y:              f32,
    pub pos_z:              f32,
    pub vel_x:              f32,
    pub vel_y:              f32,
    pub vel_z:              f32,
    pub world_forward_x:    i16,
    pub world_forward_y:    i16,
    pub world_forward_z:    i16,
    pub world_right_x:      i16,
    pub world_right_y:      i16,
    pub world_right_z:      i16,
    pub lat_g_force:        f32,
    pub long_g_force:       f32,
    pub vert_g_force:       f32,
    pub yaw_rads:           f32,
    pub pitch_rads:         f32,
    pub roll_rads:          f32
}

impl MotionDataPacket {
    pub fn decode_motion_data_pkt(packet_chunk: &[u8]) -> Self {
        let mut rdr = Cursor::new(packet_chunk);

        Self {
            time:               Utc::now(),
            pos_x:              rdr.read_f32::<LittleEndian>().unwrap(),
            pos_y:              rdr.read_f32::<LittleEndian>().unwrap(),
            pos_z:              rdr.read_f32::<LittleEndian>().unwrap(),
            vel_x:              rdr.read_f32::<LittleEndian>().unwrap(),
            vel_y:              rdr.read_f32::<LittleEndian>().unwrap(),
            vel_z:              rdr.read_f32::<LittleEndian>().unwrap(),
            world_forward_x:    rdr.read_i16::<LittleEndian>().unwrap(),
            world_forward_y:    rdr.read_i16::<LittleEndian>().unwrap(),
            world_forward_z:    rdr.read_i16::<LittleEndian>().unwrap(),
            world_right_x:      rdr.read_i16::<LittleEndian>().unwrap(),
            world_right_y:      rdr.read_i16::<LittleEndian>().unwrap(),
            world_right_z:      rdr.read_i16::<LittleEndian>().unwrap(),
            lat_g_force:        rdr.read_f32::<LittleEndian>().unwrap(),
            long_g_force:       rdr.read_f32::<LittleEndian>().unwrap(),
            vert_g_force:       rdr.read_f32::<LittleEndian>().unwrap(),
            yaw_rads:           rdr.read_f32::<LittleEndian>().unwrap(),
            pitch_rads:         rdr.read_f32::<LittleEndian>().unwrap(),
            roll_rads:          rdr.read_f32::<LittleEndian>().unwrap(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
#[repr(C)]
pub struct LapDataPacket{
    pub time:          DateTime<Utc>,
    pub last_lap_time:     f32,        
    pub current_lap_time:  f32,   
    pub sector1TimeInMS:   i16,        
    pub sector2TimeInMS:   i16,        
    pub bestLapTime:       f32,        
    pub bestLapNum:        i8,        
    pub bestLapSector1TimeInMS:     i16,
    pub bestLapSector2TimeInMS:     i16,
    pub bestLapSector3TimeInMS:     i16,
    pub bestOverallSector1TimeInMS: i16,
    pub bestOverallSector1LapNum:   i8,
    pub bestOverallSector2TimeInMS: i16,
    pub bestOverallSector2LapNum:   i8,
    pub bestOverallSector3TimeInMS: i16,
    pub bestOverallSector3LapNum:   i8,
    pub lapDistance:       f32,                                         
    pub totalDistance:     f32,                                    
    pub safetyCarDelta:    f32,       
    pub carPosition:       i8,        
    pub currentLapNum:     i8,         
    pub pitStatus:         i8,         
    pub sector:            i8,         
    pub currentLapInvalid: i8,         
    pub penalties:         i8,        
    pub gridPosition:      i8,        
    pub driverStatus:      i8,                                   
    pub resultStatus:      i8,        
}

impl LapDataPacket {
    pub fn decode_lap_data_packet(packet_chunk: &[u8]) -> Self {
        let mut rdr = Cursor::new(packet_chunk);

        Self {
            time:                           Utc::now(),
            last_lap_time:                  rdr.read_f32::<LittleEndian>().unwrap(),
            current_lap_time:               rdr.read_f32::<LittleEndian>().unwrap(),
            sector1TimeInMS:                rdr.read_i16::<LittleEndian>().unwrap(),  
            sector2TimeInMS:                rdr.read_i16::<LittleEndian>().unwrap(),
            bestLapTime:                    rdr.read_f32::<LittleEndian>().unwrap(),
            bestLapNum:                     rdr.read_i8().unwrap(),
            bestLapSector1TimeInMS:         rdr.read_i16::<LittleEndian>().unwrap(), 
            bestLapSector2TimeInMS:         rdr.read_i16::<LittleEndian>().unwrap(),
            bestLapSector3TimeInMS:         rdr.read_i16::<LittleEndian>().unwrap(),
            bestOverallSector1TimeInMS:     rdr.read_i16::<LittleEndian>().unwrap(),
            bestOverallSector1LapNum:       rdr.read_i8().unwrap(),
            bestOverallSector2TimeInMS:     rdr.read_i16::<LittleEndian>().unwrap(),
            bestOverallSector2LapNum:       rdr.read_i8().unwrap(),
            bestOverallSector3TimeInMS:     rdr.read_i16::<LittleEndian>().unwrap(),
            bestOverallSector3LapNum:       rdr.read_i8().unwrap(),
            lapDistance:                    rdr.read_f32::<LittleEndian>().unwrap(),
            totalDistance:                  rdr.read_f32::<LittleEndian>().unwrap(),
            safetyCarDelta:                 rdr.read_f32::<LittleEndian>().unwrap(),
            carPosition:                    rdr.read_i8().unwrap(),
            currentLapNum:                  rdr.read_i8().unwrap(),
            pitStatus:                      rdr.read_i8().unwrap(), 
            sector:                         rdr.read_i8().unwrap(), 
            currentLapInvalid:              rdr.read_i8().unwrap(), 
            penalties:                      rdr.read_i8().unwrap(), 
            gridPosition:                   rdr.read_i8().unwrap(), 
            driverStatus:                   rdr.read_i8().unwrap(), 
            resultStatus:                   rdr.read_i8().unwrap(), 
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
    pub engine_rpm:        i16,
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
            engine_rpm:         rdr.read_i16::<LittleEndian>().unwrap(),
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