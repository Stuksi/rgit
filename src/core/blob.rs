use getset::Getters;
use crate::lib::{errors::Errors, object::{FromId, Object}, write_object_bytes, locale, constants::OBJECTS_PATH};

#[derive(Getters, Debug, PartialEq, Clone)]
pub struct Blob {
  #[getset(get = "pub")]
  id: String,
}

impl Blob {
  pub fn new<B: AsRef<[u8]>>(bytes: B) -> Result<Self, Errors> {
    let id = write_object_bytes(Object::Blob, bytes)?;
    Ok(Blob { id })
  }
}

impl FromId for Blob {
  fn from_id(id: &str) -> Result<Blob, Errors> {
    let location = locale().join(OBJECTS_PATH).join(&id[..2]).join(&id[2..]);

    if !location.exists() {
      return Err(Errors::UnknownObjectError(String::from(id)));
    }

    Ok(
      Blob {
        id: String::from(id),
      }
    )
  }
}
