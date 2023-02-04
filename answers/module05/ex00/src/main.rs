use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();

    for i in 1..=10 {
        match writeln!(stdout, "{i}") {
            Ok(()) => (),
            Err(_) => return,
        }
    }
}
