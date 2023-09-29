use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Remastered Player Colors
// =============================================================================

/// Remastered player colors.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Crgb {
  pub colors: [[u8; 0x03]; 0x08],
  pub choice: [u8; 0x08],
}

impl From<Crgb> for Item {
  #[inline]
  fn from(other: Crgb) -> Self {
    Self::Crgb(Box::new(other))
  }
}

impl ParseChunk for Crgb {
  const TYPE: ChunkType = ChunkType::Sized(0x20);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      colors: reader.read_array(ReadExt::read_array_u8)?,
      choice: reader.read_array_u8()?,
    })
  }
}
