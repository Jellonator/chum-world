use crate::util;
use gdnative::api::Resource;
use gdnative::api::{ArrayMesh, Mesh};
use gdnative::prelude::*;
use libchum::common;
use libchum::reader::rotshape::*;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct RotShapeView {
    pub inner: RotShape,
}

#[methods]
impl RotShapeView {
    fn new(_owner: &Resource) -> Self {
        RotShapeView {
            inner: Default::default(),
        }
    }

    impl_view_node_resource!(
        RotShapeView,
        RotShape,
        "ROTSHAPE",
        |_builder: &ClassBuilder<Self>| {}
    );

    #[export]
    pub fn get_structure(&self, _owner: &Resource) -> Variant {
        use libchum::structure::ChumStruct;
        let data = self.inner.structure();
        util::struct_to_dict(&data).into_shared().to_variant()
    }

    #[export]
    pub fn import_structure(&mut self, owner: &Resource, data: Dictionary) {
        use libchum::structure::ChumStruct;
        let structure = util::dict_to_struct(&data);
        self.inner = RotShape::destructure(&structure).unwrap();
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_materialanim(&self, _owner: &Resource) -> i32 {
        self.inner.materialanim_id
    }

    /// Generates an ArrayMesh based on the contents of the rotshape in this view.
    /// The material are not automatically applied, and must be applied manually.
    #[export]
    pub fn generate_array_mesh(&self, _owner: &Resource) -> Ref<ArrayMesh, Shared> {
        use libchum::structure::ChumEnum;
        let pos_tl = self.inner.size[0];
        let pos_br = self.inner.size[1];
        let pos_bl = common::Vector3::new(pos_tl.x, pos_br.y, pos_br.z);
        let pos_tr = common::Vector3::new(pos_br.x, pos_tl.y, pos_tl.z);
        let uv2 = Vector2::new((self.inner.billboard_mode.to_u32() + 2) as f32, 0.0);
        let mesh = Ref::<ArrayMesh, Unique>::new();
        let mut verts = Vector3Array::new();
        verts.push(pos_tl + self.inner.offset);
        verts.push(pos_tr + self.inner.offset);
        verts.push(pos_br + self.inner.offset);
        verts.push(pos_bl + self.inner.offset);
        let mut texcoords = Vector2Array::new();
        texcoords.push(self.inner.texcoords[0]);
        texcoords.push(self.inner.texcoords[1]);
        texcoords.push(self.inner.texcoords[2]);
        texcoords.push(self.inner.texcoords[3]);
        let mut uv2coords = Vector2Array::new();
        uv2coords.push(uv2);
        uv2coords.push(uv2);
        uv2coords.push(uv2);
        uv2coords.push(uv2);
        let mut normals = Vector3Array::new();
        normals.push(Vector3::new(0.0, 0.0, 1.0));
        normals.push(Vector3::new(0.0, 0.0, 1.0));
        normals.push(Vector3::new(0.0, 0.0, 1.0));
        normals.push(Vector3::new(0.0, 0.0, 1.0));
        let meshdata = VariantArray::new();
        meshdata.resize(ArrayMesh::ARRAY_MAX as i32);
        meshdata.set(ArrayMesh::ARRAY_VERTEX as i32, verts);
        meshdata.set(ArrayMesh::ARRAY_NORMAL as i32, normals);
        meshdata.set(ArrayMesh::ARRAY_TEX_UV as i32, texcoords);
        meshdata.set(ArrayMesh::ARRAY_TEX_UV2 as i32, uv2coords);
        mesh.add_surface_from_arrays(
            Mesh::PRIMITIVE_TRIANGLE_FAN,
            meshdata.into_shared(),
            VariantArray::new().into_shared(),
            97280,
        );
        mesh.into_shared()
    }
}
