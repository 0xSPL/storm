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
pub struct Iver {
  pub version: u16,
}

impl Iver {
  /// Beta/Obsolete versions.
  pub const V1: u16 = 0x0009;

  /// Current versions.
  pub const V2: u16 = 0x000A;
}

impl From<Iver> for Item {
  #[inline]
  fn from(other: Iver) -> Self {
    Self::Iver(other)
  }
}

impl ParseChunk for Iver {
  const TYPE: ChunkType = ChunkType::Sized(0x2);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      version: reader.read_u16_le()?,
    })
  }
}
