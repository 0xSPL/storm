use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// WAV String Indexes
// =============================================================================

/// Indicates which index is used for a WAV file in the MPQ.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Wav {
  pub wav: [u32; 0x200],
}

impl From<Wav> for Item {
  #[inline]
  fn from(other: Wav) -> Self {
    Self::Wav(Box::new(other))
  }
}

impl ParseChunk for Wav {
  // TODO: This can be less data than expected
  const TYPE: ChunkType = ChunkType::Sized(0x800);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      wav: reader.read_array_u32()?,
    })
  }
}
