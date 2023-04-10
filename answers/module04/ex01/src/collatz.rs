fn collayz<F: Fn(u32)>(start: u32, mut f: F) {
    let mut n = start;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            f(n);
            n = 3 * n + 1;
        }
    }
    f(n);
}

fn main() {
    collayz(11, |n| {
        for _ in 0..n {
            print!("Y");
        }
        println!();
    });
}
