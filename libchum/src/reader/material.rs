//! See https://github.com/Jellonator/chum-world/wiki/MATERIAL for more information

use crate::common;
use crate::format::TotemFormat;
use crate::structure::*;
use std::io::{self, Read, Write};

/// Material data
pub struct Material {
    pub color: common::Color,
    pub unk1: common::Vector3,
    pub unk2: f32,
    pub transform: common::Mat3x3,
    pub unk3: [f32; 5],
    pub unk4: [u8; 13],
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
        let color = common::Color::read_from(file, fmt)?;
        let unk1 = common::Vector3::read_from(file, fmt)?;
        let unk2 = fmt.read_f32(file)?;
        let transform = common::Mat3x3::read_from(file, fmt)?;
        let mut unk3 = [0.0; 5];
        fmt.read_f32_into(file, &mut unk3)?;
        let mut unk4 = [0u8; 13];
        fmt.read_u8_into(file, &mut unk4)?;
        let tex = fmt.read_i32(file)?;
        let tex_ref = fmt.read_i32(file)?;
        Ok(Material {
            color,
            unk1,
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
        self.color.write_to(writer, fmt)?;
        self.unk1.write_to(writer, fmt)?;
        fmt.write_f32(writer, self.unk2)?;
        self.transform.write_to(writer, fmt)?;
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

impl ChumStruct for Material {
    fn structure(&self) -> ChumStructVariant {
        use ChumStructVariant::*;
        use IntType::*;
        Struct(vec![
            ("color".to_owned(), Color(self.color.clone())),
            ("unk1".to_owned(), Vec3(self.unk1.clone())),
            ("unk2".to_owned(), Float(self.unk2)),
            ("transform".to_owned(), Transform2D(self.transform.clone())),
            (
                "unk3".to_owned(),
                Array(ArrayData {
                    data: self.unk3.iter().map(|x| Float(*x)).collect(),
                    default_value: Box::new(Float(0.0)),
                    can_resize: false,
                }),
            ),
            (
                "unk4".to_owned(),
                Array(ArrayData {
                    data: self.unk4.iter().map(|x| Integer(*x as i64, U8)).collect(),
                    default_value: Box::new(Integer(0, U8)),
                    can_resize: false,
                }),
            ),
            (
                "texture".to_owned(),
                Reference(self.texture, Some("BITMAP".to_owned())),
            ),
            (
                "reflection".to_owned(),
                Reference(self.texture_reflection, Some("BITMAP".to_owned())),
            ),
        ])
    }

    fn destructure(data: ChumStructVariant) -> Result<Self, Box<dyn std::error::Error>> {
        let color = data
            .get_struct_item("color")
            .unwrap()
            .get_color()
            .unwrap()
            .clone();
        let unk1 = *data.get_struct_item("unk1").unwrap().get_vec3().unwrap();
        let unk2 = data.get_struct_item("unk2").unwrap().get_f32().unwrap();
        let transform = data
            .get_struct_item("transform")
            .unwrap()
            .get_transform2d()
            .unwrap()
            .clone();
        let unk3_array: Vec<f32> = data
            .get_struct_item("unk3")
            .unwrap()
            .get_array()
            .unwrap()
            .iter()
            .map(|x| x.get_f32().unwrap())
            .collect();
        let unk4_array: Vec<u8> = data
            .get_struct_item("unk4")
            .unwrap()
            .get_array()
            .unwrap()
            .iter()
            .map(|x| x.get_i64().unwrap() as u8)
            .collect();
        let texture = data
            .get_struct_item("texture")
            .unwrap()
            .get_reference_id()
            .unwrap();
        let texture_reflection = data
            .get_struct_item("texture_reflection")
            .unwrap()
            .get_reference_id()
            .unwrap();
        if unk3_array.len() != 5 {
            panic!();
        }
        if unk4_array.len() != 13 {
            panic!();
        }
        let mut unk3 = [0.0f32; 5];
        let mut unk4 = [0u8; 13];
        unk3.copy_from_slice(&unk3_array);
        unk4.copy_from_slice(&unk4_array);
        Ok(Material {
            color,
            unk1,
            unk2,
            transform,
            unk3,
            unk4,
            texture,
            texture_reflection,
        })
    }
}
