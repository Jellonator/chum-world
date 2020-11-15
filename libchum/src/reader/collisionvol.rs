use crate::common::*;

chum_struct_generate_readwrite! {
    pub struct CollisionVol {
        pub transform: [struct TransformationHeader],
        pub unk1: [u32],
        pub local_transform: [Mat4x4],
        pub local_transform_inv: [Mat4x4],
        pub unk2: [u32],
        pub unk3: [u32],
        pub node_ids: [fixed array [i32] 10],
        pub unk4: [fixed array [f32] 10],
        pub unk5: [dynamic array [u32] [i32] 0],
        pub bitmaps: [dynamic array [u32] [reference BITMAP] 0],
        pub volume_type: [i32],
        pub unk6: [u32],
    }
}
