use std::collections::HashSet;
use std::env::temp_dir;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use crate::add::add;
use crate::init::create;

static CWD_LOCK: Mutex<()> = Mutex::new(());

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
fn add_component_to_invalid_project_structure() {
  let _guard = CWD_LOCK.lock().unwrap();
  let project = ProjectBuf::new("yew-test-add-invalid-project-structure");
  fs::create_dir_all(&project.path).unwrap();

  std::env::set_current_dir(&project.path).unwrap();

  let result = add("button", &mut HashSet::new());
  assert!(result.is_err());
  assert!(result.unwrap_err().to_string().contains("must be run inside a Yewi project directory"));
}

#[test]
fn add_component_to_existing_project() {
  let _guard = CWD_LOCK.lock().unwrap();
  let project_path = ProjectBuf::new("yew-test-add-existing-project");
  let project = project_path.path.clone();

  create(
    project.to_str().unwrap(),
    Some("sky".into()),
    Some(false)
  ).unwrap();

  std::env::set_current_dir(project.clone()).unwrap();
  add("button", &mut HashSet::new()).unwrap();

  assert!(project.join("src").join("components").join("button").exists());
  let component_mod = project.join("src").join("components").join("mod.rs");
  assert!(component_mod.exists());
  let content = std::fs::read_to_string(component_mod).unwrap();
  assert!(content.contains("mod button;"));
  let scss_file = project.join("src").join("styles").join("components.scss");
  assert!(scss_file.exists());
  let content = std::fs::read_to_string(scss_file).unwrap();
  assert!(content.contains("@use \"../components/button/button\";"));
}

#[test]
fn add_existing_component() {
  let _guard = CWD_LOCK.lock().unwrap();
  let project_path = ProjectBuf::new("yew-test-add-existing-component");
  let project = project_path.path.clone();
  create(
    project.to_str().unwrap(),
    Some("sky".into()),
    Some(false)
  ).unwrap();

  std::env::set_current_dir(project.clone()).unwrap();
  add("button", &mut HashSet::new()).unwrap();
  let result = add("button", &mut HashSet::new());
  assert!(result.is_ok());
  assert!(fs::remove_dir_all(project).is_ok());
}

#[test]
fn add_dependencies_before_component() {
  let _guard = CWD_LOCK.lock().unwrap();
  let project_path = ProjectBuf::new("yew-test-add-dependencies-before-component");
  let project = project_path.path.clone();

  create(
    project.to_str().unwrap(),
    Some("sky".into()),
    Some(false)
  ).unwrap();

  std::env::set_current_dir(project.clone()).unwrap();
  let mut added = HashSet::new();

  add("modal", &mut added);

  assert!(project.join("src").join("components").join("button").exists());
  assert!(project.join("src").join("components").join("modal").exists());
  assert!(added.contains("button"));
  assert!(fs::remove_dir_all(project).is_ok());
}