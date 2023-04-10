pub struct Groups<'a, F> {
    s: &'a str,
    f: F,
}

impl<'a, F> Groups<'a, F> {
    pub fn new(s: &'a str, f: F) -> Self
    where
        F: FnMut(char) -> bool,
    {
        Self { s, f }
    }
}

impl<'a, F> Iterator for Groups<'a, F>
where
    F: FnMut(char) -> bool,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.s = self.s.trim_start_matches(|c| !(self.f)(c));
        let mut splits = self.s.splitn(2, |c| !(self.f)(c));
        let ret = splits.next();
        self.s = splits.next().unwrap_or("");

        if ret == Some("") {
            None
        } else {
            ret
        }
    }
}

impl<'a, F> DoubleEndedIterator for Groups<'a, F>
where
    F: FnMut(char) -> bool,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.s = self.s.trim_end_matches(|c| !(self.f)(c));
        let mut splits = self.s.rsplitn(2, |c| !(self.f)(c));
        let ret = splits.next();
        self.s = splits.next().unwrap_or("");

        if ret == Some("") {
            None
        } else {
            ret
        }
    }
}

#[test]
fn forward() {
    let mut groups = Groups::new("  hello,\tworld ", char::is_alphabetic);

    assert_eq!(groups.next(), Some("hello"));
    assert_eq!(groups.next(), Some("world"));
    assert_eq!(groups.next(), None);
}

#[test]
fn backward() {
    let mut groups = Groups::new("  hello,\tworld ", char::is_alphabetic);

    assert_eq!(groups.next_back(), Some("world"));
    assert_eq!(groups.next_back(), Some("hello"));
    assert_eq!(groups.next_back(), None);
}
