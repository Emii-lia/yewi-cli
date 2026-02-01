use inquire::error::InquireResult;
use inquire::Select;
use crate::types::color::Color;

pub fn inquire_init() -> InquireResult<Color> {
  let color = Select::new("Select a theme: ", Color::get_colors())
    .prompt()?;

  Ok(color)
}