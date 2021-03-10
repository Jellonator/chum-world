use crate::common::*;

chum_struct_binary! {
    #[derive(Default)]
    pub struct Light {
        pub header: [struct THeader],
        pub item_type: [ignore [u16] ITEM_TYPE_LIGHT],
        pub item_flags: [u16],
        pub unk1: [fixed array [f32] 4],
        pub unk2: [fixed array [f32] 3],
        pub direction: [Vector3],
        pub unk3: [fixed array [f32] 3],
        pub unk4: [u8],
        pub junk: [ignore [fixed array [u8] 3] [0u8; 3]],
        pub unk5: [fixed array [f32] 3],
    }
}
