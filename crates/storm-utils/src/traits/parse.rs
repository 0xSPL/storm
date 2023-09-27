use std::io::Cursor;
use std::io::Result;

use crate::traits::ReadExt;

pub trait Parse: Sized {
  fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self>;

  #[inline]
  fn from_slice(slice: &[u8]) -> Result<Self> {
    Self::from_reader(&mut Cursor::new(slice))
  }
}
