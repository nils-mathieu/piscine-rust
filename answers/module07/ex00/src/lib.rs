pub fn ft_swap<T>(a: &mut T, b: &mut T) {
    unsafe {
        // SAFETY:
        //  We are logically moving the value of `a` into `tmp`. Accessing `a` right now would
        //  be undefined behavior.
        let mut tmp = std::ptr::read(a);

        // SAFETY:
        //  We are moving `b` into `a`, overwriting the invalid value that was previously there.
        //  Accessing `a` is valid again, but `b` is now invalid.
        std::ptr::write(a, std::ptr::read(b));

        // SAFETY:
        //  We are moving the temporary value into `b`, making `b` valid again.
        std::ptr::write(b, tmp);
    }
}

/// # Safety
///
/// The caller must ensure that `s` is a valid pointer to a null-terminated string. It must be safe
/// to read from `s` until a null byte is found.
pub unsafe fn ft_strlen(s: *const u8) -> usize {
    let mut size = 0;

    // SAFETY:
    //  The caller ensures that reading `s` until the first null byte is valid.
    while unsafe { *s.add(size).read() } != 0 {
        size += 1;
    }
    size
}

/// # Safety
///
/// The caller must ensure that `dst` is a valid pointer to a buffer of at least
/// `ft_strlen(src) + 1` bytes. `src` must be a valid pointer to a null-terminated string.
pub unsafe fn ft_strcpy(dst: *mut u8, src: *const u8) -> *mut u8 {
    let mut i = 0;

    // SAFETY:
    //  The caller ensures that reading `src` until the first null byte is valid.
    while unsafe { *src.add(i).read() } != 0 {
        // SAFETY:
        //  The caller ensures that writing to `dst` for the first `ft_strlen(src)` bytes is valid.
        unsafe { *dst.add(i).write(*src.add(i).read()) };
        i += 1;
    }
    // SAFETY:
    //  The caller ensures that writing to `dst` at index `ft_strlen(src)` is valid.
    unsafe { *dst.add(i).write(0) };

    dst
}
