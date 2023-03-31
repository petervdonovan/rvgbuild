use std::path::Path;

use clap::Parser;
use super::goal::Goal;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, default_value="execution")]
    pub goal: Goal,
    #[arg(short, long, default_value="./rvgbuild")]
    build_file: String,
    file_name: Option<String>,
}

impl Args {
    fn build_file(&self) -> &Path {
        Path::new(&self.build_file)
    }
    fn file_name(&self) -> &Path {
        match &self.file_name {
            Some(p) => Path::new(p),
            None => Path::new(".")
        }

    }
}
