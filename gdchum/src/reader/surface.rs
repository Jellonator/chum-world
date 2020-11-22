use crate::chumfile::ChumFile;
use crate::reader::ChumReader;
use crate::util;
use gdnative::prelude::*;
use gdnative::api::{ArrayMesh, Mesh, Material};
use libchum::common;
use libchum::reader::surface;

pub struct SurfaceResult {
    // pub surface: Reference,
    pub transform: common::Transform3D,
    pub surfaces: Vec<Ref<ArrayMesh,Shared>>,
}

pub fn read_surface(
    data: &Vec<u8>,
    fmt: libchum::format::TotemFormat,
    reader: &mut ChumReader,
    file: &ChumFile,
) -> Option<SurfaceResult> {
    let surfaceobj = match surface::SurfaceObject::read_data(data, fmt) {
        Ok(x) => x,
        Err(err) => {
            display_err!("Error loading SURFACE: {}\n{}", file.get_name_str(), err);
            return None;
        }
    };
    let mut meshes: Vec<Ref<ArrayMesh,Shared>> = Vec::new();
    let surfaces = surfaceobj.generate_meshes(surface::SurfaceGenMode::BezierInterp(4));
    for surface in surfaces {
        let mesh = Ref::<ArrayMesh,Unique>::new();
        let mut verts = Vector3Array::new();
        let mut texcoords = Vector2Array::new();
        let mut uv2 = Vector2Array::new();
        let mut normals = Vector3Array::new();
        let meshdata = VariantArray::new();
        for quad in surface.quads.iter() {
            for tri in &quad.tris() {
                for point in &tri.points {
                    verts.push(Vector3::new(
                        point.vertex.x,
                        point.vertex.y,
                        point.vertex.z,
                    ));
                    texcoords.push(Vector2::new(point.texcoord.x, point.texcoord.y));
                    normals.push(Vector3::new(
                        point.normal.x,
                        point.normal.y,
                        point.normal.z,
                    ));
                    uv2.push(Vector2::new(
                        point.uv2.x,
                        point.uv2.y
                    ));
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
        let mesh = mesh.into_shared();
        meshes.push(mesh.clone());
        let unsafe_archive_instance = file.get_archive_instance();
        let archiveinstance = unsafe { unsafe_archive_instance.assume_safe() };
        archiveinstance
            .map(|archive, res| {
                if let Some(materialfile) =
                    archive.get_file_from_hash(&res, surface.material_index)
                {
                    let materialdict = reader.read_materialanim_nodeless(materialfile.clone());
                    if materialdict.get("exists").to_bool() == true {
                        let material: Ref<Material, Shared> = materialdict
                            .get("material")
                            .try_to_object()
                            .unwrap();
                        unsafe {mesh.assume_safe()}.surface_set_material(0, material);
                    } else {
                        display_warn!(
                            "Could not apply materialanim {} to surface {}.",
                            unsafe { materialfile.assume_safe() }.map(|x,_| x.get_name_str().to_owned()).unwrap(),
                            file.get_name_str()
                        );
                    }
                } else {
                    display_warn!(
                        "No such materialanim with ID {} to apply to surface {}.",
                        surface.material_index,
                        file.get_name_str()
                    );
                }
            })
            .unwrap();
    }
    Some(SurfaceResult {
        surfaces: meshes,
        transform: surfaceobj.transform.transform.clone(),
    })
}

pub fn read_surface_from_res(data: &ChumFile, reader: &mut ChumReader) -> Dictionary<Unique> {
    let fmt = data.get_format();
    let dict = Dictionary::new();
    match read_surface(&data.get_data_as_vec(), fmt, reader, data) {
        Some(mesh) => {
            dict.insert("exists", true);
            dict.insert("surfaces", mesh.surfaces);
            dict.insert(
                "transform",
                util::transform3d_to_godot(&mesh.transform),
            );
        }
        None => {
            godot_print!("read_tmesh returned None");
            dict.insert("exists", false);
        }
    }
    dict
}
