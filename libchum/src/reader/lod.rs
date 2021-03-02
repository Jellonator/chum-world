// use crate::binary::ChumBinary;
use crate::common::*;
// use crate::format::TotemFormat;
// use crate::util::error;
use crate::error::*;
use std::io;

chum_struct_generate_readwrite! {
    pub struct Lod {
        pub transform: [struct THeaderNoType],
        pub item_type: [ignore [u16] 5u16],
        pub item_subtype: [custom_structure [u16]
            // Do not present the subtype
            structure: |_lod: &Lod| {
                None
            };
            // The value of `item_subtype` depends on the presence of the `sounds` value
            destructure: |data: &crate::structure::ChumStructVariant| {
                if let Some(_) = data.get_struct_item("sounds").unwrap().get_optional_value().unwrap() {
                    2
                } else {
                    0
                }
            };
        ],
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
        pub sounds: [custom_binary
            [option [struct LodSoundData] LodSoundData::default()]
            // Only read `sounds` if the subtype is 2
            read: |lod: &Inner, file, fmt| -> StructUnpackResult<Option<LodSoundData>> {
                match lod.item_subtype.unwrap() {
                    2 => match LodSoundData::read_from(file, fmt) {
                        Ok(value) => Ok(Some(value)),
                        Err(e) => Err(e.structuralize("Lod", "sounds"))
                    },
                    0 => Ok(None),
                    o => Err(StructUnpackError {
                        structname: "Lod".to_owned(),
                        structpath: "sounds".to_owned(),
                        error: UnpackError::InvalidEnumeration {
                            enum_name: "item_subtype".to_owned(),
                            value: o as i64
                        }
                    })
                }
            };
            // Only write to file if not None
            write: |value: &Option<LodSoundData>, file, fmt| -> io::Result<()> {
                if let Some(ref inner) = value {
                    inner.write_to(file, fmt)
                } else {
                    Ok(())
                }
            };
        ]
    }
}

chum_struct_generate_readwrite! {
    pub struct LodTransform {
        pub transform: [Transform3D],
        pub junk: [ignore [fixed array [u8] 16] [0u8; 16]]
    }
}

impl Default for LodTransform {
    fn default() -> Self {
        LodTransform {
            transform: Transform3D::default(),
            junk: (),
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
            animation_id: 0,
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
        LodSoundData { data: Vec::new() }
    }
}

chum_struct_generate_readwrite! {
    pub struct LodSoundEntry {
        pub symbol: [u32],
        pub sound_id: [reference SOUND],
    }
}
