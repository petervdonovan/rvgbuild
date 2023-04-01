use std::process::Command;

use super::args;
use super::buildfile;

pub fn execute(mut args: args::Args) -> Result<(), std::io::Error> {
    let targets = buildfile::targets(&args.build_file())?;
    for line in targets {
        let mut s = Vec::new();
        match args.goal {
            Some(ref g) => g.append_to(&mut s),
            None => ()
        }
        let ll = line?;
        for i in 0..ll.len() {
            s.push(String::from(ll[i].to_str().unwrap()))
        }
        let status = Command::new("rvg").args(s).status();
        if !status?.success() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "command failed"));
        }
    }
    Ok(())
}
