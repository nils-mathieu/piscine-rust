fn main() {
    if ftkit::ARGS.len() != 3 {
        eprintln!("error: expected two arguments");
        return;
    }

    let query = &ftkit::ARGS[1];
    let pattern = &ftkit::ARGS[2];

    if module00_ex07::strpcmp(query.as_bytes(), pattern.as_bytes()) {
        println!("yes");
    } else {
        println!("no");
    }
}
