use crate::common::*;

chum_struct_binary! {
    pub struct Omni {
        pub header: [struct THeader],
        pub item_type: [ignore [u16] ITEM_TYPE_OMNI],
        pub item_flags: [u16],
        pub color: [Vector3 rgb],
        pub unknown1: [u8],
        pub junk: [fixed array [u8] 3],
        pub unknown2: [Vector2]
    }
}
