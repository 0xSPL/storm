#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct Owner(u8);

impl Owner {
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
      0x00 => "Inactive",
      0x01 => "Computer (game)",
      0x02 => "Human (occupied)",
      0x03 => "Rescue Passive",
      0x04 => "Unused",
      0x05 => "Computer",
      0x06 => "Human (open slot)",
      0x07 => "Neutral",
      0x08 => "Closed Slot",
      _ => "Unknown",
    }
  }
}

delegate_fmt!(Owner -> as_str);
