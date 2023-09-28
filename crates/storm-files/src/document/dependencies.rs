use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use storm_core::error::Error;
use storm_utils::traits::Parse;
use storm_utils::traits::ReadExt;

// =============================================================================
// Dependencies
// =============================================================================

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dependencies {
  inner: Vec<String>,
}

impl Dependencies {
  #[inline]
  fn with_capacity(capacity: usize) -> Self {
    Self {
      inner: Vec::with_capacity(capacity),
    }
  }
}

impl Debug for Dependencies {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str("Dependencies ")?;
    f.debug_list().entries(self.iter()).finish()
  }
}

impl Deref for Dependencies {
  type Target = [String];

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl Parse for Dependencies {
  type Error = Error;

  fn from_reader<R: ReadExt + ?Sized>(reader: &mut R) -> Result<Self, Self::Error> {
    let size: u32 = reader.read_u32_le()?;
    let mut this: Self = Self::with_capacity(size as usize);

    for _ in 0..size {
      let mut bytes: Vec<u8> = Vec::with_capacity(0x20);

      loop {
        let byte: u8 = reader.read_u8()?;

        if byte == 0 {
          break;
        }

        bytes.push(byte);
      }

      this.inner.push(String::from_utf8(bytes)?);
    }

    Ok(this)
  }
}
