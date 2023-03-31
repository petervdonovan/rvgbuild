pub mod goal;
pub mod args;
pub mod buildfile;

use clap::Parser;

fn main() {
    let args = args::Args::parse();
    println!("{:?}", args.goal);
}
