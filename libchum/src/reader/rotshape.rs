use crate::common::*;
use crate::format::TotemFormat;
use std::io::{self, Read};

pub struct RotShape {
    pub transform: TransformationHeader,
    pub unk5: Vector3,
    pub unk7: f32,
    pub size: [Vector3; 2],
    pub texcoords: [Vector2; 4],
    pub materialanim_id: i32,
    pub billboard_mode: BillBoardMode,
}

impl RotShape {
    /// Read a RotShape from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<RotShape> {
        let transform = TransformationHeader::read_from(file, fmt)?;
        let _unk4 = fmt.read_u32(file)?;
        let unk5 = read_vec3(file, fmt)?;
        let _unk6 = fmt.read_u32(file)?;
        let unk7 = fmt.read_f32(file)?;
        let _unk8 = fmt.read_u32(file)?;
        let size = [read_vec3(file, fmt)?, read_vec3(file, fmt)?];
        let _unk9 = fmt.read_u32(file)?;
        let texcoords = [
            read_vec2(file, fmt)?,
            read_vec2(file, fmt)?,
            read_vec2(file, fmt)?,
            read_vec2(file, fmt)?,
        ];
        let _unk10 = fmt.read_u32(file)?;
        let materialanim_id = fmt.read_i32(file)?;
        let billboard_mode = BillBoardMode::from_u16(fmt.read_u16(file)?).unwrap();
        Ok(RotShape {
            transform,
            unk5,
            unk7,
            size,
            texcoords,
            materialanim_id,
            billboard_mode,
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BillBoardMode {
    YAxis,
    Full,
}

impl BillBoardMode {
    pub fn to_u16(&self) -> u16 {
        match self {
            BillBoardMode::YAxis => 0,
            BillBoardMode::Full => 1,
        }
    }

    pub fn from_u16(value: u16) -> Option<BillBoardMode> {
        match value {
            0 => Some(BillBoardMode::YAxis),
            1 => Some(BillBoardMode::Full),
            _ => None,
        }
    }
}
