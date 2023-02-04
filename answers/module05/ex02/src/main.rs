use std::io;
use std::path::Path;

fn write_human_size(size: u64) {
    print!("\x1B[A\x1B[K");
    match size {
        0..=999 => println!("{size} bytes"),
        1_000..=999_999 => println!("{:.1} kilobytes", size as f64 / 1_000.0),
        1_000_000..=999_999_999 => println!("{:.1} megabytes", size as f64 / 1_000_000.0),
        1_000_000_000.. => println!("{:.1} gigabytes", size as f64 / 1_000_000_000.0),
    }
}

fn write_size(pathname: &Path, total_size: &mut u64) -> io::Result<()> {
    let metadata = std::fs::metadata(pathname)?;

    *total_size += metadata.len();
    write_human_size(*total_size);

    if !metadata.is_dir() {
        return Ok(());
    }

    for entry_result in std::fs::read_dir(pathname)? {
        let entry = entry_result?;
        write_size(&entry.path(), total_size)?;
    }

    Ok(())
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let arg = match args.next() {
        Some(ok) => ok,
        None => {
            eprintln!("error: missing argument");
            return;
        }
    };
    if args.next().is_some() {
        eprintln!("error: too many arguments");
        return;
    }

    let mut total_size = 0;
    println!();
    match write_size(arg.as_ref(), &mut total_size) {
        Ok(()) => (),
        Err(err) => eprintln!("error: {}", err),
    }
}
