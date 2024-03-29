use std::{num::TryFromIntError, string::FromUtf8Error};

#[derive(Copy, Clone, Debug)]
pub enum Base64Error {
    /// A lookup in the base64 table failed (invalid index)
    LookupFailed,
    /// Something went wrong while converting from hex
    HexConversion,
    /// Something went wrong while converting from UTF-8
    Utf8Conversion,
}

pub type Base64Result<T> = Result<T, Base64Error>;

impl From<FromUtf8Error> for Base64Error {
    fn from(_error: FromUtf8Error) -> Self {
        Self::Utf8Conversion
    }
}

impl From<TryFromIntError> for Base64Error {
    fn from(_error: TryFromIntError) -> Self {
        Self::LookupFailed
    }
}
