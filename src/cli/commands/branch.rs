use crate::{lib::{errors::Errors, constants::BRANCHES_PATH, locale}, core::{branch::Branch, head::{Head, Reference}}};

pub fn branch(delete: Option<String>) -> Result<(), Errors> {
  if let Some(name) = delete {
    Branch::remove(&name)?;
  } else {
    let head = Head::get()?;
    let branch_name = match head.reference() {
      Reference::Branch(branch) => branch.name(),
      Reference::Commit(_) => "",
    };

    let location = locale().join(BRANCHES_PATH);
    let mut branch_list = String::new();

    for entry in location.read_dir()? {
      let entry = entry?;
      let file_name = entry.file_name();
      let file_name = file_name.to_str().unwrap();

      if file_name == branch_name {
        branch_list += &format!("-> {}\n", file_name);
      } else {
        branch_list += &format!("   {}\n", file_name);
      }
    }

    print!("{}", branch_list);
  }

  Ok(())
}
