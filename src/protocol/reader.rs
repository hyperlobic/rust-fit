use std::collections::HashMap;
use std::io::Read;
use std::mem;

use log::{debug, trace};

use crate::base_type::BaseType;
use crate::byte_order::ByteOrder;
use crate::error::{Error, Result};
use crate::protocol::{DataField, DeveloperDataField, DeveloperFieldDefinition, FileHeader, MessageType};
use crate::stream_reader::StreamReader;

use super::{RecordHeader, Data, DataMessage, DefinitionMessage, FieldDefinition};

pub struct Reader<T> {
    reader: StreamReader<T>,
}

impl<T: Read> Reader<T> {
    pub fn new(reader: T) -> Self {
        Reader {
            reader: StreamReader::new(reader),
        }
    }

    pub fn set_crc_enabled(&mut self, enable: bool) {
        self.reader.set_crc_enabled(enable);
    }

    pub fn crc(&self) -> u16 {
        self.reader.crc()
    }

    pub fn position(&self) -> u64 {
        self.reader.position()
    }

    pub fn read_crc(&mut self) -> Result<u16> {
        self.reader.read_crc().map_err(|err| err.into())
    }

    pub fn is_eof(&self, data_size: u32) -> bool {
        self.reader.position() == data_size as u64
    }

    pub fn read_fit_header(&mut self) -> Result<FileHeader> {
        let header_size = self.reader.read_u8()?;
        let mut extra = if header_size > 14 { header_size - 14 } else { 0 };

        let protocol_version = self.reader.read_u8()?;
        let profile_version = self.reader.read_u16(ByteOrder::LitteEndian)?;
        let data_size = self.reader.read_u32(ByteOrder::LitteEndian)?;
        let data_type = self.reader.read_string_fixed(4)?;
        let crc = if header_size >= 14 {
            self.reader.read_u16(ByteOrder::LitteEndian)?
        } else {
            0
        };

        while extra > 0 {
            self.reader.read_u8()?;
            extra -= 1;
        }

        if data_type != ".FIT" {
            Err(Error::InvalidFile)?
        }

        Ok(FileHeader {
            header_size,
            protocol_version,
            profile_version,
            data_size,
            data_type,
            crc,
        })
    }

    pub fn read_record_header(&mut self) -> Result<RecordHeader> {
        let header_byte = self.reader.read_u8()?;
        Ok(RecordHeader(header_byte))
    }

    pub fn read_field_definition(&mut self) -> Result<FieldDefinition> {
        let num = self.reader.read_u8()?;
        let size = self.reader.read_u8()?;
        let base_type: BaseType = self.reader.read_u8()?.try_into()?;

        Ok(FieldDefinition {
            field_def_num: num,
            size,
            base_type,
        })
    }

    fn read_array<K>(size: u32, mut read: impl FnMut() -> std::result::Result<K, std::io::Error>) -> Result<Vec<K>> {
        let type_size = mem::size_of::<K>() as u32;
        let count = size / type_size;
        let buf = (0..count).map(|_| read()).collect::<std::result::Result<Vec<K>, _>>()?;
        Ok(buf)
    }

    pub fn read_data_field(&mut self, field_def: &FieldDefinition, byte_order: ByteOrder) -> Result<Data> {
        use crate::base_type::BaseType::*;

        let data_size = field_def.size as u32;
        let base_type = &field_def.base_type;
        let is_array = field_def.size > base_type.info().size;

        let data = match (base_type, is_array) {
            (Enum, false) => Data::Enum(self.reader.read_u8()?),
            (Enum, true) => Data::EnumArray(Self::read_array(data_size, || self.reader.read_u8())?),
            (Uint8, false) | (Uint8z, false) => Data::Uint8(self.reader.read_u8()?),
            (Uint8, true) | (Uint8z, true) => Data::Uint8Array(Self::read_array(data_size, || self.reader.read_u8())?),
            (Sint8, false) => Data::Sint8(self.reader.read_i8()?),
            (Sint8, true) => Data::Sint8Array(Self::read_array(data_size, || self.reader.read_i8())?),
            (Uint16, false) | (Uint16z, false) => Data::Uint16(self.reader.read_u16(byte_order)?),
            (Uint16, true) | (Uint16z, true) => {
                Data::Uint16Array(Self::read_array(data_size, || self.reader.read_u16(byte_order))?)
            }
            (Sint16, false) => Data::Sint16(self.reader.read_i16(byte_order)?),
            (Sint16, true) => Data::Sint16Array(Self::read_array(data_size, || self.reader.read_i16(byte_order))?),
            (Sint32, false) => Data::Sint32(self.reader.read_i32(byte_order)?),
            (Sint32, true) => Data::Sint32Array(Self::read_array(data_size, || self.reader.read_i32(byte_order))?),
            (Uint32, false) | (Uint32z, false) => Data::Uint32(self.reader.read_u32(byte_order)?),
            (Uint32, true) | (Uint32z, true) => {
                Data::Uint32Array(Self::read_array(data_size, || self.reader.read_u32(byte_order))?)
            }
            (Sint64, false) => Data::Sint64(self.reader.read_i64(byte_order)?),
            (Sint64, true) => Data::Sint64Array(Self::read_array(data_size, || self.reader.read_i64(byte_order))?),
            (Uint64, false) | (Uint64z, false) => Data::Uint64(self.reader.read_u64(byte_order)?),
            (Uint64, true) | (Uint64z, true) => {
                Data::Uint64Array(Self::read_array(data_size, || self.reader.read_u64(byte_order))?)
            }
            (Float32, false) => Data::Float32(self.reader.read_f32(byte_order)?),
            (Float32, true) => Data::Float32Array(Self::read_array(data_size, || self.reader.read_f32(byte_order))?),
            (Float64, false) => Data::Float64(self.reader.read_f64(byte_order)?),
            (Float64, true) => Data::Float64Array(Self::read_array(data_size, || self.reader.read_f64(byte_order))?),
            (Byte, false) => Data::Byte(self.reader.read_u8()?),
            (Byte, true) => Data::ByteArray(Self::read_array(data_size, || self.reader.read_u8())?),
            (String, _) => Data::String(self.reader.read_string_null_term(data_size)?),
        };

        Ok(data)
    }

    pub fn read_definition_record_content(&mut self, record_header: RecordHeader) -> Result<DefinitionMessage> {
        debug!(
            "read definition mesg {:x} {:x}",
            self.reader.position() - 1,
            record_header.0
        );

        if MessageType::Definition != record_header.mesg_type() {
            Err(Error::ParseRecord("not a definition record".to_string()))?
        }

        let _reserved = self.reader.read_u8()?;
        let architecture = self.reader.read_u8().map(ByteOrder::try_from)??;

        let global_mesg_num = self.reader.read_u16(architecture)?;

        let num_fields = self.reader.read_u8()?;
        let mut fields: Vec<FieldDefinition> = vec![];

        for _n in 0..num_fields {
            let field_def = self.read_field_definition()?;
            fields.push(field_def);
        }

        let mut developer_fields: Vec<DeveloperFieldDefinition> = vec![];
        if record_header.has_dev() {
            let num_developer_fields = self.reader.read_u8()?;

            for _n in 0..num_developer_fields {
                let num = self.reader.read_u8()?;
                let size = self.reader.read_u8()?;
                let index = self.reader.read_u8()?;

                developer_fields.push(DeveloperFieldDefinition {
                    field_num: num,
                    size,
                    dev_data_index: index,
                });
            }
        }

        let mesg = DefinitionMessage {
            architecture,
            global_mesg_num,
            local_mesg_num: record_header.local_mesg_num(),
            num_fields,
            fields,
            developer_fields,
        };

        trace!("definition mesg: {:?}", mesg);

        Ok(mesg)
    }

    fn read_developer_data_field(&mut self, dev_field_def: &DeveloperFieldDefinition) -> Result<Data> {
        //TODO: parse out developer data to correct types
        let mut buf = vec![0u8; dev_field_def.size as usize];
        self.reader.read_exact(buf.as_mut_slice())?;
        Ok(Data::ByteArray(buf))
    }

    pub fn read_data_record_content(
        &mut self,
        record_header: RecordHeader,
        definition: &DefinitionMessage,
    ) -> Result<DataMessage> {
        debug!(
            "read data mesg at pos {:x}, header byte {:x}, global mesg num {}",
            self.reader.position() - 1,
            record_header.0,
            definition.global_mesg_num
        );

        if !record_header.is_data() && !record_header.is_compressed() {
            Err(Error::ParseRecord("not a data record".to_string()))?
        }

        let fields = definition
            .fields
            .iter()
            .map(|field_def| {
                self.read_data_field(field_def, definition.architecture).map(|data| {
                    (
                        field_def.field_def_num,
                        DataField {
                            field_def_num: field_def.field_def_num,
                            data,
                        },
                    )
                })
            })
            .collect::<Result<HashMap<u8, DataField>>>()?;

        let dev_fields = definition
            .developer_fields
            .iter()
            .map(|dev_field_def| {
                self.read_developer_data_field(dev_field_def)
                    .map(|field| DeveloperDataField {
                        num: dev_field_def.field_num,
                        index: dev_field_def.dev_data_index,
                        data: field,
                    })
            })
            .collect::<Result<Vec<DeveloperDataField>>>()?;

        let data_mesg = DataMessage {
            mesg_num: definition.global_mesg_num.into(),
            fields,
            dev_fields,
        };

        Ok(data_mesg)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_14_byte_header() {
        let header_bytes: [u8; 14] = [
            0x0E, 0x10, 0x7D, 0x52, 0xC8, 0x08, 0x00, 0x00, 0x2E, 0x46, 0x49, 0x54, 0x91, 0x01,
        ];

        use std::io::Cursor;
        let mut reader = Reader::new(Cursor::new(&header_bytes));

        let header = reader.read_fit_header().unwrap();

        assert_eq!(header.header_size, 14);
        assert_eq!(header.data_type, ".FIT");
        assert_eq!(header.data_size, 2248);
        assert_eq!(header.crc, 0x0191);
    }

    #[test]
    fn test_read_12_byte_header() {
        let header_bytes: [u8; 12] = [0x0C, 0x10, 0x7D, 0x52, 0xC8, 0x08, 0x00, 0x00, 0x2E, 0x46, 0x49, 0x54];

        use std::io::Cursor;
        let mut reader = Reader::new(Cursor::new(&header_bytes));

        let header = reader.read_fit_header().unwrap();

        assert_eq!(header.header_size, 12);
        assert_eq!(header.data_type, ".FIT");
        assert_eq!(header.data_size, 2248);
        assert_eq!(header.crc, 0)
    }

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

        let mut reader = Reader::new(Cursor::new(&data));

        let header = reader.read_record_header().unwrap();
        let mesg = reader.read_definition_record_content(header).unwrap();

        assert_eq!(mesg.local_mesg_num, 0);
        assert_eq!(mesg.architecture, ByteOrder::LitteEndian);
        assert_eq!(mesg.fields.len(), 4);

        assert_field_type(&mesg.fields[0], 0, &BaseType::Enum);
        assert_field_type(&mesg.fields[1], 1, &BaseType::Uint16);
        assert_field_type(&mesg.fields[2], 2, &BaseType::Uint16);
        assert_field_type(&mesg.fields[3], 4, &BaseType::Uint32);

        assert_eq!(mesg.developer_fields.len(), 0);
    }

    #[test]
    fn test_read_file_definition_record_with_dev_fields() {
        let data: [u8; 0x16] = [
            0b01100000, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x01, 0x00, 0x01, 0x02, 0x84, 0x02, 0x02, 0x84, 0x04, 0x04,
            0x86, 0x01, 0x00, 0x02, 0x01,
        ];

        let mut reader = Reader::new((Cursor::new(&data)));

        let header = reader.read_record_header().unwrap();
        let mesg = reader.read_definition_record_content(header).unwrap();

        assert_eq!(mesg.developer_fields.len(), 1);
        assert_eq!(mesg.developer_fields[0].field_num, 0);
        assert_eq!(mesg.developer_fields[0].size, 2);
        assert_eq!(mesg.developer_fields[0].dev_data_index, 1);
    }

    #[test]
    fn test_read_data_record() {
        let data: [u8; 10] = [0x00, 0x04, 0x0F, 0x00, 0x01, 0x00, 0x12, 0x07, 0xE6, 0x29];

        let definition_mesg = DefinitionMessage {
            architecture: ByteOrder::LitteEndian,
            global_mesg_num: 0,
            local_mesg_num: 0,
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
        };

        let mut reader = Reader::new(Cursor::new(&data));
        let header = reader.read_record_header().unwrap();

        let mesg = reader.read_data_record_content(header, &definition_mesg).unwrap();

        assert_eq!(mesg.fields().count(), 4);
        assert_eq!(mesg.data(0), Some(&Data::Enum(0x04)));
        assert_eq!(mesg.data(1), Some(&Data::Uint16(0x0F)));
        assert_eq!(mesg.data(2), Some(&Data::Uint16(0x01)));
        assert_eq!(mesg.data(3), Some(&Data::Uint32(702940946)));
    }
}
