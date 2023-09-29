use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::id::UpgradeId;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Upgrade Settings
// =============================================================================

/// This section contains upgrade settings.
///
/// Required for Vanilla and Hybrid (in Original mode). Not required for Melee.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Upgs {
  pub defaults: [u8; UpgradeId::CLASSIC],
  pub min_cost_initial: [u16; UpgradeId::CLASSIC],
  pub min_cost_upgrade: [u16; UpgradeId::CLASSIC],
  pub gas_cost_initial: [u16; UpgradeId::CLASSIC],
  pub gas_cost_upgrade: [u16; UpgradeId::CLASSIC],
  pub time_initial: [u16; UpgradeId::CLASSIC],
  pub time_upgrade: [u16; UpgradeId::CLASSIC],
}

impl Upgs {
  /// Returns `true` if the `unit` uses default settings.
  #[inline]
  pub const fn defaults(&self, upgrade: UpgradeId) -> bool {
    debug_assert!(upgrade.is_classic());
    self.defaults[upgrade as usize] == 0x01
  }

  /// Returns the base mineral cost for each level of `upgrade`.
  #[inline]
  pub const fn min_cost_initial(&self, upgrade: UpgradeId) -> u16 {
    debug_assert!(upgrade.is_classic());
    self.min_cost_initial[upgrade as usize]
  }

  /// Returns the mineral cost factor for each level of `upgrade`.
  #[inline]
  pub const fn min_cost_upgrade(&self, upgrade: UpgradeId) -> u16 {
    debug_assert!(upgrade.is_classic());
    self.min_cost_upgrade[upgrade as usize]
  }

  /// Returns the base gas cost for each level of `upgrade`.
  #[inline]
  pub const fn gas_cost_initial(&self, upgrade: UpgradeId) -> u16 {
    debug_assert!(upgrade.is_classic());
    self.gas_cost_initial[upgrade as usize]
  }

  /// Returns the gas cost factor for each level of `upgrade`.
  #[inline]
  pub const fn gas_cost_upgrade(&self, upgrade: UpgradeId) -> u16 {
    debug_assert!(upgrade.is_classic());
    self.gas_cost_upgrade[upgrade as usize]
  }

  /// Returns the base time for each level of `upgrade`.
  #[inline]
  pub const fn time_initial(&self, upgrade: UpgradeId) -> u16 {
    debug_assert!(upgrade.is_classic());
    self.time_initial[upgrade as usize]
  }

  /// Returns the time factor for each level of `upgrade`.
  #[inline]
  pub const fn time_upgrade(&self, upgrade: UpgradeId) -> u16 {
    debug_assert!(upgrade.is_classic());
    self.time_upgrade[upgrade as usize]
  }
}

impl From<Upgs> for Item {
  #[inline]
  fn from(other: Upgs) -> Self {
    Self::Upgs(Box::new(other))
  }
}

impl ParseChunk for Upgs {
  const TYPE: ChunkType = ChunkType::Sized(0x256);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      defaults: reader.read_array_u8()?,
      min_cost_initial: reader.read_array_u16()?,
      min_cost_upgrade: reader.read_array_u16()?,
      gas_cost_initial: reader.read_array_u16()?,
      gas_cost_upgrade: reader.read_array_u16()?,
      time_initial: reader.read_array_u16()?,
      time_upgrade: reader.read_array_u16()?,
    })
  }
}
