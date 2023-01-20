pub fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a < b {
        a
    } else {
        b
    }
}

#[test]
fn basic_tests() {
    assert_eq!(min(&1, &0), &0);
    assert_eq!(min(&0, &1), &0);
    assert_eq!(min(&1, &1), &1);
}

#[test]
fn min_max() {
    assert_eq!(min(&i32::MIN, &i32::MAX), &i32::MIN);
}

/*
#[test]
fn spike() {
    let a = 0;
    let c;

    {
        let b = 0;
        c = min(&a, &b);
    }

    let _error = c;
}
*/
