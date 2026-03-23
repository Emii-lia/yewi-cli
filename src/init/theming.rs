use crate::handlers::inquire_init::inquire_init;
use crate::types::color::Color;
use crate::utils::shade::is_valid_hex;

pub fn init_theme(theme: Option<String>) -> Result<String, String> {
  let color = match theme {
    Some(t) => {
      match Color::from_str(&t) {
        Color::Custom(s) => {
          if is_valid_hex(&s) {
            s
          } else {
            return Err(format!("Invalid hex color code: {}", s));
          }
        },
        _ => t,
      }
    },
    None => {
      let selected = inquire_init()
        .map_err(|e| format!("Failed to determine theme settings: {}", e))?;
      
      match selected {
        Color::Custom(c) => c,
        _ => selected.to_string(),
      }
    },
  };
  
  Ok(color)
}