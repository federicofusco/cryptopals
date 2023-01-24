use hex::FromHexError;

#[derive(Copy, Clone, Debug)]
pub enum XorError {
    /// An invalid hex value was given
    HexConversion, 
}

pub type XorResult<T> = Result<T, XorError>;

impl From<FromHexError> for XorError {
    fn from ( _error: FromHexError ) -> Self {
        Self::HexConversion
    }
}