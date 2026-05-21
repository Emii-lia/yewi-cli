use std::error::Error;
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use crate::types::color::Color;
use crate::types::node_package::NodePackageMan;
use crate::utils::shade::{is_valid_hex, shades_of, ShadeKey};

pub(crate) fn update_cargo_toml(project_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
  let cargo_toml_path = project_dir.join("Cargo.toml");
  let mut cargo_toml_content = fs::read_to_string(&cargo_toml_path)
    .map_err(|e| format!("❌ Failed to read Cargo.toml: {}", e))?;

  cargo_toml_content = cargo_toml_content.replace(
    "name = \"yewi-template\"",
    &format!("name = \"{}\"", project_dir.file_name().unwrap().to_string_lossy()),
  );

  fs::write(&cargo_toml_path, cargo_toml_content)
    .map_err(|e| format!("❌ Failed to write Cargo.toml: {}", e))?;

  Ok(())
}

pub(crate) fn update_package_json(project_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
  let package_json_path = project_dir.join("package.json");
  let mut package_json_content = fs::read_to_string(&package_json_path)
    .map_err(|e| format!("❌ Failed to read package.json: {}", e))?;

  package_json_content = package_json_content.replace(
    "\"name\": \"yewi-template\"",
    &format!("\"name\": \"{}\"", project_dir.file_name().unwrap().to_string_lossy()),
  );

  fs::write(&package_json_path, package_json_content)
    .map_err(|e| format!("❌ Failed to write package.json: {}", e))?;

  Ok(())
}

pub fn update_theme(project_dir: &PathBuf, color: String) -> Result<(), Box<dyn Error>> {
  let style_path = project_dir.join("src/styles/main.scss");
  let mut style_content = fs::read_to_string(&style_path)
    .map_err(|e| format!("Failed to read styles/main.scss: {}", e))?;

  match Color::from_str(&color) {
    Color::Custom(c) => {
      if is_valid_hex(&c) {
        let shades = shades_of(&c)
          .map_err(|e| format!("Failed to generate shades for custom color: {}", e))?;
        for (shade, value) in shades {
          match shade {
            ShadeKey::U(key) => {
              let re =Regex::new(&format!(r"--primary-{}\s*:\s*[^;]+;", key))
                .map_err(|e| format!("Failed to compile regex pattern: {}", e))?;
              style_content = re.replace_all(&style_content, format!("--primary-{}: {};", shade, value)).into_owned();
            }
            ShadeKey::Default => {}
          }
        }
      }
    }
    _ => {
      let shades: Vec<i32> = vec![50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

      for shade in &shades {
        let color_value = format!("theme(\"colors.{}.{}\")", &color.to_string().to_lowercase(), shade);
        let re = Regex::new(&format!(r"--primary-{}\s*:\s*[^;]+;", shade))
          .map_err(|e| format!("Failed to compile regex pattern: {}", e))?;
        style_content = re.replace_all(&style_content, format!("--primary-{}: {};", shade, &color_value)).into_owned();
      }
    }
  }

  fs::write(&style_path, style_content)
    .map_err(|e| format!("❌ Failed to update theme in main.scss: {}", e))?;

  Ok(())
}

pub fn update_node_package_man(project_dir: &PathBuf, package_man: &str) -> Result<(), Box<dyn Error>> {
  let trunk_config_path = project_dir.join("Trunk.toml");
  let mut trunk_config_content = fs::read_to_string(&trunk_config_path)
    .map_err(|e| format!("Failed to read trunk.toml: {}", e)).unwrap_or_else(|_|
      fs::read_to_string(project_dir.join("trunk.toml"))
        .map_err(|e| format!("Failed to read trunk.toml: {}", e))
        .expect("Failed to read trunk.toml")
  );

  trunk_config_content = trunk_config_content.replace(
    "command = \"npm\"",
    &format!("command = \"{}\"", package_man.to_lowercase()),
  );
  trunk_config_content = trunk_config_content.replace(
    "command_arguments = [\"run\", \"build\"]",
    &format!(
      "command_arguments = [\"{}\"]", 
      NodePackageMan::from_str(package_man)
        .unwrap_or(NodePackageMan::default())
        .get_build_command()
      .join("\", \"")
    ),
  );
  
  fs::write(&trunk_config_path, trunk_config_content)
    .map_err(|e| format!("Failed to update Trunk.toml: {}", e))?;
  Ok(())
}