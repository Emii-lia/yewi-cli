use std::error::Error;
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use crate::types::color::Color;

pub(crate) fn update_cargo_toml(project_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
  let cargo_toml_path = project_dir.join("Cargo.toml");
  let mut cargo_toml_content = fs::read_to_string(&cargo_toml_path)?;

  cargo_toml_content = cargo_toml_content.replace(
    "name = \"yewi-template\"",
    &format!("name = \"{}\"", project_dir.file_name().unwrap().to_string_lossy()),
  );

  fs::write(&cargo_toml_path, cargo_toml_content)?;

  Ok(())
}

pub(crate) fn update_package_json(project_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
  let package_json_path = project_dir.join("package.json");
  let mut package_json_content = fs::read_to_string(&package_json_path)?;

  package_json_content = package_json_content.replace(
    "\"name\": \"yewi-template\"",
    &format!("\"name\": \"{}\"", project_dir.file_name().unwrap().to_string_lossy()),
  );

  fs::write(&package_json_path, package_json_content)?;

  Ok(())
}

pub(crate) fn update_theme(project_dir: &PathBuf, color: Color) -> Result<(), Box<dyn Error>> {
  let style_path = project_dir.join("src/styles/main.scss");
  let shades: Vec<i32> = vec![50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];
  let mut style_content = fs::read_to_string(&style_path)?;

  for shade in &shades {
    let color_value = format!("theme(\"colors.{}.{}\")", &color.to_string().to_lowercase(), shade);
    let re = Regex::new(&format!(r"--primary-{}\s*:\s*[^;]+;", shade))
      .map_err(|e| format!("Failed to compile regex: {}", e))?;
    style_content = re.replace_all(&style_content, format!("--primary-{}: {};", shade, &color_value)).into_owned();
  }

  fs::write(&style_path, style_content).map_err(|e| format!("Failed to update theme: {}", e))?;

  Ok(())
}