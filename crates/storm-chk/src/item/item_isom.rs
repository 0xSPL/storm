use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::BoxedSize;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Isometric Terrain
// =============================================================================

/// This section is required to place isometric terrain on the map.
///
/// It provides data about the nature of the isometrical "diamonds".
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Isom {
  pub tiles: Box<[u16]>,
}

impl From<Isom> for Item {
  #[inline]
  fn from(other: Isom) -> Self {
    Self::Isom(other)
  }
}

impl ParseChunk for Isom {
  const TYPE: ChunkType = ChunkType::Boxed(BoxedSize::Dyn);

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self> {
    Ok(Self {
      tiles: Self::read_misaligned(reader, size)?,
    })
  }
}
