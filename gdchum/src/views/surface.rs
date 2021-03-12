use libchum::reader::surface::*;
use gdnative::prelude::*;
use gdnative::api::Resource;
use gdnative::api::{ArrayMesh, Mesh};
use crate::util;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct SurfaceView {
    pub inner: SurfaceObject,
}

#[methods]
impl SurfaceView {
    fn new(_owner: &Resource) -> Self {
        SurfaceView { inner: Default::default() }
    }

    impl_view_node_resource!(SurfaceView, SurfaceObject, "SURFACE",
        |_builder: &ClassBuilder<Self>| {},
        |_self, _owner, _data| {}
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
    pub fn get_num_surfaces(&self, _owner: &Resource) ->  usize {
        self.inner.surfaces.len()
    }

    #[export]
    pub fn generate_surface(&self, _owner: &Resource, index: usize, quality: usize) -> Option<Dictionary> {
        if index < self.inner.surfaces.len() {
            return None;
        }
        let surface = self.inner.generate_mesh(SurfaceGenMode::BezierInterp(quality), index);
        let mesh = Ref::<ArrayMesh, Unique>::new();
        let mut verts = Vector3Array::new();
        let mut texcoords = Vector2Array::new();
        let mut uv2 = Vector2Array::new();
        let mut normals = Vector3Array::new();
        let meshdata = VariantArray::new();
        for quad in surface.quads.iter() {
            for tri in &quad.tris() {
                for point in &tri.points {
                    verts.push(Vector3::new(point.vertex.x, point.vertex.y, point.vertex.z));
                    texcoords.push(Vector2::new(point.texcoord.x, point.texcoord.y));
                    normals.push(Vector3::new(point.normal.x, point.normal.y, point.normal.z));
                    uv2.push(Vector2::new(point.uv2.x, point.uv2.y));
                }
            }
        }
        meshdata.resize(ArrayMesh::ARRAY_MAX as i32);
        meshdata.set(ArrayMesh::ARRAY_VERTEX as i32, verts);
        meshdata.set(ArrayMesh::ARRAY_NORMAL as i32, normals);
        meshdata.set(ArrayMesh::ARRAY_TEX_UV as i32, texcoords);
        meshdata.set(ArrayMesh::ARRAY_TEX_UV2 as i32, uv2);
        mesh.add_surface_from_arrays(
            Mesh::PRIMITIVE_TRIANGLES,
            meshdata.into_shared(),
            VariantArray::new().into_shared(),
            97280,
        );
        let dict = Dictionary::new();
        dict.insert("mesh", mesh);
        dict.insert("material", surface.material_index);
        Some(dict.into_shared())
    }
}
