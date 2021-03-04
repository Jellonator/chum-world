// use crate::common;
use crate::error;
use crate::format::TotemFormat;
use std::io::{self, Read, Write};

pub trait ChumBinary: Sized {
    fn read_from(file: &mut dyn Read, fmt: TotemFormat) -> error::StructUnpackResult<Self>;
    fn write_to(&self, writer: &mut dyn Write, fmt: TotemFormat) -> io::Result<()>;
}
