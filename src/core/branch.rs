use getset::Getters;
use crate::lib::errors::Errors;
use super::commit::Commit;

#[derive(Getters)]
pub struct Branch {
  #[getset(get = "pub")]
  name: String,

  #[getset(get = "pub")]
  commit_id: String,
}

impl Branch {
  pub fn new(name: &str, commit_id: &str) -> Result<Self, Errors> {
    todo!()
  }

  pub fn get(name: &str) -> Result<Self, Errors> {
    todo!()
  }

  pub fn set(name: &str, commit_id: &str) -> Result<(), Errors> {
    todo!()
  }

  pub fn remove(name: &str) -> Result<(), Errors> {
    todo!()
  }

  pub fn commit(&self) -> Option<Commit> {
    todo!()
  }
}
