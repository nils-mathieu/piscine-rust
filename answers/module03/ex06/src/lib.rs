use std::ops::Index;
use std::ops::IndexMut;

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }

    fn with_next(value: T, next: Box<Node<T>>) -> Self {
        Self {
            value,
            next: Some(next),
        }
    }
}

#[derive(Clone, Default)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, value: T) {
        if let Some(head) = self.head.take() {
            self.head = Some(Box::new(Node::with_next(value, head)));
        } else {
            self.head = Some(Box::new(Node::new(value)));
        }
    }

    pub fn push_back(&mut self, value: T) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }

        *current = Some(Box::new(Node::new(value)));
    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            current = &node.next;
            count += 1;
        }
        count
    }

    pub fn get(&self, mut i: usize) -> Option<&T> {
        let mut current = &self.head;
        while let Some(node) = current {
            if i == 0 {
                return Some(&node.value);
            }
            current = &node.next;
            i -= 1;
        }
        None
    }

    pub fn get_mut(&mut self, mut i: usize) -> Option<&mut T> {
        let mut current = &mut self.head;
        while let Some(node) = current {
            if i == 0 {
                return Some(&mut node.value);
            }
            current = &mut node.next;
            i -= 1;
        }
        None
    }

    pub fn remove_front(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            self.head = head.next;
            Some(head.value)
        } else {
            None
        }
    }

    pub fn remove_back(&mut self) -> Option<T> {
        #[allow(clippy::question_mark)]
        if self.head.is_none() {
            return None;
        }

        let mut current = &mut self.head;
        while current.is_some() {
            if current.as_ref().unwrap().next.is_none() {
                break;
            }
            current = &mut current.as_mut().unwrap().next;
        }

        #[allow(clippy::manual_map)]
        match current.take() {
            Some(node) => Some(node.value),
            None => None,
        }
    }

    pub fn clear(&mut self) {
        self.head = None;
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            Some(val) => val,
            None => panic!("tried to access out of bound index {index}",),
        }
    }
}

impl<T> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self.get_mut(index) {
            Some(val) => val,
            None => panic!("tried to access out of bound index {index}"),
        }
    }
}

#[cfg(test)]
#[test]
fn default_list_is_empty() {
    let list: List<i32> = Default::default();
    assert_eq!(list.count(), 0);
}

#[cfg(test)]
#[test]
fn cloned_list_are_equal() {
    let mut list = List::new();
    list.push_back(String::from("Hello"));
    list.push_back(String::from("World"));

    let cloned = list.clone();
    assert_eq!(cloned.count(), list.count());
    assert_eq!(&cloned[0], &cloned[0]);
    assert_eq!(&cloned[1], &cloned[1]);
}

#[cfg(test)]
#[test]
#[should_panic(expected = "tried to access out of bound index 10")]
fn out_of_bound_access_panics() {
    let mut list: List<u32> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list[10], 42);
}
