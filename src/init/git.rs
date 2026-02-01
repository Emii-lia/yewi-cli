use std::env::temp_dir;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use crate::init::download_file::download_file;
use crate::init::extract_zip::extract_zip;
use crate::utils::constants::get_repo_config;

const TEMPLATE_REPO: &str = "yew-app-template";

fn is_curl_available() -> bool {
  Command::new("curl")
    .arg("--version")
    .output()
    .map(|output| output.status.success())
    .unwrap_or(false)
}
pub(crate) fn clone_with_git(project_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
  let (repo_owner, _repo_name, _repo_branch, _raw_github_url) = get_repo_config();
  let repo_url = format!(
    "https://github.com/{}/{}.git",
    repo_owner, TEMPLATE_REPO
  );
  let parent_dir = project_dir
    .parent()
    .ok_or("Failed to get parent directory")?;
  let project_name = project_dir
    .file_name()
    .ok_or("Failed to get project directory name")?;

  let git_paths = vec!["git", "/usr/bin/git", "/usr/local/bin/git"];

  for git_cmd in git_paths {
    let result = Command::new(git_cmd)
      .arg("clone")
      .arg("--depth")
      .arg("1")
      .arg(&repo_url)
      .arg(project_name)
      .current_dir(parent_dir)
      .output();

    match result {
      Ok(output) => {
        if output.status.success() {
          return Ok(());
        } else {
          let stderr = String::from_utf8_lossy(&output.stderr);
          return Err(format!(
            "Failed to clone: {}\n{}",
            &repo_url, stderr
          ).into());
        }
      }
      Err(_) => {
        continue;
      }
    }
  }

  Err("Git not found in any standard location. Please install git: https://git-scm.com/downloads".into())
}

pub(crate) fn clone_with_api(project_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
  if !is_curl_available() {
    return Err(
      "curl is not installed. Please install curl or use git to clone the template.\n\
       Install: https://curl.se/download.html".into()
    );
  }

  let (repo_owner, _repo_name, repo_branch, _raw_github_url) = get_repo_config();
  let temp_zip = temp_dir().join("yew_app_template.zip");

  let zip_url = format!(
    "https://github.com/{}/{}/archive/refs/heads/{}.zip",
    repo_owner, TEMPLATE_REPO, repo_branch
  );

  println!("  From: {}", zip_url);
  download_file(&zip_url, &temp_zip)?;

  extract_zip(&temp_zip, project_dir)?;

  fs::remove_file(&temp_zip).ok();

  Ok(())
}

pub(crate) fn is_git_available() -> bool {
  let paths = vec![
    "git",
    "/usr/bin/git",
    "/usr/local/bin/git",
    "C:\\Program Files\\Git\\cmd\\git.exe",
  ];

  for git_path in paths {
    if let Ok(output) = Command::new(git_path)
      .arg("--version")
      .output()
    {
      if output.status.success() {
        return true;
      }
    }
  }

  false
}