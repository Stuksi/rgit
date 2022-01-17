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
  pub fn create(name: &str, commit_id: &str) -> Result<(), Errors> {
    todo!()
  }

  pub fn read(name: &str) -> Result<Self, Errors> {
    todo!()
  }

  pub fn update(name: &str, commit_id: &str) -> Result<(), Errors> {
    todo!()
  }

  pub fn delete(name: &str) -> Result<(), Errors> {
    todo!()
  }

  pub fn commit() -> Option<Commit> {
    todo!()
  }
}
