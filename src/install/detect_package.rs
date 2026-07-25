use std::error::Error;
use std::fs;
use std::path::PathBuf;
use regex::Regex;

pub fn detect_package() -> Result<String, Box<dyn Error>> {
  let project_dir = PathBuf::from(".");
  let trunk_config_path = project_dir.join("Trunk.toml");

  let trunk_config_content = fs::read_to_string(&trunk_config_path)
    .map_err(|e| format!("Failed to read trunk.toml: {}", e)).unwrap_or_else(|_|
    fs::read_to_string(project_dir.join("trunk.toml"))
      .map_err(|e| format!("Failed to read trunk.toml: {}", e))
      .expect("Failed to read trunk.toml")
  );

  let package_re = Regex::new(r#"command\s*=\s*"([^"]+)""#)
    .map_err(|e| format!("Failed to compile regex pattern: {}", e))?;

  let package = package_re.captures(&trunk_config_content)
    .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
    .ok_or("Failed to detect package manager from trunk.toml")?;

  Ok(package)
}