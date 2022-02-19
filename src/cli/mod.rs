mod commands;

use std::{env, path::PathBuf, fs};
use camino::Utf8PathBuf;
use clap::Parser;
use path_clean::PathClean;
use crate::lib::{errors::Errors, constants::{PROJECT_ENV, REPOSITORY_PATH, REPOSITORY_FOLDER_NAME}, locale};
use commands::init::init;
use self::commands::{Commands, add::add, restore::restore, commit::commit, switch::switch, config::config};

#[derive(Parser)]
#[clap(name = "rgit")]
#[clap(author = "Stoyan Grozdanov <grozdanovstoqn01@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "Version control system developed in rust")]
pub struct Interface {
  #[clap(subcommand)]
  commands: Commands,
}

impl Interface {
  pub fn run() -> Result<(), Errors> {
    let interface = Self::parse();

    if let Commands::Init = interface.commands {
      return init();
    }

    Self::detect_repository()?;

    match interface.commands {
      Commands::Add { paths } => add(paths),
      Commands::Restore { paths } => restore(paths),
      Commands::Commit { message } => commit(message),
      Commands::Switch { new, commit, target } => switch(new, commit, target),
      Commands::Config { username, email } => config(username, email),
      _ => Ok(())
    }
  }

  // private

  fn detect_repository() -> Result<(), Errors> {
    let current_dir = env::current_dir()?;
    let mut current_path = Some(current_dir.as_path());

    while let Some(path) = current_path {
      if path.join(REPOSITORY_PATH).exists() {
        env::set_var(PROJECT_ENV, path);
        return Ok(());
      }

      current_path = path.parent();
    }

    Err(Errors::NotARepositoryError)
  }
}

pub fn configure_input_paths(paths: &[PathBuf]) -> Result<Vec<Utf8PathBuf>, Errors> {
  let mut configured_paths = Vec::new();
  let locale = locale();

  for path in paths {
    let path = Utf8PathBuf::from_path_buf(PathBuf::from(path).clean()).map_err(|_| Errors::BadFilePath)?;

    if path.is_absolute() {
      if !path.starts_with(&locale) {
        return Err(Errors::BadFilePath);
      }

      configured_paths.push(Utf8PathBuf::from(path));
    } else {
      configured_paths.push(locale.join(path));
    }
  }

  Ok(configured_paths)
}

pub fn clear_project() -> Result<(), Errors> {
  for entry in locale().read_dir()? {
    let entry = entry?;
    let path = entry.path();

    if entry.file_name() != REPOSITORY_FOLDER_NAME {
      if path.is_dir() {
        fs::remove_dir_all(&path)?;
      } else {
        fs::remove_file(&path)?;
      }
    }
  }

  Ok(())
}
