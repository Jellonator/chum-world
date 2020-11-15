use crate::chumfile::ChumFile;
use crate::reader::ChumReader;
use crate::util;
use gdnative::prelude::*;
use gdnative::api::{ArrayMesh, Material, Mesh};
use libchum::common;
use libchum::reader::rotshape;
use libchum::structure::ChumEnum;

pub fn read_rotshape(
    data: &Vec<u8>,
    fmt: libchum::format::TotemFormat,
    reader: &mut ChumReader,
    file: &ChumFile,
) -> Option<Dictionary<Unique>> {
    let rsdata = match rotshape::RotShape::read_from(&mut data.as_slice(), fmt) {
        Ok(x) => x,
        Err(err) => {
            display_err!("Error loading ROTSHAPE: {}\n{}", file.get_name_str(), err);
            return None;
        }
    };
    let data = Dictionary::new();
    let pos_tl = rsdata.size[0];
    let pos_br = rsdata.size[1];
    let pos_bl = common::Vector3::new(pos_tl.x, pos_br.y, pos_br.z);
    let pos_tr = common::Vector3::new(pos_br.x, pos_tl.y, pos_tl.z);
    let uv2 = Vector2::new((rsdata.billboard_mode.to_u32() + 2) as f32, 0.0);
    let mesh = ArrayMesh::new();
    let mut verts = Vector3Array::new();
    verts.push(util::vec3_to_godot(&(pos_tl + rsdata.offset)));
    verts.push(util::vec3_to_godot(&(pos_tr + rsdata.offset)));
    verts.push(util::vec3_to_godot(&(pos_br + rsdata.offset)));
    verts.push(util::vec3_to_godot(&(pos_bl + rsdata.offset)));
    let mut texcoords = Vector2Array::new();
    texcoords.push(util::vec2_to_godot(&rsdata.texcoords[0]));
    texcoords.push(util::vec2_to_godot(&rsdata.texcoords[1]));
    texcoords.push(util::vec2_to_godot(&rsdata.texcoords[2]));
    texcoords.push(util::vec2_to_godot(&rsdata.texcoords[3]));
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
    // println!("ROT.unk5: {}", rsdata.unk5);
    // println!("ROT.unk7: {}", rsdata.unk7);
    mesh.add_surface_from_arrays(
        Mesh::PRIMITIVE_TRIANGLE_FAN,
        meshdata.into_shared(),
        VariantArray::new().into_shared(),
        97280,
    );
    let unsafe_archive_instance = file.get_archive_instance();
    let archiveinstance = unsafe { unsafe_archive_instance.assume_safe() };
    archiveinstance
        .map(|archive, res| {
            if let Some(materialfile) =
                archive.get_file_from_hash(res, rsdata.materialanim_id)
            {
                let materialdict = reader.read_materialanim_nodeless(materialfile.clone());
                if materialdict.get("exists").to_bool() == true {
                    let material: Ref<Material,Shared> = materialdict
                        .get("material")
                        .try_to_object()
                        .unwrap();
                    mesh.surface_set_material(0, material);
                } else {
                    display_warn!(
                        "Could not apply material {} to rotshape {}.",
                        unsafe { materialfile.assume_safe() }.map(|x, _| x.get_name_str().to_owned()).unwrap(),
                        file.get_name_str()
                    );
                }
            } else {
                display_warn!(
                    "No such material with ID {} to apply to rotshape {}.",
                    rsdata.materialanim_id,
                    file.get_name_str()
                );
            }
        })
        .unwrap();
    data.insert("mesh", mesh);
    Some(data)
}

pub fn read_rotshape_from_res(data: &ChumFile, reader: &mut ChumReader) -> Dictionary<Unique> {
    let fmt = data.get_format();
    let dict = Dictionary::new();
    match read_rotshape(&data.get_data_as_vec(), fmt, reader, data) {
        Some(mesh) => {
            dict.insert("exists", true);
            dict.insert("rotshape", mesh);
        }
        None => {
            godot_print!("read_skin returned None");
            dict.insert("exists", false);
        }
    }
    dict
}
