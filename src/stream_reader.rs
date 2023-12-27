use crate::byte_order::ByteOrder;
use std::io::{Read, Seek, SeekFrom};

pub struct StreamReader<T: Read + Seek> {
    reader: T,
    crc: u16,
}

impl<T: Read + Seek> StreamReader<T> {
    pub fn new(reader: T) -> StreamReader<T> {
        Self { reader, crc: 0 }
    }

    const CRC_TABLE: [u16; 16] =
    [
       0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401,
       0xA001, 0x6C00, 0x7800, 0xB401, 0x5000, 0x9C01, 0x8801, 0x4400
    ];

    fn update_crc(&mut self, bytes: &[u8]) {
        for byte in bytes {
            let mut tmp = Self::CRC_TABLE[(self.crc & 0xF) as usize];
            let mut crc = (self.crc >> 4) & 0x0FFF;
            crc = crc ^ tmp ^ Self::CRC_TABLE[(byte & 0xF) as usize];
        
            // now compute checksum of upper four bits of byte
            tmp = Self::CRC_TABLE[(crc & 0xF) as usize];
            crc = (crc >> 4) & 0x0FFF;
            crc = crc ^ tmp ^ Self::CRC_TABLE[((byte >> 4) & 0xF) as usize];
            self.crc = crc;
        }
    }

    pub fn crc(&self) -> u16 {
        self.crc
    }

    pub(crate) fn read_crc(&mut self) -> Result<u16, std::io::Error> {
        let mut buf = [0u8; 2];

        self.reader.read_exact(&mut buf)?;
        let crc = u16::from_le_bytes(buf);
        
        if crc != self.crc {
            Err(std::io::Error::other("crc check failed"))?
        }

        Ok(crc)
    }

    pub(crate) fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
        self.reader.read_exact(buf)?;
        self.update_crc(&buf);
        Ok(())
    }

    pub(crate) fn stream_position(&mut self) -> Result<u64, std::io::Error> {
        self.reader.stream_position()
    }

    pub(crate) fn peek_byte(&mut self) -> Result<u8, std::io::Error> {
        let mut buf = [0u8; 1];

        self.reader.read_exact(&mut buf)?;
        self.reader.seek(SeekFrom::Current(-1))?;

        Ok(buf[0])
    }

    pub(crate) fn read_u8(&mut self) -> Result<u8, std::io::Error> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        self.update_crc(&buf);

        Ok(buf[0])
    }

    pub(crate) fn read_i8(&mut self) -> Result<i8, std::io::Error> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        self.update_crc(&buf);

        Ok(buf[0] as i8)
    }

    pub(crate) fn read_u16(&mut self, byte_order: ByteOrder) -> Result<u16, std::io::Error> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        self.update_crc(&buf);

        let result = match byte_order {
            ByteOrder::LitteEndian => u16::from_le_bytes(buf),
            ByteOrder::BigEndian => u16::from_be_bytes(buf),
        };

        Ok(result)
    }

    pub(crate) fn read_i16(&mut self, byte_order: ByteOrder) -> Result<i16, std::io::Error> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        self.update_crc(&buf);

        let result = match byte_order {
            ByteOrder::LitteEndian => i16::from_le_bytes(buf),
            ByteOrder::BigEndian => i16::from_be_bytes(buf),
        };

        Ok(result)
    }

    pub(crate) fn read_u32(&mut self, byte_order: ByteOrder) -> Result<u32, std::io::Error> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        self.update_crc(&buf);

        let result = match byte_order {
            ByteOrder::LitteEndian => u32::from_le_bytes(buf),
            ByteOrder::BigEndian => u32::from_be_bytes(buf),
        };

        Ok(result)
    }

    pub(crate) fn read_i32(&mut self, byte_order: ByteOrder) -> Result<i32, std::io::Error> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        self.update_crc(&buf);

        let result = match byte_order {
            ByteOrder::LitteEndian => i32::from_le_bytes(buf),
            ByteOrder::BigEndian => i32::from_be_bytes(buf),
        };

        Ok(result)
    }

    pub(crate) fn read_f32(&mut self, byte_order: ByteOrder) -> Result<f32, std::io::Error> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        self.update_crc(&buf);

        let result = match byte_order {
            ByteOrder::LitteEndian => f32::from_le_bytes(buf),
            ByteOrder::BigEndian => f32::from_be_bytes(buf),
        };

        Ok(result)
    }

    pub(crate) fn read_u64(&mut self, byte_order: ByteOrder) -> Result<u64, std::io::Error> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        self.update_crc(&buf);

        let result = match byte_order {
            ByteOrder::LitteEndian => u64::from_le_bytes(buf),
            ByteOrder::BigEndian => u64::from_be_bytes(buf),
        };

        Ok(result)
    }

    pub(crate) fn read_i64(&mut self, byte_order: ByteOrder) -> Result<i64, std::io::Error> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        self.update_crc(&buf);

        let result = match byte_order {
            ByteOrder::LitteEndian => i64::from_le_bytes(buf),
            ByteOrder::BigEndian => i64::from_be_bytes(buf),
        };

        Ok(result)
    }

    pub(crate) fn read_f64(&mut self, byte_order: ByteOrder) -> Result<f64, std::io::Error> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        self.update_crc(&buf);

        let result = match byte_order {
            ByteOrder::LitteEndian => f64::from_le_bytes(buf),
            ByteOrder::BigEndian => f64::from_be_bytes(buf),
        };

        Ok(result)
    }

    pub(crate) fn read_string_null_term(&mut self, size: usize) -> Result<String, anyhow::Error> {
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .take(size)
            .collect::<Result<Vec<u8>, _>>()?;
        self.update_crc(&buf);
        let actual = buf.into_iter().take_while(|b| *b != 0).collect::<Vec<u8>>();

        let str = String::from_utf8(actual)?;

        Ok(str)
    }

    pub(crate) fn read_string_fixed(&mut self, mut size: u32) -> Result<String, std::io::Error> {
        let mut buf = [0u8; 1];
        let mut str = String::new();

        while size > 0 {
            self.reader.read_exact(&mut buf)?;
            self.update_crc(&buf);

            str.push(buf[0].into());
            size -= 1;
        }

        Ok(str)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_string_null_term_should_read_a_string() {
        let data: [u8; 11] = [97, 98, 99, 100, 49, 50, 51, 52, 122, 113, 0];

        let mut reader = StreamReader::new(Cursor::new(&data));

        let str = reader.read_string_null_term(11).unwrap();
        let pos = reader.stream_position().unwrap();

        assert_eq!(str, "abcd1234zq");
        assert_eq!(pos, 11);
    }

    #[test]
    fn read_string_null_term_should_read_a_string_with_extra_nulls() {
        let data: [u8; 13] = [97, 98, 99, 100, 49, 50, 51, 52, 122, 113, 0, 0, 0];

        let mut reader = StreamReader::new(Cursor::new(&data));

        let str = reader.read_string_null_term(13).unwrap();
        let pos = reader.stream_position().unwrap();

        assert_eq!(str, "abcd1234zq");
        assert_eq!(pos, 13);
    }

    #[test]
    fn read_string_null_term_should_handle_empty_string() {
        let data: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        let mut reader = StreamReader::new(Cursor::new(&data));

        let str = reader.read_string_null_term(9).unwrap();
        let pos = reader.stream_position().unwrap();

        assert_eq!(str, "");
        assert_eq!(pos, 9);
    }
}
