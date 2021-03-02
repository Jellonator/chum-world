// use crate::common;
use crate::format::TotemFormat;
use crate::error;
use std::io::{self, Read, Write};

pub trait ChumBinary: Sized {
    fn read_from(file: &mut dyn Read, fmt: TotemFormat) -> error::StructUnpackResult<Self>;
    fn write_to(&self, writer: &mut dyn Write, fmt: TotemFormat) -> io::Result<()>;
}
