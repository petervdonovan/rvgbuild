pub mod args;
pub mod buildfile;
pub mod execute;

use clap::Parser;

fn main() {
  let mut args = args::Args::parse();
  match execute::execute(&mut args) {
    Ok(_) => (),
    Err(e) => panic!("Error while executing command: {:?}", e),
  }
}
