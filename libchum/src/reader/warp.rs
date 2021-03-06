chum_struct_binary! {
    #[derive(Clone, Default)]
    pub struct Warp {
        pub size: [f32],
        pub material_ids: [fixed array [reference MATERIAL] 6],
        pub vertices: [fixed array [Vector3] 8],
        pub texcoords: [fixed array [Vector2] 4],
    }
}
