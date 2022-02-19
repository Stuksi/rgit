#![allow(dead_code)]

mod core;
mod lib;
mod cli;

#[cfg(test)]
mod tests;

fn main() {
  if let Err(error) = cli::Interface::run() {
    lib::errors::ErrorsInterface::handle(error);
  }
}
