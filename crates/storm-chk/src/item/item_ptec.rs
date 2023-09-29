use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::id::TechId;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;
use crate::types::Player;

// =============================================================================
// Tech Restrictions
// =============================================================================

/// This section contains player technology availability restrictions.
///
/// Required for Vanilla and Hybrid (in Original mode). Not required for Melee.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Ptec {
  pub player_allowed: [[u8; TechId::CLASSIC]; Player::TOTAL], // TODO: bitsets
  pub player_studied: [[u8; TechId::CLASSIC]; Player::TOTAL], // TODO: bitsets
  pub global_allowed: [u8; TechId::CLASSIC],                  // TODO: bitset
  pub global_studied: [u8; TechId::CLASSIC],                  // TODO: bitset
  pub global_default: [[u8; TechId::CLASSIC]; Player::TOTAL], // TODO: bitsets
}

impl Ptec {
  /// Returns `true` if `technology` is available for the given `player`.
  #[inline]
  pub const fn is_allowed(&self, player: Player, technology: TechId) -> bool {
    debug_assert!(technology.is_classic());
    self.player_allowed[player.as_usize()][technology as usize] == 0x01
  }

  /// Returns `true` if `technology` is researched for the given `player`.
  #[inline]
  pub const fn is_studied(&self, player: Player, technology: TechId) -> bool {
    debug_assert!(technology.is_classic());
    self.player_studied[player.as_usize()][technology as usize] == 0x01
  }

  /// Returns `true` if `technology` is available by default.
  #[inline]
  pub const fn is_global_allowed(&self, technology: TechId) -> bool {
    debug_assert!(technology.is_classic());
    self.global_allowed[technology as usize] == 0x01
  }

  /// Returns `true` if `technology` is researched by default.
  #[inline]
  pub const fn is_global_studied(&self, technology: TechId) -> bool {
    debug_assert!(technology.is_classic());
    self.global_studied[technology as usize] == 0x01
  }

  /// Returns `true` if `player` uses global defaults for `technology` research.
  #[inline]
  pub const fn is_global_default(&self, player: Player, technology: TechId) -> bool {
    debug_assert!(technology.is_classic());
    self.global_default[player.as_usize()][technology as usize] == 0x01
  }
}

impl From<Ptec> for Item {
  #[inline]
  fn from(other: Ptec) -> Self {
    Self::Ptec(Box::new(other))
  }
}

impl ParseChunk for Ptec {
  const TYPE: ChunkType = ChunkType::Sized(0x390);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      player_allowed: reader.read_array(ReadExt::read_array_u8)?,
      player_studied: reader.read_array(ReadExt::read_array_u8)?,
      global_allowed: reader.read_array_u8()?,
      global_studied: reader.read_array_u8()?,
      global_default: reader.read_array(ReadExt::read_array_u8)?,
    })
  }
}
