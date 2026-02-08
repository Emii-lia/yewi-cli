mod extract_zip;
mod download_file;
mod git;
mod update_config;

use std::error::Error;
use std::fs;
use std::path::{PathBuf};
use indicatif::ProgressBar;
use std::time::Duration;
use crate::handlers::inquire_init::inquire_init;
use crate::init::git::{clone_with_api, clone_with_git, is_git_available};
use crate::init::update_config::{update_cargo_toml, update_package_json, update_theme};

pub(crate) fn create(project_name: &str) -> Result<(), Box<dyn Error>> {
  let project_dir = PathBuf::from(project_name);
  println!();
  println!("Creating a new Yew project: {}", project_name);
  println!();

  if project_dir.exists() {
    return Err(format!(
      "❌ Directory '{}' already exists. Please choose a different project name.",
      project_name
    ).into());
  }

  let color = inquire_init()
    .map_err(|e| format!("❌ Failed to initialize project settings: {}", e))?;

  fs::create_dir_all(&project_dir)
    .map_err(|e| format!("❌ Failed to create project directory: {}", e))?;

  clone_template(&project_dir)?;

  let spinner = ProgressBar::new_spinner();
  spinner.set_style(
    indicatif::ProgressStyle::default_spinner()
      .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
      .template("{spinner:.cyan} {msg}")
      .unwrap()
  );
  spinner.enable_steady_tick(Duration::from_millis(80));
  spinner.set_message("Applying theme configuration...");

  update_theme(&project_dir, color)
    .map_err(|e| format!("❌ Failed to apply theme: {}", e))?;

  spinner.finish_and_clear();

  println!();
  println!("✓ Successfully created '{}' project", project_name);
  println!();
  println!("Next steps:");
  println!("   1. cd {}", project_name);
  println!("   2. yarn && yarn build");
  println!("   3. cargo add yew web-sys wasm-logger yew-router gloo");
  println!("   4. trunk serve");
  println!();
  println!("Then add components with: yewi add <component-name>");
  println!();

  Ok(())
}

fn clone_template(project_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
  let spinner = ProgressBar::new_spinner();
  spinner.set_style(
    indicatif::ProgressStyle::default_spinner()
      .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
      .template("{spinner:.cyan} {msg}")
      .unwrap()
  );
  spinner.enable_steady_tick(Duration::from_millis(80));

  let git_available = is_git_available();

  if git_available {
    match clone_with_git(project_dir) {
      Ok(_) => {
        spinner.set_message("Downloading template with git...");
        spinner.finish_and_clear();
      }
      Err(_) => {
        println!("Downloading template via API...");
        clone_with_api(project_dir)?;
        spinner.finish_and_clear();
      }
    }
  } else {
    println!("Downloading template via API...");
    clone_with_api(project_dir)?;
    spinner.finish_and_clear();
  }

  let git_dir = project_dir.join(".git");
  if git_dir.exists() {
    fs::remove_dir_all(git_dir).ok();
  }

  let spinner = ProgressBar::new_spinner();
  spinner.set_style(
    indicatif::ProgressStyle::default_spinner()
      .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
      .template("{spinner:.cyan} {msg}")
      .unwrap()
  );
  spinner.enable_steady_tick(Duration::from_millis(80));

  spinner.set_message("Updating configuration files...");
  update_cargo_toml(project_dir)
    .map_err(|e| format!("❌ Failed to update Cargo.toml: {}", e))?;
  update_package_json(project_dir)
    .map_err(|e| format!("❌ Failed to update package.json: {}", e))?;

  spinner.finish_and_clear();

  Ok(())
}
