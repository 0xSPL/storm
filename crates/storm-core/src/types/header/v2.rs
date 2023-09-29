use core::mem::size_of;
use core::ops::Deref;
use storm_utils::traits::ParseContext;
use storm_utils::traits::ReadExt;

use crate::error::Error;
use crate::types::HeaderV1;

// =============================================================================
// Static Assertions
// =============================================================================

const_assert_size!(HeaderV2, 0x2C);

// =============================================================================
// Header V2
// =============================================================================

/// Archive Header (V2).
///
/// ## Layout
///
/// `0x00` = [`v1`][HeaderV1] \
/// `0x20` = `hi_block_table_offset` \
/// `0x28` = `htable_offset_hi` \
/// `0x2A` = `btable_offset_hi`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeaderV2 {
  /// Base header.
  pub v1: HeaderV1,
  /// Offset to the beginning of the extended block table.
  pub hi_btable_offset: u64,
  /// High 16 bits of the hash table offset for large archives.
  pub htable_offset_hi: u16,
  /// High 16 bits of the block table offset for large archives.
  pub btable_offset_hi: u16,
}

impl HeaderV2 {
  pub(crate) const RUST_PAD: usize = 0x4; // TODO: This is kinda not ideal

  /// The size of a V2 header.
  pub const SIZE: usize = size_of::<Self>() - Self::RUST_PAD;
}

impl Deref for HeaderV2 {
  type Target = HeaderV1;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.v1
  }
}

impl ParseContext<HeaderV1> for HeaderV2 {
  type Error = Error;

  /// Parse a V2 header from the given `reader`.
  fn from_reader<R: ReadExt + ?Sized>(
    context: HeaderV1,
    reader: &mut R,
  ) -> Result<Self, Self::Error> {
    Ok(Self {
      v1: context,
      hi_btable_offset: reader.read_u64_le()?,
      htable_offset_hi: reader.read_u16_le()?,
      btable_offset_hi: reader.read_u16_le()?,
    })
  }
}

only_serde! {
  use serde::__private::ser::FlatMapSerializer;
  use serde::ser::SerializeMap;
  use serde::Serialize;
  use serde::Serializer;

  impl Serialize for HeaderV2 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      let mut state: S::SerializeMap = serializer.serialize_map(Some(12))?;
      self.v1.serialize(FlatMapSerializer(&mut state))?;
      state.serialize_entry("hi_btable_offset", &self.hi_btable_offset)?;
      state.serialize_entry("htable_offset_hi", &self.htable_offset_hi)?;
      state.serialize_entry("btable_offset_hi", &self.btable_offset_hi)?;
      state.end()
    }
  }
}
