//! See https://github.com/Jellonator/chum-world/wiki/MATERIAL for more information

use crate::common;
use crate::format::TotemFormat;
use std::io::{self, Read, Write};

// Material data
chum_struct! {
    pub struct Material {
        pub color: [Color],
        pub emission: [Vector3 rgb],
        pub unk2: [f32],
        pub transform: [Mat3x3],
        pub unk3: [fixed array [f32] 5],
        pub unk4: [fixed array [u8] 13],
        pub texture: [reference BITMAP],
        pub texture_reflection: [reference BITMAP],
    }
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
        let color = common::read_color(file, fmt)?;
        let emission = common::read_vec3(file, fmt)?;
        let unk2 = fmt.read_f32(file)?;
        let transform = common::read_mat3(file, fmt)?;
        let mut unk3 = [0.0; 5];
        fmt.read_f32_into(file, &mut unk3)?;
        let mut unk4 = [0u8; 13];
        fmt.read_u8_into(file, &mut unk4)?;
        let tex = fmt.read_i32(file)?;
        let tex_ref = fmt.read_i32(file)?;
        Ok(Material {
            color,
            emission,
            unk2,
            transform,
            unk3,
            unk4,
            texture: tex,
            texture_reflection: tex_ref,
        })
    }

    /// Read a TMesh from data
    pub fn read_data(data: &[u8], fmt: TotemFormat) -> io::Result<Material> {
        Material::read_from(&mut data.as_ref(), fmt)
    }

    /// Write a Material to a file
    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        common::write_color(&self.color, writer, fmt)?;
        common::write_vec3(&self.emission, writer, fmt)?;
        fmt.write_f32(writer, self.unk2)?;
        common::write_mat3(&self.transform, writer, fmt)?;
        for value in self.unk3.iter() {
            fmt.write_f32(writer, *value)?;
        }
        for value in self.unk4.iter() {
            fmt.write_u8(writer, *value)?;
        }
        fmt.write_i32(writer, self.texture)?;
        fmt.write_i32(writer, self.texture_reflection)?;
        Ok(())
    }
}