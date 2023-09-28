use serde::Serialize;
use serde_json::to_writer_pretty;
use std::fs::File;
use std::io::Result;
use std::path::Path;

mod private {
  pub trait Sealed {}
}

pub trait ExportJson: Serialize + private::Sealed {
  fn export_json<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<()> {
    let file: File = File::create(path)?;

    to_writer_pretty(file, self)?;

    Ok(())
  }
}

impl<T: Serialize> private::Sealed for T {}
impl<T: Serialize> ExportJson for T {}
