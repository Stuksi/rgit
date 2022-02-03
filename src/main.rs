mod core;
mod lib;
mod cli;

#[cfg(test)]
mod tests;

fn main() {
  cli::run().unwrap();
}
