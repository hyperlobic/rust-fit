use std::io::{Read, Seek, SeekFrom};
use crate::byte_order::ByteOrder;

pub trait ReadExt: Read + Seek {
    fn read_u8(&mut self) -> Result<u8, std::io::Error> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_i8(&mut self) -> Result<i8, std::io::Error> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    fn read_u16(&mut self, byte_order: ByteOrder) -> Result<u16, std::io::Error>  {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;

        let result = match byte_order {
            ByteOrder::LitteEndian => u16::from_le_bytes(buf),
            ByteOrder::BigEndian => u16::from_be_bytes(buf),
        };

        Ok(result)
    }

    fn read_i16(&mut self, byte_order: ByteOrder) -> Result<i16, std::io::Error>  {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;

        let result = match byte_order {
            ByteOrder::LitteEndian => i16::from_le_bytes(buf),
            ByteOrder::BigEndian => i16::from_be_bytes(buf),
        };

        Ok(result)
    }

    fn read_u32(&mut self, byte_order: ByteOrder) -> Result<u32, std::io::Error>  {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;

        let result = match byte_order {
            ByteOrder::LitteEndian => u32::from_le_bytes(buf),
            ByteOrder::BigEndian => u32::from_be_bytes(buf),
        };

        Ok(result)
    }

    fn read_i32(&mut self, byte_order: ByteOrder) -> Result<i32, std::io::Error>  {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;

        let result = match byte_order {
            ByteOrder::LitteEndian => i32::from_le_bytes(buf),
            ByteOrder::BigEndian => i32::from_be_bytes(buf),
        };

        Ok(result)
    }

    fn read_f32(&mut self, byte_order: ByteOrder) -> Result<f32, std::io::Error> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;

        let result = match byte_order {
            ByteOrder::LitteEndian => f32::from_le_bytes(buf),
            ByteOrder::BigEndian => f32::from_be_bytes(buf),
        };

        Ok(result)
    }

    fn read_u64(&mut self, byte_order: ByteOrder) -> Result<u64, std::io::Error> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;

        let result = match byte_order {
            ByteOrder::LitteEndian => u64::from_le_bytes(buf),
            ByteOrder::BigEndian => u64::from_be_bytes(buf),
        };

        Ok(result)
    }

    fn read_i64(&mut self, byte_order: ByteOrder) -> Result<i64, std::io::Error> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;

        let result = match byte_order {
            ByteOrder::LitteEndian => i64::from_le_bytes(buf),
            ByteOrder::BigEndian => i64::from_be_bytes(buf),
        };

        Ok(result)
    }

    fn read_f64(&mut self, byte_order: ByteOrder) -> Result<f64, std::io::Error> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;

        let result = match byte_order {
            ByteOrder::LitteEndian => f64::from_le_bytes(buf),
            ByteOrder::BigEndian => f64::from_be_bytes(buf),
        };

        Ok(result)
    }

    fn read_string_null_term(&mut self) -> Result<String, std::io::Error> {
        let mut buf = [0u8; 1];
        let mut str = String::new();

        self.read_exact(&mut buf)?;

        while buf[0] != 0 {
            str.push(buf[0].into());
            self.read_exact(&mut buf)?;
        }

        Ok(str)
    }

    fn read_string_fixed(&mut self, mut size: u32) -> Result<String, std::io::Error> {
        let mut buf = [0u8; 1];
        let mut str = String::new();

        while size > 0 {
            self.read_exact(&mut buf)?;
            str.push(buf[0].into());
            size -= 1;
        }

        Ok(str)
    }

    fn peek_byte(&mut self) -> Result<u8, std::io::Error> {
        let mut buf = [0u8; 1];

        self.read_exact(&mut buf)?;
        self.seek(SeekFrom::Current(-1))?;

        Ok(buf[0])
    }
}

impl<T: Read + Seek> ReadExt for T {}