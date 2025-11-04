use std::ops::{Add, Sub, Mul, Div};
use std::string;
use super::errors::ParserError;
use super::varint::VarInt;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct McString{
    len: i32,
    data: String,
}
impl Construct<&str> for McString {
    fn new(data: &str) -> Self {
        let length = data.len() as i32;
        Self{
            len: length,
            data: String::from(data),
        }
    }
}
impl GetValue for McString {
    type Output = String;
    fn value(&self) -> String {
        self.0
    }
}
impl ToBytes<&str> for McString{
    fn to_bytes(data: &str) -> Vec<u8>{
    	let mut output = VarInt::i32_to_bytes(data.len() as i32);
        output.extend_from_slice(data.as_bytes());
        output
    }
}
impl FromBytes<String> for McString{
    fn from_bytes(data: &[u8]) -> Result<(String, usize), ParserError>{
        let(len, varint_len) = match <VarInt as FromBytes<VarInt>>::from_bytes(&data[0..5]) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };
        let str_data = data[varint_len..(varint_len+len)].to_vec();
        Ok((
            match String::from_utf8(str_data) {
                Ok(value) => value,
                Err(e) => return Err(e),
            },
            (len as uszie + varint_len),
        ))
    }
}
