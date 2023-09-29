use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::BoxedSize;
use crate::parse::ChunkType;
use crate::parse::ParseChunk;
use crate::types::ChkString;
use crate::types::Item;

// =============================================================================
// String Data
// =============================================================================

/// This section contains all the strings in the map.
///
/// Required for all versions and all game types (or [`STRx`][super::Strx]).
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Str {
  pub entries: u16,
  pub offsets: Box<[u16]>,
  pub content: Box<[u8]>,
}

impl Str {
  #[inline]
  pub fn get(&self, index: usize) -> Option<&ChkString> {
    ChkString::read(index, &self.offsets, &self.content)
  }
}

impl From<Str> for Item {
  #[inline]
  fn from(other: Str) -> Self {
    Self::Str(Box::new(other))
  }
}

impl ParseChunk for Str {
  const TYPE: ChunkType = ChunkType::Boxed(BoxedSize::Dyn);

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self> {
    let entries: u16 = reader.read_u16_le()?;
    let offsets: Box<[u16]> = reader.read_boxed_u16(entries as usize)?;

    // Set cursor according to previously parsed data
    let cursor: u16 = 2 + (entries << 1);

    // Adjust offsets accordingly
    //
    // TODO: Optimistic adjustment to avoid additional allocation
    let offsets: Box<[u16]> = offsets
      .iter()
      .copied()
      .map(|offset| offset - cursor)
      .collect();

    // Read all the string data
    let length: usize = (size - cursor as u32) as usize;
    let content: Box<[u8]> = reader.read_boxed_u8(length)?;

    Ok(Self {
      entries,
      offsets,
      content,
    })
  }
}
