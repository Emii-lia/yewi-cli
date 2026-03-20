use clap::ArgAction;
use std::collections::HashSet;
use std::error::Error;
use clap::{Parser, Subcommand};
use crate::add::add;
use crate::handlers::select_components::select_components;
use crate::init::create;
use crate::list::list;

mod add;
mod utils;
mod types;
mod handlers;
mod init;
mod list;

#[derive(Subcommand)]
enum Commands {
  #[command(
    alias = "create",
    about = "Create a new Yewi project",
    long_about = "Create a new Yewi project with all pre-configured settings and dependencies. Alias: 'create'",
    override_usage = "yewi new <project_name>",
  )]
  New {
    project_name: String,
    #[arg(long, short, required = false, help = "Optional theme color for the new project. If not specified, you will be prompted to choose a theme interactively.")]
    theme: Option<String>,
    #[arg(long, short, required = false, action = ArgAction::SetTrue ,help = "Optional i18n setting for the new project. If not specified, you will be prompted to choose a i18n setting interactively.")]
    i18n: Option<bool>,
  },
  #[command(
    aliases = &["install", "i"],
    about = "Add one or more Yewi components to an existing project.",
    long_about = "Add one or more Yewi components to an existing project. You can specify multiple component names. Aliases: 'install', 'i'",
    override_usage = "yewi add <component_name1> <component_name2> ...",
  )]
  Add {
    component_names: Option<Vec<String>>
  },
  #[command(about = "List all available Yewi components")]
  List
}
#[derive(Parser)]
#[command(name = "yewi")]
#[command(version, about, author = "Emii-lia")]
#[command(next_line_help = true)]
#[command(
  about = "Yewi CLI - A tool to manage Yewi components",
  long_about = None,
  override_usage = "\n\n yewi new <project_name>\n yewi add <component_name1> <component_name2> ..."
)]
struct Cli {
  #[command(subcommand)]
  command: Commands
}

fn main() -> Result<(), Box<dyn Error>> {
  let cli = Cli::parse();

  match cli.command {
    Commands::New { project_name, theme, i18n } => {
      create(&project_name, theme, i18n)?;
    }
    Commands::Add { component_names } => {
      let component_names = component_names.unwrap_or_else(|| {
        let components = list();
        select_components(components).expect("Failed to select components. Please try again.")
      });
      for component_name in component_names {
        let mut added = HashSet::new();
        add(&component_name, &mut added)?;
      };
    },
    Commands::List => {
      for component in list::list() {
        println!("- {}", component);
      }
    }
  }

  Ok(())
}
