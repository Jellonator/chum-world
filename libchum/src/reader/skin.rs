use crate::common::*;
// use std::error::Error;
use crate::scene;
use std::collections::HashMap;

chum_binary! {
    #[derive(Clone, Debug)]
    pub struct Skin {
        pub header: [struct THeader],
        pub item_type: [ignore [u16] ITEM_TYPE_SKIN],
        pub item_flags: [u16],
        pub meshes: [dynamic array [u32] [reference MESH] 0i32],
        pub unk_zero: [ignore [u32] 0u32],
        pub vertex_groups: [dynamic array [u32] [struct VertexGroup] VertexGroup::default()],
        pub anims: [option [struct AnimSection] AnimSection::default()],
        pub unknown: [dynamic array [u32] [struct UnknownEntry] UnknownEntry::default()],
    }
}

chum_binary! {
    #[derive(Clone, Debug, Default)]
    pub struct VertexGroup {
        pub group_id: [reference],
        pub sections: [dynamic array [u32] [struct VertexGroupSection] VertexGroupSection::default()],
    }
}

chum_binary! {
    #[derive(Clone, Debug, Default)]
    pub struct VertexGroupSection {
        pub mesh_index: [u16],
        pub vertices: [dynamic array [u32] [struct VertexGroupVertex] VertexGroupVertex::default()],
        pub normals: [dynamic array [u32] [struct VertexGroupNormal] VertexGroupNormal::default()],
    }
}

chum_binary! {
    #[derive(Clone, Debug, Default)]
    pub struct VertexGroupVertex {
        pub vertex_id: [u32],
        pub weight: [f32],
    }
}

chum_binary! {
    #[derive(Clone, Debug, Default)]
    pub struct VertexGroupNormal {
        pub normal_id: [u32],
        pub weight: [f32],
    }
}

chum_binary! {
    #[derive(Clone, Debug, Default)]
    pub struct AnimSection {
        pub entries: [dynamic array [u32] [struct AnimEntry] AnimEntry::default()],
    }
}

chum_binary! {
    #[derive(Clone, Debug, Default)]
    pub struct AnimEntry {
        pub symbol: [u32],
        pub anim_id: [reference ANIMATION],
    }
}

chum_binary! {
    #[derive(Clone, Debug, Default)]
    pub struct UnknownEntry {
        pub vertices: [dynamic array [u32] [u32] 0u32],
        pub normals: [dynamic array [u32] [u32] 0u32],
    }
}

/// specialized stucture used for skin export
#[derive(Clone, Copy)]
pub struct SkinInfo<'a, 'b> {
    pub names: &'a HashMap<i32, String>,
    pub skin: &'b Skin,
    pub skin_id: i32,
    pub mesh_id: i32,
}

impl Skin {
    pub fn generate_scene_skin_joints(
        &self,
        names: &HashMap<i32, String>,
    ) -> Vec<scene::SkinJoint> {
        self.vertex_groups
            .iter()
            .map(|x| scene::SkinJoint {
                transform: Transform3D::identity(),
                name: names
                    .get(&x.group_id)
                    .map(|x| x.to_string())
                    .unwrap_or(format!("{}", x.group_id)),
            })
            .collect()
    }
}
