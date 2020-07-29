use crate::common::*;
use crate::format::TotemFormat;
use std::io::{self, Read};

pub struct CollisionVol {
    pub transform: TransformationHeader,
    pub unk1: u32,
    pub local_transform: Mat4x4,
    pub local_transform_inv: Mat4x4,
    pub unk2: u32,
    pub unk3: u32,
    pub node_ids: [i32; 10],
    pub unk4: [f32; 10],
    pub unk5: Vec<i32>,
    pub bitmaps: Vec<i32>,
    pub volume_type: i32,
    pub unk6: u32
}

impl CollisionVol {
    /// Read a CollisionVol from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<CollisionVol> {
        Ok(CollisionVol {
            transform: TransformationHeader::read_from(file, fmt)?,
            unk1: fmt.read_u32(file)?,
            local_transform: read_mat4(file, fmt)?,
            local_transform_inv: read_mat4(file, fmt)?,
            unk2: fmt.read_u32(file)?,
            unk3: fmt.read_u32(file)?,
            node_ids: {
                let mut data = [0i32; 10];
                fmt.read_i32_into(file, &mut data)?;
                data
            },
            unk4: {
                let mut data = [0f32; 10];
                fmt.read_f32_into(file, &mut data)?;
                data
            },
            unk5: {
                let num = fmt.read_u32(file)?;
                let mut vec = Vec::with_capacity(num as usize);
                for _ in 0..num {
                    vec.push(fmt.read_i32(file)?);
                }
                vec
            },
            bitmaps: {
                let num = fmt.read_u32(file)?;
                let mut vec = Vec::with_capacity(num as usize);
                for _ in 0..num {
                    vec.push(fmt.read_i32(file)?);
                }
                vec
            },
            volume_type: fmt.read_i32(file)?,
            unk6: fmt.read_u32(file)?
        })
    }
}