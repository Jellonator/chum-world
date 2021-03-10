use crate::common::*;

chum_struct_binary! {
    #[derive(Default)]
    pub struct CollisionVol {
        pub header: [struct THeaderTyped],
        pub unk1: [u32],
        pub local_transform: [Transform3D],
        pub local_transform_inv: [custom_structure [Transform3D]
            // Inverse is calculated based on transform
            structure: |_lod: &CollisionVol| {
                None
            };
            // The value of `item_subtype` depends on the presence of the `sounds` value
            destructure: |data: &crate::structure::ChumStructVariant| {
                let tx: &Transform3D = data.get_struct_item("local_transform").unwrap().get_transform3d().unwrap();
                tx.inverse().unwrap()
            };
        ],
        pub unk2: [u32],
        pub unk3: [u32],
        pub node_ids: [fixed array [reference NODE] 10],
        pub unk4: [fixed array [f32] 10],
        pub unk5: [dynamic array [u32] [reference] 0],
        pub bitmaps: [dynamic array [u32] [reference BITMAP] 0],
        pub volume_type: [reference],
        pub unk6: [u32],
    }
}
