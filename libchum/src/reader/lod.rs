// use crate::animsymbol::AnimSymbol;
use crate::common::*;

chum_struct_generate_readwrite! {
    pub struct Lod {
        pub transform: [struct THeaderNoType],
        pub item_type: [ignore [u16] 5u16],
        pub item_subtype: [binary_ignore [u16]
            write => |this: &Lod| {
                match this.sounds {
                    Some(_) => 2u16,
                    None => 0u16
                }
            }
        ],
        // no subtype, it will be handled by `sounds`
        pub unk1: [option [fixed array [f32] 4] [0f32; 4]],
        pub unk2: [option [fixed array [f32] 4] [0f32; 4]],
        pub unk3: [option [fixed array [f32] 9] [0f32; 9]],
        pub unk4: [option [struct LodTransform] LodTransform::default()],
        pub unk5: [option [fixed array [f32] 4] [0f32; 4]],
        pub unk6: [ignore [u8] 0u8],
        pub unk7: [option [struct LodTransform] LodTransform::default()],
        pub unk8: [option [fixed array [f32] 4] [0f32; 4]],
        pub unk9: [ignore [u8] 0u8],
        pub unk10: [option [struct LodTransform] LodTransform::default()],
        pub unk11: [option [fixed array [f32] 4] [0f32; 4]],
        pub unk12: [ignore [u8] 0u8],
        pub unk13: [ignore [u8] 0u8],
        pub unk14: [option [fixed array [f32] 4] [0f32; 4]],
        pub unk15: [option [struct LodUnkStruct] LodUnkStruct::default()],
        pub unk16: [option [fixed array [f32] 4] [0f32; 4]],
        pub unk17: [option [fixed array [f32] 4] [0f32; 4]],
        pub unk18: [fixed array [f32] 2],
        pub unk19: [u16],
        pub skin_ids: [dynamic array [u32] [reference SKIN] 0i32],
        pub anims: [dynamic array [u32] [struct LodAnimEntry] LodAnimEntry::default()],
        pub sounds: [option [struct LodSoundData] LodSoundData::default()]
    }
}

// pub struct Lod {
//     pub transform: THeaderTyped,
//     pub unk1: Option<[f32; 4]>,
//     pub unk2: Option<[f32; 4]>,
//     pub unk3: Option<[f32; 9]>,
//     pub unk4: Option<Mat4x4>,
//     pub unk5: Option<[f32; 4]>,
//     pub unk6: Option<()>,
//     pub unk7: Option<Mat4x4>,
//     pub unk8: Option<[f32; 4]>,
//     pub unk9: Option<()>,
//     pub unk10: Option<Mat4x4>,
//     pub unk11: Option<[f32; 4]>,
//     pub unk12: Option<()>,
//     pub unk13: Option<()>,
//     pub unk14: Option<[f32; 4]>,
//     pub unk15: Option<LodUnkStruct>,
//     pub unk16: Option<[f32; 4]>,
//     pub unk17: Option<[f32; 4]>,
//     pub unk18: [f32; 2],
//     pub unk19: u16,
//     pub skin_ids: Vec<i32>,
//     pub anims: Vec<LodAnimEntry>,
//     pub sounds: Option<LodSoundData>, // Only exists if transform.subtype == 2
// }

/*
impl Lod {
    /// Read a Lod from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> StructUnpackResult<Lod> {
        use crate::structure::ChumBinary;
        let transform = unpack_map(
            THeaderTyped::read_from(file, fmt),
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
*/

chum_struct_generate_readwrite! {
    pub struct LodTransform {
        pub transform: [Mat4x4],
        pub junk: [ignore [fixed array [u8] 16] [0u8; 16]]
    }
}

impl Default for LodTransform {
    fn default() -> Self {
        LodTransform {
            transform: Mat4x4::default(),
            junk: ()
        }
    }
}

chum_struct_generate_readwrite! {
    pub struct LodUnkStruct {
        pub unk1: [fixed array[f32] 4],
        pub unk2: [u32],
    }
}

impl Default for LodUnkStruct {
    fn default() -> Self {
        LodUnkStruct {
            unk1: [0f32; 4],
            unk2: 0u32,
        }
    }
}

chum_struct_generate_readwrite! {
    pub struct LodAnimEntry {
        // pub symbol: AnimSymbol, Jimmy files differ on animsymbols, so this is disabled for now
        pub symbol: [u32],
        pub animation_id: [reference ANIMATION],
    }
}

impl Default for LodAnimEntry {
    fn default() -> Self {
        LodAnimEntry {
            symbol: 0,
            animation_id: 0
        }
    }
}

chum_struct_generate_readwrite! {
    pub struct LodSoundData {
        pub data: [
            dynamic array [u32] [struct LodSoundEntry]
            LodSoundEntry {symbol: 0u32, sound_id: 0i32}
        ],
    }
}

impl Default for LodSoundData {
    fn default() -> Self {
        LodSoundData {
            data: Vec::new()
        }
    }
}

chum_struct_generate_readwrite! {
    pub struct LodSoundEntry {
        pub symbol: [u32],
        pub sound_id: [reference SOUND],
    }
}
