/// Computes the index of the smallest value in `slice`.
///
/// When there provided slice admits several minimums, the last one is returned.
///
/// # Panics
///
/// If the slice is empty, the function panics.
#[allow(clippy::needless_range_loop)]
fn min_of(slice: &[i32]) -> usize {
    assert!(
        !slice.is_empty(),
        "refusing to compute a minimum among no numbers"
    );

    let mut min_index = 0;
    let mut min_so_far = i32::MAX;

    for i in 0..slice.len() {
        if slice[i] <= min_so_far {
            min_so_far = slice[i];
            min_index = i;
        }
    }

    min_index
}

pub fn sort_slice(mut slice: &mut [i32]) {
    while !slice.is_empty() {
        let min_index = min_of(slice);
        slice.swap(0, min_index);
        slice = &mut slice[1..];
    }
}

#[test]
#[should_panic(expected = "refusing to compute a minimum among no numbers")]
fn min_of_empty() {
    min_of(&[]);
    unreachable!();
}

#[test]
fn min_of_one() {
    assert_eq!(min_of(&[1]), 0);
}

#[test]
fn min_of_two() {
    assert_eq!(min_of(&[1, 2]), 0);
    assert_eq!(min_of(&[2, 1]), 1);
}

#[test]
fn multiple_min_gives_last() {
    assert_eq!(min_of(&[1, 2, 1]), 2);
}

#[test]
fn sort_empty() {
    let mut arr = [];
    sort_slice(&mut arr);
    assert_eq!(arr, []);
}

#[test]
fn sort_one() {
    let mut arr = [1];
    sort_slice(&mut arr);
    assert_eq!(arr, [1]);
}

#[test]
fn sort_two() {
    let mut arr = [2, 1];
    sort_slice(&mut arr);
    assert_eq!(arr, [1, 2]);
    sort_slice(&mut arr);
    assert_eq!(arr, [1, 2]);
}

#[test]
fn sort_five() {
    let mut arr = [4, 3, 2, 5, 1];
    sort_slice(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}

#[test]
fn sort_almost_sorted() {
    let mut arr = [1, 2, 3, 5, 4];
    sort_slice(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}
