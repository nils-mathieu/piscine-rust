pub fn add(a: &i32, b: i32) -> i32 {
    *a + b
}

pub fn add_assign(a: &mut i32, b: i32) {
    *a += b;
}

#[test]
fn add_one() {
    assert_eq!(add(&1, 2), 3);
    assert_eq!(add(&1, 1), 2);
    assert_eq!(add(&2, 1), 3);
}

#[test]
fn add_zero() {
    assert_eq!(add(&0, 1), 1);
    assert_eq!(add(&1, 0), 1);
    assert_eq!(add(&0, 0), 0);
}

#[test]
fn add_assign_one() {
    let mut a = 2;
    add_assign(&mut a, 1);
    assert_eq!(a, 3);
    add_assign(&mut a, 1);
    assert_eq!(a, 4);
}

#[test]
fn add_assign_zero() {
    let mut a = 1;
    add_assign(&mut a, 0);
    assert_eq!(a, 1);
}
