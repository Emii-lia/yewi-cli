mod extract_zip;
mod download_file;
mod git;
mod update_config;

use std::error::Error;
use std::fs;
use std::path::{PathBuf};
use crate::handlers::inquire_init::inquire_init;
use crate::init::git::{clone_with_api, clone_with_git, is_git_available};
use crate::init::update_config::{update_cargo_toml, update_package_json, update_theme};

pub(crate) fn create(project_name: &str) -> Result<(), Box<dyn Error>> {
  let project_dir = PathBuf::from(project_name);
  println!(" Creating a new Yew project: {}", project_name);
  println!();

  if project_dir.exists() {
    return Err(format!(
      "Directory '{}' already exists.",
      project_name
    ).into());
  }

  let color = inquire_init()
    .map_err(|e| format!("Failed to initiate project: {}", e))?;


  fs::create_dir_all(&project_dir)?;
  clone_template(&project_dir)?;

  println!("Updating theme");
  update_theme(&project_dir, color)?;

  println!();
  println!(" Success! Created '{}' project.", project_name);
  println!();
  println!("Next steps:");
  println!("1. cd {}", project_name);
  println!("2. yarn && yarn build");
  println!("3. cargo add yew web-sys wasm-logger yew-router gloo");
  println!("4. trunk serve");
  println!();
  println!("Then add components with: yewi add <component-name>");

  Ok(())
}

fn clone_template(project_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
  println!(" Downloading template...");

  let git_available = is_git_available();

  if git_available {
    println!("  Using git clone...");
    match clone_with_git(project_dir) {
      Ok(_) => {
        println!("  Git clone successful");
      }
      Err(e) => {
        println!("  Git clone failed: {}", e);
        println!("  Falling back to API download...");
        clone_with_api(project_dir)?;
      }
    }
  } else {
    println!("  Git not found, using API download method...");
    clone_with_api(project_dir)?;
  }

  let git_dir = project_dir.join(".git");
  if git_dir.exists() {
    fs::remove_dir_all(git_dir).ok();
  }
  update_cargo_toml(project_dir)?;
  update_package_json(project_dir)?;

  println!("  Template ready");
  Ok(())
}

