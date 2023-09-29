use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::BoxedSize;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Triggers
// =============================================================================

/// This section contains all the triggers in the map.
///
/// Required for all versions. Not required for Melee.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Trig {
  pub triggers: Box<[TrigData]>,
}

impl From<Trig> for Item {
  #[inline]
  fn from(other: Trig) -> Self {
    Self::Trig(other)
  }
}

impl ParseChunk for Trig {
  const TYPE: ChunkType = ChunkType::Boxed(BoxedSize::Int(0x960));

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self> {
    Ok(Self {
      triggers: Self::read_boxed(reader, size, TrigData::from_reader)?,
    })
  }
}

// =============================================================================
// Trigger Data
// =============================================================================

// TODO: bitflags
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TrigData {
  /// Conditions.
  pub conditions: [TrigCondition; 0x10],
  /// Actions.
  pub actions: [TrigAction; 0x40],
  /// Execution flags.
  pub execution: u32,
  /// The player(s) this trigger applies to.
  pub players: [u8; 0x1B],
  /// Index of the current action.
  pub current: u8,
}

impl TrigData {
  pub fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self> {
    Ok(Self {
      conditions: Trig::read_array(reader, TrigCondition::from_reader)?,
      actions: Trig::read_array(reader, TrigAction::from_reader)?,
      execution: reader.read_u32_le()?,
      players: reader.read_array_u8()?,
      current: reader.read_u8()?,
    })
  }
}

// =============================================================================
// Trigger Condition
// =============================================================================

// TODO: bitflags
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TrigCondition {
  /// Location number for the condition.
  pub location: u32,
  /// Group that the condition applies to.
  pub group: u32,
  /// Qualified number (how many/resource amount).
  pub count: u32,
  /// Unit this condition applies to.
  pub unit_id: u16,
  /// Numeric comparison/switch state
  pub state: u8,
  /// Condition type.
  pub condition: u8,
  /// Resource type/Score type/Switch number.
  pub kind: u8,
  /// Condition Flags.
  pub flags: u8,
  /// Mask flag.
  pub mask: u16,
}

impl TrigCondition {
  pub fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self> {
    Ok(Self {
      location: reader.read_u32_le()?,
      group: reader.read_u32_le()?,
      count: reader.read_u32_le()?,
      unit_id: reader.read_u16_le()?,
      state: reader.read_u8()?,
      condition: reader.read_u8()?,
      kind: reader.read_u8()?,
      flags: reader.read_u8()?,
      mask: reader.read_u16_le()?,
    })
  }
}

// =============================================================================
// Trigger Action
// =============================================================================

// TODO: bitflags
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TrigAction {
  /// Action location (source/destination).
  pub location: u32,
  /// String number for trigger text.
  pub txt_index: u32,
  /// WAV string number.
  pub wav_index: u32,
  /// Seconds/milliseconds of time.
  pub time: u32,
  /// First (or only) Group/Player affected.
  pub group: u32,
  /// Trigger-specific property.
  ///
  /// ## Possible Values
  ///   Second group affected \
  ///   Secondary location (1-based) \
  ///   CUWP # \
  ///   Number \
  ///   AI script (4-byte string) \
  ///   Switch (0-based #)
  pub state: u32,
  /// Unit type, score type, resource type, alliance status.
  pub kind: u16,
  /// Action type.
  pub action: u8,
  /// Number of units.
  pub units: u8,
  /// Flags.
  pub flags: u8,
  /// Unused (?)
  pub _padding: u8,
  /// Mask flag.
  pub mask: u16,
}

impl TrigAction {
  pub fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self> {
    Ok(Self {
      location: reader.read_u32_le()?,
      txt_index: reader.read_u32_le()?,
      wav_index: reader.read_u32_le()?,
      time: reader.read_u32_le()?,
      group: reader.read_u32_le()?,
      state: reader.read_u32_le()?,
      kind: reader.read_u16_le()?,
      action: reader.read_u8()?,
      units: reader.read_u8()?,
      flags: reader.read_u8()?,
      _padding: reader.read_u8()?,
      mask: reader.read_u16_le()?,
    })
  }
}
