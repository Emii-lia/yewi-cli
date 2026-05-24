pub mod test;

use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use indicatif::ProgressBar;
use crate::handlers::inquire_node_package::inquire_node_package_man;
use crate::init::theming::init_theme;
use crate::init::update_config::{update_node_package_man, update_theme};

pub(crate) fn update(theme: Option<String>, package: Option<String>) -> Result<(), Box<dyn  Error>> {
  let project_dir = PathBuf::from(".");

  if !project_dir.join("src").exists() {
    return Err("This command must be run inside a Yewi project directory.".into());
  }

  let spinner = ProgressBar::new_spinner();
  if theme.is_none() && package.is_none() {
    return Err("Please specify at least one of --theme or --package to update.".into());
  }
  if theme.is_some() {
    let color = init_theme(theme)?;
    spinner.set_style(
      indicatif::ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner:.cyan} {msg}")?
    );

    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner.set_message("Applying theme configuration...");
    update_theme(&project_dir, color)
      .map_err(|e| format!("Error applying theme configuration: {}", e))?;
    spinner.finish_and_clear();
    println!();
    println!("Successfully applied theme configuration.");
  }
  if package.is_some() {
    let package_manager = match package {
      Some(package_manager) => package_manager,
      None => inquire_node_package_man()
        .map_err(|e| format!(" Failed to determine node package manager: {}", e))?
    };
    spinner.set_style(
      indicatif::ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner:.cyan} {msg}")?
    );
    spinner.set_message("Updating package manager...");

    update_node_package_man(&project_dir, &package_manager)
      .map_err(|e| format!("Error updating package manager: {}", e))?;

    spinner.finish_and_clear();
    println!();
    println!("Successfully updated package manager.");
  }

  Ok(())
}