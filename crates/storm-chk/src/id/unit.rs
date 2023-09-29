// TODO: All of this
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum UnitId {
  U1 = 0,
}

impl UnitId {
  pub const TOTAL: usize = 0xE4;

  pub const fn is_classic(&self) -> bool {
    panic!("UnitId::is_classic")
  }
}
