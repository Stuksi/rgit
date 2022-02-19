use std::{fs::{File, self}, io::{Read, Write}};
use lipsum::lipsum;
use serial_test::serial;
use crate::{tests::{run_unit, factory::{tree, commit}}, lib::{locale, constants::INDEX_PATH, decompress}, core::{index::Index, head::{Head, Reference}, blob::Blob}};

#[test]
#[serial]
fn add_given_untracked_file_paths_inserts_to_index() {
  run_unit(|| {
    let locale = locale();
    let tree = tree();
    tree.unpack(&locale).unwrap();

    let blob_1 = tree.get("a/a/a").unwrap().into_blob().unwrap();
    let blob_2 = tree.get("a/a/b").unwrap().into_blob().unwrap();

    Index::add(&[locale.join("a/a/a"), locale.join("a/a/b")]).unwrap();

    let mut compressed_data = Vec::new();
    File::open(locale.join(INDEX_PATH)).unwrap().read_to_end(&mut compressed_data).unwrap();

    let text = String::from_utf8(decompress(compressed_data).unwrap()).unwrap();

    assert!(text.contains(&format!("a/a/a {}", blob_1.id())));
    assert!(text.contains(&format!("a/a/b {}", blob_2.id())));
  });
}

#[test]
#[serial]
fn add_given_modified_file_paths_inserts_to_index() {
  run_unit(|| {
    let locale = locale();
    let commit = commit();
    commit.tree().unwrap().unpack(&locale).unwrap();

    Head::set(&Reference::Commit(commit)).unwrap();
    File::create(locale.join("a/a/a")).unwrap().write_all(lipsum(20).as_bytes()).unwrap();
    File::create(locale.join("a/a/b")).unwrap().write_all(lipsum(20).as_bytes()).unwrap();

    let blob_1 = Blob::from_path(locale.join("a/a/a")).unwrap();
    let blob_2 = Blob::from_path(locale.join("a/a/b")).unwrap();

    Index::add(&[locale.join("a/a/a"), locale.join("a/a/b")]).unwrap();

    let mut compressed_data = Vec::new();
    File::open(locale.join(INDEX_PATH)).unwrap().read_to_end(&mut compressed_data).unwrap();

    let text = String::from_utf8(decompress(compressed_data).unwrap()).unwrap();

    assert!(text.contains(&format!("a/a/a {}", blob_1.id())));
    assert!(text.contains(&format!("a/a/b {}", blob_2.id())));
  });
}

#[test]
#[serial]
fn add_given_deleted_file_paths_inserts_to_index() {
  run_unit(|| {
    let locale = locale();
    let commit = commit();
    commit.tree().unwrap().unpack(&locale).unwrap();

    Head::set(&Reference::Commit(commit)).unwrap();
    fs::remove_dir_all(locale.join("a/a")).unwrap();

    Index::add(&[locale.join("a/a/a"), locale.join("a/a/b")]).unwrap();

    let mut compressed_data = Vec::new();
    File::open(locale.join(INDEX_PATH)).unwrap().read_to_end(&mut compressed_data).unwrap();

    let text = String::from_utf8(decompress(compressed_data).unwrap()).unwrap();

    assert!(text.contains(&format!("a/a/a DELETED")));
    assert!(text.contains(&format!("a/a/b DELETED")));
  });
}

#[test]
#[serial]
fn remove_given_file_paths_removes_from_index() {
  run_unit(|| {
    let locale = locale();

    tree().unpack(&locale).unwrap();
    Index::add(&[locale.join("a/a/a"), locale.join("a/a/b")]).unwrap();

    Index::remove(&[locale.join("a/a/a"), locale.join("a/a/b")]).unwrap();

    let mut data = Vec::new();
    File::open(locale.join(INDEX_PATH)).unwrap().read_to_end(&mut data).unwrap();

    assert!(data.is_empty());
  });
}
