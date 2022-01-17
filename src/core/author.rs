use getset::Getters;
use crate::lib::errors::Errors;

#[derive(Getters)]
pub struct Author {
  #[getset(get = "pub")]
  username: String,

  #[getset(get = "pub")]
  email: String,
}

impl Author {
  pub fn get() -> Result<Self, Errors> {
    todo!()
  }

  pub fn set(username: &str, email: &str) -> Result<(), Errors> {
    todo!()
  }
}
