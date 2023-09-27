use core::mem::size_of;
use core::ops::Deref;
use storm_utils::traits::ParseContext;
use storm_utils::traits::ReadExt;

use crate::error::Error;
use crate::types::HeaderV2;

// =============================================================================
// Static Assertions
// =============================================================================

const_assert_size!(HeaderV3, 0x44);

// =============================================================================
// Header V3
// =============================================================================

/// Archive Header (V3).
///
/// ## Layout
///
/// `0x00` = [`v2`][HeaderV2] \
/// `0x34` = `bet_table_position` \
/// `0x3C` = `het_table_position`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeaderV3 {
  /// Base header.
  pub v2: HeaderV2,
  /// 64-bit version of [`archive_size`][HeaderV1::archive_size].
  pub archive_size_64: u64,
  /// Position of the BET table.
  pub bet_table_position: u64,
  /// Position of the HET table.
  pub het_table_position: u64,
}

impl HeaderV3 {
  pub(crate) const RUST_PAD: usize = HeaderV2::RUST_PAD; // TODO: This is kinda not ideal

  /// The size of a V3 header.
  pub const SIZE: usize = size_of::<Self>() - Self::RUST_PAD;
}

impl Deref for HeaderV3 {
  type Target = HeaderV2;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.v2
  }
}

impl ParseContext for HeaderV3 {
  type Context = HeaderV2;
  type Error = Error;

  /// Parse a V3 header from the given `reader`.
  fn from_reader<R: ReadExt>(context: Self::Context, reader: &mut R) -> Result<Self, Self::Error> {
    Ok(Self {
      v2: context,
      archive_size_64: reader.read_u64_le()?,
      bet_table_position: reader.read_u64_le()?,
      het_table_position: reader.read_u64_le()?,
    })
  }
}
