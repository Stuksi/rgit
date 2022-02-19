use std::path::PathBuf;
use crate::{lib::errors::Errors, cli::configure_input_paths, core::index::Index};

pub fn restore(paths: Vec<PathBuf>) -> Result<(), Errors> {
  let file_paths = configure_input_paths(&paths)?;
  Index::remove(&file_paths)
}
