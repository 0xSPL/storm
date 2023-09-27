//! Utilities for working with MPQ archives.

pub mod consts;
mod macros;
pub mod traits;
pub mod utils;

// Re-export required for extension macro
#[doc(hidden)]
pub extern crate bitflags;
