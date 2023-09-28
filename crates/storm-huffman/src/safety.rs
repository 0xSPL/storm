use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::mem::MaybeUninit;

use crate::error::Error;
use crate::error::ErrorKind;

mod ffi {
  #![allow(dead_code)]
  #![allow(non_snake_case)]
  #![allow(non_upper_case_globals)]
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// =============================================================================
// Compress
// =============================================================================

/// Representation of an in-memory compression stream.
pub struct Compress {
  bytes_src: usize,
  bytes_dst: usize,
  comp_type: i32,
}

impl Compress {
  /// Create a new compression stream.
  #[inline]
  pub const fn new(comp_type: i32) -> Self {
    Self {
      bytes_src: 0,
      bytes_dst: 0,
      comp_type,
    }
  }

  /// Compress `input` into `output`.
  pub fn compress(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), Error> {
    let mut pointer_tree: MaybeUninit<ffi::THuffmannTree> = MaybeUninit::uninit();
    let mut pointer_data: MaybeUninit<ffi::TOutputStream> = MaybeUninit::uninit();

    let in_len: c_int = input.len().min(c_int::MAX as usize) as c_int;
    let in_ptr: *mut c_void = input.as_ptr().cast::<c_void>().cast_mut();

    let out_len: usize = output.len();
    let out_ptr: *mut c_void = output.as_mut_ptr().cast();

    // Initialize Huffman Tree
    unsafe {
      ffi::THuffmannTree_THuffmannTree(pointer_tree.as_mut_ptr(), true);
    }

    // Initialize Output Stream
    unsafe {
      ffi::TOutputStream_TOutputStream(pointer_data.as_mut_ptr(), out_ptr, out_len);
    }

    // Compress
    let result: c_uint = unsafe {
      ffi::THuffmannTree_Compress(
        pointer_tree.as_mut_ptr(),
        pointer_data.as_mut_ptr(),
        in_ptr,
        in_len,
        self.comp_type,
      )
    };

    // Deinitialize Huffman Tree
    unsafe {
      ffi::THuffmannTree_THuffmannTree_destructor(pointer_tree.as_mut_ptr());
    }

    if result != 0 {
      self.bytes_src = input.len();
      self.bytes_dst = result as usize;

      Ok(())
    } else {
      Err(Error::new(ErrorKind::Compression))
    }
  }

  /// Returns the total number of input bytes processed.
  #[inline]
  pub const fn total_in(&self) -> usize {
    self.bytes_src
  }

  /// Returns the total number of output bytes processed.
  #[inline]
  pub const fn total_out(&self) -> usize {
    self.bytes_dst
  }
}

// =============================================================================
// Decompress
// =============================================================================

/// Representation of an in-memory decompression stream.
pub struct Decompress {
  bytes_src: usize,
  bytes_dst: usize,
}

impl Decompress {
  /// Create a new decompression stream.
  #[inline]
  pub const fn new() -> Self {
    Self {
      bytes_src: 0,
      bytes_dst: 0,
    }
  }

  /// Decompress `input` into `output`.
  pub fn decompress(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), Error> {
    let mut pointer_tree: MaybeUninit<ffi::THuffmannTree> = MaybeUninit::uninit();
    let mut pointer_data: MaybeUninit<ffi::TInputStream> = MaybeUninit::uninit();

    let in_len: usize = input.len();
    let in_ptr: *mut c_void = input.as_ptr().cast::<c_void>().cast_mut();

    let out_len: c_uint = output.len().min(c_uint::MAX as usize) as c_uint;
    let out_ptr: *mut c_void = output.as_mut_ptr().cast();

    // Initialize Huffman Tree
    unsafe {
      ffi::THuffmannTree_THuffmannTree(pointer_tree.as_mut_ptr(), false);
    }

    // Initialize Input Stream
    unsafe {
      ffi::TInputStream_TInputStream(pointer_data.as_mut_ptr(), in_ptr, in_len);
    }

    // Decompress
    let result: c_uint = unsafe {
      ffi::THuffmannTree_Decompress(
        pointer_tree.as_mut_ptr(),
        out_ptr,
        out_len,
        pointer_data.as_mut_ptr(),
      )
    };

    // Deinitialize Huffman Tree
    unsafe {
      ffi::THuffmannTree_THuffmannTree_destructor(pointer_tree.as_mut_ptr());
    }

    if result != 0 {
      self.bytes_src = input.len();
      self.bytes_dst = result as usize;

      Ok(())
    } else {
      Err(Error::new(ErrorKind::Decompression))
    }
  }

  /// Returns the total number of input bytes processed.
  #[inline]
  pub const fn total_in(&self) -> usize {
    self.bytes_dst
  }

  /// Returns the total number of output bytes processed.
  #[inline]
  pub const fn total_out(&self) -> usize {
    self.bytes_dst
  }
}
