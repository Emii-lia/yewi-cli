use std::error::Error;
use std::path::PathBuf;
use github::download_component;
use crate::add::components::{component_exists, update_components_mod, update_components_scss};

mod github;
pub mod components;

use indicatif::ProgressBar;
use std::time::Duration;

pub(crate) fn add(component_name: &str) -> Result<(), Box<dyn Error>> {
  let project_dir = PathBuf::from(".");

  if !project_dir.join("src").exists() {
    return Err("This command must be run inside a Yewi project directory.".into());
  }

  if component_exists(&project_dir, component_name) {
    println!("Component '{}' already exists in your project.", component_name);
    return Ok(());
  }

  download_component(component_name, &project_dir)?;

  let spinner = ProgressBar::new_spinner();
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