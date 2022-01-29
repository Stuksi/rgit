use lipsum::lipsum;
use crate::core::{blob::Blob, tree::{Tree, Node}, commit::Commit};

const TREE_PATHS: &[&str] = &[
  "a/a/a", "a/a/b", "a/a/c",
  "a/b/a", "a/b/b", "a/b/c",
  "a/c/a", "a/c/b", "a/c/c",
  "b/a/a", "b/a/b", "b/a/c",
  "b/b/a", "b/b/b", "b/b/c",
  "b/c/a", "b/c/b", "b/c/c",
  "c/a/a", "c/a/b", "c/a/c",
  "c/b/a", "c/b/b", "c/b/c",
  "c/c/a", "c/c/b", "c/c/c",
];

pub fn blob() -> Blob {
  Blob::new(lipsum(25).as_bytes()).unwrap()
}

pub fn blob_and_text() -> (Blob, String) {
  let text = lipsum(25);
  (Blob::new(text.as_bytes()).unwrap(), text)
}

pub fn tree() -> Tree {
  let mut tree = Tree::new();

  for path in TREE_PATHS {
    tree.insert(path, Node::Blob(blob()));
  }

  tree.pack().unwrap();
  tree
}

pub fn commit() -> Commit {
  Commit::new(None, tree().id()).unwrap()
}
