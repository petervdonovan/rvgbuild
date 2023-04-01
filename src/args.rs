use std::path::{PathBuf};

use clap::Parser;
use super::goal::Goal;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, default_value="execution")]
    pub goal: Goal,
    #[arg(short, long, value_parser = parse_file_name, default_value="./rvgbuild")]
    pub build_file: PathBuf,
    #[arg(value_parser = optional_parse_file_name, default_value=None)]
    pub file_name: Option<PathBuf>,
}

fn parse_file_name(s: &str) -> Result<PathBuf, std::io::Error> {
    let p = PathBuf::from(s);
    if p.exists() {
        Ok(p)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("{s} not found")))
    }
}

fn optional_parse_file_name(s: &str) -> Result<Option<PathBuf>, std::io::Error> {
    if s == "" { Ok(None) } else { Ok(Some(parse_file_name(s)?)) }
}
