use std::{fs::File, io::Read};
use serial_test::serial;
use crate::{core::tree::{Tree, Node}, tests::{run_unit, factory::{blob, blob_and_text}}, lib::{decompress, locale, constants::OBJECTS_PATH, object::FromId}};

#[test]
#[serial]
fn new_initializes_empty_tree() {
  run_unit(|| {
    let root = Tree::new();

    assert!(root.id().is_empty());
    assert!(root.children().is_empty());
  });
}

#[test]
#[serial]
fn insert_given_single_component_path_and_node_creates_a_direct_child() {
  run_unit(|| {
    let mut root = Tree::new();
    let blob = blob();
    let tree = Tree::new();

    root.insert("blob", Node::Blob(blob.clone()));
    root.insert("tree", Node::Tree(tree.clone()));

    assert_eq!(root.children().get("blob").unwrap(), &blob);
    assert_eq!(root.children().get("tree").unwrap(), &tree);
  });
}

#[test]
#[serial]
fn insert_given_multi_component_path_and_node_creates_child_with_preceding_trees() {
  run_unit(|| {
    let mut root = Tree::new();
    let blob = blob();
    let tree = Tree::new();

    root.insert("tree/tree/blob", Node::Blob(blob.clone()));
    root.insert("tree/tree/tree", Node::Tree(tree.clone()));

    let inner_tree = root.children().get("tree").unwrap().into_tree().unwrap()
      .children().get("tree").unwrap().into_tree().unwrap();

    assert_eq!(inner_tree.children().get("blob").unwrap(), &blob);
    assert_eq!(inner_tree.children().get("tree").unwrap(), &tree);
  });
}

#[test]
#[serial]
fn insert_given_multi_component_path_and_node_creates_child_overwriting_any_obstructing_nodes() {
  run_unit(|| {
    let mut root = Tree::new();
    let blob = blob();
    let tree = Tree::new();

    root.insert("tree/blob", Node::Blob(blob.clone()));
    root.insert("tree/blob/blob", Node::Blob(blob.clone()));
    root.insert("tree/blob/tree", Node::Tree(tree.clone()));

    let inner_tree = root.children().get("tree").unwrap().into_tree().unwrap()
      .children().get("blob").unwrap().into_tree().unwrap();

    assert_eq!(inner_tree.children().get("blob").unwrap(), &blob);
    assert_eq!(inner_tree.children().get("tree").unwrap(), &tree);
  });
}

#[test]
#[serial]
fn remove_given_non_existing_path_has_no_effect() {
  run_unit(|| {
    let mut root = Tree::new();
    let blob = blob();

    root.insert("tree/blob", Node::Blob(blob.clone()));
    root.remove("tree/tree");

    let inner_tree = root.children().get("tree").unwrap().into_tree().unwrap();

    assert_eq!(inner_tree.children().get("blob").unwrap(), &blob);
  });
}

#[test]
#[serial]
fn remove_given_existing_path_removes_the_target_node() {
  run_unit(|| {
    let mut root = Tree::new();
    let blob = blob();
    let tree = Tree::new();

    root.insert("tree/blob", Node::Blob(blob.clone()));
    root.insert("tree/tree", Node::Tree(tree.clone()));

    root.remove("tree/blob");
    root.remove("tree/tree");

    let inner_tree = root.children().get("tree").unwrap().into_tree().unwrap();

    assert_eq!(inner_tree.children().get("blob"), None);
    assert_eq!(inner_tree.children().get("tree"), None);
  });
}

#[test]
#[serial]
fn get_given_non_existing_path_returns_none() {
  run_unit(|| {
    let mut root = Tree::new();
    let blob = blob();
    let tree = Tree::new();

    root.insert("tree/blob", Node::Blob(blob.clone()));
    root.insert("tree/tree", Node::Tree(tree.clone()));

    let node_1 = tree.get("tree/tree/non_blob");
    let node_2 = tree.get("tree/non_tree");
    let node_3 = tree.get("non_tree");

    assert_eq!(node_1, None);
    assert_eq!(node_2, None);
    assert_eq!(node_3, None);
  });
}

#[test]
#[serial]
fn get_given_existing_path_returns_the_node() {
  run_unit(|| {
    let mut root = Tree::new();
    let blob = blob();
    let tree = Tree::new();

    root.insert("tree/blob", Node::Blob(blob.clone()));
    root.insert("tree/tree", Node::Tree(tree.clone()));

    let node_1 = root.get("tree/blob").unwrap().into_blob().unwrap();
    let node_2 = root.get("tree/tree").unwrap().into_tree().unwrap();
    let node_3 = root.get("tree").unwrap().into_tree().unwrap();

    assert_eq!(node_1, &blob);
    assert_eq!(node_2, &tree);
    assert_eq!(node_3.children().get("blob").unwrap(), node_1);
    assert_eq!(node_3.children().get("tree").unwrap(), node_2);
  });
}

#[test]
#[serial]
fn pack_compresses_and_saves_the_tree_inside_the_repository_and_modifies_and_returns_its_id() {
  run_unit(|| {
    let mut root = Tree::new();
    let blob_1 = blob();
    let blob_2 = blob();

    root.insert("blob", Node::Blob(blob_1.clone()));
    root.insert("tree/blob", Node::Blob(blob_2.clone()));

    root.pack().unwrap();

    let inner_tree = root.get("tree").unwrap().into_tree().unwrap();

    let mut compressed = Vec::new();
    let location = locale().join(OBJECTS_PATH).join(&root.id()[..2]).join(&root.id()[2..]);
    File::open(location).unwrap().read_to_end(&mut compressed).unwrap();

    let expected = format!("\
      tree\
      blob blob {}\n\
      tree tree {}\
    ", blob_1.id(), inner_tree.id());

    assert_eq!(decompress(&compressed).unwrap(), expected.as_bytes());
  })
}

#[test]
#[serial]
fn unpack_given_path_directory_prefix_creates_directories_and_files_based_on_the_tree_structure() {
  run_unit(|| {
    let mut root = Tree::new();
    let (blob_1, text_1) = blob_and_text();
    let (blob_2, text_2) = blob_and_text();

    root.insert("blob", Node::Blob(blob_1.clone()));
    root.insert("tree/blob", Node::Blob(blob_2.clone()));

    let prefix = locale();
    root.unpack(&prefix).unwrap();

    let mut file_text_1 = String::new();
    let mut file_text_2 = String::new();

    File::open(prefix.join("blob")).unwrap().read_to_string(&mut file_text_1).unwrap();
    File::open(prefix.join("tree/blob")).unwrap().read_to_string(&mut file_text_2).unwrap();

    assert_eq!(file_text_1, text_1);
    assert_eq!(file_text_2, text_2);
  })
}

#[test]
#[serial]
fn from_id_given_existing_id_returns_tree() {
  run_unit(|| {
    let mut root = Tree::new();

    root.insert("blob", Node::Blob(blob()));
    root.insert("tree/blob", Node::Blob(blob()));

    root.pack().unwrap();

    let tree = Tree::from_id(root.id()).unwrap();

    assert_eq!(root, tree);
  });
}
