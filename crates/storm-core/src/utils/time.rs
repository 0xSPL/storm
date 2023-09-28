const WINDOWS_TICK: u64 = 10000000;
const WINDOWS_EPOCH: u64 = 11644473600;
const WINDOWS_OFFSET: u64 = WINDOWS_EPOCH * WINDOWS_TICK;

// Convert windows FILETIME to unix time
//
// https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-filetime
pub fn convert_filetime(lo: u32, hi: u32) -> u64 {
  let mut time: u64 = ((hi as u64) << 32) + lo as u64;

  if time < WINDOWS_OFFSET {
    panic!("FILETIME Out of Range")
  }

  time -= WINDOWS_OFFSET;
  time /= WINDOWS_TICK;
  time
}
