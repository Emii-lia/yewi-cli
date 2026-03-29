use std::env::temp_dir;
use std::path::PathBuf;
use crate::init::create;
use crate::update::update;
use crate::utils::path::CWD_LOCK;
use crate::utils::shade::{shades_of, ShadeKey};

struct ProjectBuf {
  pub path: PathBuf,
}

impl ProjectBuf {
  fn new(name: &str) -> Self {
    let path = temp_dir().join(format!("{}-{}", name, std::process::id()));
    if path.exists() {
      std::fs::remove_dir_all(&path).ok();
    }

    Self { path }
  }
}

impl Drop for ProjectBuf {
  fn drop(&mut self) {
    if self.path.exists() {
      std::fs::remove_dir_all(&self.path).ok();
    }
  }
}

#[test]
fn update_an_invalid_project_structure() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let project_buf = ProjectBuf::new("yew-update-invalid-project");
  let original_cwd = std::env::current_dir().unwrap();
  std::fs::create_dir_all(&project_buf.path).unwrap();

  std::env::set_current_dir(&project_buf.path).unwrap();

  let result = update(Some("slate".to_string()));
  assert!(result.is_err());
  assert!(result.unwrap_err().to_string().contains("must be run inside a Yewi project directory"));
  std::env::set_current_dir(&original_cwd).unwrap();
}

#[test]
fn update_theme_to_an_existing_project() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let project_buf = ProjectBuf::new("yew-update-theme");
  let project = project_buf.path.clone();
  let theme = "zinc".to_string();
  let new_theme = "emerald".to_string();
  let original_cwd = std::env::current_dir().unwrap();

  create(
    project.to_str().unwrap(),
    Some(theme.clone()),
    Some(true)
  ).unwrap();

  std::env::set_current_dir(&project).unwrap();
  let result = update(Some(String::from(new_theme.clone())));

  assert!(result.is_ok());
  assert!(project.join("src").exists());
  let main_style = project.join("src").join("styles").join("main.scss");
  assert!(main_style.exists());
  let content = std::fs::read_to_string(main_style).unwrap();
  let shades = ShadeKey::shades();
  for shade in shades {
    match shade {
      ShadeKey::U(key) => {
        assert!(content.contains(&format!("--primary-{}: theme(\"colors.{}.{}\")", key, new_theme.clone(), key)));
      }
      ShadeKey::Default => {
        assert!(content.contains("--primary: var(--primary-500)"))
      }
    }
  }
  std::env::set_current_dir(&original_cwd).unwrap();
}

#[test]
fn update_custom_theme_to_an_existing_project() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let original_cwd = std::env::current_dir().unwrap();
  let project_buf = ProjectBuf::new("yew-update-custom-theme");
  let project = project_buf.path.clone();
  let hex = "#eb9868";

  create(
    project.to_str().unwrap(),
    Some("zinc".to_string()),
    Some(true)
  ).unwrap();
  std::env::set_current_dir(&project).unwrap();

  let result = update(Some(hex.into()));
  assert!(result.is_ok());

  let shades = shades_of(hex).unwrap();
  let main_style = project.join("src").join("styles").join("main.scss");
  assert!(main_style.exists());

  let content = std::fs::read_to_string(main_style).unwrap();
  for (shade, value) in shades {
    match shade {
      ShadeKey::U(key) => {
        assert!(content.contains(&format!("--primary-{}: {}", key, value)));
      }
      ShadeKey::Default => {
        assert!(content.contains("--primary: var(--primary-500)"))
      }
    }
  }
  std::env::set_current_dir(&original_cwd).unwrap();
}