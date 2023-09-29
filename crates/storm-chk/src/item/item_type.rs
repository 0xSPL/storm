use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Map Type
// =============================================================================

/// This section specifies the type of scenario.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Type {
  pub scenario: u32,
}

impl Type {
  /// 1.04 StarCraft and above ("hybrid")
  pub const SWAR: u32 = 0x53574152;

  /// Brood War
  pub const BWAR: u32 = 0x42574152;
}

impl From<Type> for Item {
  #[inline]
  fn from(other: Type) -> Self {
    Self::Type(other)
  }
}

impl ParseChunk for Type {
  const TYPE: ChunkType = ChunkType::Sized(0x4);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      scenario: reader.read_u32_le()?,
    })
  }
}
