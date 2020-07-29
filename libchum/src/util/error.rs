use std::fmt;
use std::error::Error;

/// Error that occurs when failing to read a structure
pub struct ChumReadError {
    pub structname: String,
    pub structpath: String,
    pub error: Box<dyn Error>
}

// #[macro_export]
// macro_rules! cread_map {
//     ($x: expr, $sname: expr, $spath: expr) => {
//         $x.map_err(|y| {
//             ChumReadError {
//                 structname: $sname.to_string(),
//                 structpath: $spath.to_string(),
//                 error: Box::new(y)
//             }
//         })
//     };
// }

pub fn cread_map<T, E, A, B>(x: Result<T, E>, sname: A, spath: B) -> Result<T, ChumReadError>
where
    E: Error + 'static,
    A: ToString,
    B: ToString
{
    x.map_err(|y| {
        ChumReadError {
            structname: sname.to_string(),
            structpath: spath.to_string(),
            error: Box::new(y)
        }
    })
}

/// If the expression results in an Err,
/// transform that result into a ChumReadError and return it.
#[macro_export]
macro_rules! cread_try {
    ($x: expr, $sname: expr, $spath: expr) => {
        cread_map($x, $sname, $spath)?
    };
}

/// Create a new error by prepending the given path into
/// the error's path.
/// E.g. if you have an error with the path "foo.bar" and you prepend "baz",
/// the resulting error will have the path "baz.foo.bar".
/// This macro will preserve the error and will replace the structure name.
#[macro_export]
macro_rules! cread_prepend {
    ($x: expr) => {
        $x.map_err(|y| {
            ChumReadError {
                structname: $sname.to_string(),
                structpath: format!("{}.{}", $spath, y.structpath),
                error: y.error
            }
        })
    };
}