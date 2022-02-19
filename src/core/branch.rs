use std::{fs::{File, OpenOptions, self}, io::{Read, Write}};
use getset::Getters;
use crate::lib::{errors::Errors, locale, constants::BRANCHES_PATH, object::FromId};
use super::commit::Commit;

#[derive(Getters, Debug, PartialEq, Clone)]
pub struct Branch {
  #[getset(get = "pub")]
  name: String,

  #[getset(get = "pub")]
  commit_id: Option<String>,
}

impl Branch {
  pub fn new(name: &str, commit_id: Option<&str>) -> Result<Self, Errors> {
    let location = locale().join(BRANCHES_PATH).join(name);

    if location.exists() {
      return Err(Errors::ExistingBranch(String::from(name)));
    }

    File::create(location)?.write_all(commit_id.unwrap_or("").as_bytes())?;

    Ok(
      Branch {
        name: String::from(name),
        commit_id: commit_id.map(|commit_id| String::from(commit_id)),
      }
    )
  }

  pub fn get(name: &str) -> Result<Self, Errors> {
    let location = locale().join(BRANCHES_PATH).join(name);

    if !location.exists() {
      return Err(Errors::UnrecognisedBranch(String::from(name)));
    }

    let mut commit_id = String::new();
    File::open(location)?.read_to_string(&mut commit_id)?;

    let commit_id = if commit_id != "" {
      Some(commit_id)
    } else {
      None
    };

    Ok(
      Branch {
        name: String::from(name),
        commit_id,
      }
    )
  }

  pub fn set(name: &str, commit_id: &str) -> Result<(), Errors> {
    let location = locale().join(BRANCHES_PATH).join(name);

    if !location.exists() {
      return Err(Errors::UnrecognisedBranch(String::from(name)));
    }

    OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(location)?
      .write_all(commit_id.as_bytes())?;

    Ok(())
  }

  pub fn remove(name: &str) -> Result<(), Errors> {
    let location = locale().join(BRANCHES_PATH).join(name);

    if !location.exists() {
      return Err(Errors::UnrecognisedBranch(String::from(name)));
    }

    fs::remove_file(location)?;

    Ok(())
  }

  pub fn commit(&self) -> Option<Commit> {
    if let Some(commit_id) = &self.commit_id {
      Some(Commit::from_id(commit_id).unwrap())
    } else {
      None
    }
  }
}
