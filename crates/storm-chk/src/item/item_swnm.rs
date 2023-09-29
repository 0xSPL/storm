use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Switch Names
// =============================================================================

/// This section contains the strings used for each switch.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Swnm {
  pub switches: [u32; 0x100],
}

impl From<Swnm> for Item {
  #[inline]
  fn from(other: Swnm) -> Self {
    Self::Swnm(Box::new(other))
  }
}

impl ParseChunk for Swnm {
  const TYPE: ChunkType = ChunkType::Sized(0x400);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      switches: reader.read_array_u32()?,
    })
  }
}
