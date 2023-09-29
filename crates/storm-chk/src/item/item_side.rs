use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;
use crate::types::Player;
use crate::types::Race;

// =============================================================================
// Player Races
// =============================================================================

/// This section contains the species/race of each player.
///
/// Required for all versions and all game types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Side {
  pub species: [Race; Player::TOTAL],
}

impl From<Side> for Item {
  #[inline]
  fn from(other: Side) -> Self {
    Self::Side(other)
  }
}

impl ParseChunk for Side {
  const TYPE: ChunkType = ChunkType::Sized(Player::TOTAL as u32);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      species: reader.read_array_u8()?.map(Race::from_u8),
    })
  }
}
