use crate::common::*;

chum_struct_generate_readwrite! {
    pub struct Light {
        pub header: [struct THeaderNoType],
        pub item_type: [ignore [u16] 12u16],
        pub item_subtype: [ignore [u16] 48u16],
        pub unk1: [fixed array [f32] 4],
        pub unk2: [fixed array [f32] 3],
        pub direction: [Vector3],
        pub unk3: [fixed array [f32] 3],
        pub unk4: [u8],
        pub junk: [ignore [fixed array [u8] 3] [0u8; 3]],
        pub unk5: [fixed array [f32] 3],
    }
}
