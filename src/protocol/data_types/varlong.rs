use std::ops::{Add, Sub, Mul, Div};
use super::super::errors::ParserError;


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct VarLong(pub i64);

impl Add for VarLong {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0+other.0)
    }
}

impl Sub for VarLong {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0-other.0)
    }
}

impl Mul for VarLong {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0*other.0)
    }
}

impl Div for VarLong {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self(self.0/other.0)
    }
}

impl VarLong{
    pub fn new(value: i64) -> Self{
        VarLong(value)
    }
    pub fn value(&self) -> i64 {
        self.0
    }
    pub fn set(&mut self, value: i64) {
        self.0 = value
    }
    pub fn parse(data: &[u8]) -> Result<(Self, usize), ParserError>{
        if data.len() > 10 {
        return Err(ParserError::BufferTooLong);
        }
        let mut result: i64 = 0;
        let mut shift: u32 = 0;
        for (i,&byte) in data.iter().enumerate(){
            result |= ((byte & 0b0111_1111) as i64) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                return Ok((VarLong(result), i+1))
            }
        }
        return Err(ParserError::BufferTooShort);
    }
    pub fn encode(mut number: i64) -> Vec<u8>{
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

}