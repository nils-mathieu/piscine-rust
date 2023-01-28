fn main() {
    println!("Hey! I'm the other bin target!");

    if !cfg!(debug_assertions) {
        println!("I'm in release mode!");
    }
}
