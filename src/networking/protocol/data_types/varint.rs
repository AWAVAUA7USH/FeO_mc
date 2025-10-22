use std::ops::{Add, Sub, Mul, Div};
use crate::networking::protocol::data_types::errors::*;
use crate::networking::protocol::data_types::traits::*;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct VarInt(i32);
impl GetValue for VarInt {
    type Output = i32;
    fn value(&self) -> i32 {
        self.0
    }
}
impl FromBytes<i32> for VarInt {
    fn from_bytes(data: &[u8]) -> Result<(i32, usize), ParserError>{
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
}
impl FromBytes<Self> for VarInt {
    fn from_bytes(data: &[u8]) -> Result<(Self, usize), ParserError> {
        if data.len() > 5 {
        return Err(ParserError::BufferTooLong);
        }
        let mut result: i32 = 0;
        let mut shift: u32 = 0;
        for (i,&byte) in data.iter().enumerate(){
            result |= ((byte & 0b0111_1111) as i32) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                return Ok((VarInt(result), i+1))
            }
        }
        return Err(ParserError::BufferTooShort);
    }
}
impl ToBytes<i32> for VarInt {
    fn to_bytes(data: &i32) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut num = *data;
        loop {
            let mut byte = (num & 0x7f) as u8;
            num >>= 7;
            if num != 0 {
                byte |= 0x80;
            }
            buffer.push(byte);
            if num == 0 {
                break;
            }
        }
        buffer
    }
}
impl ToBytes<Self> for VarInt {
    fn to_bytes(data: &Self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut num = data.0;
        loop {
            let mut byte = (num & 0x7f) as u8;
            num >>= 7;
            if num != 0 {
                byte |= 0x80;
            }
            buffer.push(byte);
            if num == 0 {
                break;
            }
        }
        buffer
    }
}