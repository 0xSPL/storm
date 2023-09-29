#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Tileset(u16);

impl Tileset {
  #[inline]
  pub const fn from_u16(value: u16) -> Self {
    Self(value)
  }

  #[inline]
  pub const fn as_u16(self) -> u16 {
    self.0
  }

  #[inline]
  pub const fn as_str(&self) -> &'static str {
    const MASK: u16 = 0x0007;

    match self.0 & MASK {
      0x0000 => "Badlands",
      0x0001 => "Space Platform",
      0x0002 => "Installation",
      0x0003 => "Ashworld",
      0x0004 => "Jungle",
      0x0005 => "Desert",
      0x0006 => "Arctic",
      0x0007 => "Twilight",
      _ => unreachable!(),
    }
  }
}

delegate_fmt!(Tileset -> as_str);
