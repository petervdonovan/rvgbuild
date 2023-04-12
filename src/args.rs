use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
pub struct Args {
  #[command(subcommand)]
  pub goal: Option<Goal>,
  #[command(flatten)]
  args: CommonArgs,
}

#[derive(Debug, Clone, Parser)]
pub struct CommonArgs {
  #[arg(short, long, value_parser = parse_file_name)]
  build_file: Option<PathBuf>,
  #[arg(value_parser = parse_file_name, default_value=None)]
  pub file_name: Option<PathBuf>,
}

impl CommonArgs {
  pub fn build_file<'a>(&mut self) -> &PathBuf {
    println!("build file is {:?}", self.build_file);
    match self.build_file {
      Some(ref p) => &p,
      None => {
        self.build_file = Some(PathBuf::from("./rvgbuild"));
        self.build_file()
      }
    }
  }
}

#[derive(Debug, Clone, Parser)]
pub struct CommonArgsWithPosition {
  line: u32,
  col: u32,
  #[command(flatten)]
  common: CommonArgs,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Goal {
  Tokens(CommonArgs),
  Hover(CommonArgsWithPosition),
  Definition(CommonArgsWithPosition),
}

impl Args {
  pub fn build_file(&mut self) -> &PathBuf {
    match &mut self.goal {
      Some(ref mut g) => g.build_file(),
      None => self.args.build_file(),
    }
  }
}

impl Goal {
  pub fn to_string(&self) -> &str {
    match self {
      Goal::Tokens { .. } => "tokens",
      Goal::Hover { .. } => "hover",
      Goal::Definition { .. } => "definition",
    }
  }
  pub fn append_to<'a>(&'a self, v: &'a mut Vec<String>) {
    v.push(String::from(self.to_string()));
    match &self {
      Goal::Tokens { .. } => (),
      Goal::Hover(args) => Self::append_position(&args, v),
      &Goal::Definition(args) => Self::append_position(&args, v),
    }
  }
  pub fn build_file(&mut self) -> &PathBuf {
    self.common().build_file()
  }
  pub fn file_name(&mut self) -> &Option<PathBuf> {
    &self.common().file_name
  }
  fn append_position(args: &CommonArgsWithPosition, v: &mut Vec<String>) {
    v.push(args.line.to_string());
    v.push(args.col.to_string());
  }
  fn common<'a>(&'a mut self) -> &mut CommonArgs {
    match self {
      Goal::Tokens(ref mut args) => args,
      Goal::Hover(ref mut args) => &mut args.common,
      Goal::Definition(ref mut args) => &mut args.common,
    }
  }
}

fn parse_file_name(s: &str) -> Result<PathBuf, std::io::Error> {
  let p = PathBuf::from(s);
  if p.exists() {
    Ok(p)
  } else {
    Err(std::io::Error::new(
      std::io::ErrorKind::NotFound,
      format!("{s} not found"),
    ))
  }
}

// fn optional_parse_file_name(s: &str) -> Result<PathBuf, std::io::Error> {
//     if s == "" { Ok(None) } else { Ok(Some(parse_file_name(s)?)) }
// }
