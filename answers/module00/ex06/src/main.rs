use std::cmp::Ordering::*;

fn main() {
    let number_to_guess = ftkit::random_number(1..=20);

    println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");
    loop {
        let guess = ftkit::read_number();

        match guess.cmp(&number_to_guess) {
            Less => println!("Sometimes I wonder whether I should retire. I would have guessed higher."),
            Greater => println!("This student might not be as smart as I was told. This answer is obviously too weak."),
            Equal => break,
        }
    }
    println!("That is right! The secret was indeed the number {number_to_guess}, which you have brilliantly discovered!")
}
