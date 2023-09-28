use crate::error::Error;
use crate::error::ErrorKind;

mod ffi {
  #![allow(dead_code)]
  #![allow(non_snake_case)]
  #![allow(non_upper_case_globals)]
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// =============================================================================
// Stream
// =============================================================================

struct Stream {
  bytes_src: usize,
  bytes_dst: usize,
}

impl Stream {
  #[inline]
  const fn new() -> Self {
    Self {
      bytes_src: 0,
      bytes_dst: 0,
    }
  }

  #[inline]
  const fn total_in(&self) -> usize {
    self.bytes_src
  }

  #[inline]
  const fn total_out(&self) -> usize {
    self.bytes_dst
  }
}

// =============================================================================
// Compress
// =============================================================================

/// Representation of an in-memory compression stream.
pub struct Compress {
  stream: Stream,
}

impl Compress {
  /// Create a new compression stream.
  #[inline]
  pub const fn new() -> Self {
    Self {
      stream: Stream::new(),
    }
  }

  /// Compress `input` into `output`.
  pub fn compress(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), Error> {
    panic!("Compress::compress");
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
  pub const fn new() -> Self {
    Self {
      stream: Stream::new(),
    }
  }

  /// Decompress `input` into `output`.
  pub fn decompress(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), Error> {
    panic!("Decompress::decompress");
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
}
