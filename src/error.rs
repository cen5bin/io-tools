use std::str::Utf8Error;
use std::error::Error;

#[derive(Debug)]
pub enum IOErrorCode {
    BufferOverFlow = 1,
    Utf8Error = 2,
}

#[derive(Debug)]
pub struct IOError {
    err_code: IOErrorCode,
    err_msg: String,
}

impl IOError {
    pub fn new(err_code: IOErrorCode, err_msg: &str) -> Self {
        IOError {
            err_code,
            err_msg: err_msg.to_owned(),
        }
    }

    pub fn create_buffer_overflow_err() -> Self {
        Self::new(IOErrorCode::BufferOverFlow, "buffer overflow")
    }
}

pub type IOResult<T> = Result<T, IOError>;

impl From<Utf8Error> for IOError {
    fn from(err: Utf8Error) -> Self {
        IOError::new(IOErrorCode::Utf8Error, err.description())
    }
}