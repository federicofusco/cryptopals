#[derive(Copy, Clone, Debug)]
pub enum Base64Error {
    /// A lookup in the base64 table failed (invalid index)
    LookupFailed,
    /// The given hex string was invalid
    InvalidHexString,
    /// Overflowed while iterating through input
    /// Please submit a report if you encounter this, as it should be impossible
    Overflow,
}

pub type Base64Result<T> = Result<T, Base64Error>;