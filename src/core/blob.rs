use getset::Getters;
use crate::lib::{errors::Errors, hash};

#[derive(Getters)]
pub struct Blob {
  #[getset(get = "pub")]
  id: String,
}

impl Blob {
  pub fn new(bytes: &[u8]) -> Self {
    Blob {
      id: hash(bytes),
    }
  }
}
