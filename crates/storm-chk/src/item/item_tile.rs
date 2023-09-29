use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::BoxedSize;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// StarEdit Terrain
// =============================================================================

/// This section will only be different from the [`MTXM`][super::Mtxm] section
/// in tiles where doodads are present.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Tile {
  pub tiles: Box<[u16]>,
}

impl From<Tile> for Item {
  #[inline]
  fn from(other: Tile) -> Self {
    Self::Tile(other)
  }
}

impl ParseChunk for Tile {
  const TYPE: ChunkType = ChunkType::Boxed(BoxedSize::Dyn);

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self> {
    Ok(Self {
      tiles: Self::read_misaligned(reader, size)?,
    })
  }
}
