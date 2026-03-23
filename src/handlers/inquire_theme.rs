use inquire::error::InquireResult;
use inquire::Text;
use inquire::validator::Validation;
use crate::utils::shade::is_valid_hex;

pub fn inquire_custom_theme() -> InquireResult<String> {
  let validator = |s: &str| if is_valid_hex(s) {
    Ok(Validation::Valid)
  } else {
    Ok(Validation::Invalid("Please enter a valid hex color code.".into()))
  };

  let theme = Text::new("Enter custom theme color (hex format): ")
    .with_placeholder("#rrggbb or #rgb or rrggbb or rgb")
    .with_validator(validator)
    .prompt()?;

  Ok(theme)
}