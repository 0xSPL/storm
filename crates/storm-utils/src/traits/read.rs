use byteorder::ReadBytesExt;
use byteorder::BE;
use byteorder::LE;
use core::mem::MaybeUninit;
use std::io;
use std::io::Read;

use crate::traits::Parse;
use crate::traits::ParseContext;

pub trait ReadExt: Read {
  #[inline]
  fn read_bytes(&mut self, buffer: &mut [u8]) -> io::Result<()> {
    <Self as Read>::read_exact(self, buffer)
  }

  fn read_all(&mut self, capacity: usize) -> io::Result<Vec<u8>> {
    let mut data: Vec<u8> = Vec::with_capacity(capacity);

    <Self as Read>::read_to_end(self, &mut data)?;

    Ok(data)
  }

  // ===========================================================================
  // Parse Ext.
  // ===========================================================================

  #[inline]
  fn parse<T: Parse>(&mut self) -> Result<T, T::Error> {
    T::from_reader(self)
  }

  #[inline]
  fn parse_context<T: ParseContext<C>, C>(&mut self, context: C) -> Result<T, T::Error> {
    T::from_reader(context, self)
  }

  // ===========================================================================
  // Integer Readers
  // ===========================================================================

  #[inline]
  fn read_u8(&mut self) -> io::Result<u8> {
    ReadBytesExt::read_u8(self)
  }

  #[inline]
  fn read_u16_le(&mut self) -> io::Result<u16> {
    ReadBytesExt::read_u16::<LE>(self)
  }

  #[inline]
  fn read_u32_le(&mut self) -> io::Result<u32> {
    ReadBytesExt::read_u32::<LE>(self)
  }

  #[inline]
  fn read_u64_le(&mut self) -> io::Result<u64> {
    ReadBytesExt::read_u64::<LE>(self)
  }

  #[inline]
  fn read_u16_be(&mut self) -> io::Result<u16> {
    ReadBytesExt::read_u16::<BE>(self)
  }

  #[inline]
  fn read_u32_be(&mut self) -> io::Result<u32> {
    ReadBytesExt::read_u32::<BE>(self)
  }

  #[inline]
  fn read_u64_be(&mut self) -> io::Result<u64> {
    ReadBytesExt::read_u64::<BE>(self)
  }

  // ===========================================================================
  // Array Readers
  // ===========================================================================

  #[inline]
  fn read_array<T, F, const S: usize>(&mut self, f: F) -> io::Result<[T; S]>
  where
    F: Fn(&mut Self) -> io::Result<T>,
  {
    // SAFETY: An uninitialized `[MaybeUninit<T>; S]` is valid.
    let mut array: [MaybeUninit<T>; S] = unsafe { MaybeUninit::zeroed().assume_init() };

    for item in array.iter_mut() {
      item.write(f(self)?);
    }

    // SAFETY: All elements of the array were just initialized
    Ok(array.map(|item| unsafe { MaybeUninit::assume_init(item) }))
  }

  #[inline]
  fn read_array_u8<const S: usize>(&mut self) -> io::Result<[u8; S]> {
    let mut array: [u8; S] = [0; S];

    <Self as Read>::read_exact(self, &mut array)?;

    Ok(array)
  }

  #[inline]
  fn read_array_u16<const S: usize>(&mut self) -> io::Result<[u16; S]> {
    self.read_array(Self::read_u16_le)
  }

  #[inline]
  fn read_array_u32<const S: usize>(&mut self) -> io::Result<[u32; S]> {
    self.read_array(Self::read_u32_le)
  }

  #[inline]
  fn read_array_u64<const S: usize>(&mut self) -> io::Result<[u64; S]> {
    self.read_array(Self::read_u64_le)
  }

  // ===========================================================================
  // Boxed Slice Readers
  // ===========================================================================

  #[inline]
  fn read_boxed_slice<T, F>(&mut self, size: usize, f: F) -> io::Result<Box<[T]>>
  where
    F: Fn(&mut Self) -> io::Result<T>,
  {
    (0..size).map(|_| f(self)).collect()
  }

  #[inline]
  fn read_boxed_u8(&mut self, size: usize) -> io::Result<Box<[u8]>> {
    let mut boxed: Vec<u8> = vec![0; size];

    <Self as Read>::read_exact(self, &mut boxed)?;

    Ok(boxed.into_boxed_slice())
  }

  #[inline]
  fn read_boxed_u16(&mut self, size: usize) -> io::Result<Box<[u16]>> {
    self.read_boxed_slice(size, Self::read_u16_le)
  }

  #[inline]
  fn read_boxed_u32(&mut self, size: usize) -> io::Result<Box<[u32]>> {
    self.read_boxed_slice(size, Self::read_u32_le)
  }

  #[inline]
  fn read_boxed_u64(&mut self, size: usize) -> io::Result<Box<[u64]>> {
    self.read_boxed_slice(size, Self::read_u64_le)
  }
}

impl<R: ReadBytesExt + ?Sized> ReadExt for R {}
