/// Returns whether `a` fits into `b`.
fn fits_into(a: [u32; 2], b: [u32; 2]) -> bool {
    a[0] <= b[0] && a[1] <= b[1]
}

/// Returns the index of the largest box.
///
/// # Panics
///
/// * If the input slice is empty, the function panics.
fn find_max(boxes: &[[u32; 2]]) -> usize {
    let mut max_so_far = 0;

    for i in 1..boxes.len() {
        if fits_into(boxes[max_so_far], boxes[i]) {
            max_so_far = i;
        }
    }

    max_so_far
}

pub fn sort_boxes(boxes: &mut [[u32; 2]]) {
    let mut unsorted: &mut [[u32; 2]] = &mut boxes[..];

    while !unsorted.is_empty() {
        let max = find_max(unsorted);
        unsorted.swap(0, max);
        unsorted = &mut unsorted[1..];
    }

    for i in 0..boxes.len() - 1 {
        assert!(fits_into(boxes[i + 1], boxes[i]));
    }
}

#[cfg(test)]
#[test]
#[should_panic]
fn test_impossible() {
    let mut boxes = [[1, 2], [2, 1]];
    sort_boxes(&mut boxes);
}

#[test]
#[cfg(test)]
fn example() {
    let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
    sort_boxes(&mut boxes);
    assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
}
