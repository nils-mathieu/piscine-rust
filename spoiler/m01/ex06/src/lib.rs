pub fn split_once_at_null(s: &str) -> (&str, &str) {
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b'\0' {
            return (&s[..i + 1], &s[i + 1..]);
        }
    }

    panic!("no '\\0' character found in the string");
}

#[test]
#[should_panic(expected = "no '\\0' character found in the string")]
fn split_panics_when_no_null() {
    split_once_at_null("Hello, World!");
    unreachable!();
}

#[test]
fn split_basic() {
    assert_eq!(split_once_at_null("Hello\0World"), ("Hello\0", "World"));
}

#[test]
fn split_empty() {
    assert_eq!(split_once_at_null("\0"), ("\0", ""));
}
