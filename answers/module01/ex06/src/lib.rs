pub fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert!(!a.is_empty() && !b.is_empty());

    let mut result = Vec::new();
    let mut carry = false;

    let max_len = if a.len() > b.len() { a.len() } else { b.len() };

    for i in 1..=max_len {
        let digit_a = if i <= a.len() && a.len() - i < a.len() {
            assert!(a[a.len() - i].is_ascii_digit());
            a[a.len() - i] - b'0'
        } else {
            0
        };

        let digit_b = if i <= b.len() && b.len() - i < b.len() {
            assert!(b[b.len() - i].is_ascii_digit());
            b[b.len() - i] - b'0'
        } else {
            0
        };

        let mut digit = digit_a + digit_b;
        if carry {
            carry = false;
            digit += 1;
        }
        if digit >= 10 {
            digit -= 10;
            carry = true;
        }

        result.push(digit + b'0');
    }

    if carry {
        result.push(b'1');
    }

    result.reverse();
    result
}

#[cfg(test)]
#[test]
fn zero_plus_zero() {
    assert_eq!(big_add(b"0", b"0"), b"0");
}

#[cfg(test)]
#[test]
#[should_panic]
fn empty_string() {
    assert_eq!(big_add(b"", b"123"), b"");
}

#[cfg(test)]
#[should_panic]
#[test]
fn non_digits() {
    assert_eq!(big_add(b"abc", b"123"), b"");
}

#[cfg(test)]
#[test]
fn basic_add() {
    assert_eq!(big_add(b"1", b"1"), b"2");
    assert_eq!(big_add(b"10", b"1"), b"11");
    assert_eq!(big_add(b"100", b"23"), b"123");
}

#[cfg(test)]
#[test]
fn middle_carry() {
    assert_eq!(big_add(b"1", b"19"), b"20");
    assert_eq!(big_add(b"5", b"18"), b"23");
}

#[cfg(test)]
#[test]
fn end_carry() {
    assert_eq!(big_add(b"9", b"1"), b"10");
    assert_eq!(big_add(b"9", b"3"), b"12");
    assert_eq!(big_add(b"900", b"100"), b"1000");
}

#[cfg(test)]
#[test]
fn full_carry() {
    assert_eq!(big_add(b"999", b"1"), b"1000");
}

#[cfg(test)]
#[test]
fn huge_numbers() {
    assert_eq!(
        big_add(
            b"100000000000000000000000000000000",
            b"100000000000000000000000000000000"
        ),
        b"200000000000000000000000000000000"
    );
}
