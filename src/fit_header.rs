use std::{io::Read, cmp::min};
use anyhow::anyhow;

#[derive(Default, Debug)]
pub struct FitHeader {
    header_size: u8,
    protocol_version: u8,
    profile_version: u16,
    data_size: u32,
    data_type: [u8; 4],
    crc: u16
}

pub fn read_header(reader: &mut impl Read) -> Result<FitHeader, anyhow::Error> {
    let mut contents = [0; 14];
    
    reader.read_exact(&mut contents[..=11])?;

    let header_size = contents[0];
    let remaining = header_size - 12;

    if remaining == 2 {
        reader.read_exact(&mut contents[12..(12 + remaining).into()])?;
    } else {
        // discard extra bytes - we don't know what's in here
        let mut buf = vec![0u8; remaining.into()];
        reader.read_exact(&mut buf)?;
    }

    let protocol_version = contents[1];
    let profile_version = u16::from_le_bytes(contents[2..=3].try_into()?);
    let data_size = u32::from_le_bytes(contents[4..=7].try_into()?);
    let data_type: &[u8; 4] = contents[8..=11].try_into()?;
    let crc =  if header_size == 14 {
        u16::from_le_bytes(contents[12..=13].try_into()?)
    } else {
        0
    };
    
    Ok(FitHeader {
        header_size,
        protocol_version,
        profile_version,
        data_size,
        data_type: *data_type,
        crc
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_14_byte_header() {
        let header_bytes: [u8; 14] = [
            0x0E, 0x10, 0x7D, 0x52, 0xC8, 0x08, 0x00, 0x00, 0x2E, 0x46, 0x49, 0x54, 0x91, 0x01
        ];

        use std::io::Cursor;
        let mut buff = Cursor::new(&header_bytes);

        let header = read_header(&mut buff).unwrap();

        assert_eq!(header.header_size, 14);
        assert_eq!(header.data_type[0], '.' as u8);
        assert_eq!(header.data_type[1], 'F' as u8);
        assert_eq!(header.data_type[2], 'I' as u8);
        assert_eq!(header.data_type[3], 'T' as u8);
        assert_eq!(header.data_size, 2248);
        assert_eq!(header.crc, 0x0191);
    }

    #[test]
    fn test_read_12_byte_header() {
        let header_bytes: [u8; 12] = [
            0x0C, 0x10, 0x7D, 0x52, 0xC8, 0x08, 0x00, 0x00, 0x2E, 0x46, 0x49, 0x54
        ];

        use std::io::Cursor;
        let mut buff = Cursor::new(&header_bytes);

        let header = read_header(&mut buff).unwrap();

        assert_eq!(header.header_size, 12);
        assert_eq!(header.data_type[0], '.' as u8);
        assert_eq!(header.data_type[1], 'F' as u8);
        assert_eq!(header.data_type[2], 'I' as u8);
        assert_eq!(header.data_type[3], 'T' as u8);
        assert_eq!(header.data_size, 2248);
        assert_eq!(header.crc, 0)
    }
}

