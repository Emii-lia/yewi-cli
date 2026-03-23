use inquire::error::InquireResult;
use inquire::Select;
use crate::handlers::inquire_theme::inquire_custom_theme;
use crate::types::color::Color;

pub fn inquire_init() -> InquireResult<Color> {
  let choice = Select::new("Select a theme: ", Color::get_colors())
    .prompt()?;

  let color = match choice {
    Color::Custom(_) => {
      let custom_color = inquire_custom_theme()
        .map_err(|e| format!(" Failed to determine custom theme settings: {}", e))
        .unwrap();
      Color::Custom(custom_color)
    }
    _ => choice,
  };

  Ok(color)
}