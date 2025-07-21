use postgres::{Client, Error};

use crate::pkt_structs::{CarTelemetryPacket, Header};

pub fn db_create_tables(client: &mut Client) -> Result<(), Error> {

//     packet_format:     u16,
//     maj_version:       u8,
//     min_version:       u8, 
//     packet_version:    u8,
//     pub packet_id:     u8,
//     session_uid:       u64, 
//     session_time:      f32,
//     frame_id:          u32, 
//     player_car_idx:    u8, 
//     sec_player_car_id: u8,

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


// pub time:          DateTime<Utc>,
//     speed_kph:         u16,
//     throttle:          f32,
//     steer:             f32, 
//     brake:             f32,
//     clutch:            u8,
//     gear:              i8,
//     eng_rpm:           u16,
//     drs:               u8,
//     rev_light_percent: u8,
//     brake_1_temp:      u16,
//     brake_2_temp:      u16,
//     brake_3_temp:      u16,
//     brake_4_temp:      u16,
//     tyre_1_surface_temp: u8,
//     tyre_2_surface_temp: u8,
//     tyre_3_surface_temp: u8,
//     tyre_4_surface_temp: u8,
//     tyre_1_inner_temp:   u8, 
//     tyre_2_inner_temp:   u8, 
//     tyre_3_inner_temp:   u8, 
//     tyre_4_inner_temp:   u8,
//     engine_temp:         u16,
//     tyre_1_pressure:     f32, 
//     tyre_2_pressure:     f32, 
//     tyre_3_pressure:     f32, 
//     tyre_4_pressure:     f32,
//     tyre_1_surf_type:    u8, 
//     tyre_2_surf_type:    u8, 
//     tyre_3_surf_type:    u8, 
//     tyre_4_surf_type:    u8, 

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
        tyre_1_pressure     SMALLINT    NOT NULL,
        tyre_2_pressure     SMALLINT    NOT NULL,
        tyre_3_pressure     SMALLINT    NOT NULL,
        tyre_4_pressure     SMALLINT    NOT NULL,
        tyre_1_surf_type    SMALLINT    NOT NULL,
        tyre_2_surf_type    SMALLINT    NOT NULL,
        tyre_3_surf_type    SMALLINT    NOT NULL,
        tyre_4_surf_type    SMALLINT    NOT NULL);
        
        SELECT create_hypertable('tbl_car_tlm', 'time', if_not_exists => TRUE);"
    )?;


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
        $1, 
        $2,
        $3, 
        $4, 
        $5,
        $6, 
        $7,
        $8, 
        $9, 
        $10, 
        $11, 
        $12,
        $13, 
        $14, 
        $15,
        $16, 
        $17, 
        $18,
        $19, 
        $20, 
        $21,
        $22, 
        $23,
        $24, 
        $25,
        $26, 
        $27,
        $28, 
        $29,
        $30,
        $31);", &[
            &pkt.time,
            &pkt.speed_kph,
            &pkt.throttle,           
            &pkt.steer,              
            &pkt.brake,              
            &(pkt.clutch as i16),             
            &(pkt.gear as i16),               
            &pkt.eng_rpm,         
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
            &(pkt.tyre_1_pressure as i16),    
            &(pkt.tyre_2_pressure as i16),    
            &(pkt.tyre_3_pressure as i16),    
            &(pkt.tyre_4_pressure as i16),    
            &(pkt.tyre_1_surf_type as i16),   
            &(pkt.tyre_2_surf_type as i16),   
            &(pkt.tyre_3_surf_type as i16),   
            &(pkt.tyre_4_surf_type as i16)
        ])?;

    Ok(())
}

pub fn header_insert(client: &mut Client, p_header: &Header)-> Result<(), Error> {
        // println!("{:#?}", p_header);

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

        // println!("Received {} bytes from {} -> \n\t{:?}", bytes, src, p_header);
        // println!("{:?}", packet[23]);

        Ok(())
}