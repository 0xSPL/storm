use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;
use crate::types::Owner;
use crate::types::Player;

// =============================================================================
// StarEdit Player Types
// =============================================================================

/// This section specifies the owner of each player.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Iown {
  pub owner: [Owner; Player::TOTAL],
}

impl From<Iown> for Item {
  #[inline]
  fn from(other: Iown) -> Self {
    Self::Iown(other)
  }
}

impl ParseChunk for Iown {
  const TYPE: ChunkType = ChunkType::Sized(Player::TOTAL as u32);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      owner: reader.read_array_u8()?.map(Owner::from_u8),
    })
  }
}
