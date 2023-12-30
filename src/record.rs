use crate::base_type::BaseType;
use crate::byte_order::ByteOrder;
use crate::error::{FitError, Result};
use crate::profile::types::MesgNum;
use crate::{
    file_header::{read_fit_header, FileHeader},
    stream_reader::StreamReader,
};
use log::debug;
use serde::Serialize;
use std::{
    collections::HashMap,
    io::{Read, Seek},
    mem,
};

use self::record_header::RecordHeader;

pub mod record_header {
    use bits::*;
    use serde::Serialize;

    use super::MessageType;

    #[derive(Serialize, Debug)]
    pub struct RecordHeader(pub u8);

    #[rustfmt::skip]
    pub mod bits {
        pub const HEADER_TYPE_BIT: u8         = 0b10000000;
        pub const MESSAGE_TYPE_BIT: u8        = 0b01000000;
        pub const MESSAGE_DEV_FLAG_BIT: u8    = 0b00100000;
        pub const LOCAL_MESSAGE_TYPE_BITS: u8 = 0b00001111;
    }

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
}

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Definition,
    Data,
    CompressedHeader,
}

#[derive(Serialize, Debug)]
pub struct FieldDefinition {
    pub field_def_num: u8,
    pub size: u8,
    pub base_type: BaseType,
}

#[derive(Serialize, Debug)]
pub struct DeveloperFieldDefinition {
    pub field_num: u8,
    pub size: u8,
    pub dev_data_index: u8,
}

#[derive(Serialize, Debug)]
pub struct DefinitionMessage {
    pub architecture: ByteOrder,
    pub global_msg_number: u16,
    pub num_fields: u8,
    pub fields: Vec<FieldDefinition>,
    pub developer_fields: Vec<DeveloperFieldDefinition>,
}

#[derive(Serialize, Debug)]
pub struct DefinitionRecord {
    pub header: RecordHeader,
    pub content: DefinitionMessage,
}

impl DefinitionRecord {
    pub fn data_size(&self) -> u32 {
        self.content.fields.iter().map(|x| x.size as u32).sum::<u32>()
            + self.content.developer_fields.iter().map(|x| x.size as u32).sum::<u32>()
    }
}

#[derive(Serialize, Debug)]
pub struct DataMessage {
    pub mesg_num: u16,
    pub fields: Vec<DataField>,
    pub dev_fields: Vec<DeveloperDataField>,
}

#[derive(Serialize, Debug)]
pub struct DataRecord {
    pub header: RecordHeader,
    pub content: DataMessage,
}

#[derive(PartialEq, Debug, Serialize)]
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

#[derive(Debug, Serialize, PartialEq)]
pub struct DataField {
    pub field_def_num: u8,
    pub data: Data,
}

#[derive(Debug, Serialize)]
pub struct DeveloperDataField {
    pub num: u8,
    pub index: u8,
    pub data: Data,
}

#[derive(Serialize, Debug)]
pub struct FitFile {
    pub header: FileHeader,
    pub data: Vec<FitDataMessage>,
    pub crc: u16,
}

#[derive(Serialize, Debug)]
pub struct FitDataMessage {
    pub mesg_num: MesgNum,
    pub fields: HashMap<u8, DataField>,
    pub dev_fields: Vec<DeveloperDataField>,
}

impl FitDataMessage {
    ///  Returns the data for the given field definition number
    pub fn data(&self, field_def_num: u8) -> Option<&Data> {
        self.fields.get(&field_def_num).map(|x| &x.data)
    }
}

fn read_record_header<T: Read + Seek>(reader: &mut StreamReader<T>) -> Result<RecordHeader> {
    let header_byte = reader.read_u8()?;
    Ok(RecordHeader(header_byte))
}

fn read_field_definition<T: Read + Seek>(reader: &mut StreamReader<T>) -> Result<FieldDefinition> {
    let num = reader.read_u8()?;
    let size = reader.read_u8()?;
    let base_type: BaseType = reader.read_u8()?.try_into()?;

    Ok(FieldDefinition {
        field_def_num: num,
        size,
        base_type,
    })
}

pub fn read_definition_record<T: Read + Seek>(reader: &mut StreamReader<T>) -> Result<DefinitionRecord> {
    let header_byte = reader.read_u8()?;
    debug!(
        "read definition mesg {:x} {:x}",
        reader.stream_position()? - 1,
        header_byte
    );
    let header = RecordHeader(header_byte);

    if MessageType::Definition != header.msg_type() {
        Err(FitError::ParseRecord("not a definition record".to_string()))?
    }

    let _reserved = reader.read_u8()?;
    let architecture = reader.read_u8().map(ByteOrder::try_from)??;

    let global_msg_number = reader.read_u16(architecture)?;

    let num_fields = reader.read_u8()?;
    let mut fields: Vec<FieldDefinition> = vec![];

    for _n in 0..num_fields {
        let field_def = read_field_definition(reader)?;
        fields.push(field_def);
    }

    let mut developer_fields: Vec<DeveloperFieldDefinition> = vec![];
    if header.has_dev() {
        let num_developer_fields = reader.read_u8()?;

        for _n in 0..num_developer_fields {
            let num = reader.read_u8()?;
            let size = reader.read_u8()?;
            let index = reader.read_u8()?;

            developer_fields.push(DeveloperFieldDefinition {
                field_num: num,
                size,
                dev_data_index: index,
            });
        }
    }

    let mesg = DefinitionRecord {
        header,
        content: DefinitionMessage {
            architecture,
            global_msg_number,
            num_fields,
            fields,
            developer_fields,
        },
    };

    debug!("definition mesg: {:?}", mesg);

    Ok(mesg)
}

fn read_array<T>(size: usize, mut read: impl FnMut() -> std::result::Result<T, std::io::Error>) -> Result<Vec<T>> {
    let type_size = mem::size_of::<T>();
    let count = size / type_size;
    let buf = (0..count).map(|_| read()).collect::<std::result::Result<Vec<T>, _>>()?;
    Ok(buf)
}

fn read_data_field<T: Read + Seek>(
    reader: &mut StreamReader<T>,
    field_def: &FieldDefinition,
    byte_order: ByteOrder,
) -> Result<Data> {
    use crate::base_type::BaseType::*;

    let data_size = field_def.size as usize;
    let base_type = &field_def.base_type;
    let is_array = field_def.size > base_type.info().size;

    let data = match (base_type, is_array) {
        (Enum, false) => Data::Enum(reader.read_u8()?),
        (Enum, true) => Data::EnumArray(read_array(data_size, || reader.read_u8())?),
        (Uint8, false) | (Uint8z, false) => Data::Uint8(reader.read_u8()?),
        (Uint8, true) | (Uint8z, true) => Data::Uint8Array(read_array(data_size, || reader.read_u8())?),
        (Sint8, false) => Data::Sint8(reader.read_i8()?),
        (Sint8, true) => Data::Sint8Array(read_array(data_size, || reader.read_i8())?),
        (Uint16, false) | (Uint16z, false) => Data::Uint16(reader.read_u16(byte_order)?),
        (Uint16, true) | (Uint16z, true) => Data::Uint16Array(read_array(data_size, || reader.read_u16(byte_order))?),
        (Sint16, false) => Data::Sint16(reader.read_i16(byte_order)?),
        (Sint16, true) => Data::Sint16Array(read_array(data_size, || reader.read_i16(byte_order))?),
        (Sint32, false) => Data::Sint32(reader.read_i32(byte_order)?),
        (Sint32, true) => Data::Sint32Array(read_array(data_size, || reader.read_i32(byte_order))?),
        (Uint32, false) | (Uint32z, false) => Data::Uint32(reader.read_u32(byte_order)?),
        (Uint32, true) | (Uint32z, true) => Data::Uint32Array(read_array(data_size, || reader.read_u32(byte_order))?),
        (Sint64, false) => Data::Sint64(reader.read_i64(byte_order)?),
        (Sint64, true) => Data::Sint64Array(read_array(data_size, || reader.read_i64(byte_order))?),
        (Uint64, false) | (Uint64z, false) => Data::Uint64(reader.read_u64(byte_order)?),
        (Uint64, true) | (Uint64z, true) => Data::Uint64Array(read_array(data_size, || reader.read_u64(byte_order))?),
        (Float32, false) => Data::Float32(reader.read_f32(byte_order)?),
        (Float32, true) => Data::Float32Array(read_array(data_size, || reader.read_f32(byte_order))?),
        (Float64, false) => Data::Float64(reader.read_f64(byte_order)?),
        (Float64, true) => Data::Float64Array(read_array(data_size, || reader.read_f64(byte_order))?),
        (Byte, false) => Data::Byte(reader.read_u8()?),
        (Byte, true) => Data::ByteArray(read_array(data_size, || reader.read_u8())?),
        (String, _) => Data::String(reader.read_string_null_term(data_size)?),
    };

    Ok(data)
}

fn read_developer_data_field<T: Read + Seek>(
    reader: &mut StreamReader<T>,
    dev_field_def: &DeveloperFieldDefinition,
) -> Result<Data> {
    //TODO: parse out developer data to correct types
    let mut buf = vec![0u8; dev_field_def.size as usize];
    reader.read_exact(buf.as_mut_slice())?;
    Ok(Data::ByteArray(buf))
}

pub fn read_data_record<T: Read + Seek>(
    reader: &mut StreamReader<T>,
    definition: &DefinitionRecord,
) -> Result<DataRecord> {
    let header = read_record_header(reader)?;

    debug!(
        "read data mesg at pos {:x}, header byte {:x}",
        reader.stream_position()? - 1,
        header.0
    );

    if !header.is_data() && !header.is_compressed() {
        Err(FitError::ParseRecord("not a data record".to_string()))?
    }

    let fields = definition
        .content
        .fields
        .iter()
        .map(|field_def| {
            read_data_field(reader, field_def, definition.content.architecture).map(|data| DataField {
                field_def_num: field_def.field_def_num,
                data,
            })
        })
        .collect::<Result<Vec<DataField>>>()?;

    let dev_fields = definition
        .content
        .developer_fields
        .iter()
        .map(|dev_field_def| {
            read_developer_data_field(reader, dev_field_def).map(|field| DeveloperDataField {
                num: dev_field_def.field_num,
                index: dev_field_def.dev_data_index,
                data: field,
            })
        })
        .collect::<Result<Vec<DeveloperDataField>>>()?;

    Ok(DataRecord {
        header,
        content: DataMessage {
            mesg_num: definition.content.global_msg_number,
            fields,
            dev_fields,
        },
    })
}

pub fn read_fit<T: Read + Seek>(reader: &mut StreamReader<T>) -> Result<FitFile> {
    let header = read_fit_header(reader)?;

    let mut data: Vec<DataRecord> = vec![];
    let mut definitions: HashMap<u8, DefinitionRecord> = HashMap::new();

    while let Ok(b) = reader.peek_byte() {
        let record_header = RecordHeader(b);

        if record_header.is_definition() {
            let def_record = read_definition_record(reader)?;
            definitions.insert(def_record.header.local_msg_type(), def_record);
        } else if let Some(definition) = definitions.get(&record_header.local_msg_type()) {
            let data_record = read_data_record(reader, definition)?;
            data.push(data_record);
        } else {
            Err(FitError::MissingDefinition {
                stream_pos: reader.stream_position().unwrap_or(0),
                local_msg_type: record_header.local_msg_type(),
            })?
        }

        if reader.stream_position()? == header.mesg_size() {
            let crc = reader.read_crc()?;
            if crc != reader.crc() {
                Err(FitError::Crc)?
            }
        }
    }

    let data: Vec<FitDataMessage> = data
        .into_iter()
        .map(|data_rec| FitDataMessage {
            mesg_num: data_rec.content.mesg_num.into(),
            fields: data_rec
                .content
                .fields
                .into_iter()
                .map(|x| (x.field_def_num, x))
                .collect::<HashMap<u8, DataField>>(),
            dev_fields: data_rec.content.dev_fields,
        })
        .collect();

    Ok(FitFile {
        header,
        data,
        crc: reader.crc(),
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    fn assert_field_type(field_def: &FieldDefinition, num: u8, base_type: &BaseType) {
        assert_eq!(field_def.field_def_num, num);
        assert_eq!(field_def.base_type, *base_type);
        assert_eq!(field_def.size, base_type.info().size);
    }

    #[test]
    fn test_read_file_definition_record() {
        let data: [u8; 0x12] = [
            0x40, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x01, 0x00, 0x01, 0x02, 0x84, 0x02, 0x02, 0x84, 0x04, 0x04, 0x86,
        ];

        let mut buff = StreamReader::new(Cursor::new(&data));

        let mesg = read_definition_record(&mut buff).unwrap();

        assert_eq!(mesg.header.msg_type(), MessageType::Definition);
        assert_eq!(mesg.header.local_msg_type(), 0);
        assert!(mesg.header.is_normal());
        assert_eq!(mesg.content.architecture, ByteOrder::LitteEndian);
        assert_eq!(mesg.content.fields.len(), 4);

        assert_field_type(&mesg.content.fields[0], 0, &BaseType::Enum);
        assert_field_type(&mesg.content.fields[1], 1, &BaseType::Uint16);
        assert_field_type(&mesg.content.fields[2], 2, &BaseType::Uint16);
        assert_field_type(&mesg.content.fields[3], 4, &BaseType::Uint32);

        assert_eq!(mesg.content.developer_fields.len(), 0);
    }

    #[test]
    fn test_read_file_definition_record_with_dev_fields() {
        let data: [u8; 0x16] = [
            0b01100000, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x01, 0x00, 0x01, 0x02, 0x84, 0x02, 0x02, 0x84, 0x04, 0x04,
            0x86, 0x01, 0x00, 0x02, 0x01,
        ];

        let mut reader = StreamReader::new(Cursor::new(&data));

        let mesg = read_definition_record(&mut reader).unwrap();

        assert_eq!(mesg.content.developer_fields.len(), 1);
        assert_eq!(mesg.content.developer_fields[0].field_num, 0);
        assert_eq!(mesg.content.developer_fields[0].size, 2);
        assert_eq!(mesg.content.developer_fields[0].dev_data_index, 1);
    }

    #[test]
    fn test_read_data_record() {
        let data: [u8; 10] = [0x00, 0x04, 0x0F, 0x00, 0x01, 0x00, 0x12, 0x07, 0xE6, 0x29];

        let definition_mesg = DefinitionRecord {
            header: RecordHeader(0x40),
            content: DefinitionMessage {
                architecture: ByteOrder::LitteEndian,
                global_msg_number: 0,
                num_fields: 4,
                fields: vec![
                    FieldDefinition {
                        base_type: BaseType::Enum,
                        field_def_num: 0,
                        size: BaseType::Enum.info().size,
                    },
                    FieldDefinition {
                        base_type: BaseType::Uint16,
                        field_def_num: 1,
                        size: BaseType::Uint16.info().size,
                    },
                    FieldDefinition {
                        base_type: BaseType::Uint16,
                        field_def_num: 2,
                        size: BaseType::Uint16.info().size,
                    },
                    FieldDefinition {
                        base_type: BaseType::Uint32,
                        field_def_num: 3,
                        size: BaseType::Uint32.info().size,
                    },
                ],
                developer_fields: vec![],
            },
        };

        let mut reader = StreamReader::new(Cursor::new(&data));

        let mesg = read_data_record(&mut reader, &definition_mesg).unwrap();

        assert_eq!(mesg.content.fields.len(), 4);
        assert_eq!(
            mesg.content.fields[0],
            DataField {
                field_def_num: 0,
                data: Data::Enum(0x04)
            }
        );
        assert_eq!(
            mesg.content.fields[1],
            DataField {
                field_def_num: 1,
                data: Data::Uint16(0x0F)
            }
        );
        assert_eq!(
            mesg.content.fields[2],
            DataField {
                field_def_num: 2,
                data: Data::Uint16(0x01)
            }
        );
        assert_eq!(
            mesg.content.fields[3],
            DataField {
                field_def_num: 3,
                data: Data::Uint32(702940946)
            }
        );
    }

    #[test]
    fn test_read_file() {
        let data: [u8; 0x115] = [
            0x0E, 0x10, 0x57, 0x08, 0x05, 0x01, 0x00, 0x00, 0x2E, 0x46, 0x49, 0x54, 0x66, 0x21, 0x40, 0x00, 0x00, 0x00,
            0x00, 0x04, 0x00, 0x01, 0x00, 0x01, 0x02, 0x84, 0x02, 0x02, 0x84, 0x04, 0x04, 0x86, 0x00, 0x04, 0x0F, 0x00,
            0x01, 0x00, 0x12, 0x07, 0xE6, 0x29, 0x40, 0x00, 0x00, 0x15, 0x00, 0x05, 0xFD, 0x04, 0x86, 0x00, 0x01, 0x00,
            0x01, 0x01, 0x00, 0x03, 0x04, 0x86, 0x04, 0x01, 0x02, 0x00, 0x12, 0x07, 0xE6, 0x29, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x14, 0x00, 0x04, 0xFD, 0x04, 0x86, 0x02, 0x02, 0x84, 0x05, 0x04, 0x86,
            0x06, 0x02, 0x84, 0x00, 0x12, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13,
            0x07, 0xE6, 0x29, 0x33, 0x0F, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x07, 0xE6, 0x29, 0x33, 0x0F,
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x15, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x16, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x17, 0x07, 0xE6,
            0x29, 0x33, 0x0F, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x29, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x1A, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0xB9, 0x00, 0x00, 0x00, 0x5C, 0x00, 0x00, 0x1B, 0x07, 0xE6, 0x29, 0x33,
            0x0F, 0x13, 0x01, 0x00, 0x00, 0x98, 0x00, 0x00, 0x1C, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x5F, 0x01, 0x00, 0x00,
            0xD1, 0x00, 0x00, 0x1D, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0xA6, 0x01, 0x00, 0x00, 0x06, 0x01, 0x00, 0x1E, 0x07,
            0xE6, 0x29, 0x33, 0x0F, 0xED, 0x01, 0x00, 0x00, 0x33, 0x01, 0x00, 0x1F, 0x07, 0xE6, 0x29, 0x33, 0x0F, 0x3D,
            0x02, 0x00, 0x00, 0x70, 0x01, 0xB8, 0xCC,
        ];

        let mut reader = StreamReader::new(Cursor::new(&data));

        let fit = read_fit(&mut reader).unwrap();

        assert_eq!(fit.data.len(), 16);
        assert_eq!(reader.crc(), 0xCCB8)
    }

    #[test]
    fn test_read_file_with_dev_data() {
        let data: [u8; 0xB2] = [
            0x0E, 0x20, 0x68, 0x06, 0xA2, 0x00, 0x00, 0x00, 0x2E, 0x46, 0x49, 0x54, 0xBE, 0xD0, 0x40, 0x00, 0x01, 0x00,
            0x00, 0x04, 0x01, 0x02, 0x84, 0x00, 0x01, 0x00, 0x02, 0x02, 0x84, 0x03, 0x04, 0x8C, 0x00, 0x00, 0x0F, 0x04,
            0x23, 0x29, 0x00, 0x00, 0x06, 0xA5, 0x40, 0x00, 0x01, 0x00, 0xCF, 0x02, 0x01, 0x10, 0x0D, 0x03, 0x01, 0x02,
            0x00, 0x01, 0x01, 0x02, 0x03, 0x05, 0x08, 0x0D, 0x15, 0x22, 0x37, 0x59, 0x90, 0xE9, 0x79, 0x62, 0xDB, 0x00,
            0x40, 0x00, 0x01, 0x00, 0xCE, 0x05, 0x00, 0x01, 0x02, 0x01, 0x01, 0x02, 0x02, 0x01, 0x02, 0x03, 0x11, 0x07,
            0x08, 0x0A, 0x07, 0x00, 0x00, 0x00, 0x01, 0x64, 0x6F, 0x75, 0x67, 0x68, 0x6E, 0x75, 0x74, 0x73, 0x5F, 0x65,
            0x61, 0x72, 0x6E, 0x65, 0x64, 0x00, 0x64, 0x6F, 0x75, 0x67, 0x68, 0x6E, 0x75, 0x74, 0x73, 0x00, 0x60, 0x00,
            0x01, 0x00, 0x14, 0x04, 0x03, 0x01, 0x02, 0x04, 0x01, 0x02, 0x05, 0x04, 0x86, 0x06, 0x02, 0x84, 0x01, 0x00,
            0x01, 0x00, 0x00, 0x8C, 0x58, 0x00, 0x00, 0xC7, 0x38, 0xB9, 0x80, 0x01, 0x00, 0x8F, 0x5A, 0x00, 0x03, 0x2C,
            0x80, 0x8E, 0x40, 0x02, 0x00, 0x90, 0x5C, 0x00, 0x05, 0xA9, 0x38, 0x8A, 0x10, 0x03, 0xD3, 0x9E,
        ];

        let mut reader = StreamReader::new(Cursor::new(&data));

        let fit = read_fit(&mut reader).unwrap();

        assert_eq!(fit.data.len(), 6);
        assert_eq!(reader.crc(), 0x9ED3);
    }
}
