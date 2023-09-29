use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::id::UnitId;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;
use crate::types::Player;

// =============================================================================
// Player Unit Restrictions
// =============================================================================

/// This section contains player unit restrictions.
///
/// Required for all versions. Not required for Melee.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Puni {
  pub player_allowed: [[u8; UnitId::TOTAL]; Player::TOTAL], // TODO: bitsets
  pub global_allowed: [u8; UnitId::TOTAL],                  // TODO: bitset
  pub global_default: [[u8; UnitId::TOTAL]; Player::TOTAL], // TODO: bitsets
}

impl Puni {
  /// Returns `true` if `unit` if available for production by the given `player`.
  #[inline]
  pub const fn is_allowed(&self, player: Player, unit: UnitId) -> bool {
    self.player_allowed[player.as_usize()][unit as usize] == 0x01
  }

  /// Returns `true` if `unit` if available for production.
  #[inline]
  pub const fn is_global_allowed(&self, unit: UnitId) -> bool {
    self.global_allowed[unit as usize] == 0x01
  }

  /// Returns `true` if `player` uses global defaults for `unit` production.
  #[inline]
  pub const fn is_global_default(&self, player: Player, unit: UnitId) -> bool {
    self.global_default[player.as_usize()][unit as usize] == 0x01
  }
}

impl From<Puni> for Item {
  #[inline]
  fn from(other: Puni) -> Self {
    Self::Puni(Box::new(other))
  }
}

impl ParseChunk for Puni {
  const TYPE: ChunkType = ChunkType::Sized(0x1644);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      player_allowed: reader.read_array(ReadExt::read_array_u8)?,
      global_allowed: reader.read_array_u8()?,
      global_default: reader.read_array(ReadExt::read_array_u8)?,
    })
  }
}
