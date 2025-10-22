use std::ops::{Add, Sub, Mul, Div};
use super::errors::ParserError;


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct VarLong(pub i64);
impl VarLong{
    pub fn from_i64(value: i64) -> Self{
        VarLong(value)
    }
    pub fn to_i64(&self) -> i64 {
        self.0
    }
    pub fn from_bytes(data: &[u8]) -> Result<(Self, usize), ParserError>{
        let (number, size) = match Self::bytes_to_i64(data) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };
        Ok((Self::from_i64(number), size))
    }
    pub fn bytes_to_i64(data: &[u8]) -> Result<(i64, usize), ParserError>{
        if data.len() > 10 {
        return Err(ParserError::BufferTooLong);
        }
        let mut result: i64 = 0;
        let mut shift: u32 = 0;
        for (i,&byte) in data.iter().enumerate(){
            result |= ((byte & 0b0111_1111) as i64) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                return Ok((result, i+1))
            }
        }
        return Err(ParserError::BufferTooShort);
    }
    pub fn i64_to_bytes(mut number: i64) -> Vec<u8>{
        let mut buffer = Vec::new();
        loop {
            let mut byte = (number & 0x7f) as u8;
            number >>= 7;
            if number != 0 {
                byte |= 0x80;
            }
            buffer.push(byte);
            if number == 0 {
                break;
            }
        }
        buffer
    }
    pub fn to_bytes(&self) -> Vec<u8>{
        Self::i64_to_bytes(self.0)
    }

}