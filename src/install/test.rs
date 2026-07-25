use std::env::temp_dir;
use std::fs;
use std::path::PathBuf;
use crate::init::create;
use crate::install::install;
use crate::utils::path::CWD_LOCK;

struct ProjectBuf {
  pub path: PathBuf,
}

impl ProjectBuf {
  fn new(name: &str) -> Self {
    let path = temp_dir().join(format!("{}-{}", name, std::process::id()));
    if path.exists() {
      fs::remove_dir_all(&path).ok();
    }

    Self { path }
  }
}

impl Drop for ProjectBuf {
  fn drop(&mut self) {
    if self.path.exists() {
      fs::remove_dir_all(&self.path).ok();
    }
  }
}

#[test]
fn install_creates_global_css() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let project_path = ProjectBuf::new("yew-test-install-creates-global-css");
  let project = project_path.path.clone();

  let original_cwd = std::env::current_dir().unwrap();

  create(
    project.to_str().unwrap(),
    Some("sky".into()),
    Some(true),
    Some("npm".to_string())
  ).unwrap();

  std::env::set_current_dir(project.clone()).unwrap();

  install().unwrap();

  let global_css = project.join("src").join("styles").join("global.css");
  assert!(global_css.exists());
  let content = std::fs::read_to_string(global_css).unwrap();
  assert!(!content.is_empty());
  std::env::set_current_dir(&original_cwd).unwrap();
}