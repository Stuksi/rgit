pub mod errors;
pub mod constants;

use camino::Utf8PathBuf;
use self::errors::Errors;

pub fn locale() -> Utf8PathBuf {
  todo!()
}

pub fn hash(bytes: &[u8]) -> String {
  todo!()
}

pub fn compress(bytes: &[u8]) -> Result<Vec<u8>, Errors> {
  todo!()
}

pub fn decompress(bytes: &[u8]) -> Result<Vec<u8>, Errors> {
  todo!()
}
