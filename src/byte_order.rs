use anyhow::anyhow;
use serde::Serialize;

#[derive(PartialEq, Debug, Serialize, Copy, Clone)]
pub enum ByteOrder {
    LitteEndian,
    BigEndian,
}

impl TryFrom<u8> for ByteOrder {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ByteOrder::LitteEndian),
            1 => Ok(ByteOrder::BigEndian),
            _ => Err(anyhow!("invalid byte order value")),
        }
    }
}
