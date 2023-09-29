use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::id::UpgradeId;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Brood War Upgrade Settings
// =============================================================================

/// This section the same as [`UPGS`][super::Upgs] except section except it uses
/// the Brood War set of 61 upgrades instead of the original 46.
///
/// Required for Hybrid (in Expansion mode) and Brood War. Not required for Melee.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Upgx {
  pub defaults: [u8; UpgradeId::BROOD_WAR],
  pub _padding: u8,
  pub min_cost_initial: [u16; UpgradeId::BROOD_WAR],
  pub min_cost_upgrade: [u16; UpgradeId::BROOD_WAR],
  pub gas_cost_initial: [u16; UpgradeId::BROOD_WAR],
  pub gas_cost_upgrade: [u16; UpgradeId::BROOD_WAR],
  pub time_initial: [u16; UpgradeId::BROOD_WAR],
  pub time_upgrade: [u16; UpgradeId::BROOD_WAR],
}

impl Upgx {
  /// Returns `true` if the `unit` uses default settings.
  #[inline]
  pub const fn defaults(&self, upgrade: UpgradeId) -> bool {
    self.defaults[upgrade as usize] == 0x01
  }

  /// Returns the base mineral cost for each level of `upgrade`.
  #[inline]
  pub const fn min_cost_initial(&self, upgrade: UpgradeId) -> u16 {
    self.min_cost_initial[upgrade as usize]
  }

  /// Returns the mineral cost factor for each level of `upgrade`.
  #[inline]
  pub const fn min_cost_upgrade(&self, upgrade: UpgradeId) -> u16 {
    self.min_cost_upgrade[upgrade as usize]
  }

  /// Returns the base gas cost for each level of `upgrade`.
  #[inline]
  pub const fn gas_cost_initial(&self, upgrade: UpgradeId) -> u16 {
    self.gas_cost_initial[upgrade as usize]
  }

  /// Returns the gas cost factor for each level of `upgrade`.
  #[inline]
  pub const fn gas_cost_upgrade(&self, upgrade: UpgradeId) -> u16 {
    self.gas_cost_upgrade[upgrade as usize]
  }

  /// Returns the base time for each level of `upgrade`.
  #[inline]
  pub const fn time_initial(&self, upgrade: UpgradeId) -> u16 {
    self.time_initial[upgrade as usize]
  }

  /// Returns the time factor for each level of `upgrade`.
  #[inline]
  pub const fn time_upgrade(&self, upgrade: UpgradeId) -> u16 {
    self.time_upgrade[upgrade as usize]
  }
}

impl From<Upgx> for Item {
  #[inline]
  fn from(other: Upgx) -> Self {
    Self::Upgx(Box::new(other))
  }
}

impl ParseChunk for Upgx {
  const TYPE: ChunkType = ChunkType::Sized(0x31A);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      defaults: reader.read_array_u8()?,
      _padding: reader.read_u8()?,
      min_cost_initial: reader.read_array_u16()?,
      min_cost_upgrade: reader.read_array_u16()?,
      gas_cost_initial: reader.read_array_u16()?,
      gas_cost_upgrade: reader.read_array_u16()?,
      time_initial: reader.read_array_u16()?,
      time_upgrade: reader.read_array_u16()?,
    })
  }
}
