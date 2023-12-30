use crate::error::Result;
use crate::{byte_order::ByteOrder, stream_reader::StreamReader};
use serde::Serialize;
use std::io::{Read, Seek};

#[derive(Default, Debug, Serialize)]
pub struct FileHeader {
    pub header_size: u8,
    pub protocol_version: u8,
    pub profile_version: u16,
    pub data_size: u32,
    pub data_type: String,
    pub crc: u16,
}

impl FileHeader {
    pub fn mesg_size(&self) -> u64 {
        self.header_size as u64 + self.data_size as u64
    }
}

pub fn read_fit_header<T: Read + Seek>(reader: &mut StreamReader<T>) -> Result<FileHeader> {
    let header_size = reader.read_u8()?;
    let mut extra = if header_size > 14 { header_size - 14 } else { 0 };

    let protocol_version = reader.read_u8()?;
    let profile_version = reader.read_u16(ByteOrder::LitteEndian)?;
    let data_size = reader.read_u32(ByteOrder::LitteEndian)?;
    let data_type = reader.read_string_fixed(4)?;
    let crc = if header_size >= 14 {
        reader.read_u16(ByteOrder::LitteEndian)?
    } else {
        0
    };

    while extra > 0 {
        reader.read_u8()?;
        extra -= 1;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_14_byte_header() {
        let header_bytes: [u8; 14] = [
            0x0E, 0x10, 0x7D, 0x52, 0xC8, 0x08, 0x00, 0x00, 0x2E, 0x46, 0x49, 0x54, 0x91, 0x01,
        ];

        use std::io::Cursor;
        let mut reader = StreamReader::new(Cursor::new(&header_bytes));

        let header = read_fit_header(&mut reader).unwrap();

        assert_eq!(header.header_size, 14);
        assert_eq!(header.data_type, ".FIT");
        assert_eq!(header.data_size, 2248);
        assert_eq!(header.crc, 0x0191);
    }

    #[test]
    fn test_read_12_byte_header() {
        let header_bytes: [u8; 12] = [0x0C, 0x10, 0x7D, 0x52, 0xC8, 0x08, 0x00, 0x00, 0x2E, 0x46, 0x49, 0x54];

        use std::io::Cursor;
        let mut reader = StreamReader::new(Cursor::new(&header_bytes));

        let header = read_fit_header(&mut reader).unwrap();

        assert_eq!(header.header_size, 12);
        assert_eq!(header.data_type, ".FIT");
        assert_eq!(header.data_size, 2248);
        assert_eq!(header.crc, 0)
    }
}
