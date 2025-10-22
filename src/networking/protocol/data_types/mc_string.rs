use std::ops::{Add, Sub, Mul, Div};
use std::string;
use super::errors::ParserError;
use super::varint::VarInt;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct McString{
    len: i32,
    data: String,
}
impl McString{
    pub fn from_bytes(data: &[u8]) -> Result<Self, ParserError> {
        let (len, varint_len) = match VarInt::from_bytes(&data[0..5]) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };
        let str_data = data[varint_len..].to_vec();
        Ok(
            Self{
                len: len.0,
                data: match String::from_utf8(str_data) {
                    Ok(value) => value,
                    Err(e) => return Err(ParserError::StringParsingFailed),
                }
            }
        )
    }
    pub fn string_to_bytes(data: &str) -> Vec<u8> {
        let mut output = VarInt::i32_to_bytes(data.len() as i32);
        output.extend_from_slice(data.as_bytes());
        output
    }
}