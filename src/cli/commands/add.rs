use std::path::PathBuf;
use crate::{lib::errors::Errors, core::index::Index, cli::configure_input_paths};

pub fn add(paths: Vec<PathBuf>) -> Result<(), Errors> {
  let file_paths = configure_input_paths(&paths)?;
  Index::add(&file_paths)
}
