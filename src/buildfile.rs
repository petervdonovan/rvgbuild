use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::path::{PathBuf, Path};

pub fn targets(build_file: &Path) -> Result<Vec<Result<Vec<PathBuf>, Error>>, Error> {
    let f = File::open(build_file)?;
    let buf = BufReader::new(f);
    Ok(buf.lines().map(|x| match x {
        Ok(l) => Ok(parse(l, build_file.canonicalize()?.parent().unwrap().to_path_buf())),
        Err(e) => Err(e)
    }).collect())
}

fn parse(line: String, relative_to: PathBuf) -> Vec<PathBuf> {
    line.split("->")
        .map(|x| x.trim().to_string())
        .map(PathBuf::from)
        .map(|p| relative_to.join(p))
        .collect()
}
