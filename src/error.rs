use std::{array::TryFromSliceError, str::Utf8Error, string::FromUtf8Error};
use thiserror::Error;
pub type Result<T> = std::result::Result<T, FitError>;

#[derive(Error, Debug)]
pub enum FitError {
    #[error("io error")]
    Io(std::io::Error),
    #[error("parse value error: {0}")]
    ParseValue(String),
    #[error("parse record error: {0}")]
    ParseRecord(String),
    #[error(
        "definition message not found while reading data record at stream pos {stream_pos}, msg type {local_msg_type}"
    )]
    MissingDefinition { stream_pos: u64, local_msg_type: u8 },
    #[error("CRC check failed")]
    Crc,
}

impl From<std::io::Error> for FitError {
    fn from(value: std::io::Error) -> Self {
        FitError::Io(value)
    }
}

impl From<TryFromSliceError> for FitError {
    fn from(value: TryFromSliceError) -> Self {
        FitError::ParseValue(format!("error slicing array: {}", value))
    }
}

impl From<Utf8Error> for FitError {
    fn from(value: Utf8Error) -> Self {
        FitError::ParseValue(format!("error parsing UTF-8 string: {}", value).to_string())
    }
}

impl From<FromUtf8Error> for FitError {
    fn from(value: FromUtf8Error) -> Self {
        FitError::ParseValue(format!("error parsing UTF-8 string: {}", value).to_string())
    }
}
