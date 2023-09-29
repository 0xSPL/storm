use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Map Version
// =============================================================================

/// This section "additionally identifies" the map version.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Ive2 {
  pub version: u16,
}

impl Ive2 {
  /// 1.04 StarCraft and above ("hybrid") or Brood War.
  pub const V1: u16 = 0x000B;
}

impl From<Ive2> for Item {
  #[inline]
  fn from(other: Ive2) -> Self {
    Self::Ive2(other)
  }
}

impl ParseChunk for Ive2 {
  const TYPE: ChunkType = ChunkType::Sized(0x2);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      version: reader.read_u16_le()?,
    })
  }
}
