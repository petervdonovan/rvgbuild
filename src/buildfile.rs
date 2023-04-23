use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;

use super::args;

pub fn targets(args: &mut args::Args) -> Result<Vec<Result<Vec<PathBuf>, Error>>, Error> {
  let f = File::open(args.build_file())?;
  let buf = BufReader::new(f);
  let relative_to = args
    .build_file()
    .canonicalize()?
    .parent()
    .unwrap()
    .to_path_buf();
  let cwd = env::current_dir().expect("cannot determine cwd");
  let predicate = |s: &Vec<PathBuf>| match args.file_name() {
    Some(p) => {
      s.len() > 0
        && cwd
          .join(s[s.len() - 1].clone())
          .canonicalize()
          .expect(&format!(
            "failed to canonicalize {}",
            s[s.len() - 1]
              .to_str()
              .expect("failed to represent path during handling of canonicalization failure")
          ))
          == cwd.join(p).canonicalize().expect("failed to canonicalize")
    }
    None => true,
  };
  Ok(
    buf
      .lines()
      .map(|r| r.expect("Failed to read line"))
      .map(|x| {
        parse(
          x,
          relative_to.clone(), // inefficient
        )
      })
      .filter(predicate)
      .map(|x| Ok(x))
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
