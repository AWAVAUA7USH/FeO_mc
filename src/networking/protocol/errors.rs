#[derive(Debug)]
pub enum ParserError {
    BufferTooLong,
    BufferTooShort,
    StringParsingFailed,
}