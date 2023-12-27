use anyhow::anyhow;
use serde::Serialize;

pub struct BaseTypeInfo {
    pub base_type_num: u8,
    pub type_name: &'static str,
    pub invalid_value: u64,
    pub size: u8,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum BaseType {
    Enum,
    SInt8,
    UInt8,
    SInt16,
    UInt16,
    SInt32,
    UInt32,
    String,
    Float32,
    Float64,
    UInt8z,
    UInt16z,
    UInt32z,
    Byte,
    SInt64,
    UInt64,
    UInt64z,
}

#[rustfmt::skip]
pub mod base_type_nums {
    pub const ENUM: u8    = 0;
    pub const SINT8: u8   = 1;
    pub const UINT8: u8   = 2;
    pub const SINT16: u8  = 0x83;
    pub const UINT16: u8  = 0x84;
    pub const SINT32: u8  = 0x85;
    pub const UINT32: u8  = 0x86;
    pub const STRING: u8  = 0x07;
    pub const FLOAT32: u8 = 0x88;
    pub const FLOAT64: u8 = 0x89;
    pub const UINT8Z: u8  = 0x0A;
    pub const UINT16Z: u8 = 0x8B;
    pub const UINT32Z: u8 = 0x8C;
    pub const BYTE: u8    = 0x0D;
    pub const SINT64: u8  = 0x8E;
    pub const UINT64: u8  = 0x8F;
    pub const UINT64Z: u8 = 0x90;
}

impl BaseType {
    pub fn info(&self) -> &BaseTypeInfo {
        use BaseType::*;
        match self {
            Enum => &BaseTypeInfo {
                base_type_num: base_type_nums::ENUM,
                type_name: "enum",
                invalid_value: 0xFF,
                size: 1,
            },
            SInt8 => &BaseTypeInfo {
                base_type_num: base_type_nums::SINT8,
                type_name: "sint8",
                invalid_value: 0x7F,
                size: 1,
            },
            UInt8 => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT8,
                type_name: "uint8",
                invalid_value: 0xFF,
                size: 1,
            },
            UInt8z => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT8Z,
                type_name: "uint8z",
                invalid_value: 0,
                size: 1,
            },
            SInt16 => &BaseTypeInfo {
                base_type_num: base_type_nums::SINT16,
                type_name: "sint16",
                invalid_value: 0x7FFF,
                size: 2,
            },
            UInt16 => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT16,
                type_name: "uint16",
                invalid_value: 0xFFFF,
                size: 2,
            },
            UInt16z => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT16Z,
                type_name: "uint16z",
                invalid_value: 0,
                size: 2,
            },
            SInt32 => &BaseTypeInfo {
                base_type_num: base_type_nums::SINT32,
                type_name: "sint32",
                invalid_value: 0x7FFFFFFF,
                size: 4,
            },
            UInt32 => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT32,
                type_name: "uint32",
                invalid_value: 0xFFFFFFFF,
                size: 4,
            },
            UInt32z => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT32Z,
                type_name: "uint32z",
                invalid_value: 0,
                size: 4,
            },
            Float32 => &BaseTypeInfo {
                base_type_num: base_type_nums::FLOAT32,
                type_name: "float32",
                invalid_value: 0xFFFFFFFF,
                size: 4,
            },
            Float64 => &BaseTypeInfo {
                base_type_num: base_type_nums::FLOAT64,
                type_name: "float64",
                invalid_value: 0xFFFFFFFFFFFFFFFF,
                size: 8,
            },
            SInt64 => &BaseTypeInfo {
                base_type_num: base_type_nums::SINT64,
                type_name: "sint64",
                invalid_value: 0x7FFFFFFFFFFFFFFF,
                size: 8,
            },
            UInt64 => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT64,
                type_name: "uint64",
                invalid_value: 0,
                size: 8,
            },
            UInt64z => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT64Z,
                type_name: "uint64z",
                invalid_value: 0,
                size: 8,
            },
            Byte => &BaseTypeInfo {
                base_type_num: base_type_nums::BYTE,
                type_name: "byte",
                invalid_value: 0xFF,
                size: 1,
            },
            String => &BaseTypeInfo {
                base_type_num: base_type_nums::STRING,
                type_name: "string",
                invalid_value: 0,
                size: 1,
            },
        }
    }
}

impl TryFrom<u8> for BaseType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use base_type_nums::*;
        use BaseType::*;

        match value {
            ENUM => Ok(Enum),
            SINT8 => Ok(SInt8),
            UINT8 => Ok(UInt8),
            UINT8Z => Ok(UInt8z),
            SINT16 => Ok(SInt16),
            UINT16 => Ok(UInt16),
            UINT16Z => Ok(UInt16z),
            SINT32 => Ok(SInt32),
            UINT32 => Ok(UInt32),
            UINT32Z => Ok(UInt32z),
            SINT64 => Ok(SInt64),
            UINT64 => Ok(UInt64),
            UINT64Z => Ok(UInt64z),
            FLOAT32 => Ok(Float32),
            FLOAT64 => Ok(Float64),
            BYTE => Ok(Byte),
            STRING => Ok(String),
            _ => Err(anyhow!("unknown type")),
        }
    }
}
