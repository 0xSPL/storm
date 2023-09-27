use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::str::from_utf8_unchecked;
use storm_utils::traits::Parse;
use storm_utils::traits::ReadExt;
use storm_utils::utils::Hex;

use crate::consts::MAGIC_BET;
use crate::consts::MAGIC_HET;
use crate::consts::MAGIC_ID;
use crate::consts::MAGIC_SIGN;
use crate::consts::MAGIC_UD;
use crate::error::Error;
use crate::error::ErrorKind;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Magic([u8; 4]);

impl Magic {
  /// MPQ header identifier.
  pub const ID: Self = Self(MAGIC_ID);

  /// User data identifier.
  pub const UD: Self = Self(MAGIC_UD);

  /// HET table identifier.
  pub const HET: Self = Self(MAGIC_HET);

  /// BET table identifier.
  pub const BET: Self = Self(MAGIC_BET);

  /// Signature block identifier.
  pub const SIGN: Self = Self(MAGIC_SIGN);

  /// Create a new `Magic` structure with no validation checks.
  ///
  /// # Safety
  ///
  /// Caller guarantees that `magic` is valid UTF-8 bytes.
  #[inline]
  pub const unsafe fn new_unchecked(magic: [u8; 4]) -> Self {
    Self(magic)
  }

  /// Returns the UTF-8 representation of this magic signature.
  #[inline]
  pub const fn as_str(&self) -> &str {
    // SAFETY: We only parse valid UTF-8
    unsafe { from_utf8_unchecked(&self.0) }
  }

  /// Returns a `Hex` formatter for the magic signature.
  #[inline]
  pub const fn as_hex(&self) -> Hex<'_, 8> {
    Hex::new(&self.0)
  }

  #[inline]
  const fn known(magic: [u8; 4]) -> bool {
    matches!(
      magic,
      MAGIC_ID | MAGIC_UD | MAGIC_HET | MAGIC_BET | MAGIC_SIGN,
    )
  }
}

impl Debug for Magic {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(self.as_str(), f)
  }
}

impl Display for Magic {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self.as_str(), f)
  }
}

impl Deref for Magic {
  type Target = [u8; 4];

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl AsRef<[u8]> for Magic {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    &self.0
  }
}

impl Parse for Magic {
  type Error = Error;

  /// Parse a magic signature from the given `reader`.
  fn from_reader<R: ReadExt>(reader: &mut R) -> Result<Self, Self::Error> {
    let magic: [u8; 4] = reader.read_array_u8()?;

    if Self::known(magic) {
      // SAFETY: Well-known magic is valid UTF-8.
      return Ok(unsafe { Self::new_unchecked(magic) });
    }

    Err(Error::new(ErrorKind::InvalidMagic))
  }
}
