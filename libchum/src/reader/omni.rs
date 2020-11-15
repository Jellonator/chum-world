use crate::common::*;
use crate::format::TotemFormat;
use crate::util::error::*;
// use std::error::Error;
use std::io::{self, Read, Write};

chum_struct! {
    pub struct Omni {
        pub transform: [struct TransformationHeader],
        pub color: [Vector3 rgb],
        pub unknown1: [u8],
        pub junk: [fixed array [u8] 3],
        pub unknown2: [Vector2]
    }
}

impl Omni {
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> StructUnpackResult<Omni> {
        Ok(Omni{
            transform: unpack_map(TransformationHeader::read_from(file, fmt), "Omni", "transform")?,
            color: unpack_map(read_vec3(file, fmt), "Omni", "color")?,
            unknown1: unpack_map(fmt.read_u8(file), "Omni", "unknown1")?,
            junk: [
                unpack_map_index(fmt.read_u8(file), "Omni", "junk", 0)?,
                unpack_map_index(fmt.read_u8(file), "Omni", "junk", 1)?,
                unpack_map_index(fmt.read_u8(file), "Omni", "junk", 2)?,
            ],
            unknown2: unpack_map(read_vec2(file, fmt), "Omni", "unknown2")?,
        })
    }

    /// Write a Material to a file
    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        self.transform.write_to(writer, fmt)?;
        write_vec3(&self.color, writer, fmt)?;
        fmt.write_u8(writer, self.unknown1)?;
        for value in self.junk.iter() {
            fmt.write_u8(writer, *value)?;
        }
        write_vec2(&self.unknown2, writer, fmt)?;
        Ok(())
    }
}