use crate::common::*;

chum_struct_binary! {
    pub struct Omni {
        pub transform: [struct THeaderNoType],
        pub item_type: [ignore [u16] 16u16],
        pub item_subtype: [ignore [u16] 0u16],
        pub color: [Vector3 rgb],
        pub unknown1: [u8],
        pub junk: [fixed array [u8] 3],
        pub unknown2: [Vector2]
    }
}
