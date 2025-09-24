use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn read_varint(stream: &mut TcpStream) -> i32 {
    let mut num_read = 0;
    let mut result = 0;
    loop {
        let mut buf = [0u8; 1];
        stream.read_exact(&mut buf).unwrap();
        let byte = buf[0];

        result |= ((byte & 0x7F) as i32) << (7 * num_read);
        num_read += 1;

        if byte & 0x80 == 0 {
            break;
        }
    }
    result
}

fn write_varint(mut value: i32, buffer: &mut Vec<u8>) {
    loop {
        let mut temp = (value & 0b0111_1111) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0b1000_0000;
        }
        buffer.push(temp);
        if value == 0 {
            break;
        }
    }
}

fn read_string(stream: &mut TcpStream) -> String {
    let length = read_varint(stream) as usize;
    let mut buffer = vec![0u8; length];
    stream.read_exact(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

fn make_status_response() -> Vec<u8> {
    let json = r#"{
        "version": {"name": "1.21.8", "protocol": 772},
        "players": {"max": 20, "online": 1},
        "description": {"text": "Hello from FeO_mc (The numbers are fake)"}
    }"#;

    let mut packet_data = Vec::new();
    write_varint(0x00, &mut packet_data);

    let json_bytes = json.as_bytes();
    write_varint(json_bytes.len() as i32, &mut packet_data);
    packet_data.extend_from_slice(json_bytes);

    let mut packet = Vec::new();
    write_varint(packet_data.len() as i32, &mut packet);
    packet.extend_from_slice(&packet_data);

    packet
}

fn handle_client(mut stream: TcpStream) {
    let _packet_length = read_varint(&mut stream);
    let packet_id = read_varint(&mut stream);

    if packet_id != 0x00 {
        println!("Starting packet is not handshake");
        return;
    }

    let protocol_version = read_varint(&mut stream);
    let server_address = read_string(&mut stream);

    let mut port_bytes = [0u8; 2];
    stream.read_exact(&mut port_bytes).unwrap();
    let server_port = u16::from_be_bytes(port_bytes);

    let next_state = read_varint(&mut stream);

    println!(
        "Handshake received: proto={}, addr={}, port={}, next_state={}",
        protocol_version, server_address, server_port, next_state
    );

    if next_state == 1 {
        let _len = read_varint(&mut stream);
        let req_id = read_varint(&mut stream);

        if req_id == 0x00 {
            let response = make_status_response();
            stream.write_all(&response).unwrap();
            println!("MOTD JSON sent");
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:25565")?;
    println!("Server started on port 25565");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => println!("Connection error: {}", e),
        }
    }
    Ok(())
}
