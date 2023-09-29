use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;
use crate::types::Tileset;

// =============================================================================
// Tileset
// =============================================================================

/// This section indicates the tileset of the scenario.
///
/// Required for all versions and all game types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Era {
  pub tileset: Tileset,
}

impl Era {
  /// Returns the name of the tileset.
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.tileset.as_str()
  }
}

impl From<Era> for Item {
  #[inline]
  fn from(other: Era) -> Self {
    Self::Era(other)
  }
}

impl ParseChunk for Era {
  const TYPE: ChunkType = ChunkType::Sized(0x2);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      tileset: Tileset::from_u16(reader.read_u16_le()?),
    })
  }
}
