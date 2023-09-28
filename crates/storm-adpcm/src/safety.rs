use core::ffi::c_int;
use core::ffi::c_void;

use crate::error::Error;
use crate::error::ErrorKind;

mod ffi {
  #![allow(dead_code)]
  #![allow(non_snake_case)]
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// =============================================================================
// Stream
// =============================================================================

struct Stream {
  bytes_src: usize,
  bytes_dst: usize,
  channels: usize,
}

impl Stream {
  #[inline]
  const fn new(channels: usize) -> Self {
    Self {
      bytes_src: 0,
      bytes_dst: 0,
      channels,
    }
  }

  fn compress(&mut self, input: &[u8], output: &mut [u8], comp_type: i32) -> c_int {
    let (in_ptr, in_len): (*mut c_void, c_int) = self.input(input);
    let (out_ptr, out_len): (*mut c_void, c_int) = self.output(output);

    let result: c_int = unsafe {
      ffi::CompressADPCM(
        out_ptr,
        out_len,
        in_ptr,
        in_len,
        comp_type,
        self.channels_c(),
      )
    };

    if result != 0 {
      self.bytes_src = input.len();
      self.bytes_dst = result as usize;
    }

    result
  }

  fn decompress(&mut self, input: &[u8], output: &mut [u8]) -> c_int {
    let (in_ptr, in_len): (*mut c_void, c_int) = self.input(input);
    let (out_ptr, out_len): (*mut c_void, c_int) = self.output(output);

    let result: c_int =
      unsafe { ffi::DecompressADPCM(out_ptr, out_len, in_ptr, in_len, self.channels_c()) };

    if result != 0 {
      self.bytes_src = input.len();
      self.bytes_dst = result as usize;
    }

    result
  }

  #[inline]
  const fn total_in(&self) -> usize {
    self.bytes_src
  }

  #[inline]
  const fn total_out(&self) -> usize {
    self.bytes_dst
  }

  #[inline]
  const fn channels(&self) -> usize {
    self.channels
  }

  #[inline]
  fn input(&self, input: &[u8]) -> (*mut c_void, c_int) {
    let in_ptr: *mut c_void = input.as_ptr().cast_mut().cast();
    let in_len: c_int = input.len().min(c_int::MAX as usize) as c_int;

    (in_ptr, in_len)
  }

  #[inline]
  fn output(&self, output: &mut [u8]) -> (*mut c_void, c_int) {
    let out_ptr: *mut c_void = output.as_mut_ptr().cast();
    let out_len: c_int = output.len().min(c_int::MAX as usize) as c_int;

    (out_ptr, out_len)
  }

  #[inline]
  fn channels_c(&self) -> c_int {
    self.channels.min(c_int::MAX as usize) as c_int
  }
}

// =============================================================================
// Compress
// =============================================================================

/// Representation of an in-memory compression stream.
pub struct Compress {
  stream: Stream,
  comp_type: i32,
}

impl Compress {
  /// Create a new compression stream.
  #[inline]
  pub const fn new(channels: usize, comp_type: i32) -> Self {
    Self {
      stream: Stream::new(channels),
      comp_type,
    }
  }

  /// Compress `input` into `output`.
  #[inline]
  pub fn compress(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), Error> {
    let result: c_int = self.stream.compress(input, output, self.comp_type);

    if result != 0 {
      Ok(())
    } else {
      Err(Error::new(ErrorKind::Compression))
    }
  }

  /// Returns the total number of input bytes processed.
  #[inline]
  pub const fn total_in(&self) -> usize {
    self.stream.total_in()
  }

  /// Returns the total number of output bytes processed.
  #[inline]
  pub const fn total_out(&self) -> usize {
    self.stream.total_out()
  }

  /// Returns the type of number of channels used.
  #[inline]
  pub const fn channels(&self) -> usize {
    self.stream.channels()
  }

  /// Returns the type of compression stream.
  #[inline]
  pub const fn comp_type(&self) -> i32 {
    self.comp_type
  }
}

// =============================================================================
// Decompress
// =============================================================================

/// Representation of an in-memory decompression stream.
pub struct Decompress {
  stream: Stream,
}

impl Decompress {
  /// Create a new decompression stream.
  #[inline]
  pub const fn new(channels: usize) -> Self {
    Self {
      stream: Stream::new(channels),
    }
  }

  /// Decompress `input` into `output`.
  #[inline]
  pub fn decompress(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), Error> {
    let result: c_int = self.stream.decompress(input, output);

    if result != 0 {
      Ok(())
    } else {
      Err(Error::new(ErrorKind::Decompression))
    }
  }

  /// Returns the total number of input bytes processed.
  #[inline]
  pub const fn total_in(&self) -> usize {
    self.stream.total_in()
  }

  /// Returns the total number of output bytes processed.
  #[inline]
  pub const fn total_out(&self) -> usize {
    self.stream.total_out()
  }

  /// Returns the type of number of channels used.
  #[inline]
  pub const fn channels(&self) -> usize {
    self.stream.channels()
  }
}
