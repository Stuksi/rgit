use camino::{Utf8Path, Utf8PathBuf};
use crate::lib::errors::Errors;

pub struct Repository;

impl Repository {
  pub fn initialize() -> Result<(), Errors> {
    todo!()
  }

  pub fn locate_object(id: &str) -> Utf8PathBuf {
    todo!()
  }
}

pub trait RepositoryObject {
  fn from_id(id: &str) -> Result<Self, Errors> where Self: Sized;
  fn pack(&self) -> Result<String, Errors>;
  fn unpack<P: AsRef<Utf8Path>>(&self, prefix: P) -> Result<(), Errors>;
}
