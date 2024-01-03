pub mod reader;
use crate::base_type::BaseType;
use crate::byte_order::ByteOrder;
use crate::profile::types::MesgNum;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct FileHeader {
    pub header_size: u8,
    pub protocol_version: u8,
    pub profile_version: u16,
    pub data_size: u32,
    pub data_type: String,
    pub crc: u16,
}

impl FileHeader {
    /// The size of the header + data portions of the file, not including the 2 byte CRC.
    pub fn file_size(&self) -> u32 {
        self.header_size as u32 + self.data_size
    }
}

#[derive(Debug, Clone)]
pub struct RecordHeader(pub u8);

#[rustfmt::skip]
mod bits {
    pub mod record_header {
        pub const HEADER_TYPE_BIT: u8         = 0b10000000;
        pub const MESSAGE_TYPE_BIT: u8        = 0b01000000;
        pub const MESSAGE_DEV_FLAG_BIT: u8    = 0b00100000;
        pub const LOCAL_MESSAGE_TYPE_BITS: u8 = 0b00001111;
    }
}

impl RecordHeader {
    pub fn mesg_type(&self) -> MessageType {
        if self.0 & bits::record_header::HEADER_TYPE_BIT != 0 {
            MessageType::CompressedHeader
        } else if self.0 & bits::record_header::MESSAGE_TYPE_BIT != 0 {
            MessageType::Definition
        } else {
            MessageType::Data
        }
    }
    
    pub fn has_dev(&self) -> bool {
        self.0 & bits::record_header::MESSAGE_DEV_FLAG_BIT != 0 && !self.is_compressed()
    }

    pub fn is_data(&self) -> bool {
        self.mesg_type() == MessageType::Data
    }

    pub fn is_definition(&self) -> bool {
        self.mesg_type() == MessageType::Definition
    }

    pub fn is_compressed(&self) -> bool {
        self.mesg_type() == MessageType::CompressedHeader
    }

    pub fn local_msg_type(&self) -> u8 {
        if self.is_compressed() {
            (self.0 >> 5) & 0b00000011
        } else {
            self.0 & bits::record_header::LOCAL_MESSAGE_TYPE_BITS
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Definition,
    Data,
    CompressedHeader,
}

#[derive(Debug, Clone)]
pub struct FieldDefinition {
    pub field_def_num: u8,
    pub size: u8,
    pub base_type: BaseType,
}

#[derive(Debug, Clone)]
pub struct DeveloperFieldDefinition {
    pub field_num: u8,
    pub size: u8,
    pub dev_data_index: u8,
}

#[derive(Debug, Clone)]
pub struct DefinitionMessage {
    pub architecture: ByteOrder,
    pub global_mesg_number: u16,
    pub local_mesg_number: u8,
    pub num_fields: u8,
    pub fields: Vec<FieldDefinition>,
    pub developer_fields: Vec<DeveloperFieldDefinition>,
}

#[derive(Debug)]
pub struct DataMessage {
    pub(crate) mesg_num: MesgNum,
    pub(crate) fields: HashMap<u8, DataField>,
    pub(crate) dev_fields: Vec<DeveloperDataField>,
}

impl DataMessage {
    pub fn mesg_num(&self) -> &MesgNum {
        &self.mesg_num
    }

    pub fn has_data_field(&self, field_def_num: u8) -> bool {
        self.fields.contains_key(&field_def_num)
    }

    pub fn fields_len(&self) -> u8 {
        self.fields.len() as u8
    }

    pub fn dev_fields_len(&self) -> u8 {
        self.dev_fields.len() as u8
    }

    ///  Returns the data for the given field definition number
    pub fn data(&self, field_def_num: u8) -> Option<&Data> {
        self.fields.get(&field_def_num).map(|x| &x.data)
    }

    pub fn fields(&self) -> impl Iterator<Item = &DataField> {
        self.fields.values()
    }

    pub fn dev_fields(&self) -> impl Iterator<Item = &DeveloperDataField> {
        self.dev_fields.iter()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Data {
    Enum(u8),
    EnumArray(Vec<u8>),
    Sint8(i8),
    Sint8Array(Vec<i8>),
    Uint8(u8),
    Uint8Array(Vec<u8>),
    Sint16(i16),
    Sint16Array(Vec<i16>),
    Uint16(u16),
    Uint16Array(Vec<u16>),
    Sint32(i32),
    Sint32Array(Vec<i32>),
    Uint32(u32),
    Uint32Array(Vec<u32>),
    Float32(f32),
    Float32Array(Vec<f32>),
    Float64(f64),
    Float64Array(Vec<f64>),
    Sint64(i64),
    Sint64Array(Vec<i64>),
    Uint64(u64),
    Uint64Array(Vec<u64>),
    Byte(u8),
    ByteArray(Vec<u8>),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataField {
    pub field_def_num: u8,
    pub data: Data,
}

#[derive(Debug, Clone)]
pub struct DeveloperDataField {
    pub num: u8,
    pub index: u8,
    pub data: Data,
}
