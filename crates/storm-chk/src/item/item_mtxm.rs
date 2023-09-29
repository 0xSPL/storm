use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::BoxedSize;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// StarCraft Terrain
// =============================================================================

/// Terrain section that contains a map of the level's appearance.
///
/// Required for all versions and all game types.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Mtxm {
  pub tiles: Box<[u16]>,
}

impl From<Mtxm> for Item {
  #[inline]
  fn from(other: Mtxm) -> Self {
    Self::Mtxm(other)
  }
}

impl ParseChunk for Mtxm {
  const TYPE: ChunkType = ChunkType::Boxed(BoxedSize::Dyn);

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self> {
    Ok(Self {
      tiles: Self::read_misaligned(reader, size)?,
    })
  }
}
