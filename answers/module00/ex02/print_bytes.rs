fn print_bytes(s: &str) {
    for b in s.bytes() {
        println!("{b}");
    }
}

fn main() {
    print_bytes("Déjà Vu\n");
}
