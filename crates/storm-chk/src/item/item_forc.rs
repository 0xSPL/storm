use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Force Settings
// =============================================================================

/// This section specifies the forces and the information about them.
///
/// Required for all versions and all game types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Forc {
  pub force: [u8; 0x08],
  pub names: [u16; 0x04],
  pub props: [ForcFlags; 0x04],
}

impl From<Forc> for Item {
  #[inline]
  fn from(other: Forc) -> Self {
    Self::Forc(other)
  }
}

impl ParseChunk for Forc {
  const TYPE: ChunkType = ChunkType::Sized(0x14);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      force: reader.read_array_u8()?,
      names: reader.read_array_u16()?,
      props: reader.read_array_u8()?.map(ForcFlags::from_value),
    })
  }
}

// =============================================================================
// Forc Flags
// =============================================================================

bitflags! {
  #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
  pub struct ForcFlags: u8 {
    /// Random start location.
    const RANDOM = 0x01;
    /// Allies.
    const ALLIES = 0x02;
    /// Allied victory.
    const VICTORY = 0x04;
    /// Shared vision.
    const VISION = 0x08;
  }
}
