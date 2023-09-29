use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::Item;

// =============================================================================
// Player Colors
// =============================================================================

/// This section indicates what color each player is.
///
/// Required for Brood War only and all game types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Colr {
  pub color: [u8; 0x08],
}

impl Colr {
  pub const RED: u8 = 0x00;
  pub const RED_RGB: [u8; 3] = [0xF4, 0x04, 0x04];

  pub const BLUE: u8 = 0x01;
  pub const BLUE_RGB: [u8; 3] = [0x0C, 0x48, 0xCC];

  pub const TEAL: u8 = 0x02;
  pub const TEAL_RGB: [u8; 3] = [0x2C, 0xB4, 0x94];

  pub const PURPLE: u8 = 0x03;
  pub const PURPLE_RGB: [u8; 3] = [0x88, 0x40, 0x9C];

  pub const ORANGE: u8 = 0x04;
  pub const ORANGE_RGB: [u8; 3] = [0xF8, 0x8C, 0x14];

  pub const BROWN: u8 = 0x05;
  pub const BROWN_RGB: [u8; 3] = [0x70, 0x30, 0x14];

  pub const WHITE: u8 = 0x06;
  pub const WHITE_RGB: [u8; 3] = [0xCC, 0xE0, 0xD0];

  pub const YELLOW: u8 = 0x07;
  pub const YELLOW_RGB: [u8; 3] = [0xFC, 0xFC, 0x38];

  pub const GREEN: u8 = 0x08;
  pub const GREEN_RGB: [u8; 3] = [0x08, 0x80, 0x08];

  pub const PALE_YELLOW: u8 = 0x09;
  pub const PALE_YELLOW_RGB: [u8; 3] = [0xFC, 0xFC, 0x7C];

  pub const TAN: u8 = 0x0A;
  pub const TAN_RGB: [u8; 3] = [0xEC, 0xC4, 0xB0];

  pub const AZURE: u8 = 0x0B;
  pub const AZURE_RGB: [u8; 3] = [0x40, 0x68, 0xD4];
}

impl From<Colr> for Item {
  #[inline]
  fn from(other: Colr) -> Self {
    Self::Colr(other)
  }
}

impl ParseChunk for Colr {
  const TYPE: ChunkType = ChunkType::Sized(0x8);

  fn from_reader<R: ReadExt>(reader: &mut R, _size: u32) -> Result<Self> {
    Ok(Self {
      color: reader.read_array_u8()?,
    })
  }
}
