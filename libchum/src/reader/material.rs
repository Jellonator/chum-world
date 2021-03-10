//! See https://github.com/Jellonator/chum-world/wiki/MATERIAL for more information

// Material data
chum_struct_binary! {
    #[derive(Default, Clone)]
    pub struct Material {
        pub color: [Color],
        pub emission: [Vector3 rgb],
        pub unk2: [f32],
        pub transform: [Transform2D],
        pub rotation: [f32],
        pub offset: [Vector2],
        pub scale: [Vector2],
        pub unk4: [fixed array [u8] 13],
        pub texture: [reference BITMAP],
        pub texture_reflection: [reference BITMAP],
    }
}

impl Material {
    /// Get the ID for this material's texture
    pub fn get_texture(&self) -> i32 {
        self.texture
    }

    /// Get the ID for this material's reflection. Might be 0.
    pub fn get_texture_reflection(&self) -> i32 {
        self.texture_reflection
    }
}
