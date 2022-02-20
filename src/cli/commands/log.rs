use std::{io::Write, process::Command};
use tempfile::NamedTempFile;
use crate::{lib::errors::Errors, core::head::Head};

pub fn log() -> Result<(), Errors> {
  let head = Head::get()?;
  let mut current_commit = head.commit();
  let mut log = String::new();

  while let Some(commit) = current_commit {
    log += &format!("commit {}\n", commit.id());
    log += &format!("Author: {} <{}>\n", commit.author().username(), commit.author().email());
    log += &format!("Date:   {}\n", commit.date());
    log += &format!("\n    {}\n\n", commit.message());

    current_commit = commit.parent();
  }

  let mut temp_file = NamedTempFile::new()?;
  temp_file.write_all(log.trim_end().as_bytes())?;

  Command::new("less")
    .arg(temp_file.path())
    .spawn()?
    .wait()?;

  Ok(())
}
