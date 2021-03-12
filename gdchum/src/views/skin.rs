use libchum::reader::skin::*;
use gdnative::prelude::*;
use gdnative::api::Resource;
use crate::util;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct SkinView {
    pub inner: Skin,
}

#[methods]
impl SkinView {
    fn new(_owner: &Resource) -> Self {
        SkinView { inner: Default::default() }
    }

    impl_view_node_resource!(SkinView, Skin, "SKIN",
        |_builder: &ClassBuilder<Self>| {
        }
    );

    #[export]
    pub fn get_structure(&self, _owner: &Resource) -> Variant {
        return Variant::new();
    }

    #[export]
    pub fn import_structure(&mut self, _owner: &Resource, _data: Dictionary) {
        // do nothing
    }

    #[export]
    pub fn get_resources(&self, _owner: &Resource) -> VariantArray {
        let arr = VariantArray::new();
        for meshid in self.inner.meshes.iter() {
            arr.push(*meshid);
        }
        arr.into_shared()
    }

    /// Get a list of vertex groups and their weights.
    /// Returns a dictionary with the following format:
    /// {
    ///     [group_id]: {
    ///         [mesh_id]: {
    ///             "vertices": {
    ///                 [vertex_index]: float (weight)
    ///             },
    ///             "normals": {
    ///                 [normal_index]: float (weight)
    ///             }
    ///         }
    ///     }
    /// }
    #[export]
    pub fn get_groups(&self, _owner: &Resource) -> Dictionary {
        let groups = Dictionary::new();
        for group in self.inner.vertex_groups.iter() {
            let group_dict = Dictionary::new();
            for section in group.sections.iter() {
                let mesh_dict = Dictionary::new();
                let vertices = Dictionary::new();
                let normals = Dictionary::new();
                for vert in section.vertices.iter() {
                    vertices.insert(vert.vertex_id, vert.weight);
                }
                for norm in section.normals.iter() {
                    normals.insert(norm.normal_id, norm.weight);
                }
                mesh_dict.insert("vertices", vertices);
                mesh_dict.insert("normals", normals);
                group_dict.insert(section.mesh_index, mesh_dict);
            }
            groups.insert(group.group_id, group_dict);
        }
        groups.into_shared()
    }
}
