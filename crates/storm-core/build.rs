use std::any::type_name;
use std::env;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::UpperHex;
use std::fs;
use std::io::Result;
use std::mem::size_of;
use std::path::PathBuf;

struct Table<'a, T> {
  name: &'a str,
  data: &'a [T],
}

impl<T: UpperHex> Display for Table<'_, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    const HEADER: &str = "// generated by `build.rs`\n\n";
    const FOOTER: &str = "];\n";

    const CHUNK: usize = 0x8;

    let name: &str = self.name;
    let kind: &str = type_name::<T>();
    let size: usize = self.data.len();

    assert!(size % CHUNK == 0x0);

    f.write_str(HEADER)?;
    f.write_fmt(format_args!("pub static {name}: [{kind}; {size:#X}] = [\n"))?;

    let size: usize = 2 + (size_of::<T>() << 1);

    for chunk in self.data.chunks_exact(CHUNK) {
      f.write_fmt(format_args!(
        "  {:#0size$X}, {:#0size$X}, {:#0size$X}, {:#0size$X}, {:#0size$X}, {:#0size$X}, {:#0size$X}, {:#0size$X},\n",
        chunk[0],
        chunk[1],
        chunk[2],
        chunk[3],
        chunk[4],
        chunk[5],
        chunk[6],
        chunk[7],
      ))?;
    }

    f.write_str(FOOTER)?;

    Ok(())
  }
}

fn generate_cryptable() -> [u32; 0x500] {
  const SEED: u32 = 0x00100001;

  let mut table: [u32; 0x500] = [0; 0x500];
  let mut seed: u32 = SEED;

  let mut tmp1: u32;
  let mut tmp2: u32;

  for index1 in 0..0x100 {
    for index2 in 0..5 {
      seed = (seed * 125 + 3) % 0x2AAAAB;
      tmp1 = (seed & 0xFFFF) << 0x10;

      seed = (seed * 125 + 3) % 0x2AAAAB;
      tmp2 = seed & 0xFFFF;

      table[index1 + index2 * 0x100] = tmp1 | tmp2;
    }
  }

  table
}

// Converts lowercase to uppercase
// Converts slash to backslash
fn generate_ascii_to_upper_sensitive() -> [u8; 0x100] {
  let mut table: [u8; 0x100] = [0; 0x100];

  for ch in 0..=0xFF {
    let ch: u8 = ch;

    if ch.is_ascii_lowercase() {
      table[ch as usize] = ch.to_ascii_uppercase();
    } else if ch == b'/' {
      table[ch as usize] = b'\\';
    } else {
      table[ch as usize] = ch;
    }
  }

  table
}

fn main() -> Result<()> {
  println!("cargo:rerun-if-changed=build.rs");

  let root: String = env::var("OUT_DIR").unwrap();
  let root: PathBuf = root.into();

  // ===========================================================================
  // Generate CRYPTO Table
  // ===========================================================================

  let table: Table<'_, u32> = Table {
    name: "CRYPTABLE",
    data: &generate_cryptable(),
  };

  fs::write(root.join("cryptable.rs"), table.to_string())?;

  // ===========================================================================
  // Generate ASCII Table
  // ===========================================================================

  let table: Table<'_, u8> = Table {
    name: "ASCII_TO_UPPER_SENSITIVE",
    data: &generate_ascii_to_upper_sensitive(),
  };

  fs::write(root.join("ascii_to_upper_sensitive.rs"), table.to_string())?;

  Ok(())
}
