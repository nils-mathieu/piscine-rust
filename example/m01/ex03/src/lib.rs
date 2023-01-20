pub fn add_vectors(a: [i32; 3], b: [i32; 3]) -> [i32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[test]
fn add_one() {
    assert_eq!(add_vectors([1, 2, 3], [1, 1, 1]), [2, 3, 4]);
}

#[test]
fn add_zero() {
    assert_eq!(add_vectors([1, 2, 3,], [0, 0, 0]), [1, 2, 3]);
}

#[test]
fn add_mix() {
    assert_eq!(add_vectors([1, 2, 3], [2, 3, 1]), [3, 5, 4]);
}
