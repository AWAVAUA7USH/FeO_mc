use std::ops::{Add, Sub, Mul, Div};
use super::super::errors::ParserError;
use super::super::data_types::{VarInt,};

pub struct PrefixedArray<T>{
    length: VarInt,
    data: Vec<T>,
}
impl PrefixedArray{
    pub fn from_bytes(data: &[u8]) -> Result<(Self, usize), ParserError>{
        let (length, varint_length) = match VarInt::from_bytes(&data[0..5]){
            Ok(value) => value,
            Err(e) => Err(e),
        };
        let content: Vec<u8> = data[varint_length..varint_length+(length*len(T))].to_vec;
    }
}