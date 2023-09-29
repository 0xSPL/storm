use core::any::type_name;
use storm_core::error::Error;
use storm_core::error::Result;
use storm_utils::traits::ReadExt;

use crate::parse::Parser;
use crate::types::Item;

type ParseFn<T, R> = fn(reader: &mut R) -> Result<T>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ChunkType {
  Boxed(BoxedSize),
  Sized(u32),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum BoxedSize {
  Dyn,
  Int(u32),
}

fn assert_length_boxed<T>(parser: &Parser, item_size: u32) -> Result<()> {
  if parser.size % item_size != 0 {
    return Err(Error::message(format_args!(
      "Invalid Size: {} - {} % {} != 0",
      type_name::<T>(),
      parser.size,
      item_size
    )));
  }

  Ok(())
}

fn assert_length_sized<T>(parser: &Parser, data_size: u32) -> Result<()> {
  if parser.size != data_size {
    return Err(Error::message(format_args!(
      "Invalid Size: {} - {} != {}",
      type_name::<T>(),
      parser.size,
      data_size
    )));
  }

  Ok(())
}

pub trait ParseChunk: Sized + Into<Item> {
  const TYPE: ChunkType;

  fn from_reader<R: ReadExt>(reader: &mut R, size: u32) -> Result<Self>;

  fn parse(parser: &mut Parser) -> Result<Item> {
    match Self::TYPE {
      ChunkType::Boxed(BoxedSize::Dyn) => {
        Self::from_reader(&mut parser.reader, parser.size).map(Into::into)
      }
      ChunkType::Boxed(BoxedSize::Int(item_size)) => {
        assert_length_boxed::<Self>(parser, item_size)?;
        Self::from_reader(&mut parser.reader, parser.size / item_size).map(Into::into)
      }
      ChunkType::Sized(data_size) => {
        assert_length_sized::<Self>(parser, data_size)?;
        Self::from_reader(&mut parser.reader, parser.size).map(Into::into)
      }
    }
  }

  fn read_boxed<T, R: ReadExt>(
    reader: &mut R,
    length: u32,
    parser: ParseFn<T, R>,
  ) -> Result<Box<[T]>> {
    (0..length).map(|_| parser(reader)).collect()
  }

  fn read_array<T, const N: usize, R: ReadExt>(
    reader: &mut R,
    parser: ParseFn<T, R>,
  ) -> Result<[T; N]> {
    let mut data: Vec<T> = Vec::with_capacity(N);

    for _ in 0..N {
      data.push(parser(reader)?);
    }

    // SAFETY: We always read *exactly* `N` items or fail early.
    Ok(unsafe { data.try_into().unwrap_unchecked() })
  }

  // MPQ Protector Guard
  //
  // Some map protectors create maps with junk ISOM chunks
  fn read_misaligned<R: ReadExt>(reader: &mut R, size: u32) -> Result<Box<[u16]>> {
    let tiles: Box<[u16]> = if size % 2 == 0 {
      reader.read_boxed_u16((size >> 1) as usize)?
    } else {
      let mut base: Vec<u16> = if size == 1 {
        Vec::with_capacity(1)
      } else {
        reader.read_boxed_u16((size >> 1) as usize)?.into()
      };

      base.push(reader.read_u8()? as u16);
      base.into_boxed_slice()
    };

    Ok(tiles)
  }
}
