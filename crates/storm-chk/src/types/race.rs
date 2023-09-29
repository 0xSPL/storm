#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Race(u8);

impl Race {
  #[inline]
  pub const fn from_u8(value: u8) -> Self {
    Self(value)
  }

  #[inline]
  pub const fn as_u8(self) -> u8 {
    self.0
  }

  #[inline]
  pub const fn as_str(&self) -> &'static str {
    match self.0 {
      0x00 => "Zerg",
      0x01 => "Terran",
      0x02 => "Protoss",
      0x03 => "Invalid (Independent)",
      0x04 => "Invalid (Neutral)",
      0x05 => "User Select",
      0x06 => "Random",
      0x07 => "Inactive",
      _ => "Unknown",
    }
  }
}

delegate_fmt!(Race -> as_str);
