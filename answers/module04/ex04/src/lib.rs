pub struct Increasing<I: Iterator> {
    iter: I,
    last: Option<I::Item>,
}

impl<I: Iterator> Increasing<I> {
    pub fn new<J>(iter: J) -> Self
    where
        J: IntoIterator<IntoIter = I>,
    {
        Self {
            iter: iter.into_iter(),
            last: None,
        }
    }
}

impl<I> Iterator for Increasing<I>
where
    I: Iterator,
    I::Item: PartialOrd + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next()?;

            if let Some(ref last) = self.last {
                if next <= *last {
                    continue;
                }
            }

            self.last = Some(next.clone());
            break Some(next);
        }
    }
}

#[test]
fn test() {
    let mut iter = Increasing::new([0.4, 0.2, 0.1, 0.2, 0.4, 0.5, 0.4, 0.6]);
    assert_eq!(iter.next(), Some(0.4));
    assert_eq!(iter.next(), Some(0.5));
    assert_eq!(iter.next(), Some(0.6));
    assert_eq!(iter.next(), None);
}
