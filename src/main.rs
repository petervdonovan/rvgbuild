pub mod args;
pub mod buildfile;
pub mod execute;

use clap::Parser;

fn main() {
  let args = args::Args::parse();
  match execute::execute(args) {
    Ok(_) => (),
    Err(e) => panic!("Error while executing command: {:?}", e),
  }
}
