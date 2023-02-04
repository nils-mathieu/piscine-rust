use std::io;
use std::io::Write;
use std::process::{Child, Command, Stdio};

fn main() {
    let mut commands: Vec<Vec<String>> = Vec::new();
    commands.push(Vec::new());
    for arg in std::env::args().skip(1) {
        if arg == "," {
            if commands.last().unwrap().is_empty() {
                eprintln!("error: empty command");
                return;
            }
            commands.push(Vec::new());
        } else {
            commands.last_mut().unwrap().push(arg);
        }
    }

    let mut children: Vec<(Vec<String>, Child)> = Vec::new();
    for cmd in commands {
        let mut args = cmd.iter();
        let name = args.next().unwrap();
        match Command::new(name)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(child) => children.push((cmd, child)),
            Err(err) => {
                eprintln!("error: {name}: {err}");
            }
        }
    }

    match write_output(children) {
        Ok(()) => (),
        Err(err) => eprintln!("error: {err}"),
    }
}

fn write_output(mut children: Vec<(Vec<String>, Child)>) -> io::Result<()> {
    let mut stdout = std::io::stdout();
    while !children.is_empty() {
        if let Some(found) = children
            .iter_mut()
            .position(|(_, child)| child.try_wait().ok().flatten().is_some())
        {
            let (cmd, child) = children.swap_remove(found);
            let output = child.wait_with_output()?;
            write!(stdout, "===== ")?;
            for arg in cmd {
                write!(stdout, "{arg} ")?;
            }
            writeln!(stdout, " =====")?;
            stdout.write_all(&output.stdout)?;
            writeln!(stdout)?;
        }
    }
    Ok(())
}
