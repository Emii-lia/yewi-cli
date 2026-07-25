use std::error::Error;
use colored::Colorize;
use crate::install::build_tailwind::build_tailwind;
use crate::install::detect_package::detect_package;
use crate::install::install_package::install_package;

pub mod install_package;
pub mod build_tailwind;
pub mod detect_package;
pub mod test;

pub fn install() -> Result<(), Box<dyn Error>> {
  let package = detect_package()
    .map_err(|e| format!("Failed to detect package: {}", e))?;
  
  install_package(package.as_str())
    .map_err(|e| format!("Failed to install package: {}", e))?;
  
  build_tailwind(package.as_str())
    .map_err(|e| format!("Failed to install package: {}", e))?;
  
  println!("All set! You can run your {} app with {} now!", "yewi".color("bright blue"), "[trunk serve]".color("bright green"));
  
  Ok(())
}