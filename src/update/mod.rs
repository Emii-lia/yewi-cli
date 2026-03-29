pub mod test;

use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use indicatif::ProgressBar;
use crate::init::theming::init_theme;
use crate::init::update_config::update_theme;

pub(crate) fn update(theme: Option<String>) -> Result<(), Box<dyn  Error>> {
  let project_dir = PathBuf::from(".");

  if !project_dir.join("src").exists() {
    return Err("This command must be run inside a Yewi project directory.".into());
  }

  let color = init_theme(theme)?;
  let spinner = ProgressBar::new_spinner();
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

  Ok(())
}