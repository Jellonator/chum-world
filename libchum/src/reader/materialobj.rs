chum_struct_binary! {
    #[derive(Clone, Default)]
    pub struct MaterialObj {
        pub material_anims: [dynamic array [u32] [reference MATERIALANIM] 0i32]
    }
}
