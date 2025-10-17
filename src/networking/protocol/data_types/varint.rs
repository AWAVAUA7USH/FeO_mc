use std::ops::{Add, Sub, Mul, Div};
use super::super::errors::ParserError;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct VarInt(i32);
impl Add for VarInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0+other.0)
    }
}
impl Sub for VarInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0-other.0)
    }
}
impl Mul for VarInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0*other.0)
    }
}
impl Div for VarInt {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self(self.0/other.0)
    }
}
impl VarInt{
    pub fn from_i32(value: i32) -> Self{
        VarInt(value)
    }
    pub fn to_i32(&self) -> i32 {
        self.0
    }
    pub fn from_bytes(data: &[u8]) -> Result<(Self, usize), ParserError>{
        let (value, size) = match Self::bytes_to_i32(data){
            Ok(value) => value;
            Err(e) => return Err(e)
        };
        let value_varint = Self::from_i32(value);
        Ok((value_varint, size))
    }
    pub fn bytes_to_i32(data: &[u8]) -> Result<(i32, usize), ParserError>{
        if data.len() > 5 {
        return Err(ParserError::BufferTooLong);
        }
        let mut result: i32 = 0;
        let mut shift: u32 = 0;
        for (i,&byte) in data.iter().enumerate(){
            result |= ((byte & 0b0111_1111) as i32) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                return Ok((result, i+1))
            }
        }
        return Err(ParserError::BufferTooShort);
    }
    pub fn i32_to_bytes(mut number: i32) -> Vec<u8>{
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
        Self::i32_to_bytes(self.0)
    }

}