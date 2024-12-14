use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum GMIError{
    NotSupported,
    CorruptedMediaFile,
    IoError(std::io::Error),
}

impl Error for GMIError{}

impl fmt::Display for GMIError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use self::GMIError::*;
        match self {
            NotSupported => f.write_str("Could not parse this file"),
            CorruptedMediaFile => f.write_str("Hit end of file without finding something"),
            IoError(e) => std::fmt::Display::fmt(&e, f)
        }
    }
}

impl From<std::io::Error> for GMIError{
    fn from(value: std::io::Error) -> Self {
        GMIError::IoError(value)
    }
}

pub type GMIResult<T> = Result<T, GMIError>;