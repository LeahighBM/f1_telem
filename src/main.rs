use std::{net::UdpSocket};
use std::env;
mod pkt_structs;
mod db;
use postgres::{Client, NoTls, Error};

use crate::db::{car_insert, db_create_tables, header_insert, lap_data_insert};

#[derive(Debug)]
#[repr(C)]
pub struct LapDataPacket{
    last_lap_time: f32,
}

fn main() -> Result<(), Error>{
    let pass = env::var("PG_PASS").expect("Could not find env var");
    let db_url = format!("postgresql://postgres:{pass}@localhost/f1_telemetry");
    let mut client = Client::connect(&db_url, NoTls)?;

    db_create_tables(&mut client)?;

    let socket = UdpSocket::bind("0.0.0.0:20777").expect("Failed to bind socket");
    let mut buffer = [0; 2048];
    
    println!("Listening...");
    loop {
        let (bytes, _) = socket.recv_from(&mut buffer).expect("Failed to receive data");

        let packet   = &buffer[..bytes];
        let pkt_data = &buffer[24..packet.len()]; // header is 24 bytes (idx 0..23) start at next place and go to the end. 

        let p_header= pkt_structs::Header::decode_header(packet);
        header_insert(&mut client, &p_header)?;
        
        match p_header.packet_id {
            0 => { // "Motion Packet"
                let mp_data = &pkt_data[1440 - 180..]; // 1464-24 = 1440. pkt is 60 bytes plus 120 additional bytes at the end for 180
                let packet = pkt_structs::MotionDataPacket::decode_motion_data_pkt(mp_data);
                // motion_data_insert(&mut client, &packet); 
                println!("{:#?}", packet)
            }
            1 => { // "Session Packet"

            } 
            2 => { // "Lap Data Packet"
                let lpd_data = &pkt_data[1166 - 53..]; // 1190 - 24 = 1166, packet is 53 bytes, driver index is last index aka last 53 bytes. 
                let packet = pkt_structs::LapDataPacket::decode_lap_data_packet(lpd_data);
                lap_data_insert(&mut client, &packet)?;
                // println!("\n{:?}", packet);
                // println!("\n{:?}", pkt_data);
            },
            3 => {} //println!("Event Packet"),
            4 => {} //println!("Particpants Packet"),
            5 => {} //println!("Car Setups Packet"),
            6 => {
                let ct_data = &pkt_data[1283 - 65..]; // 1307 - 24 = 1283. should be offset by 58 buuuut the last 7 bytes of the packet contain 
                                                             // some extra info so 58 + 7 = 65. the struct will ignore everthing after it is full.
                let packet = pkt_structs::CarTelemetryPacket::decode_car_tlm_packet(ct_data);
                car_insert(&mut client, &packet)?;
                // println!("\n{:#?}", packet);
                // println!("\n{:?}", ct_data);
            },
            7 => {} //println!("Car Status Packet"),
            8 => {} //println!("Final Classification Packet"),
            9 => {} //println!("Lobby Information Packet"),
            _ => {} //println!("Packet ID not recognized")
        };        
        
        // println!("Packet data -> {:?}", pkt_data);
    }
}