use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::consts;
use crate::item::*;
use crate::parse::ParseChunk;
use crate::parse::Parser;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Item {
  // Invalid States
  None,
  Barg(Box<[u8]>),
  // Valid States
  Colr(Colr),
  Crgb(Box<Crgb>),
  Dd2(Dd2),
  Dim(Dim),
  Era(Era),
  Forc(Forc),
  Iown(Iown),
  Isom(Isom),
  Ive2(Ive2),
  Iver(Iver),
  Mask(Mask),
  Mbrf(Mbrf),
  Mrgn(Mrgn),
  Mtxm(Mtxm),
  Ownr(Ownr),
  Ptec(Box<Ptec>),
  Ptex(Box<Ptex>),
  Puni(Box<Puni>),
  Pupx(Box<Pupx>),
  Side(Side),
  Sprp(Sprp),
  Str(Box<Str>),
  Strx(Box<Strx>),
  Swnm(Box<Swnm>),
  Tecs(Box<Tecs>),
  Tecx(Box<Tecx>),
  Thg2(Thg2),
  Tile(Tile),
  Trig(Trig),
  Type(Type),
  Unis(Box<Unis>),
  Unit(Unit),
  Unix(Box<Unix>),
  Upgr(Box<Upgr>),
  Upgs(Box<Upgs>),
  Upgx(Box<Upgx>),
  Uprp(Box<Uprp>),
  Upus(Box<Upus>),
  Vcod(Box<Vcod>),
  Ver(Ver),
  Wav(Box<Wav>),
}

impl Item {
  #[rustfmt::skip]
  pub fn parse(parser: &mut Parser) -> Result<Self> {
    if parser.size == 0 {
      return Ok(Self::None);
    }

    match parser.name {
      consts::MAGIC_COLR => Colr::parse(parser),
      consts::MAGIC_CRGB => Crgb::parse(parser),
      consts::MAGIC_DD2  => Dd2::parse(parser),
      consts::MAGIC_DIM  => Dim::parse(parser),
      consts::MAGIC_ERA  => Era::parse(parser),
      consts::MAGIC_FORC => Forc::parse(parser),
      consts::MAGIC_IOWN => Iown::parse(parser),
      consts::MAGIC_ISOM => Isom::parse(parser),
      consts::MAGIC_IVE2 => Ive2::parse(parser),
      consts::MAGIC_IVER => Iver::parse(parser),
      consts::MAGIC_MASK => Mask::parse(parser),
      consts::MAGIC_MBRF => Mbrf::parse(parser),
      consts::MAGIC_MRGN => Mrgn::parse(parser),
      consts::MAGIC_MTXM => Mtxm::parse(parser),
      consts::MAGIC_OWNR => Ownr::parse(parser),
      consts::MAGIC_PTEC => Ptec::parse(parser),
      consts::MAGIC_PTEX => Ptex::parse(parser),
      consts::MAGIC_PUNI => Puni::parse(parser),
      consts::MAGIC_PUPX => Pupx::parse(parser),
      consts::MAGIC_SIDE => Side::parse(parser),
      consts::MAGIC_SPRP => Sprp::parse(parser),
      consts::MAGIC_STR  => Str::parse(parser),
      consts::MAGIC_STRX => Strx::parse(parser),
      consts::MAGIC_SWNM => Swnm::parse(parser),
      consts::MAGIC_TECS => Tecs::parse(parser),
      consts::MAGIC_TECX => Tecx::parse(parser),
      consts::MAGIC_THG2 => Thg2::parse(parser),
      consts::MAGIC_TILE => Tile::parse(parser),
      consts::MAGIC_TRIG => Trig::parse(parser),
      consts::MAGIC_TYPE => Type::parse(parser),
      consts::MAGIC_UNIS => Unis::parse(parser),
      consts::MAGIC_UNIT => Unit::parse(parser),
      consts::MAGIC_UNIX => Unix::parse(parser),
      consts::MAGIC_UPGR => Upgr::parse(parser),
      consts::MAGIC_UPGS => Upgs::parse(parser),
      consts::MAGIC_UPGX => Upgx::parse(parser),
      consts::MAGIC_UPRP => Uprp::parse(parser),
      consts::MAGIC_UPUS => Upus::parse(parser),
      consts::MAGIC_VCOD => Vcod::parse(parser),
      consts::MAGIC_VER  => Ver::parse(parser),
      consts::MAGIC_WAV  => Wav::parse(parser),
      _ => Self::badarg(parser),
    }
  }

  fn badarg(parser: &mut Parser) -> Result<Self> {
    let size: usize = parser.size as usize;
    let data: Box<[u8]> = parser.reader.read_boxed_u8(size)?;

    Ok(Item::Barg(data))
  }
}

impl Debug for Item {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::None => f.write_str("None"),
      Self::Barg(_) => f.write_str("Barg"),
      Self::Colr(item) => Debug::fmt(item, f),
      Self::Crgb(item) => Debug::fmt(item, f),
      Self::Dd2(item) => Debug::fmt(item, f),
      Self::Dim(item) => Debug::fmt(item, f),
      Self::Era(item) => Debug::fmt(item, f),
      Self::Forc(item) => Debug::fmt(item, f),
      Self::Iown(item) => Debug::fmt(item, f),
      Self::Isom(item) => Debug::fmt(item, f),
      Self::Ive2(item) => Debug::fmt(item, f),
      Self::Iver(item) => Debug::fmt(item, f),
      Self::Mask(item) => Debug::fmt(item, f),
      Self::Mbrf(item) => Debug::fmt(item, f),
      Self::Mrgn(item) => Debug::fmt(item, f),
      Self::Mtxm(item) => Debug::fmt(item, f),
      Self::Ownr(item) => Debug::fmt(item, f),
      Self::Ptec(item) => Debug::fmt(item, f),
      Self::Ptex(item) => Debug::fmt(item, f),
      Self::Puni(item) => Debug::fmt(item, f),
      Self::Pupx(item) => Debug::fmt(item, f),
      Self::Side(item) => Debug::fmt(item, f),
      Self::Sprp(item) => Debug::fmt(item, f),
      Self::Str(item) => Debug::fmt(item, f),
      Self::Strx(item) => Debug::fmt(item, f),
      Self::Swnm(item) => Debug::fmt(item, f),
      Self::Tecs(item) => Debug::fmt(item, f),
      Self::Tecx(item) => Debug::fmt(item, f),
      Self::Thg2(item) => Debug::fmt(item, f),
      Self::Tile(item) => Debug::fmt(item, f),
      Self::Trig(item) => Debug::fmt(item, f),
      Self::Type(item) => Debug::fmt(item, f),
      Self::Unis(item) => Debug::fmt(item, f),
      Self::Unit(item) => Debug::fmt(item, f),
      Self::Unix(item) => Debug::fmt(item, f),
      Self::Upgr(item) => Debug::fmt(item, f),
      Self::Upgs(item) => Debug::fmt(item, f),
      Self::Upgx(item) => Debug::fmt(item, f),
      Self::Uprp(item) => Debug::fmt(item, f),
      Self::Upus(item) => Debug::fmt(item, f),
      Self::Vcod(item) => Debug::fmt(item, f),
      Self::Ver(item) => Debug::fmt(item, f),
      Self::Wav(item) => Debug::fmt(item, f),
    }
  }
}
