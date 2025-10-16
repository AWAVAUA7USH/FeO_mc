pub mod varint;
pub mod varlong;
pub mod mc_string;
// Re-export everything for external use
pub use varint::VarInt;
pub use varlong::VarLong;
pub use mc_string::McString;