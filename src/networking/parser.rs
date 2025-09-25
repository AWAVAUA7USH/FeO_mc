enum ParserError {
    BufferTooLong,
    BufferTooShort,
}

pub fn read_varint(data: &[u8]) -> Result<(i32, usize), ParserError>{
    let mut result: i32 = 0;
    let mut shift: u32 = 0;
    let mut index: usize = 0;
    if data.len() > 5 {
        return Err(ParserError::BufferTooLong);
    }
    for &byte in data.iter(){
        result |= ((byte & 0b0111_1111) as i32) << shift;
        shift += 7;
        index += 1;
        if byte & 0x80 == 0 {
            return Ok((result, index));
        }
    }
    return Err(ParserError::BufferTooShort);
}

pub fn write_varint(number: i32) -> Result<Vec<u8>, ParserError > {
    let mut val = number as u32;
    let mut buffer = Vec::new();

    loop {
        let mut byte = (val & 0x7F) as u8; 
        val >>= 7;
        if val != 0 {
            byte |= 0x80;
        }
        buffer.push(byte);
        if val == 0 {
            break;
        }
    }

    return Ok(buffer)
}


pub fn read_varlong(data: &[u8]) -> Result<(i64, usize), ParserError>{
    let mut result: i64 = 0;
    let mut shift: u32 = 0;
    let mut index: usize = 0;
    if data.len() > 10 {
        return Err(ParserError::BufferTooLong);
    }
    for &byte in data.iter(){
        result |= ((byte & 0b0111_1111) as i64) << shift;
        shift += 7;
        index += 1;
        if byte & 0x80 == 0 {
            return Ok((result, index));
        }
    }
    return Err(ParserError::BufferTooShort);
}

pub fn write_varint(number: i64) -> Result<Vec<u8>, ParserError > {
    let mut val = number as u64;
    let mut buffer = Vec::new();

    loop {
        let mut byte = (val & 0x7F) as u8; 
        val >>= 7;
        if val != 0 {
            byte |= 0x80;
        }
        buffer.push(byte);
        if val == 0 {
            break;
        }
    }

    return Ok(buffer)
}