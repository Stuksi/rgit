use std::{fs::File, io::Read};

use serial_test::serial;
use crate::{core::{tree::{Tree, TreeNode}, blob::Blob, repository::{RepositoryObject, Repository}}, tests::run_unit, lib::{errors::Errors, decompress, locale}};

const DUMMY_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
const DUMMY_TEXT_HASH: &str = "e7505beb754bed863e3885f73e3bb6866bdd7f8c";

// fn from(id: &str) -> Result<Self, Errors> where Self: Sized;
// fn pack(&self) -> Result<(), Errors>;
// fn unpack(&self, prefix: &Utf8Path) -> Result<(), Errors>;

#[test]
#[serial]
fn new_initializes_empty_tree() {
  run_unit(|| {
    let tree = Tree::new();

    assert_eq!(tree.id(), "");
    assert_eq!(tree.children().keys().len(), 0);
  });
}

#[test]
#[serial]
fn insert_given_single_component_path_and_node_creates_a_direct_child() {
  run_unit(|| {
    let mut tree = Tree::new();
    let blob_child = Blob::new(DUMMY_TEXT.as_bytes());
    let tree_child = Tree::new();

    tree.insert("blob", TreeNode::Blob(blob_child));
    tree.insert("tree", TreeNode::Tree(tree_child));

    if let Some(TreeNode::Blob(blob)) = tree.children().get("blob") {
      assert_eq!(blob.id(), DUMMY_TEXT_HASH);
    } else {
      unreachable!();
    }

    if let Some(TreeNode::Tree(tree)) = tree.children().get("tree") {
      assert_eq!(tree.id(), "");
      assert_eq!(tree.children().keys().len(), 0);
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn insert_given_multi_component_path_and_node_creates_child_with_preceding_trees() {
  run_unit(|| {
    let mut tree = Tree::new();
    let blob_child = Blob::new(DUMMY_TEXT.as_bytes());
    let tree_child = Tree::new();

    tree.insert("tree/tree/blob", TreeNode::Blob(blob_child));
    tree.insert("tree/tree/tree", TreeNode::Tree(tree_child));

    if let Some(TreeNode::Tree(tree)) = tree.children().get("tree") {
      if let Some(TreeNode::Tree(tree)) = tree.children().get("tree") {
        if let Some(TreeNode::Blob(blob)) = tree.children().get("blob") {
          assert_eq!(blob.id(), DUMMY_TEXT_HASH);
        } else {
          unreachable!();
        }

        if let Some(TreeNode::Tree(tree)) = tree.children().get("tree") {
          assert_eq!(tree.id(), "");
          assert_eq!(tree.children().keys().len(), 0);
        } else {
          unreachable!();
        }
      } else {
        unreachable!();
      }
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn insert_given_multi_component_path_and_node_creates_child_overwriting_any_obstructing_nodes() {
  run_unit(|| {
    let mut tree = Tree::new();
    let blob_child_1 = Blob::new(DUMMY_TEXT.as_bytes());
    let blob_child_2 = Blob::new(DUMMY_TEXT.as_bytes());
    let tree_child = Tree::new();

    tree.insert("tree/blob", TreeNode::Blob(blob_child_1));
    tree.insert("tree/blob/blob", TreeNode::Blob(blob_child_2));

    if let Some(TreeNode::Tree(tree)) = tree.children().get("tree") {
      if let Some(TreeNode::Tree(tree)) = tree.children().get("blob") {
        if let Some(TreeNode::Blob(blob)) = tree.children().get("blob") {
          assert_eq!(blob.id(), DUMMY_TEXT_HASH);
        } else {
          unreachable!();
        }
      } else {
        unreachable!();
      }
    } else {
      unreachable!();
    }

    tree.insert("tree/blob/blob", TreeNode::Tree(tree_child));

    if let Some(TreeNode::Tree(tree)) = tree.children().get("tree") {
      if let Some(TreeNode::Tree(tree)) = tree.children().get("blob") {
        if let Some(TreeNode::Tree(tree)) = tree.children().get("blob") {
          assert_eq!(tree.id(), "");
          assert_eq!(tree.children().keys().len(), 0);
        } else {
          unreachable!();
        }
      } else {
        unreachable!();
      }
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn remove_given_non_existing_path_has_no_effect() {
  run_unit(|| {
    let mut tree = Tree::new();
    let blob_child = Blob::new(DUMMY_TEXT.as_bytes());

    tree.insert("tree/blob", TreeNode::Blob(blob_child));
    tree.remove("tree/tree");

    if let Some(TreeNode::Tree(tree)) = tree.children().get("tree") {
      if let None = tree.children().get("blob") {
        unreachable!();
      }
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn remove_given_existing_path_removes_the_target_node() {
  run_unit(|| {
    let mut tree = Tree::new();
    let blob_child = Blob::new(DUMMY_TEXT.as_bytes());
    let tree_child = Tree::new();

    tree.insert("tree/blob", TreeNode::Blob(blob_child));
    tree.insert("tree/tree", TreeNode::Tree(tree_child));

    tree.remove("tree/blob");

    if let Some(TreeNode::Tree(tree)) = tree.children().get("tree") {
      if let Some(TreeNode::Blob(_)) = tree.children().get("blob") {
        unreachable!();
      }

      if let None = tree.children().get("tree") {
        unreachable!();
      }
    } else {
      unreachable!();
    }

    tree.remove("tree/tree");

    if let Some(TreeNode::Tree(tree)) = tree.children().get("tree") {
      if let Some(TreeNode::Tree(_)) = tree.children().get("tree") {
        unreachable!();
      }
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn get_given_non_existing_path_returns_none() {
  run_unit(|| {
    let mut tree = Tree::new();
    let blob_child = Blob::new(DUMMY_TEXT.as_bytes());
    let tree_child = Tree::new();

    tree.insert("tree/blob", TreeNode::Blob(blob_child));
    tree.insert("tree/tree", TreeNode::Tree(tree_child));

    let node_1 = tree.get("tree/tree/non_blob");
    let node_2 = tree.get("tree/non_tree");
    let node_3 = tree.get("non_tree");

    assert_eq!(node_1.is_none(), true);
    assert_eq!(node_2.is_none(), true);
    assert_eq!(node_3.is_none(), true);
  });
}

#[test]
#[serial]
fn get_given_existing_path_returns_the_node() {
  run_unit(|| {
    let mut tree = Tree::new();
    let blob_child = Blob::new(DUMMY_TEXT.as_bytes());
    let tree_child = Tree::new();

    tree.insert("tree/blob", TreeNode::Blob(blob_child));
    tree.insert("tree/tree", TreeNode::Tree(tree_child));

    let node_1 = tree.get("tree");
    let node_2 = tree.get("tree/blob");
    let node_3 = tree.get("tree/tree");

    if let Some(TreeNode::Tree(tree)) = node_1 {
      assert_eq!(tree.children().keys().len(), 2);
    } else {
      unreachable!();
    }

    if let Some(TreeNode::Blob(blob)) = node_2 {
      assert_eq!(blob.id(), DUMMY_TEXT_HASH);
    } else {
      unreachable!();
    }

    if let Some(TreeNode::Tree(tree)) = node_3 {
      assert_eq!(tree.id(), "");
      assert_eq!(tree.children().keys().len(), 0);
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn pack_compresses_and_saves_the_tree_inside_the_repository_and_returns_its_id() {
  run_unit(|| {
    let mut tree = Tree::new();
    let blob_child_1 = Blob::new(DUMMY_TEXT.as_bytes());
    let blob_child_2 = Blob::new(DUMMY_TEXT.as_bytes());

    tree.insert("blob", TreeNode::Blob(blob_child_1));
    tree.insert("tree/blob", TreeNode::Blob(blob_child_2));

    let tree_id = tree.pack().unwrap();

    let mut compressed_tree_bytes = Vec::new();
    File::open(Repository::locate_object(&tree_id)).unwrap().read_to_end(&mut compressed_tree_bytes).unwrap();
    let tree_content = decompress(&compressed_tree_bytes).unwrap();
    let expected_tree_content = "\
      tree\n\
      blob e7505beb754bed863e3885f73e3bb6866bdd7f8c\n\
      tree f62f3720728727fd14fc2faa37a8d6d9fd9e7796\
    ";

    assert_eq!(tree_id, "bba7e8447e2eefa4738a13d4c5867748c73f5209");
    assert_eq!(tree_content, expected_tree_content.as_bytes());
  })
}

#[test]
#[serial]
fn from_id_given_non_existing_id_return_unknown_repository_object_error() {
  run_unit(|| {
    if let Err(Errors::UnknownRepositoryObjectError(id)) = Tree::from_id("non-existing-id") {
      assert_eq!(id, "non-existing-id");
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn from_id_given_existing_id_returns_tree() {
  run_unit(|| {
    let mut tree = Tree::new();
    let blob_child_1 = Blob::new(DUMMY_TEXT.as_bytes());
    let blob_child_2 = Blob::new(DUMMY_TEXT.as_bytes());

    tree.insert("blob", TreeNode::Blob(blob_child_1));
    tree.insert("tree/blob", TreeNode::Blob(blob_child_2));

    let tree_id = tree.pack().unwrap();

    let tree_from_id = Tree::from_id(&tree_id).unwrap();

    if let Some(TreeNode::Tree(tree)) = tree_from_id.children().get("tree") {
      if let Some(TreeNode::Blob(blob)) = tree.children().get("blob") {
        assert_eq!(blob.id(), DUMMY_TEXT_HASH);
      } else {
        unreachable!();
      }
    } else {
      unreachable!();
    }

    if let Some(TreeNode::Blob(blob)) = tree_from_id.children().get("blob") {
      assert_eq!(blob.id(), DUMMY_TEXT_HASH);
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn unpack_given_path_directory_prefix_creates_directories_and_files_based_on_the_tree_structure() {
  run_unit(|| {
    let mut tree = Tree::new();
    let blob_child_1 = Blob::new(DUMMY_TEXT.as_bytes());
    let blob_child_2 = Blob::new("TEST".as_bytes());

    tree.insert("blob", TreeNode::Blob(blob_child_1));
    tree.insert("tree/blob", TreeNode::Blob(blob_child_2));

    let target_path = locale();
    tree.unpack(&target_path).unwrap();

    let mut blob_1_content = String::new();
    let mut blob_2_content = String::new();

    File::open(target_path.join("blob")).unwrap().read_to_string(&mut blob_1_content).unwrap();
    File::open(target_path.join("tree/blob")).unwrap().read_to_string(&mut blob_2_content).unwrap();

    assert_eq!(blob_1_content, DUMMY_TEXT);
    assert_eq!(blob_2_content, "TEST");
  })
}
