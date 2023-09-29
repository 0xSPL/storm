use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use std::io::Cursor;
use storm_core::error::Result;
use storm_core::types::Magic;
use storm_utils::traits::ReadExt;

use crate::consts::MAGIC_NONE;
use crate::types::AnyString;
use crate::types::Chunk;
use crate::types::Item;

pub struct Parser {
  pub(crate) reader: Cursor<Vec<u8>>,
  pub(crate) name: Magic,
  pub(crate) size: u32,
}

impl Parser {
  #[inline]
  pub fn new(input: Vec<u8>) -> Self {
    Self {
      reader: Cursor::new(input),
      name: MAGIC_NONE,
      size: 0,
    }
  }

  pub fn read(&mut self) -> Result<Chunk> {
    let name: [u8; 4] = self.reader.read_array_u8()?;

    self.name = unsafe { Magic::new_unchecked(name) };
    self.size = self.reader.read_u32_le()?;

    Ok(Chunk {
      name: self.name,
      size: self.size,
      item: Item::parse(self)?,
    })
  }

  #[inline]
  pub fn read_all<T>(self) -> Result<T>
  where
    T: FromIterator<Chunk>,
  {
    self.collect()
  }
}

impl Debug for Parser {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("Parser")
      .field("name", &AnyString::read(&self.name[..]))
      .field("size", &self.size)
      .finish_non_exhaustive()
  }
}

impl Iterator for Parser {
  type Item = Result<Chunk>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    if self.reader.get_ref().is_empty() {
      return None;
    }

    Some(self.read())
  }
}
