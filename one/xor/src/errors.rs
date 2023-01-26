use hex::FromHexError;
use std::string::FromUtf8Error;

#[derive(Copy, Clone, Debug)]
pub enum XorError {
    /// The length of two given inputs wasn't equal (e.i for a hamming distance)
    LengthNotEqual,
    /// An I/O error occurred
    IO,
    /// Something went wrong while converting from hex
    HexConversion,
    /// Something went wrong while converting from UTF-8
    Utf8Conversion,
}

pub type XorResult<T> = Result<T, XorError>;

impl From<std::io::Error> for XorError {
    fn from ( _error: std::io::Error ) -> Self {
        Self::IO
    }
}

impl From<FromHexError> for XorError {
    fn from (_error: FromHexError) -> Self {
        Self::HexConversion
    }
}

impl From<FromUtf8Error> for XorError {
    fn from ( _error: FromUtf8Error ) -> Self { 
        Self::Utf8Conversion
    }
}