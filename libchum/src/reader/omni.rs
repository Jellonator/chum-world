use crate::common::*;

chum_struct_generate_readwrite! {
    pub struct Omni {
        pub transform: [struct TransformationHeader],
        pub color: [Vector3 rgb],
        pub unknown1: [u8],
        pub junk: [fixed array [u8] 3],
        pub unknown2: [Vector2]
    }
}