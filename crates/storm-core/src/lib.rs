//! Core tools for working with MPQ archives.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
extern crate storm_utils;

pub mod consts;
pub mod error;
pub mod extract;
pub mod parse;
pub mod traits;
pub mod types;
pub mod utils;
