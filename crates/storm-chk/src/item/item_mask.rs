use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::BoxedSize;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Fog of War Layer
// =============================================================================

/// This section contains the data on fog of war for each player.
///
/// Required for all versions. Not required for Melee.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Mask {
  pub tiles: Box<[u8]>,
}

impl From<Mask> for Item {
  #[inline]
  fn from(other: Mask) -> Self {
    Self::Mask(other)
  }
}

impl ParseChunk for Mask {
  const TYPE: ChunkType = ChunkType::Boxed(BoxedSize::Dyn);

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self> {
    Ok(Self {
      tiles: reader.read_boxed_u8(size as usize)?,
    })
  }
}
