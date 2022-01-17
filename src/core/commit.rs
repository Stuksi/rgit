use chrono::{DateTime, Utc};
use getset::Getters;
use crate::lib::errors::Errors;
use super::{author::Author, tree::Tree, repository::RepositoryObject};

#[derive(Getters)]
pub struct Commit {
  #[getset(get = "pub")]
  id: String,

  #[getset(get = "pub")]
  parent_id: String,

  #[getset(get = "pub")]
  tree_id: String,

  #[getset(get = "pub")]
  date: DateTime<Utc>,

  #[getset(get = "pub")]
  author: Author,
}

impl Commit {
  pub fn new() -> Self {
    todo!()
  }

  pub fn tree() -> Tree {
    todo!()
  }

  pub fn parent() -> Option<Commit> {
    todo!()
  }
}

impl RepositoryObject for Commit {
  fn from_id(id: &str) -> Result<Self, Errors> where Self: Sized {
    todo!()
  }

  fn pack(&self) -> Result<String, Errors> {
    todo!()
  }

  fn unpack<P: AsRef<camino::Utf8Path>>(&self, prefix: P) -> Result<(), Errors> {
    todo!()
  }
}