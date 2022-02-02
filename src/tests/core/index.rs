use std::{fs::File, io::Read};
use serial_test::serial;
use crate::{tests::{run_unit, gens::tree}, lib::{locale, constants::INDEX_PATH, decompress, errors::Errors}, core::index::Index};

#[test]
#[serial]
fn add_given_existing_file_paths_inserts_to_index() {
  run_unit(|| {
    let locale = locale();
    let paths = &[locale.join("a/a/a"), locale.join("a/a/b")];
    let tree = tree();
    let blob_1 = tree.get("a/a/a").unwrap().into_blob().unwrap();
    let blob_2 = tree.get("a/a/b").unwrap().into_blob().unwrap();

    tree.unpack(&locale).unwrap();

    Index::add(paths).unwrap();

    let mut compressed_data = Vec::new();
    File::open(locale.join(INDEX_PATH)).unwrap().read_to_end(&mut compressed_data).unwrap();

    let text = String::from_utf8(decompress(compressed_data).unwrap()).unwrap();

    assert!(text.contains(&format!("a/a/a {}", blob_1.id())));
    assert!(text.contains(&format!("a/a/b {}", blob_2.id())));
  });
}

#[test]
#[serial]
fn add_given_not_existing_file_path_returns_unrecognised_path_error() {
  run_unit(|| {
    let locale = locale();
    let paths = &[locale.join("a/a/a"), locale.join("d/a/a")];
    tree().unpack(&locale).unwrap();

    if let Err(Errors::UnrecognisedPath(path)) = Index::add(paths) {
      assert_eq!(path, locale.join("d/a/a"));
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn remove_given_file_paths_removes_from_index() {
  run_unit(|| {
    let locale = locale();
    let paths = &[locale.join("a/a/a"), locale.join("a/a/b")];
    tree().unpack(&locale).unwrap();
    Index::add(paths).unwrap();

    Index::remove(paths).unwrap();

    let mut data = Vec::new();
    File::open(locale.join(INDEX_PATH)).unwrap().read_to_end(&mut data).unwrap();

    assert_eq!(data, []);
  });
}
