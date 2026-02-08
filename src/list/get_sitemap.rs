use std::error::Error;
use std::process::Command;

pub(crate) fn get_sitemap() -> Result<String, Box<dyn Error>> {
  let output = Command::new("curl")
    .arg("-s")
    .arg("https://yewi.pages.dev/sitemap.xml")
    .output()?;
  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(format!("Failed to get sitemap: {}", stderr).into());
  }
  if output.stdout.is_empty() {
    return Err("Sitemap is empty.".into());
  }
  Ok(String::from_utf8_lossy(&output.stdout).to_string())
}