use crate::common::*;

chum_struct_generate_readwrite! {
    pub struct CollisionVol {
        pub header: [struct THeaderNoType],
        pub item_type: [ignore [u16] 14u16],
        pub item_subtype: [ignore [u16] 16u16],
        pub unk1: [u32],
        pub local_transform: [Mat4x4],
        pub local_transform_inv: [custom_structure [Mat4x4]
            // Inverse is calculated based on transform
            structure: |_lod: &CollisionVol| {
                None
            };
            // The value of `item_subtype` depends on the presence of the `sounds` value
            destructure: |data: &crate::structure::ChumStructVariant| {
                let tx: &Mat4x4 = data.get_struct_item("local_transform").unwrap().get_transform3d().unwrap();
                tx.try_inverse().unwrap()
            };
        ],
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
