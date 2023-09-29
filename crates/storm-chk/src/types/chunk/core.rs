use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use storm_core::types::Magic;

use crate::types::AnyString;
use crate::types::Item;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Chunk {
  pub name: Magic,
  pub size: u32,
  pub item: Item,
}

impl Debug for Chunk {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("Chunk")
      .field("name", &AnyString::read(&self.name[..]))
      .field("size", &self.size)
      .field("item", &self.item)
      .finish()
  }
}
