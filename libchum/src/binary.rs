// use crate::common;
use crate::format::TotemFormat;
use crate::util::error;
use std::io::{self, Read, Write};

pub trait ChumBinary: Sized {
    fn read_from(file: &mut dyn Read, fmt: TotemFormat) -> error::StructUnpackResult<Self>;
    fn write_to<W: Write + Sized>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()>;
}

// macro_rules! impl_simple_chum_binary {
//     ($type:ty,$read:ident,$write:ident) => {
//         impl ChumBinary for $type {
//             fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> error::StructUnpackResult<Self> {
//                 fmt.$read(file)
//                 .map_err(|e| $crate::util::error::StructUnpackError {
//                     structname: stringify!($type).to_owned(),
//                     structpath: "".to_owned(),
//                     error: Box::new(e)
//                 })
//             }
//             fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
//                 fmt.$write(writer, *self)
//             }
//         }
//     }
// }

// impl_simple_chum_binary!(u8,read_u8,write_u8);

// impl ChumBinary for u8 {
//     fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> error::StructUnpackResult<Self> {
//         unimplemented!()
//     }
//     fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
//         unimplemented!()
//     }
// }