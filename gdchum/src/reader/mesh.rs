use crate::chumfile::ChumFile;
use crate::reader::ChumReader;
use crate::util;
use gdnative::api::{ArrayMesh, Material, Mesh};
use gdnative::prelude::*;
use libchum::common;
use libchum::reader::mesh;

#[derive(Clone, Debug)]
pub struct MeshResultSurface {
    pub vertex_ids: Vec<u16>,
    pub texcoord_ids: Vec<u16>,
    pub normal_ids: Vec<u16>,
}

// #[derive(Clone)]
pub struct MeshResult {
    pub mesh: Ref<ArrayMesh, Unique>,
    pub surfaces: Vec<MeshResultSurface>,
    pub transform: common::Transform3D,
    pub unk1: Vec<mesh::Footer1>,
    pub unk2: Vec<mesh::Footer2>,
    pub unk3: Vec<mesh::Footer3>,
    pub strip_order: Vec<u32>,
}

pub fn read_mesh(
    data: &Vec<u8>,
    fmt: libchum::format::TotemFormat,
    reader: &mut ChumReader,
    file: &ChumFile,
) -> Option<MeshResult> {
    let mesh = match mesh::Mesh::read_data(data, fmt) {
        Ok(x) => x,
        Err(err) => {
            display_err!("Error loading MESH: {}\n{}", file.get_name_str(), err);
            return None;
        }
    };
    let array_mesh = Ref::<ArrayMesh, Unique>::new();
    let generated_tris = mesh.gen_triangles();
    let mesh_materials = mesh.get_materials();
    let mut materials = Vec::new();
    let mut surfaces = Vec::new();
    for trivec in generated_tris.into_iter() {
        let mut verts = Vector3Array::new();
        let mut texcoords = Vector2Array::new();
        let mut normals = Vector3Array::new();
        let meshdata = VariantArray::new();
        let mut surface = MeshResultSurface {
            vertex_ids: Vec::new(),
            texcoord_ids: Vec::new(),
            normal_ids: Vec::new(),
        };
        for tri in trivec.tris {
            for point in &tri.points {
                verts.push(Vector3::new(point.vertex.x, point.vertex.y, point.vertex.z));
                texcoords.push(Vector2::new(point.texcoord.x, point.texcoord.y));
                normals.push(Vector3::new(point.normal.x, point.normal.y, point.normal.z));
                surface.vertex_ids.push(point.vertex_id);
                surface.texcoord_ids.push(point.texcoord_id);
                surface.normal_ids.push(point.normal_id);
            }
        }
        surfaces.push(surface);
        let mat = mesh_materials[trivec.material_index as usize % mesh_materials.len()];
        materials.push(mat);
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
    }
    let unsafe_archive_instance = file.get_archive_instance();
    let archiveinstance = unsafe { unsafe_archive_instance.assume_safe() };
    archiveinstance
        .map(|archive, res| {
            for (i, mat) in materials.iter().enumerate() {
                if let Some(materialfile) = archive.get_file_from_hash(&res, *mat) {
                    let materialdict = reader.read_material_nodeless(materialfile.clone());
                    if materialdict.get("exists").to_bool() == true {
                        let material: Ref<Material, Shared> =
                            materialdict.get("material").try_to_object().unwrap();
                        array_mesh.surface_set_material(i as i64, material);
                    } else {
                        display_warn!(
                            "Could not apply material {} to mesh {}.",
                            unsafe { materialfile.assume_safe() }
                                .map(|x, _| x.get_name_str().to_owned())
                                .unwrap(),
                            file.get_name_str()
                        );
                    }
                } else {
                    display_warn!(
                        "No such material with ID {} to apply to mesh {}.",
                        mat,
                        file.get_name_str()
                    );
                }
            }
        })
        .unwrap();
    Some(MeshResult {
        mesh: array_mesh,
        surfaces,
        transform: mesh.transform.transform.clone(),
        unk1: mesh.unk1.clone(),
        unk2: mesh.unk2.clone(),
        unk3: mesh.unk3.clone(),
        strip_order: mesh.strip_order.clone(),
    })
}

pub fn read_mesh_from_res(data: &ChumFile, reader: &mut ChumReader) -> Dictionary<Unique> {
    let fmt = data.get_format();
    let dict = Dictionary::new();
    match read_mesh(&data.get_data_as_vec(), fmt, reader, data) {
        Some(mesh) => {
            dict.insert("exists", true);
            dict.insert("mesh", mesh.mesh);
            let surfaces = VariantArray::new();
            for surface in mesh.surfaces.iter() {
                let vertices = VariantArray::new();
                let texcoords = VariantArray::new();
                let normals = VariantArray::new();
                for index in surface.vertex_ids.iter() {
                    vertices.push(&Variant::from_i64(*index as i64));
                }
                for index in surface.texcoord_ids.iter() {
                    texcoords.push(&Variant::from_i64(*index as i64));
                }
                for index in surface.normal_ids.iter() {
                    normals.push(&Variant::from_i64(*index as i64));
                }
                let surfacedict = Dictionary::new();
                surfacedict.insert("vertices", vertices);
                surfacedict.insert("texcoords", texcoords);
                surfacedict.insert("normals", normals);
                surfaces.push(surfacedict);
            }
            dict.insert("surfaces", surfaces);
            dict.insert("transform", util::transform3d_to_godot(&mesh.transform));
            dict.insert(
                "unk1",
                mesh.unk1
                    .into_iter()
                    .map(|x| {
                        let dict = Dictionary::new();
                        dict.insert("pos", x.pos);
                        dict.insert("radius", x.radius);
                        dict.into_shared()
                    })
                    .collect::<Vec<_>>(),
            );
            dict.insert(
                "unk2",
                mesh.unk2
                    .into_iter()
                    .map(|x| {
                        let dict = Dictionary::new();
                        dict.insert("transform", util::transform3d_to_godot(&x.transform));
                        dict.into_shared()
                    })
                    .collect::<Vec<_>>(),
            );
            dict.insert(
                "unk3",
                &mesh
                    .unk3
                    .into_iter()
                    .map(|x| {
                        let dict = Dictionary::new();
                        dict.insert("unk1", &(&x.unk1[..]).to_owned());
                        dict.insert("normal", x.normal);
                        dict.insert("junk", x.junk);
                        dict.insert("unk2", x.unk2);
                        dict.into_shared()
                    })
                    .collect::<Vec<_>>(),
            );
        }
        None => {
            godot_print!("read_mesh returned None");
            dict.insert("exists", false);
        }
    }
    dict
}
