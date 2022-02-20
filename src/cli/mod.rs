mod commands;

use std::{env, path::PathBuf, fs};
use camino::Utf8PathBuf;
use clap::Parser;
use path_clean::PathClean;
use crate::lib::{errors::Errors, constants::{PROJECT_ENV, REPOSITORY_PATH, REPOSITORY_FOLDER_NAME}, locale};
use commands::init::init;
use self::commands::{Commands, add::add, restore::restore, commit::commit, switch::switch, config::config, status::status, branch::branch, log::log};

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
      Commands::Status => status(),
      Commands::Branch { delete } => branch(delete),
      Commands::Log => log(),
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

    Err(Errors::MissingRepository)
  }
}

pub fn configure_input_paths(paths: &[PathBuf]) -> Result<Vec<Utf8PathBuf>, Errors> {
  let mut configured_paths = Vec::new();
  let locale = locale();

  for path in paths {
    if path.is_absolute() {
      let utf8_path = Utf8PathBuf::from_path_buf(path.clean()).map_err(|_| Errors::BadPathEncoding)?;

      if !utf8_path.starts_with(&locale) {
        return Err(Errors::UnrecognisedPath(utf8_path));
      }

      configured_paths.push(utf8_path);
    } else {
      let utf8_path = Utf8PathBuf::from_path_buf(locale.as_std_path().join(path).clean()).map_err(|_| Errors::BadPathEncoding)?;
      configured_paths.push(utf8_path);
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
