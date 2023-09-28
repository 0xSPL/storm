//! PkLib compression for storm.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

mod error;
mod safety;

pub use self::error::Error;
pub use self::error::ErrorKind;
pub use self::safety::Compress;
pub use self::safety::Decompress;
