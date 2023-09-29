use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Scenario Properties
// =============================================================================

/// Scenario properties.
///
/// Required for all versions and all game types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Sprp {
  pub name: u16,
  pub desc: u16,
}

impl From<Sprp> for Item {
  #[inline]
  fn from(other: Sprp) -> Self {
    Self::Sprp(other)
  }
}

impl ParseChunk for Sprp {
  const TYPE: ChunkType = ChunkType::Sized(0x4);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      name: reader.read_u16_le()?,
      desc: reader.read_u16_le()?,
    })
  }
}
