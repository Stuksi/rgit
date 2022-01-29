use std::{fs::{File, OpenOptions}, io::{Read, Write}};
use getset::Getters;
use crate::lib::{errors::Errors, locale, constants::HEAD_PATH, object::FromId};
use super::{branch::Branch, commit::Commit};

#[derive(Debug, PartialEq)]
pub enum Reference {
  Branch(Branch),
  Commit(Commit),
}

#[derive(Getters)]
pub struct Head {
  #[getset(get = "pub")]
  reference: Reference,
}

impl Head {
  pub fn get() -> Result<Self, Errors> {
    let location = locale().join(HEAD_PATH);
    let mut reference = String::new();
    File::open(location)?.read_to_string(&mut reference)?;

    let reference = if reference.starts_with("detached:") {
      Reference::Commit(Commit::from_id(&reference[9..])?)
    } else {
      Reference::Branch(Branch::get(&reference)?)
    };

    Ok(
      Head {
        reference,
      }
    )
  }

  pub fn set(reference: &Reference) -> Result<(), Errors> {
    let location = locale().join(HEAD_PATH);
    let reference = match &reference {
      Reference::Branch(branch) => String::from(branch.name()),
      Reference::Commit(commit) => String::from("detached:") + commit.id(),
    };

    OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(location)?
      .write_all(reference.as_bytes())?;

    Ok(())
  }

  pub fn commit(&self) -> Option<Commit> {
    match &self.reference {
      Reference::Branch(branch) => branch.commit(),
      Reference::Commit(commit) => Some(commit.clone()),
    }
  }
}
