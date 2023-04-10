fn yes<F: FnOnce() -> String>(f: F) -> ! {
    let val = f();
    loop {
        println!("{val}");
    }
}

fn main() {
    let s = "YyY".to_string();
    yes(|| s);
}
