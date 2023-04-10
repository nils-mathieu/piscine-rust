pub fn anything() -> impl Fn(&str) -> bool {
    |_| true
}

pub fn exactly<'a>(to_match: &'a str) -> impl 'a + Fn(&str) -> bool {
    move |s| s == to_match
}

pub fn alphabetic() -> impl Fn(&str) -> bool {
    |s| s.chars().all(|c| c.is_alphabetic())
}

pub fn ascii_digits() -> impl Fn(&str) -> bool {
    |s| s.chars().all(|c| c.is_ascii_digit())
}

pub fn min_size(min: usize) -> impl Fn(&str) -> bool {
    move |s| s.chars().count() >= min
}

pub fn maybe(pattern: impl Fn(&str) -> bool) -> impl Fn(&str) -> bool {
    move |s| s.is_empty() || pattern(s)
}

pub fn not(pattern: impl Fn(&str) -> bool) -> impl Fn(&str) -> bool {
    move |s| !pattern(s)
}

pub fn or(
    pattern1: impl Fn(&str) -> bool,
    pattern2: impl Fn(&str) -> bool,
) -> impl Fn(&str) -> bool {
    move |s| pattern1(s) || pattern2(s)
}

pub fn and(
    pattern1: impl Fn(&str) -> bool,
    pattern2: impl Fn(&str) -> bool,
) -> impl Fn(&str) -> bool {
    move |s| pattern1(s) && pattern2(s)
}

pub fn chain(
    pattern1: impl Fn(&str) -> bool,
    pattern2: impl Fn(&str) -> bool,
) -> impl Fn(&str) -> bool {
    move |s| (0..=s.len()).any(|i| pattern1(&s[..i]) && pattern2(&s[i..]))
}

#[test]
fn email() {
    let pattern = chain(
        chain(and(alphabetic(), min_size(1)), exactly("@")),
        chain(
            and(alphabetic(), min_size(1)),
            or(exactly(".fr"), exactly(".com")),
        ),
    );

    assert!(pattern("my@address.com"));
    assert!(pattern("a@b.fr"));
    assert!(!pattern("@example.com"));
    assert!(!pattern("abc@.com"));
    assert!(!pattern("my@address.net"));
    assert!(!pattern("myaddress.fr"));
    assert!(!pattern("my-address@domain.fr"));
    assert!(!pattern("address@my-domain.fr"));
}

#[test]
fn parens() {
    let pattern = chain(
        chain(
            exactly("("),
            not(or(
                chain(chain(anything(), exactly("(")), anything()),
                chain(chain(anything(), exactly(")")), anything()),
            )),
        ),
        exactly(")"),
    );

    assert!(pattern("()"));
    assert!(pattern("(hello)"));
    assert!(pattern("(hello 123)"));
    assert!(!pattern("(hello () 123)"));
    assert!(!pattern("(hello "));
    assert!(!pattern(")"));
    assert!(!pattern(" (test) "));
}

#[test]
fn decimal() {
    let pattern = chain(
        maybe(or(exactly("+"), exactly("-"))),
        chain(
            and(ascii_digits(), min_size(1)),
            chain(
                maybe(chain(exactly("."), and(ascii_digits(), min_size(1)))),
                maybe(chain(
                    or(exactly("e"), exactly("E")),
                    chain(
                        maybe(or(exactly("+"), exactly("-"))),
                        and(ascii_digits(), min_size(1)),
                    ),
                )),
            ),
        ),
    );

    assert!(pattern("12"));
    assert!(pattern("+12"));
    assert!(pattern("-12"));
    assert!(pattern("12.5"));
    assert!(pattern("12.5e20"));
    assert!(pattern("12E10"));
    assert!(pattern("12E+9"));
    assert!(pattern("12E-9"));
    assert!(!pattern(""));
    assert!(!pattern("+"));
    assert!(!pattern("+12."));
    assert!(!pattern("+12e"));
    assert!(!pattern("+12.e10"));
    assert!(!pattern("12e10.10"));
}
