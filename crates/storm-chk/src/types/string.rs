use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::str::from_utf8;
use encoding_rs::EUC_KR;
use encoding_rs::WINDOWS_1252;
use std::borrow::Cow;

// =============================================================================
// CHK String
// =============================================================================

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ChkString {
  inner: [u8],
}

impl ChkString {
  #[inline]
  pub const fn from_slice(slice: &[u8]) -> &Self {
    // SAFETY: ChkString is `repr(transparent)` and borrowed from `slice`.
    unsafe { &*(slice as *const [u8] as *const Self) }
  }

  #[inline]
  pub fn as_any(&self) -> AnyString<'_> {
    AnyString::read(&self.inner)
  }

  pub(crate) fn read<'a, T>(index: usize, offsets: &[T], content: &'a [u8]) -> Option<&'a Self>
  where
    T: Copy + Into<u32>,
  {
    if index == 0 || index > offsets.len() {
      return None;
    }

    let offset: u32 = offsets[index - 1].into();
    let string: &[u8] = &content[offset as usize..];

    if offset != 0 {
      Some(Self::from_slice(Self::read_to_nul(string)))
    } else {
      None
    }
  }

  fn read_to_nul(data: &[u8]) -> &[u8] {
    let mut index: usize = 0;

    while index < data.len() && data[index] != 0 {
      index += 1;
    }

    &data[..index]
  }
}

impl Debug for ChkString {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&self.as_any(), f)
  }
}

impl Display for ChkString {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.as_any(), f)
  }
}

// =============================================================================
// Encoding
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Encoding {
  Utf8,
  Cp949,
  Cp1252,
  Invalid,
}

// =============================================================================
// Any String
// =============================================================================

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnyString<'a> {
  text: Cow<'a, str>,
  kind: Encoding,
}

impl<'a> AnyString<'a> {
  pub(crate) fn read(data: &'a [u8]) -> Self {
    if let Ok(text) = from_utf8(data) {
      Self::new(Encoding::Utf8, Cow::Borrowed(text))
    } else if let (text, _, false) = EUC_KR.decode(data) {
      Self::new(Encoding::Cp949, text)
    } else if let (text, _, false) = WINDOWS_1252.decode(data) {
      Self::new(Encoding::Cp1252, text)
    } else {
      Self::new(Encoding::Invalid, Cow::Borrowed(""))
    }
  }

  const fn new(kind: Encoding, text: Cow<'a, str>) -> Self {
    Self { kind, text }
  }

  #[inline]
  pub const fn text(&self) -> &Cow<'a, str> {
    &self.text
  }

  #[inline]
  pub const fn kind(&self) -> Encoding {
    self.kind
  }

  #[inline]
  pub fn as_str(&self) -> &str {
    self.text()
  }

  #[inline]
  pub fn into_string(self) -> String {
    match self.text {
      Cow::Borrowed(text) => text.to_owned(),
      Cow::Owned(text) => text,
    }
  }
}

impl Debug for AnyString<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("AnyString")
      .field("text", &self.text)
      .field("kind", &self.kind)
      .finish()
  }
}

impl Display for AnyString<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self.as_str(), f)
  }
}
