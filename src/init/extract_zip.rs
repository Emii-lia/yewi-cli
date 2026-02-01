use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn extract_zip(
  zip_path: &Path,
  extract_to: &Path,
) -> Result<(), Box<dyn Error>> {
  let file = fs::File::open(zip_path)?;
  let mut archive = zip::ZipArchive::new(file)?;

  for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;
    let file_path = file
      .enclosed_name()
      .ok_or("Invalid file name in zip")?;

    let relative_path = file_path
      .components()
      .skip(1)
      .collect::<PathBuf>();

    let outpath = extract_to.join(&relative_path);

    if file.is_dir() {
      fs::create_dir_all(&outpath)?;
    } else {
      if let Some(p) = outpath.parent() {
        if !p.exists() {
          fs::create_dir_all(p)?;
        }
      }
      let mut outfile = fs::File::create(&outpath)?;
      std::io::copy(&mut file, &mut outfile)?;
    }
  }

  Ok(())
}