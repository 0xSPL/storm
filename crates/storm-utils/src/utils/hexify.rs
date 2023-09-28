use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::LowerHex;
use core::fmt::Result as FmtResult;
use core::fmt::UpperHex;
use core::fmt::Write;

static ALPHA_LOWER: &[u8; 16] = b"0123456789abcdef";
static ALPHA_UPPER: &[u8; 16] = b"0123456789ABCDEF";

pub struct Hex<'a>(&'a [u8]);

impl<'a> Hex<'a> {
  /// Create a new `Hex`.
  #[inline]
  pub const fn new(data: &'a [u8]) -> Self {
    Self(data)
  }

  fn write<W: Write>(
    &self,
    writer: &mut W,
    pretty: bool,
    alphabet: &'static [u8; 16],
  ) -> FmtResult {
    if pretty {
      writer.write_str("0x")?;
    }

    for byte in self.0 {
      writer.write_char(char::from(alphabet[((byte >> 4) & 0xF) as usize]))?;
      writer.write_char(char::from(alphabet[(byte & 0xF) as usize]))?;
    }

    Ok(())
  }
}

impl Debug for Hex<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.write(f, f.alternate(), ALPHA_LOWER)
  }
}

impl Display for Hex<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.write(f, f.alternate(), ALPHA_LOWER)
  }
}

impl LowerHex for Hex<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.write(f, f.alternate(), ALPHA_LOWER)
  }
}

impl UpperHex for Hex<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.write(f, f.alternate(), ALPHA_UPPER)
  }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Hex<'_> {
  #[inline]
  fn serialize<T: serde::Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
    serializer.collect_str(self)
  }
}
