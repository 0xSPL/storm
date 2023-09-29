use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Map Dimensions
// =============================================================================

/// This section contains the dimensions of the map.
///
/// Required for all versions and all game types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Dim {
  pub w: u16,
  pub h: u16,
}

impl Dim {
  pub const XS: u16 = 0x0040; // 64
  pub const SM: u16 = 0x0060; // 92
  pub const MD: u16 = 0x0080; // 128
  pub const LG: u16 = 0x00C0; // 192
  pub const XL: u16 = 0x0100; // 256
}

impl From<Dim> for Item {
  #[inline]
  fn from(other: Dim) -> Self {
    Self::Dim(other)
  }
}

impl ParseChunk for Dim {
  const TYPE: ChunkType = ChunkType::Sized(0x4);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      w: reader.read_u16_le()?,
      h: reader.read_u16_le()?,
    })
  }
}
