use getset::Getters;
use crate::lib::errors::Errors;
use super::{branch::Branch, commit::Commit};

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
    todo!()
  }

  pub fn set(reference: Reference) -> Result<Self, Errors> {
    todo!()
  }

  pub fn commit() -> Option<Commit> {
    todo!()
  }
}
