use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// CUWP Slots Used
// =============================================================================

/// This section goes along with the [`UPRP`][super::Uprp] section.
///
/// This section just indicates which of the 64 unit properties slot are used.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Upus {
  pub used: [u8; 0x40], // TODO: bitset
}

impl From<Upus> for Item {
  #[inline]
  fn from(other: Upus) -> Self {
    Self::Upus(Box::new(other))
  }
}

impl ParseChunk for Upus {
  const TYPE: ChunkType = ChunkType::Sized(0x40);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      used: reader.read_array_u8()?,
    })
  }
}
