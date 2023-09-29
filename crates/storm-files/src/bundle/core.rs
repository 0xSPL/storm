use std::path::Path;
use storm_core::error::Result;
use storm_core::types::Archive;
use storm_core::types::AttrFile;
use storm_core::types::File;
use storm_core::types::ListFile;

use crate::bundle::BundleData;
use crate::bundle::BundleFile;
use crate::bundle::Format;

// =============================================================================
// Archive Bundle
// =============================================================================

#[derive(Debug)]
pub struct Bundle {
  pub archive: Archive,
  pub list: ListFile,
  pub attr: Option<AttrFile>,
  pub data: Box<[BundleFile]>,
}

impl Bundle {
  pub fn open<P>(path: &P) -> Result<Self>
  where
    P: AsRef<Path> + ?Sized,
  {
    let archive: Archive = Archive::open(path)?;
    let list: ListFile = fetch_list(&archive)?;
    let attr: Option<AttrFile> = fetch_attr(&archive)?;
    let data: Box<[BundleFile]> = fetch_data(&archive, &list)?;

    Ok(Self {
      archive,
      list,
      attr,
      data,
    })
  }
}

fn fetch_list(archive: &Archive) -> Result<ListFile> {
  if let Ok(pointer) = archive.find_file("(listfile)") {
    pointer.try_into()
  } else {
    Ok(ListFile::empty())
  }
}

fn fetch_attr(archive: &Archive) -> Result<Option<AttrFile>> {
  if let Ok(pointer) = archive.find_file("(attributes)") {
    pointer.try_into().map(Some)
  } else {
    Ok(None)
  }
}

fn fetch_data(archive: &Archive, list: &ListFile) -> Result<Box<[BundleFile]>> {
  let mut output: Vec<BundleFile> = Vec::with_capacity(list.len());

  for entry in list.iter() {
    let name: &str = entry.as_utf8()?;

    if name == "(listfile)" || name == "(attributes)" {
      continue;
    }

    let Some(format) = Format::find(name) else {
      eprintln!("[unknown format]: {:?}", name);
      continue;
    };

    // Parse raw content
    let file: File = archive.load_file(name)?;

    // Parse format-specific content
    let data: BundleData = if let Some(parser) = format.parser() {
      parser(file)?
    } else {
      BundleData::Plain(file)
    };

    output.push(BundleFile {
      data,
      name: name.to_owned(),
    });
  }

  Ok(output.into_boxed_slice())
}
