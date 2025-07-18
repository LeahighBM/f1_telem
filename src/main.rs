use std::net::UdpSocket;
// use std::mem;
mod pkt_structs;

#[derive(Debug)]
#[repr(C)]
pub struct LapDataPacket{
    last_lap_time: f32,
}

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:20777").expect("Failed to bind socket");
    let mut buffer = [0; 2048];

    // const PKT_HEADER_SIZE: usize = mem::size_of::<pkt_structs::Header>();
    // const LPD_LEN: usize         = mem::size_of::<pkt_structs::LapDataPacket>();
    // const CAR_TLM_LEN: usize     = mem::size_of::<pkt_structs::CarTelemetryPacket>();
    // const TEST_LEN: usize        = mem::size_of::<pkt_structs::Test>();

    // println!("Header struct size {}", PKT_HEADER_SIZE); //SHOULD BE 24 but is reading 32?!?
    // println!("Header struct size {}", TEST_LEN); // testing with this shows that a u16 + a u8 = 4 bytes and so does u16 + u8 + u8. 


    println!("Listening...");
    loop {
        let (bytes, _) = socket.recv_from(&mut buffer).expect("Failed to receive data");

        let packet = &buffer[..bytes];
        // println!("{}", packet.len());
        let pkt_data = &buffer[24..packet.len()]; // header is 24 bytes (idx 0..23) start at next place and go to the end. 

        let p_header= pkt_structs::Header::decode_header(packet);

        // println!("Received {} bytes from {} -> \n\t{:?}", bytes, src, p_header);
        // println!("{:?}", packet[23]);
        

        match p_header.packet_id {
            0 => {}//println!("Motion Packet"),
            1 => {} //println!("Session Packet"),
            2 => {
                // let lpd_data = &pkt_data[1166 - 53..]; // 1190 - 24 = 1166, packet is 53 bytes, driver index is last index aka last 53 bytes. 
                // let packet = pkt_structs::LapDataPacket::decode_lap_data_packet(lpd_data);
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
                println!("\n{:?}", packet);
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