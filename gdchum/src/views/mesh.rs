use crate::util;
use gdnative::api::Resource;
use gdnative::api::{ArrayMesh, Mesh};
use gdnative::prelude::*;
use libchum::reader::mesh;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct MeshView {
    pub inner: mesh::Mesh,
}

#[methods]
impl MeshView {
    fn new(_owner: &Resource) -> Self {
        MeshView {
            inner: Default::default(),
        }
    }

    impl_view_node_resource!(MeshView, mesh::Mesh, "MESH", |_builder: &ClassBuilder<
        Self,
    >| {});

    #[export]
    pub fn get_structure(&self, _owner: &Resource) -> Variant {
        use libchum::structure::ChumStruct;
        let data = self.inner.get_struct().structure();
        util::struct_to_dict(&data).into_shared().to_variant()
    }

    #[export]
    pub fn import_structure(&mut self, owner: &Resource, data: Dictionary) {
        use libchum::structure::ChumStruct;
        let structure = util::dict_to_struct(&data);
        self.inner
            .import_struct(mesh::MeshStruct::destructure(&structure).unwrap());
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_sphere_shapes(&self, _owner: &Resource) -> VariantArray {
        let shapes = VariantArray::new();
        for shape in self.inner.sphere_shapes.iter() {
            let dict = Dictionary::new();
            dict.insert("center", shape.position);
            dict.insert("radius", shape.radius);
            shapes.push(dict);
        }
        shapes.into_shared()
    }

    #[export]
    pub fn get_cuboid_shapes(&self, _owner: &Resource) -> VariantArray {
        let shapes = VariantArray::new();
        for shape in self.inner.cuboid_shapes.iter() {
            let dict = Dictionary::new();
            dict.insert("transform", util::transform3d_to_godot(&shape.transform));
            shapes.push(dict);
        }
        shapes.into_shared()
    }

    #[export]
    pub fn get_cylinder_shapes(&self, _owner: &Resource) -> VariantArray {
        let shapes = VariantArray::new();
        for shape in self.inner.cylinder_shapes.iter() {
            let dict = Dictionary::new();
            dict.insert("position", shape.position);
            dict.insert("height", shape.height);
            dict.insert("normal", shape.normal);
            dict.insert("radius", shape.radius);
            shapes.push(dict);
        }
        shapes.into_shared()
    }

    /// Generates an ArrayMesh based on the contents of the mesh in this view.
    /// Materials are not automatically applied, and must be applied manually.
    /// The returned dictionary has the following format:
    /// {
    ///     "mesh": ArrayMesh # The actual generated mesh
    ///     "surfaces": [{ # A list of surface information to use for skinning.
    ///                    # There are as many elements in this array as 'mesh'
    ///                    # has surfaces.
    ///         "vertices":  [int], # Index for each vertex contained in the surface
    ///         "texcoords": [int], # Index for each texcoord contained in the surface
    ///         "normals":   [int], # Index for each normal contained in the surface
    ///         "material":  int    # The material to apply to this surface
    ///     }],
    #[export]
    pub fn generate_array_mesh(&self, _owner: &Resource) -> Option<Dictionary<Shared>> {
        let array_mesh = Ref::<ArrayMesh, Unique>::new();
        let generated_tris = self.inner.gen_triangles();
        let mesh_materials = self.inner.get_materials();
        let surfaces = VariantArray::new();
        for trivec in generated_tris.into_iter() {
            let mut verts = Vector3Array::new();
            let mut texcoords = Vector2Array::new();
            let mut normals = Vector3Array::new();
            let meshdata = VariantArray::new();
            let surf_vertices = VariantArray::new();
            let surf_texcoords = VariantArray::new();
            let surf_normals = VariantArray::new();
            for tri in trivec.tris {
                for point in &tri.points {
                    verts.push(Vector3::new(point.vertex.x, point.vertex.y, point.vertex.z));
                    texcoords.push(Vector2::new(point.texcoord.x, point.texcoord.y));
                    normals.push(Vector3::new(point.normal.x, point.normal.y, point.normal.z));
                    surf_vertices.push(point.vertex_id);
                    surf_texcoords.push(point.texcoord_id);
                    surf_normals.push(point.normal_id);
                }
            }
            let mat = mesh_materials[trivec.material_index as usize % mesh_materials.len()];
            meshdata.resize(ArrayMesh::ARRAY_MAX as i32);
            meshdata.set(ArrayMesh::ARRAY_VERTEX as i32, verts);
            meshdata.set(ArrayMesh::ARRAY_NORMAL as i32, normals);
            meshdata.set(ArrayMesh::ARRAY_TEX_UV as i32, texcoords);
            array_mesh.add_surface_from_arrays(
                Mesh::PRIMITIVE_TRIANGLES,
                meshdata.into_shared(),
                VariantArray::new().into_shared(),
                97280,
            );
            let surfacedict = Dictionary::new();
            surfacedict.insert("vertices", surf_vertices);
            surfacedict.insert("texcoords", surf_texcoords);
            surfacedict.insert("normals", surf_normals);
            surfacedict.insert("material", mat);
            surfaces.push(surfacedict);
        }
        let dict = Dictionary::new();
        dict.insert("surfaces", surfaces);
        dict.insert("mesh", array_mesh);
        Some(dict.into_shared())
    }
}
