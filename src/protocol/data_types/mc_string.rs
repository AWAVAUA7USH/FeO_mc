use std::ops::{Add, Sub, Mul, Div};
use std::string;
use super::super::errors::ParserError;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct McString{
    len: i32,
    data: String
}