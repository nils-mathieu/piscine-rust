fn main() {
    let mut args = std::env::args();
    args.next();

    let mut only_null_terminated = false;
    let mut min: Option<usize> = None;
    let mut max: Option<usize> = None;
    let mut file: Option<String> = None;

    loop {
        let arg = match args.next() {
            Some(arg) => arg,
            None if file.is_some() => break,
            None => {
                eprintln!("error: missing target file");
                return;
            }
        };

        match arg.as_str() {
            "-z" => only_null_terminated = true,
            "-m" => {
                if min.is_some() {
                    eprintln!("error: minimum already defined");
                    return;
                }

                let Some(arg) = args.next() else {
                    eprintln!("error: missing minimum after `-m`");
                    return;
                };

                match arg.parse() {
                    Err(_) | Ok(0) => {
                        eprintln!("error: invalid minimum");
                        return;
                    }
                    Ok(ok) => min = Some(ok),
                }
            }
            "-M" => {
                if max.is_some() {
                    eprintln!("error: maximum already defined");
                    return;
                }

                let Some(arg) = args.next() else {
                    eprintln!("error: missing maximum after `-M`");
                    return;
                };

                match arg.parse() {
                    Ok(ok) => max = Some(ok),
                    Err(_) => {
                        eprintln!("error: invalid maximum");
                        return;
                    }
                }
            }
            _ => {
                if file.is_some() {
                    eprintln!("error: can't specify multiple files");
                    return;
                }
                file = Some(arg);
            }
        }
    }

    let file = file.unwrap();
    let min = min.unwrap_or(1);
    let max = max.unwrap_or(usize::MAX);

    if max < min {
        eprintln!("error: the maximum is smaller than minimum");
        return;
    }

    let contents = match std::fs::read(&file) {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("error: {file}: {err}");
            return;
        }
    };
    let mut contents = contents.as_slice();

    while !contents.is_empty() {
        let s = match std::str::from_utf8(contents) {
            Ok(s) => s,
            Err(err) => std::str::from_utf8(&contents[..err.valid_up_to()]).unwrap(),
        };
        if s.is_empty() {
            contents = &contents[1..];
            continue;
        } else {
            contents = &contents[s.len()..];
        }

        if !only_null_terminated {
            s.split('\0')
                .filter(|s| !s.contains(char::is_control))
                .filter(|s| (min..=max).contains(&s.len()))
                .for_each(|s| println!("{}", s.escape_debug()));
        } else {
            s.split(char::is_control)
                .filter(|s| (min..=max).contains(&s.len()))
                .for_each(|s| println!("{}", s.escape_debug()));
        }
    }
}
