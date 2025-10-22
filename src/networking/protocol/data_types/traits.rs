use crate::networking::protocol::data_types::errors::*;
pub trait FromBytes<T> {
    fn from_bytes(data: &[u8]) -> Result<(T, usize), ParserError>;
}

pub trait ToBytes<T> {
    fn to_bytes(data: &T) -> Vec<u8>;
}

pub trait GetValue {
    type Output;
    fn value(&self) -> Self::Output;
}

pub trait Construct<T> {
    fn new(data: T) -> Self;
}