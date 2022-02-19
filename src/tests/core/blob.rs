use std::{fs::File, io::{Read, Write}};
use serial_test::serial;
use crate::{core::blob::Blob, lib::{decompress, constants::{OBJECTS_PATH, BLOB_TYPE}, locale, errors::Errors, object::FromId}, tests::run_unit};

const DUMMY_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
const BLOB_HASH: &str = "52837ea8d4dbe8c130c2d829727ef511bcc262a1";

#[test]
#[serial]
fn new_given_bytes_creates_blob() {
  run_unit(|| {
    let blob = Blob::new(DUMMY_TEXT).unwrap();

    let mut compressed = Vec::new();
    let location = locale().join(OBJECTS_PATH).join(&blob.id()[..2]).join(&blob.id()[2..]);
    File::open(location).unwrap().read_to_end(&mut compressed).unwrap();

    assert_eq!(blob.id(), BLOB_HASH);
    assert_eq!(decompress(&compressed).unwrap(), [BLOB_TYPE, DUMMY_TEXT].concat().as_bytes());
  });
}

#[test]
#[serial]
fn from_path_given_path_creates_blob() {
  run_unit(|| {
    File::create(locale().join("blob")).unwrap().write_all(&mut DUMMY_TEXT.as_bytes()).unwrap();
    let blob = Blob::from_path(locale().join("blob")).unwrap();

    let mut compressed = Vec::new();
    let location = locale().join(OBJECTS_PATH).join(&blob.id()[..2]).join(&blob.id()[2..]);
    File::open(location).unwrap().read_to_end(&mut compressed).unwrap();

    assert_eq!(blob.id(), BLOB_HASH);
    assert_eq!(decompress(&compressed).unwrap(), [BLOB_TYPE, DUMMY_TEXT].concat().as_bytes());
  });
}

#[test]
#[serial]
fn from_id_given_non_existing_id_return_unknown_object_error() {
  run_unit(|| {
    if let Err(Errors::UnrecognisedObject(id)) = Blob::from_id("invalid-id") {
      assert_eq!(id, "invalid-id");
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn from_id_given_existing_id_returns_blob() {
  run_unit(|| {
    let blob = Blob::new(DUMMY_TEXT).unwrap();
    let from_blob = Blob::from_id(blob.id()).unwrap();

    assert_eq!(blob, from_blob);
  });
}
