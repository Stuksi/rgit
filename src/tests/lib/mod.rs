mod user;

use std::{fs::File, io::Read, collections::HashSet};
use camino::Utf8PathBuf;
use serial_test::serial;
use crate::{lib::{*, object::Object, constants::OBJECTS_PATH}, tests::factory::tree};
use super::{run_unit, factory::TREE_PATHS};

const DUMMY_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";

#[test]
#[serial]
fn initialize_creates_the_repository_folder_in_the_current_locale() {
  run_unit(|| {
    let locale = locale();

    assert!(locale.join(".rgit/").exists());
    assert!(locale.join(".rgit/objects/").exists());
    assert!(locale.join(".rgit/branches/").exists());
    assert!(locale.join(".rgit/HEAD").exists());
    assert!(locale.join(".rgit/index").exists());

    let mut config = String::new();
    File::open(locale.join(".rgit/config")).unwrap().read_to_string(&mut config).unwrap();

    let mut head = String::new();
    File::open(locale.join(".rgit/HEAD")).unwrap().read_to_string(&mut head).unwrap();

    let mut master = String::new();
    File::open(locale.join(".rgit/branches/master")).unwrap().read_to_string(&mut master).unwrap();

    assert_eq!(config, "admin admin");
    assert_eq!(head, "master");
    assert!(master.is_empty());
  });
}

#[test]
#[serial]
fn write_object_bytes_given_its_type_and_bytes_creates_an_object_and_returns_its_id() {
  run_unit(|| {
    let id_1 = write_object_bytes(Object::Blob, DUMMY_TEXT).unwrap();
    let id_2 = write_object_bytes(Object::Tree, DUMMY_TEXT).unwrap();
    let id_3 = write_object_bytes(Object::Commit, DUMMY_TEXT).unwrap();

    let objects_path = locale().join(OBJECTS_PATH);

    assert!(id_1 != id_2);
    assert!(id_2 != id_3);
    assert!(objects_path.join(&id_1[..2]).join(&id_1[2..]).exists());
    assert!(objects_path.join(&id_2[..2]).join(&id_2[2..]).exists());
    assert!(objects_path.join(&id_3[..2]).join(&id_3[2..]).exists());
  });
}

#[test]
#[serial]
fn read_object_bytes_given_an_id_returns_the_object_data() {
  run_unit(|| {
    let id_1 = write_object_bytes(Object::Blob, DUMMY_TEXT).unwrap();
    let id_2 = write_object_bytes(Object::Tree, DUMMY_TEXT).unwrap();
    let id_3 = write_object_bytes(Object::Commit, DUMMY_TEXT).unwrap();

    let text_1 = read_object_bytes(&id_1).unwrap();
    let text_2 = read_object_bytes(&id_2).unwrap();
    let text_3 = read_object_bytes(&id_3).unwrap();

    assert_eq!(text_1, DUMMY_TEXT.as_bytes());
    assert_eq!(text_2, DUMMY_TEXT.as_bytes());
    assert_eq!(text_3, DUMMY_TEXT.as_bytes());
  });
}

#[test]
#[serial]
fn folder_files_given_path_returns_list_of_files_descending_of_path() {
  run_unit(|| {
    let locale = locale();
    tree().unpack(&locale).unwrap();

    let files_1 = folder_files(&locale).unwrap();
    let files_2 = folder_files(locale.join("b/a/")).unwrap();
    let files_3 = folder_files(locale.join("c/b/a")).unwrap();

    let all_test_file_paths = TREE_PATHS.iter()
      .map(|path| locale.join(path))
      .collect::<HashSet<Utf8PathBuf>>();

    assert_eq!(files_1, all_test_file_paths);
    assert_eq!(files_2, HashSet::from_iter([locale.join("b/a/a"), locale.join("b/a/b"), locale.join("b/a/c")].into_iter()));
    assert_eq!(files_3, HashSet::from_iter([locale.join("c/b/a")].into_iter()));
  });
}

#[test]
#[serial]
fn relative_given_path_returns_project_relative_path() {
  run_unit(|| {
    let path = locale().join("a/a/a");

    assert_eq!(relative(path), "a/a/a");
  });
}
