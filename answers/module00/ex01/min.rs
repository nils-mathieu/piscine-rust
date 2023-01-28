fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

fn main() {
    println!("min(1, 2) == {}", min(1, 2));
    println!("min(2, 2) == {}", min(2, 2));
    println!("min(2, 1) == {}", min(2, 1));
}
