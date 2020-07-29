use crate::common::*;
use crate::animsymbol::AnimSymbol;
use crate::format::TotemFormat;
use std::io::{self, Read};

fn load_option<T, F, R>(file: &mut R, fmt: TotemFormat, func: F) -> io::Result<Option<T>>
where
    F: Fn(&mut R, TotemFormat) -> io::Result<T>,
    R: Read
{
    let value = fmt.read_u8(file)?;
    match value {
        0 => Ok(None),
        1 => Ok(Some(func(file, fmt)?)),
        _ => panic!()
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
        unk2: fmt.read_u32(file)?
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
    pub sounds: Option<LodSoundData> // Only exists if transform.subtype == 2
}

impl Lod {
    /// Read a Lod from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<Lod> {
        let transform = TransformationHeader::read_from(file, fmt)?;
        let subtype = transform.item_subtype;
        Ok(Lod {
            transform,
            unk1: load_option(file, fmt, load_f32_4)?,
            unk2: load_option(file, fmt, load_f32_4)?,
            unk3: load_option(file, fmt, load_f32_9)?,
            unk4: load_option(file, fmt, load_transform)?,
            unk5: load_option(file, fmt, load_f32_4)?,
            unk6: load_option(file, fmt, load_empty)?,
            unk7: load_option(file, fmt, load_transform)?,
            unk8: load_option(file, fmt, load_f32_4)?,
            unk9: load_option(file, fmt, load_empty)?,
            unk10: load_option(file, fmt, load_transform)?,
            unk11: load_option(file, fmt, load_f32_4)?,
            unk12: load_option(file, fmt, load_empty)?,
            unk13: load_option(file, fmt, load_empty)?,
            unk14: load_option(file, fmt, load_f32_4)?,
            unk15: load_option(file, fmt, load_unkstruct)?,
            unk16: load_option(file, fmt, load_f32_4)?,
            unk17: load_option(file, fmt, load_f32_4)?,
            unk18: [
                fmt.read_f32(file)?,
                fmt.read_f32(file)?,
            ],
            unk19: fmt.read_u16(file)?,
            skin_ids: {
                let num_skins = fmt.read_u32(file)?;
                let mut skins = Vec::with_capacity(num_skins as usize);
                for _ in 0..num_skins {
                    skins.push(fmt.read_i32(file)?);
                }
                skins
            },
            anims: {
                let num_anims = fmt.read_u32(file)?;
                let mut anims = Vec::with_capacity(num_anims as usize);
                for _ in 0..num_anims {
                    anims.push(LodAnimEntry {
                        symbol: AnimSymbol::from_u32(fmt.read_u32(file)?).unwrap(),
                        animation_id: fmt.read_i32(file)?
                    });
                }
                anims
            },
            sounds: {
                match subtype {
                    0 => None,
                    2 => {
                        let num_sounds = fmt.read_u32(file)?;
                        let mut sounds = Vec::with_capacity(num_sounds as usize);
                        for _ in 0..num_sounds {
                            sounds.push(LodSoundEntry {
                                symbol: fmt.read_u32(file)?,
                                sound_id: fmt.read_i32(file)?
                            });
                        }
                        Some(LodSoundData {
                            data: sounds
                        })
                    }
                    _ => panic!()
                }
            }
        })
    }
}

pub struct LodUnkStruct {
    pub unk1: [f32; 4],
    pub unk2: u32
}

pub struct LodAnimEntry {
    pub symbol: AnimSymbol,
    pub animation_id: i32
}

pub struct LodSoundData {
    pub data: Vec<LodSoundEntry>
}

pub struct LodSoundEntry {
    pub symbol: u32,
    pub sound_id: i32
}