fn get_string(key: &i32) -> &'static str {
    match key {
        0 => "Hello!",
        1 => "Hola!",
        2 => "Bonjour!",
        3 => "Salve!",
        4 => "Konnichiwa!",
        _ => panic!("invalid key: '{key}'"),
    }
}

fn main() {
    let result;

    {
        let key = ftkit::random_number(0..=4);
        result = get_string(&key);
    }

    println!("{result}");
}
