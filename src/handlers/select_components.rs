use inquire::error::InquireResult;
use inquire::MultiSelect;

pub fn select_components(components: Vec<String>) -> InquireResult<Vec<String>> {
  let components = MultiSelect::new("Select components: ", components).prompt()?;
  Ok(components)
}