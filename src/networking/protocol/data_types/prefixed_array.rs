use std::ops::{Add, Sub, Mul, Div};
use super::super::errors::ParserError;
use super::super::varint::VarInt;

pub struct PrefixedArray{
    length: VarInt,
    data: Vec<u8>,
}
impl PrefixedArray{
    pub fn from_bytes(data: &[u8]) -> Result<(Self, usize), ParserError>{
        let (length, varint_length) = VarInt::from_bytes(&data[0..5]);
        let data: Vec<u8> = data[varint_length..].to_vec;
    }
}