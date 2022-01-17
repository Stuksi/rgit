use std::{fs::File, io::Read};
use serial_test::serial;
use crate::{core::{blob::Blob, repository::Repository}, lib::decompress, tests::run_unit};

const DUMMY_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
const DUMMY_TEXT_HASH: &str = "e7505beb754bed863e3885f73e3bb6866bdd7f8c";

#[test]
#[serial]
fn new_given_bytes_creates_blob() {
  run_unit(|| {
    let blob = Blob::new(DUMMY_TEXT.as_bytes());
    let mut compressed_blob_bytes = Vec::new();
    File::open(Repository::locate_object(blob.id())).unwrap().read_to_end(&mut compressed_blob_bytes).unwrap();
    let blob_content = decompress(&compressed_blob_bytes).unwrap();

    assert_eq!(blob.id(), DUMMY_TEXT_HASH);
    assert_eq!(blob_content, DUMMY_TEXT.as_bytes());
  });
}
