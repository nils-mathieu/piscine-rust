fn print_byes<F: FnMut() -> Option<u8>>(mut f: F) {
    while let Some(mut b) = f() {
        for _ in 0..8 {
            if b & 0x80 == 0 {
                print!("Y");
            } else {
                print!("y");
            }
            b <<= 1;
        }
        println!();
    }
}

fn main() {
    let slice = b"Hello, World!";
    let mut i = 0;

    print_byes(|| {
        if i < slice.len() {
            let b = slice[i];
            i += 1;
            Some(b)
        } else {
            None
        }
    });
}
