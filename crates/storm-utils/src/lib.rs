//! Utilities for working with MPQ archives.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod macros;

pub mod consts;
pub mod traits;
pub mod utils;

// Re-export required for extension macro
#[doc(hidden)]
pub extern crate bitflags;
