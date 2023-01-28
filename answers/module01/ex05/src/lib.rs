pub fn deduplicate(list: &mut Vec<i32>) {
    let mut i = 0;
    while i < list.len() {
        let mut j = list.len() - 1;
        while j > i {
            if list[i] == list[j] {
                list.remove(j);
            }
            j -= 1;
        }
        i += 1;
    }
}

#[cfg(test)]
#[test]
fn example() {
    let mut v = vec![1, 2, 2, 3, 2, 4, 3];
    deduplicate(&mut v);
    assert_eq!(v, [1, 2, 3, 4]);
}
