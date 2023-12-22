use anyhow::anyhow;
use serde::Serialize;
use std::{io::{Read, Seek}, collections::HashMap, mem};
use crate::fit_header::{FitHeader, read_fit_header};
use crate::read_ext::ReadExt;
use crate::byte_order::ByteOrder;

const HEADER_TYPE_BIT: u8         = 0b10000000;
const MESSAGE_TYPE_BIT: u8        = 0b01000000;
const MESSAGE_DEV_FLAG_BIT: u8    = 0b00100000;
const LOCAL_MESSAGE_TYPE_BITS: u8 = 0b00001111;

pub struct FitBaseType {
    pub base_type: u8,
    pub type_name: &'static str,
    pub invalid_value: u64,
    pub size: u8
}

enum BaseType {
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

mod base_type_nums {
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
    fn meta(&self) -> FitBaseType {
        use BaseType::*;
        match self {
            Enum => FitBaseType {
                base_type: base_type_nums::ENUM,
                type_name: "enum",
                invalid_value: 0xFF,
                size: 1
            },
            SInt8 => FitBaseType { 
                base_type: base_type_nums::SINT8, 
                type_name: "sint8",
                invalid_value: 0x7F,
                size: 1
            },
            UInt8 => FitBaseType {
                base_type: base_type_nums::UINT8,
                type_name: "uint8",
                invalid_value: 0xFF,
                size: 1
            },
            UInt8z => FitBaseType {
                base_type: base_type_nums::UINT8Z,
                type_name: "uint8z",
                invalid_value: 0,
                size: 1
            },
            SInt16 => FitBaseType {
                base_type: base_type_nums::SINT16,
                type_name: "sint16",
                invalid_value: 0x7FFF,
                size: 2
            },
            UInt16 => FitBaseType {
                base_type: base_type_nums::UINT16,
                type_name: "uint16",
                invalid_value: 0xFFFF,
                size: 2
            },
            UInt16z => FitBaseType {
                base_type: base_type_nums::UINT16Z,
                type_name: "uint16z",
                invalid_value: 0,
                size: 2
            },
            SInt32 => FitBaseType { 
                base_type: base_type_nums::SINT32,
                type_name: "sint32",
                invalid_value: 0x7FFFFFFF,
                size: 4
            },
            UInt32 => FitBaseType {
                base_type: base_type_nums::UINT32,
                type_name: "uint32",
                invalid_value: 0xFFFFFFFF,
                size: 4
            },
            UInt32z => FitBaseType {
                base_type: base_type_nums::UINT32Z,
                type_name: "uint32z",
                invalid_value: 0,
                size: 4
            },
            Float32 => FitBaseType {
                base_type: base_type_nums::FLOAT32,
                type_name: "float32",
                invalid_value: 0xFFFFFFFF,
                size: 4
            },
            Float64 => FitBaseType {
                base_type: base_type_nums::FLOAT64,
                type_name: "float64",
                invalid_value: 0xFFFFFFFFFFFFFFFF,
                size: 8
            },
            SInt64 => FitBaseType {
                base_type: base_type_nums::SINT64,
                type_name: "sint64",
                invalid_value: 0x7FFFFFFFFFFFFFFF,
                size: 8
            },
            UInt64 => FitBaseType {
                base_type: base_type_nums::UINT64,
                type_name: "uint64",
                invalid_value: 0,
                size: 8
            },
            UInt64z => FitBaseType {
                base_type: base_type_nums::UINT64Z,
                type_name: "uint64z",
                invalid_value: 0,
                size: 8
            },
            Byte => FitBaseType {
                base_type: base_type_nums::BYTE,
                type_name: "byte",
                invalid_value: 0xFF,
                size: 1
            },
            String => FitBaseType {
                base_type: base_type_nums::STRING,
                type_name: "string",
                invalid_value: 0,
                size: 1
            }
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
            SINT16 => Ok(SInt16),
            UINT16 => Ok(UInt16),
            UINT8Z => Ok(UInt8z),
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
            _ => Err(anyhow!("unknown type"))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Definition,
    Data,
    CompressedHeader
}

#[derive(Serialize, Debug)]
pub struct RecordHeader(u8);

impl RecordHeader {
    pub fn is_normal(&self) -> bool {
        self.0 & HEADER_TYPE_BIT == 0
    }

    pub fn msg_type(&self) -> MessageType {
        if self.0 & HEADER_TYPE_BIT != 0 {
            MessageType::CompressedHeader
        } else if self.0 & MESSAGE_TYPE_BIT != 0 {
            MessageType::Definition
        } else {
            MessageType::Data
        }
    }

    pub fn has_dev(&self) -> bool {
        self.0 & MESSAGE_DEV_FLAG_BIT != 0 && !self.is_compressed()
    }

    pub fn is_data(&self) -> bool {
        self.msg_type() == MessageType::Data
    }

    pub fn is_definition(&self) -> bool {
        self.msg_type() == MessageType::Definition
    }

    pub fn is_compressed(&self) -> bool {
        self.msg_type() == MessageType::CompressedHeader
    }

    pub fn local_msg_type(&self) -> u8 {
        if self.is_compressed() {
            (self.0 >> 5) & 0b00000011
        } else {
            self.0 & LOCAL_MESSAGE_TYPE_BITS
        }
    }
}

#[derive(Serialize, Debug)]
pub struct FieldDefinition {
    pub num: u8,
    pub size: u8,
    pub base_type: u8,
}

#[derive(Serialize, Debug)]
pub struct DeveloperDataField {
    pub num: u8,
    pub size: u8,
    pub index: u8,
}

#[derive(Serialize, Debug)]
pub struct DefinitionMessageContent  {
    pub architecture: ByteOrder,
    pub global_msg_number: u16,
    pub num_fields: u8,
    pub fields: Vec<FieldDefinition>,
    pub developer_fields: Vec<DeveloperDataField>,    
}

#[derive(Serialize, Debug)]
pub struct DefinitionMessage {
    pub header: RecordHeader,
    pub content: DefinitionMessageContent
}

impl DefinitionMessage {
    pub fn data_size(&self) -> u32 {
        self.content.fields.iter().map(|x| x.size as u32).sum::<u32>() +
        self.content.developer_fields.iter().map(|x| x.size as u32).sum::<u32>()
    }
}

#[derive(Serialize, Debug)]
pub struct DataMessage {
    pub header: RecordHeader,
    pub fields: HashMap<u8, DataField>,
    #[serde(skip_serializing)]
    pub dev_fields: HashMap<(u8, u8), DataField>,
}

#[derive(PartialEq, Debug, Serialize)]
pub enum DataField {
    Enum(u8),
    EnumArray(Vec<u8>),
    SInt8(i8),
    SInt8Array(Vec<i8>),
    UInt8(u8),
    UInt8Array(Vec<u8>),
    SInt16(i16),
    SInt16Array(Vec<i16>),
    UInt16(u16),
    UInt16Array(Vec<u16>),
    SInt32(i32),
    SInt32Array(Vec<i32>),
    UInt32(u32),
    UInt32Array(Vec<u32>),
    Float32(f32),
    Float32Array(Vec<f32>),
    Float64(f64),
    Float64Array(Vec<f64>),
    SInt64(i64),
    SInt64Array(Vec<i64>),
    UInt64(u64),
    UInt64Array(Vec<u64>),
    Byte(u8),
    ByteArray(Vec<u8>),
    String(String),
}

#[derive(Serialize, Debug)]
pub struct Fit {
    pub header: FitHeader,
    pub definitions: Vec<DefinitionMessage>,
    pub data: Vec<DataMessage>,
}

fn read_message_header<T: Read + Seek>(reader: &mut T) -> Result<RecordHeader, anyhow::Error> {
    let header_byte = reader.read_u8()?;
    Ok(RecordHeader(header_byte))
}

fn read_field_definition<T: Read + Seek>(reader: &mut T) -> Result<FieldDefinition, anyhow::Error> {
    let num = reader.read_u8()?;
    let size = reader.read_u8()?;
    let base_type = reader.read_u8()?;

    Ok(FieldDefinition { num, size, base_type })
}

pub fn read_definition_message<T: Read + Seek>(reader: &mut T) -> Result<DefinitionMessage, anyhow::Error> {
    let header_byte = reader.read_u8()?;
    eprintln!("read definition mesg {:x} {:x}", reader.stream_position()? - 1, header_byte);
    let header = RecordHeader(header_byte);

    if MessageType::Definition != header.msg_type() {
        Err(anyhow!("not a  definition message"))?
    }

    let _reserved = reader.read_u8()?;
    let architecture = reader.read_u8().map(|b| ByteOrder::try_from(b))??;
    
    let global_msg_number = reader.read_u16(architecture)?;

    let num_fields = reader.read_u8()?;
    let mut fields: Vec<FieldDefinition> = vec![];

    for _n in 0..num_fields {
        let field_def = read_field_definition(reader)?;
        fields.push(field_def);
    }

    let mut developer_fields: Vec<DeveloperDataField> = vec![];
    if header.has_dev() {
        let num_developer_fields = reader.read_u8()?;

        for _n in 0..num_developer_fields {
            let num = reader.read_u8()?;    
            let size = reader.read_u8()?;
            let index = reader.read_u8()?;

            developer_fields.push(DeveloperDataField { num, size, index });
        }
    }

    let mesg = DefinitionMessage {
        header,
        content: DefinitionMessageContent {
            architecture,
            global_msg_number,
            num_fields,
            fields,
            developer_fields
        }
    };

    eprintln!("{:?}", mesg);

    Ok(mesg)
}


fn read_array<T>(size: usize, mut read: impl FnMut() -> Result<T, std::io::Error>) -> Result<Vec<T>, anyhow::Error> {
    let type_size = mem::size_of::<T>();
    let count = size / type_size;
    let buf: Result<Vec<T>, _> = (0..count)
        .map(|_| read())
        .collect();
    Ok(buf?)
}

fn read_data_field<T: Read + Seek>(reader: &mut T, field_def: &FieldDefinition, byte_order: ByteOrder) -> Result<DataField, anyhow::Error> {
    use BaseType::*;
    
    let data_size = field_def.size as usize;
    let base_type: BaseType = field_def.base_type.try_into()?;
    let is_array = field_def.size > base_type.meta().size;

    let data = match (base_type, is_array) {
        (Enum, false) => 
            DataField::Enum(reader.read_u8()?),
        (Enum, true) =>
            DataField::EnumArray(read_array(data_size, || reader.read_u8())?),
        (UInt8, false) | (UInt8z, false) =>
            DataField::UInt8(reader.read_u8()?),
        (UInt8, true) | (UInt8z, true) => 
            DataField::UInt8Array(read_array(data_size, || reader.read_u8())?),
        (SInt8, false) =>
            DataField::SInt8(reader.read_i8()?),
        (SInt8, true) =>
            DataField::SInt8Array(read_array(data_size, || reader.read_i8())?),
        (UInt16, false) | (UInt16z, false) =>
            DataField::UInt16(reader.read_u16(byte_order)?),
        (UInt16, true) | (UInt16z, true) =>
            DataField::UInt16Array(read_array(data_size, || reader.read_u16(byte_order))?),
        (SInt16, false) =>
            DataField::SInt16(reader.read_i16(byte_order)?),
        (SInt16, true) =>
            DataField::SInt16Array(read_array(data_size, || reader.read_i16(byte_order))?),
        (SInt32, false) => 
            DataField::SInt32(reader.read_i32(byte_order)?),
        (SInt32, true) => 
            DataField::SInt32Array(read_array(data_size, || reader.read_i32(byte_order))?),            
        (UInt32, false) | (UInt32z, false) => 
            DataField::UInt32(reader.read_u32(byte_order)?),
        (UInt32, true) | (UInt32z, true) => 
            DataField::UInt32Array(read_array(data_size, || reader.read_u32(byte_order))?),
        (SInt64, false) => 
            DataField::SInt64(reader.read_i64(byte_order)?),
        (SInt64, true) => 
            DataField::SInt64Array(read_array(data_size, || reader.read_i64(byte_order))?),            
        (UInt64, false) | (UInt64z, false) => 
            DataField::UInt64(reader.read_u64(byte_order)?),
        (UInt64, true) | (UInt64z, true) => 
            DataField::UInt64Array(read_array(data_size, || reader.read_u64(byte_order))?),
        (Float32, false) => 
            DataField::Float32(reader.read_f32(byte_order)?),
        (Float32, true) => 
            DataField::Float32Array(read_array(data_size, || reader.read_f32(byte_order))?),
        (Float64, false) => 
            DataField::Float64(reader.read_f64(byte_order)?),
        (Float64, true) => 
            DataField::Float64Array(read_array(data_size, || reader.read_f64(byte_order))?),
        (Byte, false) => 
            DataField::Byte(reader.read_u8()?),
        (Byte, true) => 
            DataField::ByteArray(read_array(data_size, || reader.read_u8())?),
        (String, _) => DataField::String(reader.read_string_null_term(data_size as u32)?),
    };

    Ok(data)
}

fn read_developer_data_field<T: Read + Seek>(reader: &mut T, dev_field_def: &DeveloperDataField) -> Result<DataField, anyhow::Error> {
    let mut buf = vec![0u8; dev_field_def.size as usize];
    reader.read_exact(buf.as_mut_slice())?;
    Ok(DataField::ByteArray(buf))
}

pub fn read_data_mesg<T: Read + Seek>(reader: &mut T, definition: &DefinitionMessage) -> Result<DataMessage, anyhow::Error> {
    let header = read_message_header(reader)?;
    eprintln!("read data mesg {:x} {:x}", reader.stream_position()?  - 1, header.0);

    if !header.is_data() && !header.is_compressed() {
        Err(anyhow!("not a data message"))?
    }

    let fields: Result<HashMap<u8, DataField>, anyhow::Error> = definition.content.fields.iter()
        .map(|field_def| {
            read_data_field(reader, field_def, definition.content.architecture)
                .map(|field| (field_def.num, field))
        })
        .collect();

    let dev_fields: Result<HashMap<(u8, u8), DataField>, anyhow::Error> = definition.content.developer_fields.iter()
        .map(|dev_field_def| {
            read_developer_data_field(reader, dev_field_def)
                .map(|field| ((dev_field_def.num, dev_field_def.index), field))
        })
        .collect();

    Ok(DataMessage {
        header,
        fields: fields?,
        dev_fields: dev_fields?
    })
}

pub fn read_fit<T: Read + Seek>(reader: &mut T) -> Result<Fit, anyhow::Error> {
    let header = read_fit_header(reader)?;

    let mut definitions: Vec<DefinitionMessage> = vec![];
    let mut data: Vec<DataMessage> = vec![];
    let mut curr_defintions: HashMap<u8, DefinitionMessage> = HashMap::new();

    while let Ok(b) = reader.peek_byte() {
        let record_header = RecordHeader(b);

        if record_header.is_definition() {
            let def_mesg = read_definition_message(reader)?;
            let replaced = curr_defintions.insert(def_mesg.header.local_msg_type(), def_mesg);
            if let Some(replaced_def) = replaced {
                definitions.push(replaced_def);
            }
        } else {
            if let Some(ref definition) = curr_defintions.get(&record_header.local_msg_type()) {
                let data_mesg = read_data_mesg(reader, &definition)?;
                data.push(data_mesg);
            } else {
                Err(anyhow!("data mesg without preceding definition pos {} {:x}", record_header.local_msg_type(), reader.stream_position().unwrap_or(0)))?
            }
        }

        if reader.stream_position()? == header.mesg_size() {
            // read crc
            reader.read_i16(ByteOrder::LitteEndian)?;
        }
    }

    definitions.append(&mut curr_defintions.into_values().collect());

    Ok(Fit { 
        header,
        definitions,
        data,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_field_type(field_def: &FieldDefinition, num: u8, base_type: &FitBaseType) {
        assert_eq!(field_def.num, num);
        assert_eq!(field_def.size, base_type.size);
        assert_eq!(field_def.base_type, base_type.base_type);
    }

    #[test]
    fn test_read_file_definition_message() {
        let data: [u8; 0x12] = [
            0x40, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x01, 0x00, 0x01, 0x02, 0x84, 0x02, 0x02, 0x84, 0x04, 
            0x04, 0x86
        ];

        use std::io::Cursor;
        let mut buff = Cursor::new(&data);

        let mesg = read_definition_message(&mut buff).unwrap();

        assert_eq!(mesg.header.msg_type(), MessageType::Definition);
        assert_eq!(mesg.header.local_msg_type(), 0);
        assert_eq!(mesg.header.is_normal(), true);
        assert_eq!(mesg.content.architecture, ByteOrder::LitteEndian);
        assert_eq!(mesg.content.fields.len(), 4);
        
        assert_field_type(&mesg.content.fields[0], 0, &BaseType::Enum.meta());
        assert_field_type(&mesg.content.fields[1], 1, &BaseType::UInt16.meta());
        assert_field_type(&mesg.content.fields[2], 2, &BaseType::UInt16.meta());
        assert_field_type(&mesg.content.fields[3], 4, &BaseType::UInt32.meta());

        assert_eq!(mesg.content.developer_fields.len(), 0);
    }

    #[test]
    fn test_read_file_definition_message_with_dev_fields() {
        let data: [u8; 0x16] = [
            0b01100000, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x01, 0x00, 0x01, 0x02, 0x84, 0x02, 0x02, 0x84, 0x04, 
            0x04, 0x86, 0x01, 0x00, 0x02, 0x01
        ];

        use std::io::Cursor;
        let mut buff = Cursor::new(&data);

        let mesg = read_definition_message(&mut buff).unwrap();

        assert_eq!(mesg.content.developer_fields.len(), 1);
        assert_eq!(mesg.content.developer_fields[0].num, 0);
        assert_eq!(mesg.content.developer_fields[0].size, 2);
        assert_eq!(mesg.content.developer_fields[0].index, 1);
    }

    #[test]
    fn test_read_data_message() {
        let data: [u8; 10] = [
            0x00, 0x04, 0x0F, 0x00, 0x01, 0x00, 0x12, 0x07, 0xE6, 0x29
        ];

        let definition_mesg = DefinitionMessage {
            header: RecordHeader(0x40),
            content: DefinitionMessageContent { 
                architecture: ByteOrder::LitteEndian, 
                global_msg_number: 0,
                num_fields: 4,
                fields: vec![
                    FieldDefinition {
                        base_type: base_type_nums::ENUM,
                        num: 0,
                        size: BaseType::Enum.meta().size
                    },
                    FieldDefinition {
                        base_type: base_type_nums::UINT16,
                        num: 1,
                        size: BaseType::UInt16.meta().size
                    },
                    FieldDefinition {
                        base_type: base_type_nums::UINT16,
                        num: 2,
                        size: BaseType::UInt16.meta().size
                    },
                    FieldDefinition {
                        base_type: base_type_nums::UINT32,
                        num: 3,
                        size: BaseType::UInt32.meta().size
                    }
                ],
                developer_fields: vec![]
            }
        };

        use std::io::Cursor;
        let mut buff = Cursor::new(&data);

        let mesg = read_data_mesg(&mut buff, &definition_mesg).unwrap();

        assert!(mesg.header.is_data());
        assert_eq!(mesg.fields.len(), 4);
        assert!(mesg.fields.get(&0).is_some());
        assert_eq!(mesg.fields[&0], DataField::Enum(0x04));
        assert_eq!(mesg.fields[&1], DataField::UInt16(0x0F));
        assert_eq!(mesg.fields[&2], DataField::UInt16(0x01));
        assert_eq!(mesg.fields[&3], DataField::UInt32(702940946));
    }

    #[test]
    fn test_read_file() {
        let data: [u8; 0x115] = [
            0x0E, 0x10, 0x57, 0x08, 0x05, 0x01, 0x00, 0x00, 0x2E, 0x46, 0x49, 0x54, 0x66, 0x21, 0x40, 0x00, 
            0x00, 0x00, 0x00, 0x04, 0x00, 0x01, 0x00, 0x01, 0x02, 0x84, 0x02, 0x02, 0x84, 0x04, 0x04, 0x86, 
            0x00, 0x04, 0x0F, 0x00, 0x01, 0x00, 0x12, 0x07, 0xE6, 0x29, 0x40, 0x00, 0x00, 0x15, 0x00, 0x05, 
            0xFD, 0x04, 0x86, 0x00, 0x01, 0x00, 0x01, 0x01, 0x00, 0x03, 0x04, 0x86, 0x04, 0x01, 0x02, 0x00, 
            0x12, 0x07, 0xE6, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x14, 0x00, 
            0x04, 0xFD, 0x04, 0x86, 0x02, 0x02, 0x84, 0x05, 0x04, 0x86, 0x06, 0x02, 0x84, 0x00, 0x12, 0x07, 
            0xE6, 0x29, 0x33, 0x0F, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x07, 0xE6, 0x29, 0x33, 
            0x0F, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x02, 0x00, 
            0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x15, 0x00, 0x00, 0x00, 0x00, 
            0x00, 0x00, 0x16, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x17, 
            0x07, 0xE6, 0x29, 0x33, 0x0F, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x07, 0xE6, 0x29, 
            0x33, 0x0F, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x72, 
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1A, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0xB9, 0x00, 0x00, 0x00, 
            0x5C, 0x00, 0x00, 0x1B, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x13, 0x01, 0x00, 0x00, 0x98, 0x00, 0x00, 
            0x1C, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x5F, 0x01, 0x00, 0x00, 0xD1, 0x00, 0x00, 0x1D, 0x07, 0xE6, 
            0x29, 0x33, 0x0F, 0xA6, 0x01, 0x00, 0x00, 0x06, 0x01, 0x00, 0x1E, 0x07, 0xE6, 0x29, 0x33, 0x0F, 
            0xED, 0x01, 0x00, 0x00, 0x33, 0x01, 0x00, 0x1F, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x3D, 0x02, 0x00, 
            0x00, 0x70, 0x01, 0xB8, 0xCC
        ];

        use std::io::Cursor;
        let mut buff = Cursor::new(&data);

        let fit = read_fit(&mut buff).unwrap();

        assert_eq!(fit.definitions.len(), 3);
        assert_eq!(fit.data.len(), 16);
    }

    #[test]
    fn test_read_file_with_dev_data() {
        let data: [u8; 0xB2] = [
        0x0E, 0x20, 0x68, 0x06, 0xA2, 0x00, 0x00, 0x00, 0x2E, 0x46, 0x49, 0x54, 0xBE, 0xD0, 0x40, 0x00, 
        0x01, 0x00, 0x00, 0x04, 0x01, 0x02, 0x84, 0x00, 0x01, 0x00, 0x02, 0x02, 0x84, 0x03, 0x04, 0x8C, 
        0x00, 0x00, 0x0F, 0x04, 0x23, 0x29, 0x00, 0x00, 0x06, 0xA5, 0x40, 0x00, 0x01, 0x00, 0xCF, 0x02, 
        0x01, 0x10, 0x0D, 0x03, 0x01, 0x02, 0x00, 0x01, 0x01, 0x02, 0x03, 0x05, 0x08, 0x0D, 0x15, 0x22, 
        0x37, 0x59, 0x90, 0xE9, 0x79, 0x62, 0xDB, 0x00, 0x40, 0x00, 0x01, 0x00, 0xCE, 0x05, 0x00, 0x01, 
        0x02, 0x01, 0x01, 0x02, 0x02, 0x01, 0x02, 0x03, 0x11, 0x07, 0x08, 0x0A, 0x07, 0x00, 0x00, 0x00, 
        0x01, 0x64, 0x6F, 0x75, 0x67, 0x68, 0x6E, 0x75, 0x74, 0x73, 0x5F, 0x65, 0x61, 0x72, 0x6E, 0x65, 
        0x64, 0x00, 0x64, 0x6F, 0x75, 0x67, 0x68, 0x6E, 0x75, 0x74, 0x73, 0x00, 0x60, 0x00, 0x01, 0x00, 
        0x14, 0x04, 0x03, 0x01, 0x02, 0x04, 0x01, 0x02, 0x05, 0x04, 0x86, 0x06, 0x02, 0x84, 0x01, 0x00, 
        0x01, 0x00, 0x00, 0x8C, 0x58, 0x00, 0x00, 0xC7, 0x38, 0xB9, 0x80, 0x01, 0x00, 0x8F, 0x5A, 0x00, 
        0x03, 0x2C, 0x80, 0x8E, 0x40, 0x02, 0x00, 0x90, 0x5C, 0x00, 0x05, 0xA9, 0x38, 0x8A, 0x10, 0x03, 
        0xD3, 0x9E
    ];

        use std::io::Cursor;
        let mut buff = Cursor::new(&data);

        let fit = read_fit(&mut buff).unwrap();

        assert_eq!(fit.definitions.len(), 4);
        assert_eq!(fit.data.len(), 6);
    }
}
