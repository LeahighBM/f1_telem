use std::net::UdpSocket;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::mem;

#[derive(Debug)]
#[repr(C)]
pub struct Header {
    packet_format:    u16,
    maj_version:       u8,
    min_version:       u8, 
    packet_version:    u8,
    packet_id:         u8,
    session_uid:       u64, 
    session_time:      f32,
    frame_id:          u32, 
    player_car_idx:    u8, 
    sec_player_car_id: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct LapDataPacket{
    last_lap_time: f32,
}

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:20777").expect("Failed to bind socket");
    let mut buffer = [0; 2048];

    const PKT_HEADER_SIZE: usize = mem::size_of::<Header>();

    loop {
        let (bytes, src) = socket.recv_from(&mut buffer).expect("Failed to receive data");

        let packet = &buffer[..bytes];
        let pkt_data = &buffer[PKT_HEADER_SIZE..];
        let mut rdr = Cursor::new(packet);

        let mut p_header = Header {
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
        };

        println!("Received {} bytes from {} -> {:?}", bytes, src, p_header);

        if p_header.packet_id == 2 {
            rdr = Cursor::new(pkt_data);

            let packet = LapDataPacket {
                last_lap_time:  rdr.read_f32::<LittleEndian>().unwrap(),
            };

            println!("{:?}", packet);
        }
        
        
        // println!("Packet data -> {:?}", pkt_data);
    }
}