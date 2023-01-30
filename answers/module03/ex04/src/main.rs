use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
struct Time {
    hours: u32,
    minutes: u32,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} hours, {} minutes", self.hours, self.minutes)
    }
}

#[derive(Debug)]
enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}

impl Display for TimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingColon => f.write_str("missing ':'"),
            Self::InvalidLength => f.write_str("invalid length"),
            Self::InvalidNumber => f.write_str("invalid number"),
        }
    }
}

impl FromStr for Time {
    type Err = TimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        if bytes.len() != 5 {
            return Err(TimeParseError::InvalidLength);
        }

        if bytes[2] != b':' {
            return Err(TimeParseError::MissingColon);
        }

        if !bytes[0].is_ascii_digit() || !bytes[1].is_ascii_digit() {
            return Err(TimeParseError::InvalidNumber);
        }

        let hours = ((bytes[0] - b'0') * 10 + bytes[1] - b'0') as u32;
        if hours >= 60 {
            return Err(TimeParseError::InvalidNumber);
        }

        if !bytes[3].is_ascii_digit() || !bytes[4].is_ascii_digit() {
            return Err(TimeParseError::InvalidNumber);
        }

        let minutes = ((bytes[3] - b'0') * 10 + bytes[4] - b'0') as u32;
        if minutes >= 60 {
            return Err(TimeParseError::InvalidNumber);
        }

        Ok(Self { hours, minutes })
    }
}

fn main() {
    let a: Time = "12:20".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();

    println!("{a}");
    println!("{b}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
}
