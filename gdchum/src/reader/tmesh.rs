use crate::chumfile::ChumFile;
use crate::reader::ChumReader;
use crate::util;
use gdnative::*;
use libchum::common;
use libchum::reader::tmesh;

#[derive(Clone, Debug)]
pub struct TMeshResultSurface {
    pub vertex_ids: Vec<u16>,
    pub texcoord_ids: Vec<u16>,
    pub normal_ids: Vec<u16>,
}

#[derive(Clone)]
pub struct TMeshResult {
    pub mesh: Reference,
    pub surfaces: Vec<TMeshResultSurface>,
    pub transform: common::Mat4x4,
    pub unk1: Vec<tmesh::Footer1>,
    pub unk2: Vec<tmesh::Footer2>,
    pub unk3: Vec<tmesh::Footer3>,
    pub strip_order: Vec<u32>,
}

pub fn read_tmesh(
    data: &Vec<u8>,
    fmt: libchum::format::TotemFormat,
    reader: &mut ChumReader,
    file: &ChumFile,
) -> Option<TMeshResult> {
    let tmesh = match tmesh::TMesh::read_data(data, fmt) {
        Ok(x) => x,
        Err(err) => {
            display_err!("Error loading MESH: {}\n{}", file.get_name_str(), err);
            return None;
        }
    };
    let mut mesh = ArrayMesh::new();
    let generated_tris = tmesh.gen_triangles();
    let mesh_materials = tmesh.get_materials();
    let mut materials = Vec::new();
    let mut surfaces = Vec::new();
    for trivec in generated_tris.into_iter() {
        let mut verts = Vector3Array::new();
        let mut texcoords = Vector2Array::new();
        let mut normals = Vector3Array::new();
        let mut meshdata = VariantArray::new();
        let mut surface = TMeshResultSurface {
            vertex_ids: Vec::new(),
            texcoord_ids: Vec::new(),
            normal_ids: Vec::new(),
        };
        for tri in trivec.tris {
            for point in &tri.points {
                verts.push(&Vector3::new(
                    point.vertex.x,
                    point.vertex.y,
                    point.vertex.z,
                ));
                texcoords.push(&Vector2::new(point.texcoord.x, point.texcoord.y));
                normals.push(&Vector3::new(
                    point.normal.x,
                    point.normal.y,
                    point.normal.z,
                ));
                surface.vertex_ids.push(point.vertex_id);
                surface.texcoord_ids.push(point.texcoord_id);
                surface.normal_ids.push(point.normal_id);
            }
        }
        surfaces.push(surface);
        let mat = mesh_materials[trivec.material_index as usize % mesh_materials.len()];
        materials.push(mat);
        meshdata.resize(ArrayMesh::ARRAY_MAX as i32);
        meshdata.set(ArrayMesh::ARRAY_VERTEX as i32, &Variant::from(&verts));
        meshdata.set(ArrayMesh::ARRAY_NORMAL as i32, &Variant::from(&normals));
        meshdata.set(ArrayMesh::ARRAY_TEX_UV as i32, &Variant::from(&texcoords));
        mesh.add_surface_from_arrays(
            Mesh::PRIMITIVE_TRIANGLES,
            meshdata,
            VariantArray::new(),
            97280,
        );
    }
    let archiveinstance = file.get_archive_instance();
    archiveinstance
        .map(|archive, res| {
            for (i, mat) in materials.iter().enumerate() {
                if let Some(materialfile) = archive.get_file_from_hash(res.new_ref(), *mat) {
                    let materialdict = reader.read_material_nodeless(&materialfile);
                    if materialdict.get(&"exists".into()) == true.into() {
                        let material: Material = materialdict
                            .get(&"material".into())
                            .try_to_object()
                            .unwrap();
                        mesh.surface_set_material(i as i64, Some(material));
                    } else {
                        display_warn!(
                            "Could not apply material {} to mesh {}.",
                            materialfile.script().map(|x| x.get_name_str().to_owned()).unwrap(),
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
    Some(TMeshResult {
        mesh: mesh.to_reference(),
        surfaces,
        transform: tmesh.transform.transform.clone(),
        unk1: tmesh.unk1.clone(),
        unk2: tmesh.unk2.clone(),
        unk3: tmesh.unk3.clone(),
        strip_order: tmesh.strip_order.clone(),
    })
}

pub fn read_tmesh_from_res(data: &ChumFile, reader: &mut ChumReader) -> Dictionary {
    let fmt = data.get_format();
    let mut dict = Dictionary::new();
    match read_tmesh(&data.get_data_as_vec(), fmt, reader, data) {
        Some(mesh) => {
            dict.set(&"exists".into(), &true.into());
            dict.set(&"mesh".into(), &mesh.mesh.to_variant());
            let mut surfaces = VariantArray::new();
            for surface in mesh.surfaces.iter() {
                let mut vertices = VariantArray::new();
                let mut texcoords = VariantArray::new();
                let mut normals = VariantArray::new();
                for index in surface.vertex_ids.iter() {
                    vertices.push(&Variant::from_i64(*index as i64));
                }
                for index in surface.texcoord_ids.iter() {
                    texcoords.push(&Variant::from_i64(*index as i64));
                }
                for index in surface.normal_ids.iter() {
                    normals.push(&Variant::from_i64(*index as i64));
                }
                let mut surfacedict = Dictionary::new();
                surfacedict.set(&"vertices".into(), &Variant::from_array(&vertices));
                surfacedict.set(&"texcoords".into(), &Variant::from_array(&texcoords));
                surfacedict.set(&"normals".into(), &Variant::from_array(&normals));
                surfaces.push(&Variant::from_dictionary(&surfacedict));
            }
            dict.set(&"surfaces".into(), &Variant::from_array(&surfaces));
            dict.set(
                &"transform".into(),
                &util::mat4x4_to_transform(&mesh.transform).to_variant(),
            );
            dict.set(
                &"unk1".into(),
                &mesh
                    .unk1
                    .into_iter()
                    .map(|x| {
                        let mut dict = Dictionary::new();
                        dict.set(&"pos".into(), &util::vec3_to_godot(&x.pos).to_variant());
                        dict.set(&"radius".into(), &x.radius.to_variant());
                        dict.to_variant()
                    })
                    .collect::<Vec<_>>()
                    .to_variant(),
            );
            dict.set(
                &"unk2".into(),
                &mesh
                    .unk2
                    .into_iter()
                    .map(|x| {
                        let mut dict = Dictionary::new();
                        dict.set(
                            &"transform".into(),
                            &util::mat4x4_to_transform(&x.transform).to_variant(),
                        );
                        dict.to_variant()
                    })
                    .collect::<Vec<_>>()
                    .to_variant(),
            );
            dict.set(
                &"unk3".into(),
                &mesh
                    .unk3
                    .into_iter()
                    .map(|x| {
                        let mut dict = Dictionary::new();
                        dict.set(&"unk1".into(), &(&x.unk1[..]).to_owned().to_variant());
                        dict.set(
                            &"normal".into(),
                            &util::vec3_to_godot(&x.normal).to_variant(),
                        );
                        dict.set(&"junk".into(), &x.junk.to_variant());
                        dict.set(&"unk2".into(), &x.unk2.to_variant());
                        dict.to_variant()
                    })
                    .collect::<Vec<_>>()
                    .to_variant(),
            );
        }
        None => {
            godot_print!("read_tmesh returned None");
            dict.set(&"exists".into(), &false.into());
        }
    }
    dict
}
