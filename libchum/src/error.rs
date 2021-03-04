use std::error;
use std::fmt;
use thiserror;

/// Error that can occur while unpacking a structure.
/// This can be while destructuring, or reading binary data.
#[derive(Debug, thiserror::Error)]
pub enum UnpackError {
    #[error("Boolean value resolved to {value}, expected 0 or 1")]
    InvalidBoolean { value: u8 },
    #[error("Invalid value {value} for enumeration {enum_name}")]
    InvalidEnumeration { enum_name: String, value: i64 },
    #[error("Invalid value variant {value}; expected one of {expected:?}")]
    InvalidVariant {
        expected: Vec<String>,
        value: String,
    },
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Error that occurs when failing to read a structure
#[derive(Debug)]
pub struct StructUnpackError {
    pub structname: String,
    pub structpath: String,
    pub error: UnpackError,
}

impl StructUnpackError {
    pub fn prepend(self, s: &str) -> StructUnpackError {
        StructUnpackError {
            structname: self.structname,
            structpath: format!("{}{}", s, self.structpath),
            error: self.error,
        }
    }

    pub fn structuralize<S>(self, structname: S, pathname: &str) -> StructUnpackError
    where
        S: Into<String>,
    {
        StructUnpackError {
            structname: structname.into(),
            structpath: format!("{}.{}", pathname, self.structpath),
            error: self.error,
        }
    }
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

impl error::Error for StructUnpackError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.error)
    }
}

pub type StructUnpackResult<T> = Result<T, StructUnpackError>;

pub fn unpack_map<A, B, T, E>(data: Result<T, E>, sname: A, spath: B) -> StructUnpackResult<T>
where
    A: ToString,
    B: ToString,
    E: Into<UnpackError>,
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
    E: Into<UnpackError>,
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
