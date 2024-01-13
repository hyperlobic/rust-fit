use crate::error::Error;
use enum_as_inner::EnumAsInner;
use serde::Serialize;

pub struct BaseTypeInfo {
    pub base_type_num: u8,
    pub type_name: &'static str,
    pub invalid_value: Value,
    pub size: u8,
}

#[derive(Debug, PartialEq, EnumAsInner)]
pub enum Value {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::U8(value)
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum BaseType {
    Enum,
    Sint8,
    Uint8,
    Sint16,
    Uint16,
    Sint32,
    Uint32,
    String,
    Float32,
    Float64,
    Uint8z,
    Uint16z,
    Uint32z,
    Byte,
    Sint64,
    Uint64,
    Uint64z,
}

#[rustfmt::skip]
pub mod base_type_nums {
    pub const ENUM: u8    = 0x00;
    pub const SINT8: u8   = 0x01;
    pub const UINT8: u8   = 0x02;
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
    pub const fn info(&self) -> &BaseTypeInfo {
        use BaseType::*;
        use Value::*;
        match self {
            Enum => &BaseTypeInfo {
                base_type_num: base_type_nums::ENUM,
                type_name: "enum",
                invalid_value: U8(0xFF),
                size: 1,
            },
            Sint8 => &BaseTypeInfo {
                base_type_num: base_type_nums::SINT8,
                type_name: "sint8",
                invalid_value: I8(0x7F),
                size: 1,
            },
            Uint8 => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT8,
                type_name: "uint8",
                invalid_value: U8(0xFF),
                size: 1,
            },
            Uint8z => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT8Z,
                type_name: "uint8z",
                invalid_value: U8(0),
                size: 1,
            },
            Sint16 => &BaseTypeInfo {
                base_type_num: base_type_nums::SINT16,
                type_name: "sint16",
                invalid_value: I16(0x7FFF),
                size: 2,
            },
            Uint16 => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT16,
                type_name: "uint16",
                invalid_value: U16(0xFFFF),
                size: 2,
            },
            Uint16z => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT16Z,
                type_name: "uint16z",
                invalid_value: U16(0),
                size: 2,
            },
            Sint32 => &BaseTypeInfo {
                base_type_num: base_type_nums::SINT32,
                type_name: "sint32",
                invalid_value: I32(0x7FFFFFFF),
                size: 4,
            },
            Uint32 => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT32,
                type_name: "uint32",
                invalid_value: U32(0xFFFFFFFF),
                size: 4,
            },
            Uint32z => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT32Z,
                type_name: "uint32z",
                invalid_value: I32(0),
                size: 4,
            },
            Float32 => &BaseTypeInfo {
                base_type_num: base_type_nums::FLOAT32,
                type_name: "float32",
                invalid_value: F32(f32::NAN),
                size: 4,
            },
            Float64 => &BaseTypeInfo {
                base_type_num: base_type_nums::FLOAT64,
                type_name: "float64",
                invalid_value: F64(f64::NAN),
                size: 8,
            },
            Sint64 => &BaseTypeInfo {
                base_type_num: base_type_nums::SINT64,
                type_name: "sint64",
                invalid_value: I64(0x7FFFFFFFFFFFFFFF),
                size: 8,
            },
            Uint64 => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT64,
                type_name: "uint64",
                invalid_value: U64(0),
                size: 8,
            },
            Uint64z => &BaseTypeInfo {
                base_type_num: base_type_nums::UINT64Z,
                type_name: "uint64z",
                invalid_value: U64(0),
                size: 8,
            },
            Byte => &BaseTypeInfo {
                base_type_num: base_type_nums::BYTE,
                type_name: "byte",
                invalid_value: U8(0xFF),
                size: 1,
            },
            String => &BaseTypeInfo {
                base_type_num: base_type_nums::STRING,
                type_name: "string",
                invalid_value: U8(0),
                size: 1,
            },
        }
    }
}

impl TryFrom<u8> for BaseType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use base_type_nums::*;
        use BaseType::*;

        match value {
            ENUM => Ok(Enum),
            SINT8 => Ok(Sint8),
            UINT8 => Ok(Uint8),
            UINT8Z => Ok(Uint8z),
            SINT16 => Ok(Sint16),
            UINT16 => Ok(Uint16),
            UINT16Z => Ok(Uint16z),
            SINT32 => Ok(Sint32),
            UINT32 => Ok(Uint32),
            UINT32Z => Ok(Uint32z),
            SINT64 => Ok(Sint64),
            UINT64 => Ok(Uint64),
            UINT64Z => Ok(Uint64z),
            FLOAT32 => Ok(Float32),
            FLOAT64 => Ok(Float64),
            BYTE => Ok(Byte),
            STRING => Ok(String),
            unknown => Err(Error::ParseValue(format!("unknown base type {unknown}").to_string())),
        }
    }
}
