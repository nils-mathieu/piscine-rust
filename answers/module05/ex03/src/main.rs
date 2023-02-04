use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let mut args = std::env::args();
    args.next();

    let command = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("error: missing command");
            return;
        }
    };

    let mut lines = Vec::new();
    for i in std::io::stdin().lines() {
        match i {
            Ok(line) => lines.push(line),
            Err(err) => {
                eprintln!("error: stdin: {err}");
                return;
            }
        }
    }

    let err = Command::new(command).args(args).args(lines).exec();
    eprintln!("error: exec: {err}");
}
