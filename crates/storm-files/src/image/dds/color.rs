// =============================================================================
// Color Type
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ColorType {
  A8R8G8B8,
}

impl ColorType {
  pub const fn bytes_per_pixel(&self) -> u8 {
    match self {
      Self::A8R8G8B8 => 4,
    }
  }
}
