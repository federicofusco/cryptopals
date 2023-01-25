use hex::FromHexError;
use xor::XorError;

#[derive(Copy, Clone, Debug)]
pub enum SBXorError {
    /// Something went wrong while converting a hexadecimal value
    HexConversion,
    /// Something went wrong during an XOR operation
    XOR,
    /// Something went wrong during the probability calculation
    ProbabilityCalc,
}

pub type SBXorResult<T> = Result<T, SBXorError>;

impl From<FromHexError> for SBXorError {
    fn from ( _error: FromHexError ) -> Self {
        Self::HexConversion
    }
}

impl From<XorError> for SBXorError { 
    fn from ( _error: XorError ) -> Self {
        Self::XOR
    }
}