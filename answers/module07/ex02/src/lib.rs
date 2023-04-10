use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct Carton<T> {
    data: NonNull<T>,
}

impl<T> Carton<T> {
    pub fn new(val: T) -> Self {
        let layout = Layout::new::<T>();
        let data = unsafe { alloc(layout) as *mut T };
        let data = NonNull::new(data).unwrap_or_else(move || handle_alloc_error(layout));

        // SAFETY:
        //  Populate the memory that we've allocated with the input value, moving it.
        unsafe { data.as_ptr().write(val) };

        Self { data }
    }

    pub fn into_inner(self) -> T {
        // SAFETY:
        //  We're moving the value out of the memory that we've allocated.
        let val = unsafe { self.data.as_ptr().read() };

        // SAFETY:
        //  We are deallocating the memory that we've allocated.
        unsafe { dealloc(self.data.as_ptr() as *mut u8, Layout::new::<T>()) };

        val
    }
}

impl<T> Drop for Carton<T> {
    fn drop(&mut self) {
        // SAFETY:
        //  We control this value, and we know that the Carton won't ever be used, because we are
        //  dropping it.
        unsafe { std::ptr::drop_in_place(self.data.as_ptr()) };

        // SAFETY:
        //  We are deallocating the memory that we've allocated.
        unsafe { dealloc(self.data.as_ptr() as *mut u8, Layout::new::<T>()) };
    }
}

impl<T> Deref for Carton<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY:
        //  We control this value, and we know that it is valid.
        unsafe { self.data.as_ref() }
    }
}

impl<T> DerefMut for Carton<T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY:
        //  We control this value, and we know that it is valid.
        unsafe { self.data.as_mut() }
    }
}

impl<T: Clone> Clone for Carton<T> {
    fn clone(&self) -> Self {
        Self::new((**self).clone())
    }
}

#[test]
fn test() {
    #[derive(Clone)]
    struct Point {
        x: u32,
        y: u32,
    }
    let point_in_carton = Carton::new(Point { x: 1, y: 2 });
    assert_eq!(point_in_carton.x, 1);
    assert_eq!(point_in_carton.y, 2);

    let mut another_point = point_in_carton.clone();
    another_point.x = 2;
    another_point.y = 3;
    assert_eq!(another_point.x, 2);
    assert_eq!(another_point.y, 3);
    assert_eq!(point_in_carton.x, 1);
    assert_eq!(point_in_carton.y, 2);
}
