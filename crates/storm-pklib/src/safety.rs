use core::ffi::c_char;
use core::ffi::c_uchar;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::ptr;

use crate::error::Error;
use crate::error::ErrorKind;

mod ffi {
  #![allow(dead_code)]
  #![allow(non_snake_case)]
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

  #[derive(Clone, Copy, Debug)]
  #[repr(C)]
  pub struct TDataInfo {
    pub pbInBuff: *mut ::core::ffi::c_uchar,
    pub pbInBuffEnd: *mut ::core::ffi::c_uchar,
    pub pbOutBuff: *mut ::core::ffi::c_uchar,
    pub pbOutBuffEnd: *mut ::core::ffi::c_uchar,
  }
}

const WORK_CONTEXT_C: ffi::TCmpStruct = ffi::TCmpStruct {
  distance: 0,
  out_bytes: 0,
  out_bits: 0,
  dsize_bits: 0,
  dsize_mask: 0,
  ctype: 0,
  dsize_bytes: 0,
  dist_bits: [0; 0x40],
  dist_codes: [0; 0x40],
  nChBits: [0; 0x306],
  nChCodes: [0; 0x306],
  offs09AE: 0,
  param: ptr::null_mut(),
  read_buf: None,
  write_buf: None,
  offs09BC: [0; 0x204],
  offs0DC4: 0,
  phash_to_index: [0; 0x900],
  phash_to_index_end: 0,
  out_buff: [0; 0x802],
  work_buff: [0; 0x2204],
  phash_offs: [0; 0x2204],
};

const WORK_CONTEXT_D: ffi::TDcmpStruct = ffi::TDcmpStruct {
  offs0000: 0,
  ctype: 0,
  outputPos: 0,
  dsize_bits: 0,
  dsize_mask: 0,
  bit_buff: 0,
  extra_bits: 0,
  in_pos: 0,
  in_bytes: 0,
  param: ptr::null_mut(),
  read_buf: None,
  write_buf: None,
  out_buff: [0; 0x2204],
  in_buff: [0; 0x800],
  DistPosCodes: [0; 0x100],
  LengthCodes: [0; 0x100],
  offs2C34: [0; 0x100],
  offs2D34: [0; 0x100],
  offs2E34: [0; 0x80],
  offs2EB4: [0; 0x100],
  ChBitsAsc: [0; 0x100],
  DistBits: [0; 0x40],
  LenBits: [0; 0x10],
  ExLenBits: [0; 0x10],
  LenBase: [0; 0x10],
};

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

  fn compress(&mut self, input: &[u8], output: &mut [u8]) -> c_uint {
    let mut work_context: ffi::TCmpStruct = WORK_CONTEXT_C;
    let mut info_context: ffi::TDataInfo = self.info_context(input, output);

    let mut ctype: c_uint = ffi::CMP_BINARY;
    let mut dsize: c_uint;

    if input.len() < 0x600 {
      dsize = ffi::CMP_IMPLODE_DICT_SIZE1;
    } else if 0x600 <= input.len() && input.len() < 0xC00 {
      dsize = ffi::CMP_IMPLODE_DICT_SIZE2;
    } else {
      dsize = ffi::CMP_IMPLODE_DICT_SIZE3;
    }

    let result: c_uint = unsafe {
      ffi::implode(
        Some(Self::buffer_read),
        Some(Self::buffer_write),
        (&mut work_context as *mut ffi::TCmpStruct).cast(),
        (&mut info_context as *mut ffi::TDataInfo).cast(),
        &mut ctype,
        &mut dsize,
      )
    };

    if result == ffi::CMP_NO_ERROR {
      self.bytes_src = input.len();
      self.bytes_dst = self.bytes_written(output, &info_context);
    }

    result
  }

  fn decompress(&mut self, input: &[u8], output: &mut [u8]) -> c_uint {
    let mut work_context: ffi::TDcmpStruct = WORK_CONTEXT_D;
    let mut info_context: ffi::TDataInfo = self.info_context(input, output);

    let result: c_uint = unsafe {
      ffi::explode(
        Some(Self::buffer_read),
        Some(Self::buffer_write),
        (&mut work_context as *mut ffi::TDcmpStruct).cast(),
        (&mut info_context as *mut ffi::TDataInfo).cast(),
      )
    };

    if result == ffi::CMP_NO_ERROR {
      self.bytes_src = input.len();
      self.bytes_dst = self.bytes_written(output, &info_context);
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
  fn input(&self, input: &[u8]) -> (*mut c_uchar, *mut c_uchar) {
    let in_ptr: *mut c_uchar = input.as_ptr().cast_mut().cast();
    let in_end: *mut c_uchar = unsafe { in_ptr.add(input.len()) };

    (in_ptr, in_end)
  }

  #[inline]
  fn output(&self, output: &mut [u8]) -> (*mut c_uchar, *mut c_uchar) {
    let out_ptr: *mut c_uchar = output.as_mut_ptr().cast();
    let out_end: *mut c_uchar = unsafe { out_ptr.add(output.len()) };

    (out_ptr, out_end)
  }

  #[inline]
  fn bytes_written(&self, output: &[u8], info: &ffi::TDataInfo) -> usize {
    unsafe { info.pbOutBuff.sub(output.as_ptr() as usize) as usize }
  }

  #[inline]
  fn info_context(&self, input: &[u8], output: &mut [u8]) -> ffi::TDataInfo {
    let (in_ptr, in_end): (*mut c_uchar, *mut c_uchar) = self.input(input);
    let (out_ptr, out_end): (*mut c_uchar, *mut c_uchar) = self.output(output);

    ffi::TDataInfo {
      pbInBuff: in_ptr,
      pbInBuffEnd: in_end,
      pbOutBuff: out_ptr,
      pbOutBuffEnd: out_end,
    }
  }

  unsafe extern "C" fn buffer_read(
    buffer: *mut c_char,
    size: *mut c_uint,
    param: *mut c_void,
  ) -> c_uint {
    let info: &mut ffi::TDataInfo = &mut *param.cast();
    let limit: c_uint = info.pbInBuffEnd.sub(info.pbInBuff as usize) as c_uint;

    // Check the case when not enough data available
    let target: c_uint = (*size).min(limit);

    // Load data and increment offsets
    ptr::copy_nonoverlapping(info.pbInBuff.cast(), buffer, target as usize);
    info.pbInBuff = info.pbInBuff.add(target as usize);

    assert!(info.pbInBuff <= info.pbInBuffEnd);

    target
  }

  unsafe extern "C" fn buffer_write(buffer: *mut c_char, size: *mut c_uint, param: *mut c_void) {
    let info: &mut ffi::TDataInfo = &mut *param.cast();
    let limit: c_uint = info.pbOutBuffEnd.sub(info.pbOutBuff as usize) as c_uint;

    // Check the case when not enough space in the output buffer
    let target: c_uint = (*size).min(limit);

    // Write output data and increment offsets
    ptr::copy_nonoverlapping(buffer.cast(), info.pbOutBuff, target as usize);
    info.pbOutBuff = info.pbOutBuff.add(target as usize);

    assert!(info.pbOutBuff <= info.pbOutBuffEnd);
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
    match self.stream.compress(input, output) {
      ffi::CMP_NO_ERROR => Ok(()),
      ffi::CMP_INVALID_DICTSIZE => Err(Error::new(ErrorKind::InvalidDict)),
      ffi::CMP_INVALID_MODE => Err(Error::new(ErrorKind::InvalidMode)),
      result => Err(Error::new(ErrorKind::Unknown(result))),
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
    match self.stream.decompress(input, output) {
      ffi::CMP_NO_ERROR => Ok(()),
      ffi::CMP_INVALID_DICTSIZE => Err(Error::new(ErrorKind::InvalidDict)),
      ffi::CMP_INVALID_MODE => Err(Error::new(ErrorKind::InvalidMode)),
      ffi::CMP_BAD_DATA => Err(Error::new(ErrorKind::InvalidData)),
      ffi::CMP_ABORT => Err(Error::new(ErrorKind::Fatal)),
      result => Err(Error::new(ErrorKind::Unknown(result))),
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
}
