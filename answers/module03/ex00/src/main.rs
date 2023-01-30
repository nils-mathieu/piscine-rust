fn choose<T>(values: &[T]) -> &T {
    let index = ftkit::random_number(0..values.len() as i32) as usize;
    &values[index]
}

fn main() {
    println!("{}", choose(&[1, 2, 3, 4]));
    println!("{}", choose(&["abc", "def"]));
}
