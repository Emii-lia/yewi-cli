use inquire::error::InquireResult;

pub fn inquire_i18n() -> InquireResult<bool> {
  let has_i18n = inquire::Confirm::new("Do you want to use i18n?")
    .with_default(false)
    .prompt()?;
  Ok(has_i18n)
}