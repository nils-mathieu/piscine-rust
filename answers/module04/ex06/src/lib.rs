#[derive(Debug)]
pub struct FormatError;

pub type WriteFn<'a> = dyn 'a + FnMut(&str) -> Result<(), FormatError>;

pub trait Print {
    fn print(&self, f: &mut WriteFn) -> Result<(), FormatError>;
}

impl Print for u32 {
    fn print(&self, f: &mut WriteFn) -> Result<(), FormatError> {
        if *self == 0 {
            return f("0");
        }

        let mut n = *self;
        let mut buf = [0; 10];
        let mut i = buf.len();
        while n != 0 {
            i -= 1;
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
        }

        f(std::str::from_utf8(&buf[i..]).unwrap())
    }
}

impl<'a> Print for &'a str {
    fn print(&self, f: &mut WriteFn) -> Result<(), FormatError> {
        f(*self)
    }
}

fn format_with(mut s: &str, values: &[&dyn Print], write: &mut WriteFn) -> Result<(), FormatError> {
    let mut i = 0;

    loop {
        let (before, after) = match s.find('%') {
            Some(i) => (&s[..i], &s[i + 1..]),
            None => return write(s),
        };

        write(before)?;
        values[i].print(write)?;
        i += 1;

        s = after;
    }
}

pub fn format_string(s: &str, values: &[&dyn Print]) -> Result<String, FormatError> {
    let mut buffer = String::new();
    format_with(s, values, &mut |s| {
        buffer.push_str(s);
        Ok(())
    })?;
    Ok(buffer)
}

pub fn format_print(s: &str, values: &[&dyn Print]) -> Result<(), FormatError> {
    format_with(s, values, &mut |s| {
        print!("{}", s);
        Ok(())
    })?;
    println!();
    Ok(())
}

#[test]
fn test_u32() {
    let mut buffer = String::new();
    let mut write = |s: &str| {
        buffer.push_str(s);
        Ok(())
    };

    42u32.print(&mut write).unwrap();
    assert_eq!(buffer, "42");
}

#[test]
fn format_string_test() {
    let s: String = format_string("salut % les % gens!", &[&14u32, &"Hello!"]).unwrap();
    assert_eq!(s, "salut 14 les Hello! gens!");
}
