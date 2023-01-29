#![warn(
    clippy::all,
    clippy::nursery,
    clippy::missing_docs_in_private_items,
    missing_docs
)]

//! An AES crate

/// A module dedicated to handling AES operations
pub mod aes;
/// A module dedicated to handling arithmetic in a GaloisField(256)
mod galois;
