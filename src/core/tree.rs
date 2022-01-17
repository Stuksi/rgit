use crate::lib::errors::Errors;
use super::{blob::Blob, repository::RepositoryObject};
use camino::Utf8Path;
use getset::Getters;
use std::collections::HashMap;

pub enum TreeNode {
  Tree(Tree),
  Blob(Blob),
}

#[derive(Getters)]
pub struct Tree {
  #[getset(get = "pub")]
  id: String,

  #[getset(get = "pub")]
  children: HashMap<String, TreeNode>,
}

impl Tree {
  pub fn new() -> Self {
    Tree {
      id: String::from(""),
      children: HashMap::new(),
    }
  }

  pub fn insert<P: AsRef<Utf8Path>>(&mut self, path: P, node: TreeNode) {
    let path = path.as_ref();

    if let None = path.parent() {
      self.children.insert(path.to_string(), node);
      return;
    }

    let mut it = path.iter();
    let name = it.next().unwrap();

    if let Some(TreeNode::Tree(tree)) = self.children.get_mut(name) {
      tree.insert(it.as_path(), node);
    } else {
      let mut tree = Tree::new();
      tree.insert(it.as_path(), node);
      self.children.insert(String::from(name), TreeNode::Tree(tree));
    }
  }

  pub fn remove<P: AsRef<Utf8Path>>(&mut self, path: P) {
    let path = path.as_ref();

    if let None = path.parent() {
      self.children.remove(&path.to_string());
      return;
    }

    let mut it = path.iter();
    let name = it.next().unwrap();

    if let Some(TreeNode::Tree(tree)) = self.children.get_mut(name) {
      tree.remove(it.as_path());
    }
  }

  pub fn get<P: AsRef<Utf8Path>>(&self, path: P) -> Option<&TreeNode> {
    let path = path.as_ref();

    if let None = path.parent() {
      return self.children.get(&path.to_string());
    }

    let mut it = path.iter();
    let name = it.next().unwrap();

    if let Some(TreeNode::Tree(tree)) = self.children.get(name) {
      tree.get(it.as_path())
    } else {
      None
    }
  }
}

impl RepositoryObject for Tree {
  fn from_id(id: &str) -> Result<Self, Errors> where Self: Sized {
    todo!()
  }

  fn pack(&self) -> Result<String, Errors> {
    todo!()
  }

  fn unpack<P: AsRef<Utf8Path>>(&self, prefix: P) -> Result<(), Errors> {
    todo!()
  }
}
