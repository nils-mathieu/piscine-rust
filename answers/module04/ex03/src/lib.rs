pub struct Fibs {
    curr: Option<u32>,
    next: Option<u32>,
}

impl Fibs {
    pub fn new(curr: u32, next: u32) -> Self {
        Self {
            curr: Some(curr),
            next: Some(next),
        }
    }
}

impl Iterator for Fibs {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr.take()?;

        if let Some(next) = self.next.take() {
            self.next = curr.checked_add(next);
            self.curr = Some(next);
        }

        Some(curr)
    }
}

pub fn even_fibs_bellow_1000() -> u32 {
    Fibs::new(0, 1)
        .take_while(|&fib| fib < 1000)
        .filter(|&fib| fib % 2 == 0)
        .sum()
}

#[test]
fn first_fibs() {
    let mut fibs = Fibs::new(0, 1);

    assert_eq!(fibs.next(), Some(0));
    assert_eq!(fibs.next(), Some(1));
    assert_eq!(fibs.next(), Some(1));
    assert_eq!(fibs.next(), Some(2));
    assert_eq!(fibs.next(), Some(3));
    assert_eq!(fibs.next(), Some(5));
}

#[test]
fn fibs_count() {
    assert_eq!(Fibs::new(0, 1).count(), 48);
}

#[test]
fn test() {
    let mut count = 0;
    for fib in Fibs::new(0, 1) {
        if fib >= 1000 {
            break;
        }
        if fib % 2 == 0 {
            count += fib;
        }
    }

    assert_eq!(count, 798);
}

#[test]
fn test_event_fibs_bellow_1000() {
    assert_eq!(even_fibs_bellow_1000(), 798);
}
