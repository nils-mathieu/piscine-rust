fn sum_of(slice: &[u32]) -> u32 {
    let mut s = 0;
    for &b in slice {
        s += b;
    }
    s
}

pub fn smallest_subslice<'a>(slice: &'a [u32], threshold: &u32) -> &'a [u32] {
    for subslice_len in 0..=slice.len() {
        for subslice_start in 0..=slice.len() - subslice_len {
            let subslice = &slice[subslice_start..subslice_start + subslice_len];
            if sum_of(subslice) >= *threshold {
                return subslice;
            }
        }
    }

    &[]
}

#[test]
fn no_solution() {
    assert_eq!(smallest_subslice(&[1, 1, 1], &4), &[]);
}

#[test]
fn max_solution() {
    assert_eq!(smallest_subslice(&[1, 2, 3], &6), &[1, 2, 3]);
}

#[test]
fn multiple_solutions() {
    assert_eq!(smallest_subslice(&[0, 1, 2, 2, 1], &3), &[1, 2]);
}

#[test]
fn example_tests() {
    assert_eq!(smallest_subslice(&[10, 1, 11], &11), [11]);
    assert_eq!(smallest_subslice(&[10, 1, 11], &12), [1, 11]);
    assert_eq!(smallest_subslice(&[10, 1, 11], &13), [10, 1, 11]);
    assert_eq!(smallest_subslice(&[10, 1, 11], &23), []);
}

#[test]
fn test_lifetimes() {
    let array = [3, 4, 1, 2, 12];
    let result;

    {
        let threshold = 1000;
        result = smallest_subslice(&array, &threshold);
    }

    assert_eq!(result, &[]);
}
