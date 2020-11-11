use crate::common::*;
use crate::format::TotemFormat;
use std::io::Read;
use crate::util::error::*;

pub struct Warp {
    pub size: f32,
    pub material_ids: [i32; 6],
    pub vertices: [Vector3; 8],
    pub texcoords: [Vector2; 4],
}

impl Warp {
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> StructUnpackResult<Warp> {
        Ok(Warp {
            size: unpack_map(fmt.read_f32(file), "Warp", "size")?,
            material_ids: {
                let mut data = [0i32; 6];
                unpack_map(fmt.read_i32_into(file, &mut data), "Warp", "material_ids")?;
                data
            },
            vertices: {
                let mut data = [Vector3::default(); 8];
                for value in data.iter_mut() {
                    *value = unpack_map(read_vec3(file, fmt), "Warp", "vertices")?;
                }
                data
            },
            texcoords: {
                let mut data = [Vector2::default(); 4];
                for value in data.iter_mut() {
                    *value = unpack_map(read_vec2(file, fmt), "Warp", "texcoords")?;
                }
                data
            }
        })
    }
}