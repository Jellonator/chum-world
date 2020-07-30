// use crate::animsymbol::AnimSymbol;
use crate::common::*;
use crate::format::TotemFormat;
use crate::util::error::*;
use std::error::Error;
use std::io::{self, Read};

fn load_option<T, F, R>(
    file: &mut R,
    fmt: TotemFormat,
    func: F,
) -> Result<Option<T>, Box<dyn Error>>
where
    F: Fn(&mut R, TotemFormat) -> io::Result<T>,
    R: Read,
{
    let value = fmt.read_u8(file)?;
    match value {
        0 => Ok(None),
        1 => Ok(Some(func(file, fmt)?)),
        v => Err(Box::new(BooleanError::new(v))),
    }
}

fn load_f32_4<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<[f32; 4]> {
    let mut buf = [0.0f32; 4];
    fmt.read_f32_into(file, &mut buf)?;
    Ok(buf)
}

fn load_f32_9<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<[f32; 9]> {
    let mut buf = [0.0f32; 9];
    fmt.read_f32_into(file, &mut buf)?;
    Ok(buf)
}

fn load_transform<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<Mat4x4> {
    let data = read_mat4(file, fmt)?;
    fmt.skip_n_bytes(file, 16)?; // JUNK
    Ok(data)
}

fn load_empty<R: Read>(_file: &mut R, _fmt: TotemFormat) -> io::Result<()> {
    Ok(())
}

fn load_unkstruct<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<LodUnkStruct> {
    let mut buf = [0.0f32; 4];
    fmt.read_f32_into(file, &mut buf)?;
    Ok(LodUnkStruct {
        unk1: buf,
        unk2: fmt.read_u32(file)?,
    })
}

pub struct Lod {
    pub transform: TransformationHeader,
    pub unk1: Option<[f32; 4]>,
    pub unk2: Option<[f32; 4]>,
    pub unk3: Option<[f32; 9]>,
    pub unk4: Option<Mat4x4>,
    pub unk5: Option<[f32; 4]>,
    pub unk6: Option<()>,
    pub unk7: Option<Mat4x4>,
    pub unk8: Option<[f32; 4]>,
    pub unk9: Option<()>,
    pub unk10: Option<Mat4x4>,
    pub unk11: Option<[f32; 4]>,
    pub unk12: Option<()>,
    pub unk13: Option<()>,
    pub unk14: Option<[f32; 4]>,
    pub unk15: Option<LodUnkStruct>,
    pub unk16: Option<[f32; 4]>,
    pub unk17: Option<[f32; 4]>,
    pub unk18: [f32; 2],
    pub unk19: u16,
    pub skin_ids: Vec<i32>,
    pub anims: Vec<LodAnimEntry>,
    pub sounds: Option<LodSoundData>, // Only exists if transform.subtype == 2
}

impl Lod {
    /// Read a Lod from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> StructUnpackResult<Lod> {
        let transform = unpack_map(
            TransformationHeader::read_from(file, fmt),
            "Lod",
            "transform",
        )?;
        let subtype = transform.item_subtype;
        Ok(Lod {
            transform,
            unk1: unpack_map(load_option(file, fmt, load_f32_4), "Lod", "unk1")?,
            unk2: unpack_map(load_option(file, fmt, load_f32_4), "Lod", "unk2")?,
            unk3: unpack_map(load_option(file, fmt, load_f32_9), "Lod", "unk3")?,
            unk4: unpack_map(load_option(file, fmt, load_transform), "Lod", "unk4")?,
            unk5: unpack_map(load_option(file, fmt, load_f32_4), "Lod", "unk5")?,
            unk6: unpack_map(load_option(file, fmt, load_empty), "Lod", "unk6")?,
            unk7: unpack_map(load_option(file, fmt, load_transform), "Lod", "unk7")?,
            unk8: unpack_map(load_option(file, fmt, load_f32_4), "Lod", "unk8")?,
            unk9: unpack_map(load_option(file, fmt, load_empty), "Lod", "unk9")?,
            unk10: unpack_map(load_option(file, fmt, load_transform), "Lod", "unk10")?,
            unk11: unpack_map(load_option(file, fmt, load_f32_4), "Lod", "unk11")?,
            unk12: unpack_map(load_option(file, fmt, load_empty), "Lod", "unk12")?,
            unk13: unpack_map(load_option(file, fmt, load_empty), "Lod", "unk13")?,
            unk14: unpack_map(load_option(file, fmt, load_f32_4), "Lod", "unk14")?,
            unk15: unpack_map(load_option(file, fmt, load_unkstruct), "Lod", "unk15")?,
            unk16: unpack_map(load_option(file, fmt, load_f32_4), "Lod", "unk16")?,
            unk17: unpack_map(load_option(file, fmt, load_f32_4), "Lod", "unk17")?,
            unk18: [
                unpack_map(fmt.read_f32(file), "Lod", "unk18[0]")?,
                unpack_map(fmt.read_f32(file), "Lod", "unk18[1]")?,
            ],
            unk19: unpack_map(fmt.read_u16(file), "Lod", "unk19")?,
            skin_ids: {
                let num_skins = unpack_map(fmt.read_u32(file), "Lod", "skin_ids.len")?;
                let mut skins = Vec::with_capacity(num_skins.max(SAFE_CAPACITY_SMALL) as usize);
                for i in 0..num_skins {
                    skins.push(unpack_map_index(fmt.read_i32(file), "Lod", "skin_ids", i)?);
                }
                skins
            },
            anims: {
                let num_anims = unpack_map(fmt.read_u32(file), "Lod", "anims.len")?;
                let mut anims = Vec::with_capacity(num_anims.max(SAFE_CAPACITY_SMALL) as usize);
                for i in 0..num_anims {
                    // let animid = unpack_map_index(fmt.read_u32(file), "Lod", "anims", i)?;
                    anims.push(LodAnimEntry {
                        // symbol: unpack_map_index(
                        //     AnimSymbol::from_u32(animid)
                        //         .ok_or(EnumerationError::new("AnimSymbol", animid as i64)),
                        //     "Lod",
                        //     "anims",
                        //     i,
                        // )?,
                        symbol: unpack_map_index(fmt.read_u32(file), "Lod", "anims", i)?,
                        animation_id: unpack_map_index(fmt.read_i32(file), "Lod", "anims", i)?,
                    });
                }
                anims
            },
            sounds: {
                match subtype {
                    0 => None,
                    2 => {
                        let num_sounds = unpack_map(fmt.read_u32(file), "Lod", "sounds.len")?;
                        let mut sounds =
                            Vec::with_capacity(num_sounds.max(SAFE_CAPACITY_SMALL) as usize);
                        for i in 0..num_sounds {
                            sounds.push(LodSoundEntry {
                                symbol: unpack_map_index(fmt.read_u32(file), "Lod", "sounds", i)?,
                                sound_id: unpack_map_index(fmt.read_i32(file), "Lod", "sounds", i)?,
                            });
                        }
                        Some(LodSoundData { data: sounds })
                    }
                    x => {
                        return unpack_map(
                            Err(BadValueError::new(x, Some("[0, 2]"))),
                            "Lod",
                            "sounds",
                        )
                    }
                }
            },
        })
    }
}

pub struct LodUnkStruct {
    pub unk1: [f32; 4],
    pub unk2: u32,
}

pub struct LodAnimEntry {
    // pub symbol: AnimSymbol, Jimmy files differ on animsymbols, so this is disabled for now
    pub symbol: u32,
    pub animation_id: i32,
}

pub struct LodSoundData {
    pub data: Vec<LodSoundEntry>,
}

pub struct LodSoundEntry {
    pub symbol: u32,
    pub sound_id: i32,
}
