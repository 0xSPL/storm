use core::mem::size_of;
use core::ops::Deref;
use storm_utils::traits::ParseContext;
use storm_utils::traits::ReadExt;

use crate::error::Error;
use crate::types::HeaderV1;
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

impl ParseContext<HeaderV1> for HeaderV3 {
  type Error = Error;

  /// Parse a V3 header from the given `reader`.
  fn from_reader<R: ReadExt + ?Sized>(
    context: HeaderV1,
    reader: &mut R,
  ) -> Result<Self, Self::Error> {
    HeaderV2::from_reader(context, reader).and_then(|context| reader.parse_context(context))
  }
}

impl ParseContext<HeaderV2> for HeaderV3 {
  type Error = Error;

  /// Parse a V3 header from the given `reader`.
  fn from_reader<R: ReadExt + ?Sized>(
    context: HeaderV2,
    reader: &mut R,
  ) -> Result<Self, Self::Error> {
    Ok(Self {
      v2: context,
      archive_size_64: reader.read_u64_le()?,
      bet_table_position: reader.read_u64_le()?,
      het_table_position: reader.read_u64_le()?,
    })
  }
}

only_serde! {
  use serde::__private::ser::FlatMapSerializer;
  use serde::ser::SerializeMap;
  use serde::Serialize;
  use serde::Serializer;

  impl Serialize for HeaderV3 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      let mut state: S::SerializeMap = serializer.serialize_map(Some(15))?;
      self.v2.serialize(FlatMapSerializer(&mut state))?;
      state.serialize_entry("archive_size_64", &self.archive_size_64)?;
      state.serialize_entry("bet_table_position", &self.bet_table_position)?;
      state.serialize_entry("het_table_position", &self.het_table_position)?;
      state.end()
    }
  }
}
