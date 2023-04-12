use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use super::args;
use super::buildfile;

macro_rules! unsafe_file_stem {
  ( $x:expr ) => {
    $x.file_stem().unwrap().to_str().unwrap()
  };
}

fn get_applier_rec(files: &[PathBuf], result: &mut String) {
  if files.len() > 1 {
    write!(result, "[{} [mu []", unsafe_file_stem!(files[0])).unwrap();
    get_applier_rec(&files[1..], result);
    result.push_str(" ]]");
  } else if files.len() == 1 {
    result.push_str(unsafe_file_stem!(files[0]));
  }
}
fn get_applier(files: &[PathBuf]) -> String {
  let mut result = String::new();
  result.push_str("applier.rvg=");
  get_applier_rec(files, &mut result);
  result
}
fn send_to_file(child: std::process::Child, ll: Vec<PathBuf>) {
  // fs::create_dir_all("src-gen").expect("Failed to create src-gen directory.");
  let last = &ll[ll.len() - 1];
  let output_file = last
    .parent()
    .expect("Expected a file, not a root or empty path")
    .join("src-gen")
    .join(
      String::from(
        last
          .file_stem()
          .expect("Expected file name")
          .to_string_lossy(),
      ) + ".s",
    );
  fs::create_dir_all(
    output_file
      .parent()
      .expect("Output file must have a parent"),
  )
  .expect("Failed to create src-gen directory");
  fs::write(
    &output_file,
    child
      .wait_with_output()
      .expect("Failed to read stdout")
      .stdout,
  )
  .expect("Failed to write to file");
}

pub fn execute(mut args: args::Args) -> Result<(), std::io::Error> {
  let targets = buildfile::targets(&args.build_file())?;
  for line in targets {
    let mut args_s = Vec::new();
    match args.goal {
      Some(ref g) => g.append_to(&mut args_s),
      None => (),
    }
    let ll = line?;
    for i in 0..ll.len() {
      args_s.push(String::from(ll[i].to_str().unwrap()));
    }
    args_s.push(get_applier(&ll));
    let child = Command::new("rvg")
      .stdout(Stdio::piped())
      .args(args_s)
      .spawn();
    if args.goal.is_none() {
      send_to_file(child.expect("Failed to start child process"), ll);
    }
  }
  Ok(())
}
