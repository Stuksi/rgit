use std::{collections::HashMap, fs::{self, File}, io::Write};
use camino::Utf8Path;
use getset::Getters;
use crate::lib::{object::{Object, FromId}, errors::Errors, read_object_bytes, constants::{BLOB_TYPE, TREE_TYPE}, write_object_bytes};
use super::blob::Blob;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Tree(Tree),
  Blob(Blob),
}

impl PartialEq<Blob> for Node {
  fn eq(&self, other: &Blob) -> bool {
    match self {
      Self::Tree(_) => false,
      Self::Blob(blob) => blob == other,
    }
  }
}

impl PartialEq<Tree> for Node {
  fn eq(&self, other: &Tree) -> bool {
    match self {
      Self::Tree(tree) => tree == other,
      Self::Blob(_) => false,
    }
  }
}

impl Node {
  pub fn into_tree(&self) -> Result<&Tree, Errors> {
    match self {
      Node::Tree(tree) => Ok(tree),
      _ => Err(Errors::NodeConvertionError)
    }
  }

  pub fn into_blob(&self) -> Result<&Blob, Errors> {
    match self {
      Node::Blob(blob) => Ok(blob),
      _ => Err(Errors::NodeConvertionError)
    }
  }
}

#[derive(Getters, Debug, PartialEq, Clone)]
pub struct Tree {
  #[getset(get = "pub")]
  id: String,

  #[getset(get = "pub")]
  children: HashMap<String, Node>,
}

impl Tree {
  pub fn new() -> Self {
    Tree {
      id: String::from(""),
      children: HashMap::new(),
    }
  }

  pub fn insert<P: AsRef<Utf8Path>>(&mut self, path: P, node: Node) {
    let path = path.as_ref();

    if !path.to_string().contains(std::path::is_separator) {
      self.children.insert(path.to_string(), node);
      return;
    }

    let mut it = path.iter();
    let name = it.next().unwrap();

    if let Some(Node::Tree(tree)) = self.children.get_mut(name) {
      tree.insert(it.as_path(), node);
    } else {
      let mut tree = Tree::new();
      tree.insert(it.as_path(), node);
      self.children.insert(String::from(name), Node::Tree(tree));
    }
  }

  pub fn remove<P: AsRef<Utf8Path>>(&mut self, path: P) {
    let path = path.as_ref();

    if !path.to_string().contains(std::path::is_separator) {
      self.children.remove(&path.to_string());
      return;
    }

    let mut it = path.iter();
    let name = it.next().unwrap();

    if let Some(Node::Tree(tree)) = self.children.get_mut(name) {
      tree.remove(it.as_path());
    }
  }

  pub fn get<P: AsRef<Utf8Path>>(&self, path: P) -> Option<&Node> {
    let path = path.as_ref();

    if !path.to_string().contains(std::path::is_separator) {
      return self.children.get(&path.to_string());
    }

    let mut it = path.iter();
    let name = it.next().unwrap();

    if let Some(Node::Tree(tree)) = self.children.get(name) {
      tree.get(it.as_path())
    } else {
      None
    }
  }

  pub fn pack(&mut self) -> Result<&String, Errors> {
    let mut text = String::new();
    let mut data = Vec::new();

    for (name, node) in &mut self.children {
      let (object_type, id) = match node {
        Node::Blob(blob) => (BLOB_TYPE, blob.id()),
        Node::Tree(tree) => (TREE_TYPE, tree.pack()?),
      };

      data.push(format!("{} {} {}\n", object_type, name, id));
    }

    data.sort();

    for line in data {
      text += &line;
    }

    self.id = write_object_bytes(Object::Tree, text.trim_end())?;
    Ok(self.id())
  }

  pub fn unpack<P: AsRef<Utf8Path>>(&self, prefix: P) -> Result<(), Errors> {
    for (name, node) in self.children() {
      match node {
        Node::Tree(tree) => {
          tree.unpack(prefix.as_ref().join(name))?;
        },
        Node::Blob(blob) => {
          let bytes = read_object_bytes(blob.id())?;
          fs::create_dir_all(prefix.as_ref())?;
          File::create(prefix.as_ref().join(name))?.write_all(&bytes)?;
        },
      }
    }

    Ok(())
  }
}

impl FromId for Tree {
  fn from(id: &str) -> Result<Self, Errors> {
    let bytes = read_object_bytes(id)?;
    let text = String::from_utf8(bytes)?;
    let mut children = HashMap::new();

    for line in text.lines() {
      if let [object_type, name, id] = line.split_whitespace().collect::<Vec<&str>>()[..] {
        let node = match object_type {
          BLOB_TYPE => Node::Blob(<Blob as FromId>::from(id)?),
          TREE_TYPE => Node::Tree(<Tree as FromId>::from(id)?),
          _ => return Err(Errors::InvalidTreeNodeError),
        };

        children.insert(String::from(name), node);
      } else {
        return Err(Errors::BadObjectStructureError);
      }
    }

    Ok(
      Tree {
        id: String::from(id),
        children,
      }
    )
  }
}
