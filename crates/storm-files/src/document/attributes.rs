use core::fmt::Debug;
use core::fmt::DebugStruct;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use std::collections::BTreeMap;
use storm_core::error::Error;
use storm_utils::traits::Parse;
use storm_utils::traits::ReadExt;

use crate::document::Locale;

// =============================================================================
// Attributes
// =============================================================================

type Inner = BTreeMap<Locale, BTreeMap<String, String>>;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attributes {
  inner: Inner,
}

impl Attributes {
  #[inline]
  fn new() -> Self {
    Self {
      inner: BTreeMap::new(),
    }
  }
}

impl Debug for Attributes {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let mut debug: DebugStruct<'_, '_> = f.debug_struct("Attributes");

    for (locale, value) in self.iter() {
      debug.field(locale.as_str(), value);
    }

    debug.finish()
  }
}

impl Deref for Attributes {
  type Target = Inner;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl Parse for Attributes {
  type Error = Error;

  fn from_reader<R: ReadExt + ?Sized>(reader: &mut R) -> Result<Self, Self::Error> {
    fn read_string<R: ReadExt + ?Sized>(reader: &mut R) -> Result<String, Error> {
      let size: u16 = reader.read_u16_le()?;
      let data: Box<[u8]> = reader.read_boxed_u8(size as usize)?;
      let data: String = String::from_utf8(data.into())?;

      Ok(data)
    }

    let mut this: Self = Self::new();

    let size: u32 = reader.read_u32_le()?;

    for _ in 0..size {
      let data_k: String = read_string(reader)?;
      let locale: Locale = reader.parse()?;
      let data_v: String = read_string(reader)?;

      this
        .inner
        .entry(locale)
        .or_insert_with(BTreeMap::new)
        .insert(data_k, data_v);
    }

    Ok(this)
  }
}

only_serde! {
  use serde::Serialize;
  use serde::Serializer;

  impl Serialize for Attributes {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      (**self).serialize(serializer)
    }
  }
}
