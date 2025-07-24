use postgres::{Client, Error};
use crate::{pkt_structs::{CarTelemetryPacket, Header, MotionDataPacket, LapDataPacket}};

pub fn db_create_tables(client: &mut Client) -> Result<(), Error> {

    client.batch_execute(
        "DROP TABLE  IF EXISTS tbl_header_pkts;
        CREATE TABLE IF NOT EXISTS tbl_header_pkts (
        id                  BIGSERIAL,
        time                TIMESTAMPTZ PRIMARY KEY,
        packet_format       SMALLINT NOT NULL,
        maj_version         SMALLINT NOT NULL,
        min_version         SMALLINT NOT NULL,
        packet_version      SMALLINT NOT NULL,
        packet_id           SMALLINT NOT NULL,
        session_uid         BIGINT   NOT NULL,
        session_time        REAL     NOT NULL,
        frame_id            INTEGER  NOT NULL,
        player_car_idx      SMALLINT NOT NULL,
        sec_player_car_id   SMALLINT NOT NULL
       
    );
    SELECT create_hypertable('tbl_header_pkts', 'time', if_not_exists => TRUE);"
)?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS tbl_car_tlm (
        id                  BIGSERIAL,
        time                TIMESTAMPTZ NOT NULL,
        speed_kph           SMALLINT    NOT NULL,
        throttle            REAL        NOT NULL,
        steer               REAL        NOT NULL,
        brake               REAL        NOT NULL,
        clutch              SMALLINT    NOT NULL,
        gear                SMALLINT    NOT NULL,
        engine_rpm          SMALLINT    NOT NULL,
        drs                 SMALLINT    NOT NULL,
        rev_light_percent   SMALLINT    NOT NULL,
        brake_1_temp        SMALLINT    NOT NULL,
        brake_2_temp        SMALLINT    NOT NULL,
        brake_3_temp        SMALLINT    NOT NULL,
        brake_4_temp        SMALLINT    NOT NULL,
        tyre_1_surface_temp SMALLINT    NOT NULL,
        tyre_2_surface_temp SMALLINT    NOT NULL,
        tyre_3_surface_temp SMALLINT    NOT NULL,
        tyre_4_surface_temp SMALLINT    NOT NULL,
        tyre_1_inner_temp   SMALLINT    NOT NULL,
        tyre_2_inner_temp   SMALLINT    NOT NULL,
        tyre_3_inner_temp   SMALLINT    NOT NULL,
        tyre_4_inner_temp   SMALLINT    NOT NULL,
        engine_temp         SMALLINT    NOT NULL,
        tyre_1_pressure     REAL        NOT NULL,
        tyre_2_pressure     REAL        NOT NULL,
        tyre_3_pressure     REAL        NOT NULL,
        tyre_4_pressure     REAL        NOT NULL,
        tyre_1_surf_type    SMALLINT    NOT NULL,
        tyre_2_surf_type    SMALLINT    NOT NULL,
        tyre_3_surf_type    SMALLINT    NOT NULL,
        tyre_4_surf_type    SMALLINT    NOT NULL);
        
        SELECT create_hypertable('tbl_car_tlm', 'time', if_not_exists => TRUE);"
    )?;


    client.batch_execute("CREATE TABLE IF NOT EXISTS tbl_lap_data (
        id                  BIGSERIAL,
        time                TIMESTAMPTZ NOT NULL,
        last_lap_time       REAL        NOT NULL,
        curr_lap_time       REAL        NOT NULL,
        sector_1_time_ms    SMALLINT    NOT NULL,
        sector_2_time_ms    SMALLINT    NOT NULL,
        best_lap_time       REAL        NOT NULL,
        best_lap_num        SMALLINT    NOT NULL,
        best_lap_s1_ms      SMALLINT    NOT NULL,
        best_lap_s2_ms      SMALLINT    NOT NULL,
        best_lap_s3_ms      SMALLINT    NOT NULL,
        best_ovr_s1_ms      SMALLINT    NOT NULL,
        best_ovr_s1_lap     SMALLINT    NOT NULL,
        best_ovr_s2_ms      SMALLINT    NOT NULL,
        best_ovr_s2_lap     SMALLINT    NOT NULL,
        best_ovr_s3_ms      SMALLINT    NOT NULL,
        best_ovr_s3_lap     SMALLINT    NOT NULL,
        lap_distance        REAL        NOT NULL,
        total_distance      REAL        NOT NULL,
        safety_car_delta    REAL        NOT NULL,
        car_position        SMALLINT    NOT NULL,
        curr_lap_num        SMALLINT    NOT NULL,
        pit_status          SMALLINT    NOT NULL,
        sector              SMALLINT    NOT NULL,
        curr_lap_invalid    SMALLINT    NOT NULL,
        penalties           SMALLINT    NOT NULL,
        grid_postion        SMALLINT    NOT NULL,
        driver_status       SMALLINT    NOT NULL,
        result_status       SMALLINT    NOT NULL
    );
    
    SELECT create_hypertable('tbl_car_tlm', 'time', if_not_exists => TRUE);"
)?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS tbl_motion_data(
            id              BIGSERIAL,
            time            TIMESTAMPTZ NOT NULL,
            pos_x           REAL NOT NULL,
            pos_y           REAL NOT NULL,
            pos_z           REAL NOT NULL,
            vel_x           REAL NOT NULL,
            vel_y           REAL NOT NULL,
            vel_z           REAL NOT NULL,
            world_forward_x SMALLINT NOT NULL,
            world_forward_y SMALLINT NOT NULL,
            world_forward_z SMALLINT NOT NULL,
            world_right_x   SMALLINT NOT NULL,
            world_right_y   SMALLINT NOT NULL,
            world_right_z   SMALLINT NOT NULL,
            lat_g_force     REAL NOT NULL,
            long_g_force    REAL NOT NULL,
            vert_g_force    REAL NOT NULL,
            yaw_rads        REAL NOT NULL,
            pitch_rads      REAL NOT NULL,
            roll_rads       REAL NOT NULL
            );
            
            
            SELECT create_hypertable('tbl_motion_data', 'time', if_not_exists => TRUE);"
    )?; 
    Ok(())

}

pub fn header_insert(client: &mut Client, p_header: &Header)-> Result<(), Error> {
       
    client.execute(
        "INSERT INTO tbl_header_pkts (
        time,
        packet_format,
        maj_version,
        min_version,
        packet_version,
        packet_id,
        session_uid,
        session_time,
        frame_id,
        player_car_idx,
        sec_player_car_id
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);", 
        &[
                    &p_header.time,
                    &p_header.packet_format,
                    &(p_header.maj_version as i16),
                    &(p_header.min_version as i16),
                    &(p_header.packet_version as i16),
                    &(p_header.packet_id as i16),
                    &p_header.session_uid,
                    &p_header.session_time,
                    &p_header.frame_id,
                    &(p_header.player_car_idx as i16),
                    &(p_header.sec_player_car_id as i16)])?;

        Ok(())
}

pub fn motion_data_insert(client: &mut Client, pkt: &MotionDataPacket) -> Result<(), Error> {
    client.execute(
        "INSERT INTO tbl_motion_data (           
        time           ,
        pos_x          ,
        pos_y          ,
        pos_z          ,
        vel_x          ,
        vel_y          ,
        vel_z          ,
        world_forward_x,
        world_forward_y,
        world_forward_z,
        world_right_x  ,
        world_right_y  ,
        world_right_z  ,
        lat_g_force    ,
        long_g_force   ,
        vert_g_force   ,
        yaw_rads       ,
        pitch_rads     ,
        roll_rads)
        VALUES (
        $1,  $2,  $3,  $4,  $5,  $6,  $7,  $8,  $9,  $10,
        $11, $12, $13, $14, $15, $16, $17, $18, $19)", 
        &[
            &pkt.time,
            &pkt.pos_x,
            &pkt.pos_y,
            &pkt.pos_z,
            &pkt.vel_x,
            &pkt.vel_y,
            &pkt.vel_z,
            &pkt.world_forward_x,
            &pkt.world_forward_y,
            &pkt.world_forward_z,
            &pkt.world_right_x,
            &pkt.world_right_y,
            &pkt.world_right_z,
            &pkt.lat_g_force,
            &pkt.long_g_force,
            &pkt.vert_g_force,
            &pkt.yaw_rads,
            &pkt.pitch_rads, 
            &pkt.roll_rads,

        ])?;

        Ok(())
}

pub fn lap_data_insert(client: &mut Client, pkt: &LapDataPacket)-> Result<(), Error> {

    client.execute(
        "INSERT INTO tbl_lap_data (
        time,
        last_lap_time,
        curr_lap_time,
        sector_1_time_ms,
        sector_2_time_ms,
        best_lap_time,
        best_lap_num,
        best_lap_s1_ms,
        best_lap_s2_ms,
        best_lap_s3_ms,
        best_ovr_s1_ms,
        best_ovr_s1_lap,
        best_ovr_s2_ms,
        best_ovr_s2_lap,
        best_ovr_s3_ms,
        best_ovr_s3_lap,
        lap_distance,
        total_distance,
        safety_car_delta,
        car_position,
        curr_lap_num,
        pit_status,
        sector,
        curr_lap_invalid,
        penalties,
        grid_postion,
        driver_status,
        result_status)
        VALUES (
        $1,  $2,  $3,  $4,  $5,  $6,  $7,  $8,  $9,  $10,
        $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
        $21, $22, $23, $24, $25, $26, $27, $28);", 
        &[
            &pkt.time,
            &pkt.last_lap_time,
            &pkt.current_lap_time,
            &pkt.sector1TimeInMS,
            &pkt.sector2TimeInMS,
            &pkt.bestLapTime,
            &(pkt.bestLapNum as i16),
            &pkt.bestLapSector1TimeInMS,
            &pkt.bestLapSector2TimeInMS,
            &pkt.bestLapSector3TimeInMS,
            &pkt.bestOverallSector1TimeInMS,
            &(pkt.bestOverallSector1LapNum as i16),
            &pkt.bestOverallSector2TimeInMS,
            &(pkt.bestOverallSector2LapNum as i16),
            &pkt.bestOverallSector3TimeInMS,
            &(pkt.bestOverallSector3LapNum as i16),
            &pkt.lapDistance,
            &pkt.totalDistance,
            &pkt.safetyCarDelta,
            &(pkt.carPosition as i16),
            &(pkt.currentLapNum as i16),
            &(pkt.pitStatus as i16),
            &(pkt.sector as i16),
            &(pkt.currentLapInvalid as i16),
            &(pkt.penalties as i16),
            &(pkt.gridPosition as i16),
            &(pkt.driverStatus as i16),
            &(pkt.resultStatus as i16)])?;

        Ok(())
}

pub fn car_insert(client: &mut Client, pkt: &CarTelemetryPacket) -> Result<(), Error> {

    client.execute(
        "INSERT INTO tbl_car_tlm (               
        time,               
        speed_kph,          
        throttle,           
        steer,              
        brake,              
        clutch,             
        gear,               
        engine_rpm,         
        drs,                
        rev_light_percent,  
        brake_1_temp,       
        brake_2_temp,       
        brake_3_temp,       
        brake_4_temp,       
        tyre_1_surface_temp,
        tyre_2_surface_temp,
        tyre_3_surface_temp,
        tyre_4_surface_temp,
        tyre_1_inner_temp,  
        tyre_2_inner_temp,  
        tyre_3_inner_temp,  
        tyre_4_inner_temp,  
        engine_temp,       
        tyre_1_pressure,    
        tyre_2_pressure,    
        tyre_3_pressure,    
        tyre_4_pressure,    
        tyre_1_surf_type,   
        tyre_2_surf_type,   
        tyre_3_surf_type,   
        tyre_4_surf_type)
        VALUES (
        $1,  $2,  $3,  $4,  $5,  $6,  $7,  $8,  $9,  $10, 
        $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, 
        $21, $22, $23, $24, $25, $26, $27, $28, $29, $30,
        $31);", &[
            &pkt.time,
            &pkt.speed_kph,
            &pkt.throttle,           
            &pkt.steer,              
            &pkt.brake,              
            &(pkt.clutch as i16),             
            &(pkt.gear as i16),               
            &pkt.engine_rpm,         
            &(pkt.drs as i16),                
            &(pkt.rev_light_percent as i16),  
            &pkt.brake_1_temp,       
            &pkt.brake_2_temp,       
            &pkt.brake_3_temp,       
            &pkt.brake_4_temp,       
            &(pkt.tyre_1_surface_temp as i16),
            &(pkt.tyre_2_surface_temp as i16),
            &(pkt.tyre_3_surface_temp as i16),
            &(pkt.tyre_4_surface_temp as i16),
            &(pkt.tyre_1_inner_temp as i16),  
            &(pkt.tyre_2_inner_temp as i16),  
            &(pkt.tyre_3_inner_temp as i16),  
            &(pkt.tyre_4_inner_temp as i16),  
            &pkt.engine_temp,       
            &pkt.tyre_1_pressure,    
            &pkt.tyre_2_pressure,    
            &pkt.tyre_3_pressure,    
            &pkt.tyre_4_pressure,    
            &(pkt.tyre_1_surf_type as i16),   
            &(pkt.tyre_2_surf_type as i16),   
            &(pkt.tyre_3_surf_type as i16),   
            &(pkt.tyre_4_surf_type as i16)
        ])?;

    Ok(())
}

