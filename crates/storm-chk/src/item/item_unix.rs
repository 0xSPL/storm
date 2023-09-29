use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::id::UnitId;
use crate::id::WeaponId;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Brood War Unit Settings
// =============================================================================

/// This section is identical to [`UNIS`][super::Unis] section except it uses
/// the Brood War set of 130 weapons instead of the original 100.
///
/// Required for Hybrid (in Expansion mode) and Brood War. Not required for Melee.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Unix {
  pub defaults: [u8; UnitId::TOTAL],
  pub unit_hp: [u32; UnitId::TOTAL],
  pub unit_sp: [u16; UnitId::TOTAL],
  pub unit_ap: [u8; UnitId::TOTAL],
  pub build_time: [u16; UnitId::TOTAL],
  pub min_cost: [u16; UnitId::TOTAL],
  pub gas_cost: [u16; UnitId::TOTAL],
  pub string: [u16; UnitId::TOTAL],
  pub default_damage: [u16; WeaponId::BROOD_WAR],
  pub boosted_damage: [u16; WeaponId::BROOD_WAR],
}

impl Unix {
  /// Returns `true` if the `unit` uses default settings.
  #[inline]
  pub const fn defaults(&self, unit: UnitId) -> bool {
    self.defaults[unit as usize] == 0x01
  }

  /// Returns the Hit Points for `unit`.
  #[inline]
  pub const fn hp(&self, unit: UnitId) -> u32 {
    self.unit_hp[unit as usize]
  }

  /// Returns the Shield Points for `unit`.
  #[inline]
  pub const fn sp(&self, unit: UnitId) -> u16 {
    self.unit_sp[unit as usize]
  }

  /// Returns the Armor Points for `unit`.
  #[inline]
  pub const fn ap(&self, unit: UnitId) -> u8 {
    self.unit_ap[unit as usize]
  }

  /// Returns the build time (1/60 seconds) for `unit`.
  #[inline]
  pub const fn build_time(&self, unit: UnitId) -> u16 {
    self.build_time[unit as usize]
  }

  /// Returns the mineral cost of `unit`.
  #[inline]
  pub const fn min_cost(&self, unit: UnitId) -> u16 {
    self.min_cost[unit as usize]
  }

  /// Returns the gas cost of `unit`.
  #[inline]
  pub const fn gas_cost(&self, unit: UnitId) -> u16 {
    self.gas_cost[unit as usize]
  }

  /// Returns the string number of `unit`.
  #[inline]
  pub const fn string(&self, unit: UnitId) -> u16 {
    self.string[unit as usize]
  }

  /// Returns the base damage for `weapon`.
  #[inline]
  pub const fn default_damage(&self, weapon: WeaponId) -> u16 {
    self.default_damage[weapon as usize]
  }

  /// Returns the bonus damage for `weapon`.
  #[inline]
  pub const fn boosted_damage(&self, weapon: WeaponId) -> u16 {
    self.boosted_damage[weapon as usize]
  }
}

impl From<Unix> for Item {
  #[inline]
  fn from(other: Unix) -> Self {
    Self::Unix(Box::new(other))
  }
}

impl ParseChunk for Unix {
  const TYPE: ChunkType = ChunkType::Sized(0x1048);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      defaults: reader.read_array_u8()?,
      unit_hp: reader.read_array_u32()?,
      unit_sp: reader.read_array_u16()?,
      unit_ap: reader.read_array_u8()?,
      build_time: reader.read_array_u16()?,
      min_cost: reader.read_array_u16()?,
      gas_cost: reader.read_array_u16()?,
      string: reader.read_array_u16()?,
      default_damage: reader.read_array_u16()?,
      boosted_damage: reader.read_array_u16()?,
    })
  }
}
