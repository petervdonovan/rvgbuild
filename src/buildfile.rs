use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::path::Path;

pub fn files(build_file: &Path) -> Result<
    Box<dyn Iterator<Item=Result<Vec<String>, Error>>>,
    Error
> {
    let f = File::open(build_file)?;
    let buf = BufReader::new(f);
    Ok(Box::new(buf.lines().map(|x| match x {
        Ok(l) => Ok(parse(l)),
        Err(e) => Err(e)
    })))
}

fn parse(line: String) -> Vec<String> {
    line.split("->").map(|x| x.trim().to_string()).collect()
}
