use std::process::Command;

use super::args;
use super::buildfile;

pub fn execute(args: args::Args) -> Result<(), std::io::Error> {
    for line in buildfile::targets(&args.build_file)? {
        let mut s = Vec::new();
        s.push(args.goal.to_string());
        let ll = line?;
        for i in 0..ll.len() {
            s.push(ll[i].to_str().unwrap())
        }
        let status = Command::new("rvg").args(s).status();
        if !status?.success() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "command failed"));
        }
    }
    Ok(())
}
