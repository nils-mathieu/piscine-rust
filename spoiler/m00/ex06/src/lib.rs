pub fn fibs(n: u32) -> u32 {
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
}

pub fn is_prime(n: u32) -> bool {
    n >= 2 && !(2..n).any(|d| n % d == 0)
}

#[test]
fn zero_is_not_prime() {
    assert!(!is_prime(0));
}

#[test]
fn one_is_not_prime() {
    assert!(!is_prime(1));
}

#[test]
fn three_is_prime() {
    assert!(is_prime(3));
}

#[test]
fn four_is_not_prime() {
    assert!(!is_prime(4));
}

#[test]
fn first_fib_is_0() {
    assert_eq!(fibs(0), 0);
}

#[test]
fn fifth_fib_is_3() {
    assert_eq!(fibs(4), 3);
}

#[test]
fn seventeenth_fib_is_987() {
    assert_eq!(fibs(16), 987);
}
