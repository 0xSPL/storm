use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::LowerHex;
use core::fmt::Result as FmtResult;
use core::fmt::UpperHex;
use core::str::from_utf8_unchecked;

static ALPHA_LOWER: &[u8; 16] = b"0123456789abcdef";
static ALPHA_UPPER: &[u8; 16] = b"0123456789ABCDEF";

pub type HexSm<'a> = Hex<'a, 32>;
pub type HexMd<'a> = Hex<'a, 64>;
pub type HexLg<'a> = Hex<'a, 128>;
pub type HexXl<'a> = Hex<'a, 256>;

pub struct Hex<'a, const S: usize = 64>(&'a [u8]);

impl<'a, const S: usize> Hex<'a, S> {
  /// Create a new `Hex`.
  #[inline]
  pub const fn new(data: &'a [u8]) -> Self {
    Self(data)
  }

  fn fmt(&self, f: &mut Formatter<'_>, alphabet: &'static [u8; 16]) -> FmtResult {
    let mut buffer: [u8; S] = [0; S];
    let mut bytes: usize = 0;

    for (byte, slots) in self.0.iter().zip(buffer.chunks_exact_mut(2)) {
      slots[0] = alphabet[((byte >> 4) & 0xF) as usize];
      slots[1] = alphabet[(byte & 0xF) as usize];
      bytes += 2;
    }

    if f.alternate() {
      f.write_str("0x")?;
    }

    // Safety: `alphabet` only contains valid ASCII characters
    f.write_str(unsafe { from_utf8_unchecked(&buffer[..bytes]) })
  }
}

impl<const S: usize> Debug for Hex<'_, S> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.fmt(f, ALPHA_LOWER)
  }
}

impl<const S: usize> Display for Hex<'_, S> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.fmt(f, ALPHA_LOWER)
  }
}

impl<const S: usize> LowerHex for Hex<'_, S> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.fmt(f, ALPHA_LOWER)
  }
}

impl<const S: usize> UpperHex for Hex<'_, S> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.fmt(f, ALPHA_UPPER)
  }
}
