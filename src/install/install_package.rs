use std::error::Error;
use std::process::Command;
use crate::types::node_package::NodePackageMan;

pub fn install_package(package_name: &str) -> Result<(), Box<dyn Error>> {
  println!("Installing tailwind");
  let package = NodePackageMan::from_str(package_name)
    .expect("Failed to parse package manager from string");

  let output = Command::new(package_name)
    .arg("install")
    .output()
    .map_err(|e| format!("❌ Failed to execute {}: {}", package_name, e))?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(format!(
      "❌ Failed to install package: {}\n   Error: {}",
      package_name, stderr
    ).into());
  }

  if !output.stdout.is_empty() {
    println!("{}", String::from_utf8_lossy(&output.stdout));
  }

  Ok(())
}