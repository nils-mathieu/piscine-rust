fn main() {
    for n in 1..=100 {
        match (n % 3, n % 5, n % 11) {
            (0, 0, _) => println!("fizzbuzz"),
            (0, _, _) => println!("fizz"),
            (_, 0, _) => println!("buzz"),
            (_, _, 3) => println!("FIZZ"),
            (_, _, 5) => println!("BUZZ"),
            (_, _, _) => println!("{n}"),
        }
    }
}
