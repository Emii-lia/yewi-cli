use std::error::Error;
use std::process::Command;
use crate::types::node_package::NodePackageMan;

pub fn build_tailwind(package_name: &str) -> Result<(), Box<dyn Error>> {
  println!("Building tailwind...");
  let package_manager = NodePackageMan::from_str(package_name)
    .expect("Failed to parse package manager from string");

  let build_command = package_manager.get_build_command();

  let output = Command::new(package_manager.to_string())
    .args(build_command)
    .output()
    .map_err(|e| { format!("Failed to execute {}: {}", package_manager.to_string(), e) })?;

  if !output.status.success() {
    return Err(format!("Failed to build Tailwind CSS with {}: {:?}", package_manager.to_string(), output.status).into());
  }

  if !output.stdout.is_empty() {
    println!("{}", String::from_utf8_lossy(&output.stdout));
  }
  Ok(())
}