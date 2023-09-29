use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;
use crate::types::Owner;
use crate::types::Player;

// =============================================================================
// StarCraft Player Types
// =============================================================================

/// This section designates the controller of a particular player.
///
/// It is exactly the same as the [`IOWN`][super::Iown] section, except there is
/// an additional value, `0x00` for Inactive.
///
/// Required for all versions and all game types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Ownr {
  pub owner: [Owner; Player::TOTAL],
}

impl From<Ownr> for Item {
  #[inline]
  fn from(other: Ownr) -> Self {
    Self::Ownr(other)
  }
}

impl ParseChunk for Ownr {
  const TYPE: ChunkType = ChunkType::Sized(Player::TOTAL as u32);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      owner: reader.read_array_u8()?.map(Owner::from_u8),
    })
  }
}
