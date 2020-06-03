use crate::format::TotemFormat;
use std::io::{self, Read};

pub struct Material {
    texture: i32,
    texture_reflection: i32
}

impl Material {
    pub fn get_texture(&self) -> i32 {
        self.texture
    }

    pub fn get_texture_reflection(&self) -> i32 {
        self.texture_reflection
    }

    /// Read a TMesh from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<Material> {
        fmt.skip_n_bytes(file, 101)?; // unknown data
        let tex = fmt.read_i32(file)?;
        let tex_ref = fmt.read_i32(file)?;
        Ok(Material {
            texture: tex,
            texture_reflection: tex_ref
        })
    }

    /// Read a TMesh from data
    pub fn read_data(data: &[u8], fmt: TotemFormat) -> io::Result<Material> {
        Material::read_from(&mut data.as_ref(), fmt)
    }
}