/// Finds the longuest increasing sequence in `slice`, assuming that the first element of the slice
/// *must* be the first element of the LIS.
pub fn lis_(slice: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    if slice.is_empty() {
        return result;
    }

    for start in 1..slice.len() {
        if slice[0] >= slice[start] {
            continue;
        }

        let tmp = lis_(&slice[start..]);
        if tmp.len() > result.len() {
            result = tmp;
        }
    }

    result.insert(0, slice[0]);
    result
}

pub fn lis(slice: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for start in 0..slice.len() {
        let tmp = lis_(&slice[start..]);
        if tmp.len() > result.len() {
            result = tmp;
        }
    }

    result
}

#[test]
#[cfg(test)]
fn easy() {
    assert_eq!(lis(&[2, 1, 3]), [2, 3]);
}

#[test]
#[cfg(test)]
fn hard() {
    assert_eq!(lis(&[2, 1, 4, 2, 4]), [1, 2, 4]);
}

#[test]
#[cfg(test)]
fn empty_list() {
    assert_eq!(lis(&[]), []);
}
