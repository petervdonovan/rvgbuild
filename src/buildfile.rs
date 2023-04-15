use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;

use super::args;

pub fn targets(args: &mut args::Args) -> Result<Vec<Result<Vec<PathBuf>, Error>>, Error> {
  let f = File::open(args.build_file())?;
  let buf = BufReader::new(f);
  let relative_to = args.build_file().canonicalize()?.parent().unwrap().to_path_buf();
  let predicate = |s: &String| match args.file_name() {
    Some(p) => s.contains(p.file_name().unwrap().to_str().unwrap()),
    None => true
  };
  Ok(
    buf
      .lines()
      .map(|r| r.expect("Failed to read line"))
      .filter(predicate)
      .map(|x| Ok(parse(
          x,
          relative_to.clone(),  // inefficient
        )))
      .collect(),
  )
}

fn parse(line: String, relative_to: PathBuf) -> Vec<PathBuf> {
  line
    .split("->")
    .map(|x| x.trim().to_string())
    .map(PathBuf::from)
    .map(|p| relative_to.join(p))
    .collect()
}
