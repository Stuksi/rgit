pub mod errors;
pub mod constants;
pub mod user;
pub mod object;

use std::{env, io::{Read, Write}, fs::{File, self}, collections::HashSet};
use camino::{Utf8PathBuf, Utf8Path};
use flate2::{read::{ZlibEncoder, ZlibDecoder}, Compression};
use pathdiff::diff_utf8_paths;
use sha1::{Sha1, Digest};
use walkdir::WalkDir;
use self::{errors::Errors, constants::*, object::Object};

pub fn locale() -> Utf8PathBuf {
  Utf8PathBuf::from(env::var(PROJECT_ENV).unwrap())
}

pub fn relative<P: AsRef<Utf8Path>>(path: P) -> Utf8PathBuf {
  match diff_utf8_paths(path, locale()) {
    Some(path) => path,
    None => Utf8PathBuf::new()
  }
}

pub fn initialize() -> Result<(), Errors> {
  let locale = locale();
  let repository = locale.join(REPOSITORY_PATH);

  if repository.exists() {
    return Err(Errors::ExistingRepository);
  }

  let objects = locale.join(OBJECTS_PATH);
  let branches = locale.join(BRANCHES_PATH);
  let head = locale.join(HEAD_PATH);
  let config = locale.join(CONFIG_PATH);
  let index = locale.join(INDEX_PATH);

  fs::create_dir_all(&repository)?;
  fs::create_dir_all(&objects)?;
  fs::create_dir_all(&branches)?;

  File::create(branches.join("master"))?;
  File::create(head)?.write_all(b"master")?;
  File::create(config)?.write_all(b"admin admin")?;
  File::create(index)?;

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
    return Err(Errors::UnrecognisedObject(String::from(id)));
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

pub fn folder_files<P: AsRef<Utf8Path>>(path: P) -> Result<HashSet<Utf8PathBuf>, Errors> {
  let walker = WalkDir::new(locale().join(path))
    .into_iter()
    .filter_entry(|entry| entry.file_name().to_str().unwrap_or(REPOSITORY_FOLDER_NAME) != REPOSITORY_FOLDER_NAME);
  let mut file_paths = HashSet::new();

  for entry in walker {
    let entry = entry?;
    let entry_path = entry.into_path();

    if entry_path.is_file() {
      if let Ok(file_path) = Utf8PathBuf::from_path_buf(entry_path) {
        file_paths.insert(file_path);
      } else {
        return Err(Errors::BadPathEncoding);
      }
    }
  }

  Ok(file_paths)
}
