fn main() {
    let item_name = match ftkit::random_number(0..=2) {
        0 => "cool ring",
        1 => "fast car",
        2 => "big house",
        _ => unreachable!(),
    };

    let item_price = ftkit::random_number(1..=30);

    println!("Here is a fabulous '{item_name}'.");
    loop {
        let guess = ftkit::read_number();

        use std::cmp::Ordering::*;
        match guess.cmp(&item_price) {
            Greater => println!("A '{item_name}' isn't worth that much money!"),
            Less => println!("A '{item_name}' costs more than that!"),
            Equal => break,
        }
    }
    println!("Congrats! That '{item_name}' is worth ${item_price}");
}
