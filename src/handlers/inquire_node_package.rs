use inquire::error::InquireResult;
use inquire::Select;
use crate::types::node_package::NodePackageMan;

pub fn inquire_node_package_man() -> InquireResult<String> {
  let choice = Select::new("Select a package manager: ", NodePackageMan::get_packages())
  .prompt()?;
  
  Ok(choice.to_string())
}