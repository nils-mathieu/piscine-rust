fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn collatz(start: u32) {
    let mut n = start;
    while n != 1 {
        println!("{n}");
        n = if is_even(n) { n / 2 } else { n * 3 + 1 };
    }
    println!("1");
}

fn main() {
    collatz(3);
}
