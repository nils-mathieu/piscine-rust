use std::fmt::Write;

#[derive(Debug)]
pub struct EncodingError;
#[derive(Debug)]
pub struct DecodingError;

pub trait Field: Sized {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError>;
    fn decode(field: &str) -> Result<Self, DecodingError>;
}

pub trait Record: Sized {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError>;
    fn decode(line: &str) -> Result<Self, DecodingError>;
}

pub fn encode_csv<R: Record>(records: &[R]) -> Result<String, EncodingError> {
    let mut result = String::new();

    for record in records {
        record.encode(&mut result)?;
        result.push('\n');
    }

    Ok(result)
}

pub fn decode_csv<R: Record>(contents: &str) -> Result<Vec<R>, DecodingError> {
    let mut result = Vec::new();

    for line in contents.lines() {
        result.push(R::decode(line)?);
    }

    Ok(result)
}

macro_rules! impl_field_for_int {
    ( $( $t:ty ),* $(,)? ) => {
        $(
            impl Field for $t {
                fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
                    match write!(target, "{self}") {
                        Ok(()) => Ok(()),
                        Err(_) => Err(EncodingError),
                    }
                }

                fn decode(field: &str) -> Result<Self, DecodingError> {
                    match field.parse() {
                        Ok(val) => Ok(val),
                        Err(_) => Err(DecodingError),
                    }
                }
            }
        )*
    };
}

impl_field_for_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl Field for String {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
        if self.contains('\n') || self.contains(',') {
            return Err(EncodingError);
        }

        target.push_str(self);
        Ok(())
    }

    fn decode(field: &str) -> Result<Self, DecodingError> {
        Ok(field.to_string())
    }
}

impl<T: Field> Field for Option<T> {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
        match self {
            Some(v) => v.encode(target),
            None => Ok(()),
        }
    }

    fn decode(field: &str) -> Result<Self, DecodingError> {
        if field.is_empty() {
            Ok(None)
        } else {
            match T::decode(field) {
                Ok(t) => Ok(Some(t)),
                Err(err) => Err(err),
            }
        }
    }
}

#[macro_export]
macro_rules! impl_record {
    ( $ident:ident ( $first_f:ident, $( $f:ident ),* $(,)? )) => {
        impl $crate::Record for $ident {
            fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
                $crate::Field::encode(&self.$first_f, target)?;
                $(
                    target.push(',');
                    $crate::Field::encode(&self.$f, target)?;
                )*
                Ok(())
            }

            fn decode(line: &str) -> Result<Self, DecodingError> {
                let mut splits = line.split(',');

                let $first_f = match splits.next() {
                    Some(field) => $crate::Field::decode(field)?,
                    None => return Err($crate::DecodingError),
                };

                $(
                    let $f = match splits.next() {
                        Some(field) => $crate::Field::decode(field)?,
                        None => return Err($crate::DecodingError),
                    };
                )*

                if splits.next().is_some() {
                    return Err(DecodingError);
                }

                Ok(Self {
                    $first_f,
                    $( $f, )*
                })
            }
        }
    };
}
#[cfg(test)]
#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
}

#[cfg(test)]
impl_record!(User(name, age));

#[cfg(test)]
#[test]
fn test_encode() {
    let database = [
        User {
            name: "aaa".into(),
            age: 23,
        },
        User {
            name: "bb".into(),
            age: 2,
        },
    ];

    let csv = encode_csv(&database).unwrap();

    assert_eq!(
        csv,
        "\
        aaa,23\n\
        bb,2\n\
        "
    );
}

#[cfg(test)]
#[test]
fn test_decode() {
    let csv = "\
        hello,2\n\
        yes,5\n\
        no,100\n\
    ";

    let database: Vec<User> = decode_csv(csv).unwrap();

    assert_eq!(
        database,
        [
            User {
                name: "hello".into(),
                age: 2
            },
            User {
                name: "yes".into(),
                age: 5
            },
            User {
                name: "no".into(),
                age: 100
            },
        ]
    );
}

#[cfg(test)]
#[test]
fn decoding_error() {
    let csv = "\
        hello,2\n\
        yes,6\n\
        no,23,hello\n\
    ";

    decode_csv::<User>(csv).unwrap_err();
}

struct MyType {
    id: u32,
    name: String,
}

// ez
impl_record!(MyType(id, name));

#[cfg(test)]
#[test]
fn test_impl_record() {
    let records = [
        MyType {
            id: 10,
            name: "Marvin".into(),
        },
        MyType {
            id: 11,
            name: "Marvin".into(),
        },
        MyType {
            id: 12,
            name: "Marvin".into(),
        },
    ];

    let csv = encode_csv(&records).unwrap();
    assert_eq!(
        csv,
        "\
        10,Marvin\n\
        11,Marvin\n\
        12,Marvin\n\
        "
    );
}
