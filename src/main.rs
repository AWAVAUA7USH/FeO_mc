use std::string;

fn write_varint(number: i32) -> Vec<u8> {
    let mut result = Vec::new();
    let mut value = number as u32;

    while value >= 0x80 {
        result.push((value as u8) | 0x80);
        value >>= 7;
    }
    result.push(value as u8);

    result
}

fn read_varint(varint: &[u8]) -> i32 {
    let mut result: i32 = 0;
    let mut shift: u32 = 0;

    for &byte in varint.iter() {
        result |= ((byte & 0b0111_1111) as i32) << shift;
        shift += 7;
    }

    result
}

fn write_string(string: &str) -> Vec<u8> {
    let string_utf8: &[u8] = string.as_bytes();
    let mut string_vec: Vec<u8> = string_utf8.to_vec();
    let mut data_length_varint = write_varint(string_vec.len() as i32);
    data_length_varint.append(&mut string_vec);

    return data_length_varint;
}

fn read_string(bytes: Vec<u8>) -> String {
    let mut value: i32 = 0;
    let mut shift_amount: u32 = 0;
    let mut varint_byte_count: usize = 0;

    for &byte in &bytes {
        value |= ((byte & 0b0111_1111) as i32) << shift_amount;
        shift_amount += 7;
        varint_byte_count += 1;
        if (byte & 0b1000_0000) == 0 {
            break;
        }
    }

    let string_bytes: &[u8] = &bytes[varint_byte_count..];

    String::from_utf8(string_bytes.to_vec()).unwrap_or_else(|_| String::from(""))
}


fn main() {
    println!("{:?}", write_varint(1024));

    println!("{}", read_varint(&[0x0F, 0x00, 0x00]));

    println!(
        "{:?}",
        write_string(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in"
        )
    );

    println!(
        "{}",
        read_string(vec![
            128, 2, 76, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109, 32, 100, 111, 108, 111,
            114, 32, 115, 105, 116, 32, 97, 109, 101, 116, 44, 32, 99, 111, 110, 115, 101, 99, 116,
            101, 116, 117, 114, 32, 97, 100, 105, 112, 105, 115, 99, 105, 110, 103, 32, 101, 108,
            105, 116, 44, 32, 115, 101, 100, 32, 100, 111, 32, 101, 105, 117, 115, 109, 111, 100,
            32, 116, 101, 109, 112, 111, 114, 32, 105, 110, 99, 105, 100, 105, 100, 117, 110, 116,
            32, 117, 116, 32, 108, 97, 98, 111, 114, 101, 32, 101, 116, 32, 100, 111, 108, 111,
            114, 101, 32, 109, 97, 103, 110, 97, 32, 97, 108, 105, 113, 117, 97, 46, 32, 85, 116,
            32, 101, 110, 105, 109, 32, 97, 100, 32, 109, 105, 110, 105, 109, 32, 118, 101, 110,
            105, 97, 109, 44, 32, 113, 117, 105, 115, 32, 110, 111, 115, 116, 114, 117, 100, 32,
            101, 120, 101, 114, 99, 105, 116, 97, 116, 105, 111, 110, 32, 117, 108, 108, 97, 109,
            99, 111, 32, 108, 97, 98, 111, 114, 105, 115, 32, 110, 105, 115, 105, 32, 117, 116, 32,
            97, 108, 105, 113, 117, 105, 112, 32, 101, 120, 32, 101, 97, 32, 99, 111, 109, 109,
            111, 100, 111, 32, 99, 111, 110, 115, 101, 113, 117, 97, 116, 46, 32, 68, 117, 105,
            115, 32, 97, 117, 116, 101, 32, 105, 114, 117, 114, 101, 32, 100, 111, 108, 111, 114,
            32, 105, 110
        ])
    );
}
