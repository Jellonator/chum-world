//! See https://github.com/Jellonator/chum-world/wiki/MATERIAL for more information

use crate::common;
use crate::format::TotemFormat;
use std::io::{self, Read};

/// Material data
pub struct Material {
    pub color: [f32; 4],
    pub transform: common::Mat3x3,
    pub texture: i32,
    pub texture_reflection: i32,
}

impl Material {
    /// Get the ID for this material's texture
    pub fn get_texture(&self) -> i32 {
        self.texture
    }

    /// Get the ID for this material's reflection. Might be 0.
    pub fn get_texture_reflection(&self) -> i32 {
        self.texture_reflection
    }

    /// Read a TMesh from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<Material> {
        let mut coldata = [0.0; 4];
        fmt.read_f32_into(file, &mut coldata)?;
        fmt.skip_n_bytes(file, 3 * 4)?; // Unknown
        fmt.skip_n_bytes(file, 4)?; // Junk
        let mut matdata = [0.0; 9];
        fmt.read_f32_into(file, &mut matdata)?;
        fmt.skip_n_bytes(file, 20)?; // Junk(probably)
        fmt.skip_n_bytes(file, 13)?; // Unknown
        let tex = fmt.read_i32(file)?;
        let tex_ref = fmt.read_i32(file)?;
        Ok(Material {
            color: coldata,
            transform: common::Mat3x3 { mat: matdata },
            texture: tex,
            texture_reflection: tex_ref,
        })
    }

    /// Read a TMesh from data
    pub fn read_data(data: &[u8], fmt: TotemFormat) -> io::Result<Material> {
        Material::read_from(&mut data.as_ref(), fmt)
    }
}
