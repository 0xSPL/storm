use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::str::from_utf8_unchecked;
use storm_core::error::Error;
use storm_utils::traits::Parse;
use storm_utils::traits::ReadExt;

// =============================================================================
// Locale
// =============================================================================

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Locale {
  inner: [u8; 5],
}

impl Locale {
  #[inline]
  pub const fn as_str(&self) -> &str {
    unsafe { from_utf8_unchecked(&self.inner) }
  }
}

impl Debug for Locale {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(self.as_str(), f)
  }
}

impl Display for Locale {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self.as_str(), f)
  }
}

impl Deref for Locale {
  type Target = str;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_str()
  }
}

impl Parse for Locale {
  type Error = Error;

  fn from_reader<R: ReadExt + ?Sized>(reader: &mut R) -> Result<Self, Self::Error> {
    let ch4: u8 = reader.read_u8()?;
    let ch3: u8 = reader.read_u8()?;
    let ch1: u8 = reader.read_u8()?;
    let ch0: u8 = reader.read_u8()?;

    Ok(Self {
      inner: [ch0, ch1, b'-', ch3, ch4],
    })
  }
}

only_serde! {
  use serde::Serialize;
  use serde::Serializer;

  impl Serialize for Locale {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      self.as_str().serialize(serializer)
    }
  }
}
