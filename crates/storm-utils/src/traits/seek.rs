use std::io::Result;
use std::io::Seek;
use std::io::SeekFrom;

pub trait SeekExt: Seek {
  #[inline]
  fn seek(&mut self, from: SeekFrom) -> Result<u64> {
    <Self as Seek>::seek(self, from)
  }

  #[inline]
  fn seek_start(&mut self, offset: u64) -> Result<u64> {
    <Self as Seek>::seek(self, SeekFrom::Start(offset))
  }

  #[inline]
  fn seek_end(&mut self, offset: i64) -> Result<u64> {
    <Self as Seek>::seek(self, SeekFrom::End(offset))
  }

  #[inline]
  fn seek_current(&mut self, offset: i64) -> Result<u64> {
    <Self as Seek>::seek(self, SeekFrom::Current(offset))
  }

  #[inline]
  fn rewind(&mut self) -> Result<()> {
    <Self as Seek>::rewind(self)
  }

  // https://github.com/rust-lang/rust/issues/59359
  #[inline]
  fn stream_len(&mut self) -> Result<u64> {
    let old: u64 = <Self as SeekExt>::stream_position(self)?;
    let len: u64 = <Self as SeekExt>::seek_end(self, 0)?;

    if old != len {
      <Self as SeekExt>::seek_start(self, old)?;
    }

    Ok(len)
  }

  #[inline]
  fn stream_position(&mut self) -> Result<u64> {
    <Self as Seek>::stream_position(self)
  }
}

impl<S: Seek + ?Sized> SeekExt for S {}
