use hex::FromHexError;

#[derive(Copy, Clone, Debug)]
pub enum SBXorError {
    /// Something went wrong while converting a hexadecimal value
    HexConversion,
    /// Something went wrong during the probability calculation
    ProbabilityCalc,
}

pub type SBXorResult<T> = Result<T, SBXorError>;

impl From<FromHexError> for SBXorError {
    fn from ( _error: FromHexError ) -> Self {
        Self::HexConversion
    }
}

impl From<std::io::Error> for SBXorError {
    fn from ( _error: std::io::Error ) -> Self { 
        Self::ProbabilityCalc
    }
}