pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
    let mut i = 0;
    while i < pattern.len() && pattern[i] != b'*' {
        if i >= query.len() || pattern[i] != query[i] {
            return false;
        }
        i += 1;
    }
    if i == pattern.len() {
        return i == query.len();
    }
    i += 1;
    while i < pattern.len() && pattern[i] == b'*' {
        i += 1;
    }
    if i == pattern.len() {
        return true;
    }
    let mut j = 0;
    while j < query.len() {
        if strpcmp(&query[j..], &pattern[i..]) {
            return true;
        }
        j += 1;
    }
    false
}

#[test]
#[cfg(test)]
fn exact_match() {
    assert!(strpcmp(b"abc", b"abc"));
    assert!(strpcmp(b"", b""));
    assert!(strpcmp(b"hello", b"hello"));

    assert!(!strpcmp(b"", b"abc"));
    assert!(!strpcmp(b"abc", b""));
    assert!(!strpcmp(b"abc", b"def"));
}

#[test]
#[cfg(test)]
fn match_begining() {
    assert!(strpcmp(b"ab", b"ab*"));
    assert!(strpcmp(b"abc", b"ab*"));
    assert!(strpcmp(b"abcd", b"ab*"));

    assert!(!strpcmp(b"aab", b"ab*"));
}

#[test]
#[cfg(test)]
fn match_end() {
    assert!(strpcmp(b"ab", b"*ab"));
    assert!(strpcmp(b"cab", b"*ab"));
    assert!(strpcmp(b"dcab", b"*ab"));

    assert!(!strpcmp(b"abc", b"*ab"));
}

#[test]
#[cfg(test)]
fn match_start_and_end() {
    assert!(strpcmp(b"abcd", b"ab*cd"));
    assert!(strpcmp(b"ab0cd", b"ab*cd"));
    assert!(strpcmp(b"ab00cd", b"ab*cd"));
    assert!(strpcmp(b"ababcd", b"ab*cd"));

    assert!(!strpcmp(b"ab0dd", b"ab*cd"));
    assert!(!strpcmp(b"ab0cdd", b"ab*cd"));
    assert!(!strpcmp(b"bb0cd", b"ab*cd"));
    assert!(!strpcmp(b"aab0cd", b"ab*cd"));
}

#[test]
#[cfg(test)]
fn contains_substring() {
    assert!(strpcmp(b"ab000ab", b"*000*"));
    assert!(strpcmp(b"000", b"*000*"));
    assert!(strpcmp(b"000ab", b"*000*"));
    assert!(strpcmp(b"ab000", b"*000*"));

    assert!(!strpcmp(b"ab00", b"*000*"));
    assert!(!strpcmp(b"ab00ab", b"*000*"));
    assert!(!strpcmp(b"00ab", b"*000*"));
}

#[test]
#[cfg(test)]
fn full_match() {
    assert!(strpcmp(b"", b"*"));
    assert!(strpcmp(b"abc", b"*"));
}
