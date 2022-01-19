use super::errors::Errors;

pub trait FromId {
  fn from(id: &str) -> Result<Self, Errors> where Self: Sized;
}

pub enum Object {
  Blob,
  Tree,
  Commit,
}
