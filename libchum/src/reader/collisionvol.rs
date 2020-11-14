use crate::common::*;
use crate::format::TotemFormat;
use crate::util::error::*;
use std::io::{self, Read, Write};

chum_struct! {
    pub struct CollisionVol {
        pub transform: [struct TransformationHeader],
        pub unk1: [u32],
        pub local_transform: [Mat4x4],
        pub local_transform_inv: [Mat4x4],
        pub unk2: [u32],
        pub unk3: [u32],
        pub node_ids: [fixed array [i32] 10],
        pub unk4: [fixed array [f32] 10],
        pub unk5: [dynamic array [i32] 0],
        pub bitmaps: [dynamic array [reference BITMAP] 0],
        pub volume_type: [i32],
        pub unk6: [u32],
    }
}

impl CollisionVol {
    /// Read a CollisionVol from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> StructUnpackResult<CollisionVol> {
        Ok(CollisionVol {
            transform: unpack_map(
                TransformationHeader::read_from(file, fmt),
                "CollisionVol",
                "transform",
            )?,
            unk1: unpack_map(fmt.read_u32(file), "CollisionVol", "unk1")?,
            local_transform: unpack_map(read_mat4(file, fmt), "CollisionVol", "local_transform")?,
            local_transform_inv: unpack_map(
                read_mat4(file, fmt),
                "CollisionVol",
                "local_transform_inv",
            )?,
            unk2: unpack_map(fmt.read_u32(file), "CollisionVol", "unk2")?,
            unk3: unpack_map(fmt.read_u32(file), "CollisionVol", "unk3")?,
            node_ids: {
                let mut data = [0i32; 10];
                unpack_map(
                    fmt.read_i32_into(file, &mut data),
                    "CollisionVol",
                    "node_ids",
                )?;
                data
            },
            unk4: {
                let mut data = [0f32; 10];
                unpack_map(fmt.read_f32_into(file, &mut data), "CollisionVol", "unk4")?;
                data
            },
            unk5: {
                let num = unpack_map(fmt.read_u32(file), "CollisionVol", "unk5.len")?;
                let mut vec = Vec::with_capacity(num as usize);
                for _ in 0..num {
                    vec.push(unpack_map(fmt.read_i32(file), "CollisionVol", "unk5")?);
                }
                vec
            },
            bitmaps: {
                let num = unpack_map(fmt.read_u32(file), "CollisionVol", "bitmaps.len")?;
                let mut vec = Vec::with_capacity(num as usize);
                for _ in 0..num {
                    vec.push(unpack_map(fmt.read_i32(file), "CollisionVol", "bitmaps")?);
                }
                vec
            },
            volume_type: unpack_map(fmt.read_i32(file), "CollisionVol", "volume_type")?,
            unk6: unpack_map(fmt.read_u32(file), "CollisionVol", "unk6")?,
        })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        self.transform.write_to(writer, fmt)?;
        fmt.write_u32(writer, self.unk1)?;
        write_mat4(&self.local_transform, writer, fmt)?;
        write_mat4(&self.local_transform_inv, writer, fmt)?;
        fmt.write_u32(writer, self.unk2)?;
        fmt.write_u32(writer, self.unk3)?;
        for value in self.node_ids.iter() {
            fmt.write_i32(writer, *value)?;
        }
        for value in self.unk4.iter() {
            fmt.write_f32(writer, *value)?;
        }
        fmt.write_u32(writer, self.unk5.len() as u32)?;
        for value in self.unk5.iter() {
            fmt.write_i32(writer, *value)?;
        }
        fmt.write_u32(writer, self.bitmaps.len() as u32)?;
        for value in self.bitmaps.iter() {
            fmt.write_i32(writer, *value)?;
        }
        fmt.write_i32(writer, self.volume_type)?;
        fmt.write_u32(writer, self.unk6)?;
        Ok(())
    }
}
