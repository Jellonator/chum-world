use crate::common::*;
use crate::format::TotemFormat;
use crate::structure::ChumEnum;
use std::io::{self, Read, Write};

chum_struct! {
    /// Rotation shape
    pub struct RotShape {
        pub transform: [struct TransformationHeader],
        pub offset: [Vector3],
        pub unk7: [reference],
        pub size: [fixed array [Vector3] 2],
        pub texcoords: [fixed array [Vector2] 4],
        pub materialanim_id: [reference MATERIALANIM],
        pub billboard_mode: [enum BillBoardMode],
    }
}

impl RotShape {
    /// Read a RotShape from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<RotShape> {
        let transform = TransformationHeader::read_from(file, fmt)?;
        let _unk4 = fmt.read_u32(file)?;
        let offset = read_vec3(file, fmt)?;
        let _unk6 = fmt.read_u32(file)?;
        let unk7 = fmt.read_i32(file)?;
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
        let billboard_mode = BillBoardMode::from_u32(fmt.read_u16(file)? as u32).unwrap();
        Ok(RotShape {
            transform,
            offset,
            unk7,
            size,
            texcoords,
            materialanim_id,
            billboard_mode,
        })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        self.transform.write_to(writer, fmt)?;
        fmt.write_u32(writer, 1)?;
        write_vec3(&self.offset, writer, fmt)?;
        fmt.write_u32(writer, 1)?;
        fmt.write_i32(writer, self.unk7)?;
        fmt.write_u32(writer, 2)?;
        write_vec3(&self.size[0], writer, fmt)?;
        write_vec3(&self.size[1], writer, fmt)?;
        fmt.write_u32(writer, 4)?;
        for i in 0..4 {
            write_vec2(&self.texcoords[i], writer, fmt)?;
        }
        fmt.write_u32(writer, 1)?;
        fmt.write_i32(writer, self.materialanim_id)?;
        fmt.write_u16(writer, self.billboard_mode.to_u32() as u16)?;
        Ok(())
    }
}

chum_enum! {
    #[derive(Copy, Clone, Debug)]
    pub enum BillBoardMode {
        YAxis,
        Full,
    }
}