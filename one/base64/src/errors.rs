use hex::FromHexError;
use std::string::FromUtf8Error;

#[derive(Copy, Clone, Debug)]
pub enum Base64Error {
    /// A lookup in the base64 table failed (invalid index)
    LookupFailed,
    /// Overflowed while iterating through input
    /// Please submit a report if you encounter this, as it should be impossible
    Overflow,
    /// Something went wrong while converting from hex
    HexConversion,
    /// Something went wrong while converting to UTF-8
    Utf8Conversion,
} 

pub type Base64Result<T> = Result<T, Base64Error>;

impl From<FromHexError> for Base64Error {
    fn from (_error: FromHexError) -> Self {
        Self::HexConversion
    }
}

impl From<FromUtf8Error> for Base64Error {
    fn from ( _error: FromUtf8Error ) -> Self { 
        Self::Utf8Conversion
    }
}