use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

pub(crate) fn download_file(url: &str, dest_path: &Path) -> Result<(), Box<dyn Error>> {
  let output = Command::new("curl")
    .arg("-s")
    .arg("-L")
    .arg("-f")
    .arg(url)
    .output()?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(format!(
      "Failed to download from: {}\n{}",
      url, stderr
    ).into());
  }

  if output.stdout.is_empty() {
    return Err("Downloaded file is empty. Check if the URL is correct.".into());
  }

  let mut file = fs::File::create(dest_path)?;
  std::io::Write::write_all(&mut file, &output.stdout)?;

  Ok(())
}