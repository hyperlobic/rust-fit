use serde::Serialize;

use crate::error::Error;

#[derive(PartialEq, Debug, Serialize, Copy, Clone)]
pub enum ByteOrder {
    LitteEndian,
    BigEndian,
}

impl TryFrom<u8> for ByteOrder {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ByteOrder::LitteEndian),
            1 => Ok(ByteOrder::BigEndian),
            _ => Err(Error::ParseValue("invalid byte order value".to_string())),
        }
    }
}
