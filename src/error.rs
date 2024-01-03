use std::{array::TryFromSliceError, str::Utf8Error, string::FromUtf8Error};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(std::io::Error),
    #[error("parse value error: {0}")]
    ParseValue(String),
    #[error("parse record error: {0}")]
    ParseRecord(String),
    #[error("unable to find a record")]
    ReadRecord(String),
    #[error(
        "definition message not found while reading data record at stream pos {stream_pos}, msg type {local_msg_type}"
    )]
    MissingDefinition { stream_pos: u64, local_msg_type: u8 },
    #[error("CRC check failed")]
    Crc,
    #[error("invalid file type")]
    InvalidFile,
    #[error("end of file")]
    Eof,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<TryFromSliceError> for Error {
    fn from(value: TryFromSliceError) -> Self {
        Error::ParseValue(format!("error slicing array: {}", value))
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Error::ParseValue(format!("error parsing UTF-8 string: {}", value).to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::ParseValue(format!("error parsing UTF-8 string: {}", value).to_string())
    }
}
