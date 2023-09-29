use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::id::UpgradeId;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;
use crate::types::Player;

// =============================================================================
// Upgrade Restrictions
// =============================================================================

/// This section contains player upgrade restrictions: it indicates the
/// starting/maximum levels at/to which a player can perform a particular
/// upgrade.
///
/// Required for Vanilla and Hybrid (in Original mode). Not required for Melee.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Upgr {
  pub player_maximum: [[u8; UpgradeId::CLASSIC]; Player::TOTAL],
  pub player_initial: [[u8; UpgradeId::CLASSIC]; Player::TOTAL],
  pub global_maximum: [u8; UpgradeId::CLASSIC],
  pub global_initial: [u8; UpgradeId::CLASSIC],
  pub global_default: [[u8; UpgradeId::CLASSIC]; Player::TOTAL],
}

impl Upgr {
  /// Returns the maximum level of `upgrade` for `player`.
  #[inline]
  pub const fn player_maximum(&self, player: Player, upgrade: UpgradeId) -> u8 {
    debug_assert!(upgrade.is_classic());
    self.player_maximum[player.as_usize()][upgrade as usize]
  }

  /// Returns the starting level of `upgrade` for `player`.
  #[inline]
  pub const fn player_initial(&self, player: Player, upgrade: UpgradeId) -> u8 {
    debug_assert!(upgrade.is_classic());
    self.player_initial[player.as_usize()][upgrade as usize]
  }

  /// Returns the global maximum level of `upgrade`.
  #[inline]
  pub const fn global_maximum(&self, upgrade: UpgradeId) -> u8 {
    debug_assert!(upgrade.is_classic());
    self.global_maximum[upgrade as usize]
  }

  /// Returns the global starting level of `upgrade`.
  #[inline]
  pub const fn global_initial(&self, upgrade: UpgradeId) -> u8 {
    debug_assert!(upgrade.is_classic());
    self.global_initial[upgrade as usize]
  }

  /// Returns `true` if `player` uses global defaults for `upgrade`.
  #[inline]
  pub const fn global_default(&self, player: Player, upgrade: UpgradeId) -> bool {
    debug_assert!(upgrade.is_classic());
    self.global_default[player.as_usize()][upgrade as usize] == 0x01
  }
}

impl From<Upgr> for Item {
  #[inline]
  fn from(other: Upgr) -> Self {
    Self::Upgr(Box::new(other))
  }
}

impl ParseChunk for Upgr {
  const TYPE: ChunkType = ChunkType::Sized(0x6D4);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      player_maximum: reader.read_array(ReadExt::read_array_u8)?,
      player_initial: reader.read_array(ReadExt::read_array_u8)?,
      global_maximum: reader.read_array_u8()?,
      global_initial: reader.read_array_u8()?,
      global_default: reader.read_array(ReadExt::read_array_u8)?,
    })
  }
}
