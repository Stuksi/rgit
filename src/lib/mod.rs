pub mod errors;
pub mod constants;
pub mod user;
pub mod object;

use std::{env, io::{Read, Write}, fs::{File, self}};
use camino::Utf8PathBuf;
use flate2::{read::{ZlibEncoder, ZlibDecoder}, Compression};
use sha1::{Sha1, Digest};
use self::{errors::Errors, constants::{PROJECT_ENV, OBJECTS_PATH, BLOB_TYPE, TREE_TYPE, COMMIT_TYPE, REPOSITORY_PATH, BRANCHES_PATH, HEAD_PATH, CONFIG_PATH}, object::Object};

pub fn locale() -> Utf8PathBuf {
  Utf8PathBuf::from(env::var(PROJECT_ENV).unwrap())
}

pub fn initialize() -> Result<(), Errors> {
  let locale = locale();
  let repository = locale.join(REPOSITORY_PATH);

  if repository.exists() {
    return Err(Errors::ExistingRepositoryError);
  }

  let objects = locale.join(OBJECTS_PATH);
  let branches = locale.join(BRANCHES_PATH);
  let head = locale.join(HEAD_PATH);
  let config = locale.join(CONFIG_PATH);

  fs::create_dir_all(&repository)?;
  fs::create_dir_all(&objects)?;
  fs::create_dir_all(&branches)?;

  File::create(branches.join("master"))?;
  File::create(head)?.write_all(b"master")?;
  File::create(config)?.write_all(b"admin admin")?;

  Ok(())
}

pub fn hash<B: AsRef<[u8]>>(bytes: B) -> String {
  let mut hasher = Sha1::new();
  hasher.update(bytes);
  hex::encode(hasher.finalize())
}

pub fn compress<B: AsRef<[u8]>>(bytes: B) -> Result<Vec<u8>, Errors> {
  let mut compressed = Vec::new();
  ZlibEncoder::new(bytes.as_ref(), Compression::default()).read_to_end(&mut compressed)?;
  Ok(compressed)
}

pub fn decompress<B: AsRef<[u8]>>(bytes: B) -> Result<Vec<u8>, Errors> {
  let mut decompressed = Vec::new();
  ZlibDecoder::new(bytes.as_ref()).read_to_end(&mut decompressed)?;
  Ok(decompressed)
}

pub fn read_object_bytes(id: &str) -> Result<Vec<u8>, Errors> {
  let location = locale().join(OBJECTS_PATH).join(&id[..2]).join(&id[2..]);

  if !location.exists() {
    return Err(Errors::UnknownObjectError(String::from(id)));
  }

  let mut compressed = Vec::new();
  File::open(location)?.read_to_end(&mut compressed)?;

  let decompressed = decompress(&compressed)?;
  Ok(Vec::from(&decompressed[4..]))
}

pub fn write_object_bytes<B: AsRef<[u8]>>(object_type: Object, bytes: B) -> Result<String, Errors> {
  let tag = match object_type {
    Object::Blob => BLOB_TYPE,
    Object::Tree => TREE_TYPE,
    Object::Commit => COMMIT_TYPE,
  };

  let bytes = [tag.as_bytes(), bytes.as_ref()].concat();
  let compressed = compress(&bytes)?;

  let id = hash(&bytes);
  let location = locale().join(OBJECTS_PATH).join(&id[..2]).join(&id[2..]);

  if !location.exists() {
    fs::create_dir_all(location.parent().unwrap())?;
    File::create(location)?.write_all(&compressed)?;
  }

  Ok(id)
}
