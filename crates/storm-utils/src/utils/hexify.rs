use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::LowerHex;
use core::fmt::Result as FmtResult;
use core::fmt::UpperHex;
use core::fmt::Write;

static ALPHA_LOWER: &[u8; 16] = b"0123456789abcdef";
static ALPHA_UPPER: &[u8; 16] = b"0123456789ABCDEF";

/// Formatter to display byte slices as base-16.
#[repr(transparent)]
pub struct Hex<T: ?Sized = [u8]>(T);

impl Hex {
  /// Create a new `Hex` formatter from a slice of bytes.
  #[inline]
  pub const fn from_slice(data: &[u8]) -> &Self {
    // SAFETY: Hex is `repr(transparent)` and borrowed from `data`.
    unsafe {
      &*(data as *const [u8] as *const Self)
    }
  }
}

impl<T> Hex<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  fn write<W>(
    &self,
    writer: &mut W,
    pretty: bool,
    alphabet: &'static [u8; 16],
  ) -> FmtResult
  where
    W: Write,
  {
    if pretty {
      writer.write_str("0x")?;
    }

    for byte in self.0.as_ref() {
      writer.write_char(char::from(alphabet[((byte >> 4) & 0xF) as usize]))?;
      writer.write_char(char::from(alphabet[(byte & 0xF) as usize]))?;
    }

    Ok(())
  }
}

impl<T> Debug for Hex<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.write(f, f.alternate(), ALPHA_LOWER)
  }
}

impl<T> Display for Hex<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.write(f, f.alternate(), ALPHA_LOWER)
  }
}

impl<T> LowerHex for Hex<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.write(f, f.alternate(), ALPHA_LOWER)
  }
}

impl<T> UpperHex for Hex<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.write(f, f.alternate(), ALPHA_UPPER)
  }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Hex<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  #[inline]
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_str(self)
  }
}
