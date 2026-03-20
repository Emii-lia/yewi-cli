use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;
use github::download_component;
use crate::add::components::{component_exists, update_components_mod, update_components_scss};

mod github;
pub mod components;
mod metadata;
pub mod test;

use indicatif::ProgressBar;
use std::time::Duration;
use crate::add::metadata::fetch_metadata;

pub(crate) fn add(component_name: &str, added: &mut HashSet<String>) -> Result<(), Box<dyn Error>> {
  let project_dir = PathBuf::from(".");

  if !project_dir.join("src").exists() {
    return Err("This command must be run inside a Yewi project directory.".into());
  }

  if component_exists(&project_dir, component_name) {
    println!("Component '{}' already exists in your project.", component_name);
    return Ok(());
  }

  if added.contains(component_name) {
    return Ok(());
  }
  added.insert(component_name.to_string());

  let metadata = fetch_metadata(component_name)?;

  let spinner = ProgressBar::new_spinner();
  if !metadata.dependencies.is_empty() {
    spinner.set_message(format!("Adding dependencies for '{}'...", component_name));
    for dep in metadata.dependencies {
      add(&dep, added)?;
    }
  }

  download_component(component_name, &project_dir)?;

  spinner.set_style(
    indicatif::ProgressStyle::default_spinner()
      .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
      .template("{spinner:.cyan} {msg}")
      .unwrap()
  );
  spinner.enable_steady_tick(Duration::from_millis(80));

  spinner.set_message(format!("Updating src/components/mod.rs for '{}'...", component_name));
  update_components_mod(&project_dir, component_name)?;

  spinner.set_message(format!("Updating src/styles/components.scss for '{}'...", component_name));
  update_components_scss(&project_dir, component_name)?;

  spinner.finish_and_clear();

  println!("✓ '{}' component added to your project.", component_name);

  Ok(())
}