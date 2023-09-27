use std::io::Cursor;

use crate::traits::ReadExt;

pub trait Parse: Sized {
  type Error;

  fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self, Self::Error>;

  #[inline]
  fn from_slice(slice: &[u8]) -> Result<Self, Self::Error> {
    Self::from_reader(&mut Cursor::new(slice))
  }
}

pub trait ParseContext<Context>: Sized {
  type Error;

  fn from_reader<R: ReadExt>(context: Context, reader: &mut R) -> Result<Self, Self::Error>;

  #[inline]
  fn from_slice(context: Context, slice: &[u8]) -> Result<Self, Self::Error> {
    Self::from_reader(context, &mut Cursor::new(slice))
  }
}
