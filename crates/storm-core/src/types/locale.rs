use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Locale(u16);

impl Locale {
  pub const NEUTRAL: Self = Self::from_u16(0x000);
  pub const CHINESE: Self = Self::from_u16(0x404);
  pub const CZECH: Self = Self::from_u16(0x405);
  pub const GERMAN: Self = Self::from_u16(0x407);
  pub const ENGLISH: Self = Self::from_u16(0x409);
  pub const SPANISH: Self = Self::from_u16(0x40A);
  pub const FRENCH: Self = Self::from_u16(0x40C);
  pub const ITALIAN: Self = Self::from_u16(0x410);
  pub const JAPANESE: Self = Self::from_u16(0x411);
  pub const KOREN: Self = Self::from_u16(0x412);
  pub const DUTCH: Self = Self::from_u16(0x413);
  pub const POLISH: Self = Self::from_u16(0x415);
  pub const PORTUGUESE: Self = Self::from_u16(0x416);
  pub const RUSSIAN: Self = Self::from_u16(0x419);
  pub const ENGLISH_UK: Self = Self::from_u16(0x809);

  /// Convert a [`u16`] value into a `Locale`.
  #[inline]
  pub const fn from_u16(value: u16) -> Self {
    Self(value)
  }

  /// Convert the `Locale` into a [`u16`].
  #[inline]
  pub const fn into_u16(self) -> u16 {
    self.0
  }

  #[inline]
  pub const fn as_str(&self) -> &'static str {
    match *self {
      Self::NEUTRAL => "Neutral",
      Self::CHINESE => "Chinese",
      Self::CZECH => "Czech",
      Self::GERMAN => "German",
      Self::ENGLISH => "English",
      Self::SPANISH => "Spanish",
      Self::FRENCH => "French",
      Self::ITALIAN => "Italian",
      Self::JAPANESE => "Japanese",
      Self::KOREN => "Koren",
      Self::DUTCH => "Dutch",
      Self::POLISH => "Polish",
      Self::PORTUGUESE => "Portuguese",
      Self::RUSSIAN => "Russian",
      Self::ENGLISH_UK => "English UK",
      _ => "Unknown",
    }
  }
}

impl From<u16> for Locale {
  #[inline]
  fn from(other: u16) -> Self {
    Self::from_u16(other)
  }
}

impl From<Locale> for u16 {
  #[inline]
  fn from(other: Locale) -> Self {
    other.into_u16()
  }
}

impl Debug for Locale {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(self.as_str(), f)
  }
}

impl Display for Locale {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Display::fmt(self.as_str(), f)
  }
}
