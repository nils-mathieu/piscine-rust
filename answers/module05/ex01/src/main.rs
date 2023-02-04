use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut args = std::env::args();
    args.next();

    let mut files: Vec<(String, File)> = args
        .filter_map(|path| match File::create(path.as_str()) {
            Ok(file) => Some((path, file)),
            Err(err) => {
                eprintln!("error: {path}: {err}");
                None
            }
        })
        .collect();

    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();

    let mut buffer = [0u8; 4096];

    loop {
        let count = match stdin.read(&mut buffer) {
            Ok(0) => break,
            Ok(ok) => ok,
            Err(err) => {
                eprintln!("error: stdin: {err}");
                return;
            }
        };

        match stdout.write_all(&buffer[..count]) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("error: stdout: {err}");
                return;
            }
        }

        for (pathname, file) in &mut files {
            match file.write_all(&buffer[..count]) {
                Ok(()) => (),
                Err(err) => {
                    eprintln!("error: {pathname}: {err}");
                    return;
                }
            }
        }
    }
}
