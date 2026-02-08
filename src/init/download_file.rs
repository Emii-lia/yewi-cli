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
    .output()
    .map_err(|e| format!("❌ Failed to execute curl: {}", e))?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(format!(
      "❌ Failed to download template from: {}\n   Error: {}",
      url, stderr
    ).into());
  }

  if output.stdout.is_empty() {
    return Err("❌ Downloaded file is empty. The URL may be incorrect or the repository is unavailable.".into());
  }

  let mut file = fs::File::create(dest_path)
    .map_err(|e| format!("❌ Failed to create destination file: {}", e))?;
  std::io::Write::write_all(&mut file, &output.stdout)
    .map_err(|e| format!("❌ Failed to write downloaded file: {}", e))?;

  Ok(())
}