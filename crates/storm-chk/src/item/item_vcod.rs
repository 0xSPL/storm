use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Verification Code
// =============================================================================

/// This section has a verification code to make sure this is actually a CHK
/// file.
///
/// Required for all versions and all game types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Vcod {
  pub seed: [u32; 0x100],
  pub code: [u8; 0x10],
}

impl From<Vcod> for Item {
  #[inline]
  fn from(other: Vcod) -> Self {
    Self::Vcod(Box::new(other))
  }
}

impl ParseChunk for Vcod {
  const TYPE: ChunkType = ChunkType::Sized(0x410);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      seed: reader.read_array_u32()?,
      code: reader.read_array_u8()?,
    })
  }
}
