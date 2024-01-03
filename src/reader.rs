use std::{collections::HashMap, io::Read};

use crate::protocol::reader::Reader as ProtocolReader;
use crate::{
    error::{Error, Result},
    protocol::{DataMessage, DefinitionMessage, FileHeader},
};

pub struct Reader<T> {
    reader: ProtocolReader<T>,
    file_header: Option<FileHeader>,
    definitions: HashMap<u8, DefinitionMessage>,
    validate_crc: bool,
    eof: bool,
}

impl<T: Read> Reader<T> {
    pub fn from_reader(reader: T) -> Self {
        Self {
            reader: ProtocolReader::new(reader),
            file_header: None,
            definitions: HashMap::new(),
            validate_crc: true,
            eof: false,
        }
    }

    fn open_file(&mut self) -> Result<()> {
        if self.validate_crc {
            self.reader.set_crc_enabled(true)
        }
        self.file_header = match self.reader.read_fit_header() {
            Ok(header) => Some(header),
            Err(error) => Err(error)?,
        };

        Ok(())
    }

    pub fn data_messages(&mut self) -> DataMessageIterator<T> {
        DataMessageIterator { reader: self }
    }

    pub fn read_data_message(&mut self) -> Result<Option<DataMessage>> {
        if self.file_header.is_none() {
            self.open_file()?
        }

        let header_ref = self.file_header.as_ref().unwrap();

        if self.eof {
            Err(Error::Eof)?
        } else if self.reader.is_eof(header_ref.file_size()) {
            self.eof = true;

            let crc = self.reader.read_crc()?;
            if self.validate_crc && crc != self.reader.crc() {
                Err(Error::Crc)?
            }
            return Ok(None);
        }

        while let Ok(record_header) = self.reader.read_record_header() {
            if record_header.is_definition() {
                let def_record = self.reader.read_definition_record_content(record_header)?;
                self.definitions.insert(def_record.local_mesg_number, def_record);
            } else if let Some(definition) = self.definitions.get(&record_header.local_msg_type()) {
                let data_record = self.reader.read_data_record_content(record_header, definition)?;
                return Ok(Some(data_record));
            } else {
                Err(Error::MissingDefinition {
                    stream_pos: self.reader.position(),
                    local_msg_type: record_header.local_msg_type(),
                })?
            }
        }

        Err(Error::ReadRecord("failed to find a record".to_string()))?
    }
}

pub struct DataMessageIterator<'a, T> {
    reader: &'a mut Reader<T>,
}

impl<'a, T: Read> Iterator for DataMessageIterator<'a, T> {
    type Item = Result<DataMessage>;

    fn next(&mut self) -> Option<Self::Item> {
        let data_record = self.reader.read_data_message();

        match data_record {
            Err(err) => Some(Err(err)),
            Ok(Some(data)) => Some(Ok(data)),
            Ok(None) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

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

        let mut reader = Reader::from_reader(Cursor::new(&data));

        let data: Vec<DataMessage> = reader.data_messages().collect::<Result<Vec<DataMessage>>>().unwrap();

        assert_eq!(data.len(), 16);
        //assert_eq!(reader.crc(), 0xCCB8)
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

        let mut reader = Reader::from_reader(Cursor::new(&data));

        let data: Vec<DataMessage> = reader.data_messages().collect::<Result<Vec<DataMessage>>>().unwrap();

        assert_eq!(data.len(), 6);
        //assert_eq!(reader.crc(), 0x9ED3);
    }
}
