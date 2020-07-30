use std::error::Error;
use std::fmt;

/// Error that occurs when failing to parse an optional structure
#[derive(Debug, Clone)]
pub struct BooleanError {
    pub value: u8,
}

impl BooleanError {
    pub fn new(value: u8) -> BooleanError {
        BooleanError { value }
    }
}

impl fmt::Display for BooleanError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Bool resolved to {}, expected 0 or 1.", self.value)
    }
}

impl Error for BooleanError {}

/// Error that occurs when an enum is constructed from a bad integer
#[derive(Debug, Clone)]
pub struct EnumerationError {
    pub enum_name: String,
    pub value: i64,
}

impl EnumerationError {
    pub fn new(name: &str, value: i64) -> EnumerationError {
        EnumerationError {
            enum_name: name.to_owned(),
            value,
        }
    }
}

impl fmt::Display for EnumerationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Invalid enum value {} for enumeration {}",
            self.value, self.enum_name
        )
    }
}

impl Error for EnumerationError {}

/// Error that occurs when a value is mismatched
#[derive(Debug, Clone)]
pub struct BadValueError {
    pub possible_values: Option<String>,
    pub value: String,
}

impl BadValueError {
    pub fn new<A, B>(value: A, possible_values: Option<B>) -> BadValueError
    where
        A: ToString,
        B: ToString,
    {
        BadValueError {
            value: value.to_string(),
            possible_values: possible_values.map(|x| x.to_string()),
        }
    }
}

impl fmt::Display for BadValueError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.possible_values.as_ref() {
            Some(x) => write!(fmt, "'{}' is not a valid value; expected {}", self.value, x),
            None => write!(fmt, "'{}' is not a valid value", self.value),
        }
    }
}

impl Error for BadValueError {}

/// Error that occurs when failing to read a structure
#[derive(Debug)]
pub struct StructUnpackError {
    pub structname: String,
    pub structpath: String,
    pub error: Box<dyn Error>,
}

impl fmt::Display for StructUnpackError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Could not read into {}::{}: {}",
            self.structname, self.structpath, self.error
        )
    }
}

impl Error for StructUnpackError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.error.as_ref())
    }
}

pub type StructUnpackResult<T> = Result<T, StructUnpackError>;

pub fn unpack_map<A, B, T, E>(data: Result<T, E>, sname: A, spath: B) -> StructUnpackResult<T>
where
    A: ToString,
    B: ToString,
    E: Into<Box<dyn Error>>,
{
    data.map_err(|y| StructUnpackError {
        structname: sname.to_string(),
        structpath: spath.to_string(),
        error: y.into(),
    })
}

pub fn unpack_map_index<A, B, C, T, E>(
    data: Result<T, E>,
    sname: A,
    spath: B,
    pathi: C,
) -> StructUnpackResult<T>
where
    A: ToString,
    B: fmt::Display,
    C: fmt::Display,
    E: Into<Box<dyn Error>>,
{
    data.map_err(|y| StructUnpackError {
        structname: sname.to_string(),
        structpath: format!("{}[{}]", spath, pathi),
        error: y.into(),
    })
}

pub fn unpack_prepend<A, B, T>(
    data: StructUnpackResult<T>,
    sname: A,
    spath: B,
) -> StructUnpackResult<T>
where
    A: ToString,
    B: fmt::Display,
{
    data.map_err(|x| StructUnpackError {
        structname: sname.to_string(),
        structpath: format!("{}.{}", x.structpath, spath),
        error: x.error,
    })
}

/*
/// Error that occurs when failing to read a structure
#[derive(Debug)]
pub struct UnpackError {
    pub structname: String,
    pub structpath: String,
    pub error: Box<dyn Error + 'static>
}

impl fmt::Display for UnpackError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Could not read into {}::{}: {}", self.structname, self.structpath, self.error)
    }
}

impl Error for UnpackError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.error.as_ref())
    }
}

pub type UnpackResult<T> = Result<T, UnpackError>;

pub trait UnpackTrait<T> {
    fn unpack_map<A, B>(self, sname: A, spath: B) -> UnpackResult<T>
    where
        A: ToString,
        B: ToString;
}

impl<T, E> UnpackTrait<T> for Result<T, E>
where E: Into<Box<dyn Error>>
{
    fn unpack_map<A, B>(self, sname: A, spath: B) -> UnpackResult<T>
    where
        A: ToString,
        B: ToString
    {
        self.map_err(|y| {
            UnpackError {
                structname: sname.to_string(),
                structpath: spath.to_string(),
                error: y.into()
            }
        })
    }
}

pub trait UnpackResultTrait<T> {
    fn unpack_prepend<A, B>(self, sname: A, spath: B) -> UnpackResult<T>
    where
        A: ToString,
        B: fmt::Display;
}

impl<T> UnpackResultTrait<T> for UnpackResult<T> {
    fn unpack_prepend<A, B>(self, sname: A, spath: B) -> UnpackResult<T>
    where
        A: ToString,
        B: fmt::Display
    {
        self.map_err(|x| UnpackError {
            structname: sname.to_string(),
            structpath: format!("{}.{}", x.structpath, spath),
            error: x.error
        })
    }
}
*/
