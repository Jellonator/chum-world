use crate::chumfile::ChumFile;
use crate::reader::ChumReader;
use crate::util;
use gdnative::*;
use libchum::reader::rotshape;
use libchum::common;

pub fn read_rotshape(
    data: &Vec<u8>,
    fmt: libchum::format::TotemFormat,
    reader: &mut ChumReader,
    file: &ChumFile,
) -> Option<Dictionary> {
    let rsdata = match rotshape::RotShape::read_from(&mut data.as_slice(), fmt) {
        Ok(x) => x,
        Err(e) => {
            godot_print!("ROTSHAPE file invalid: {}", e);
            return None;
        }
    };
    let mut data = Dictionary::new();
    let pos_tl = rsdata.size[0];
    let pos_br = rsdata.size[1];
    let pos_bl = common::Vector3::new(pos_tl.x, pos_br.y, pos_br.z);
    let pos_tr = common::Vector3::new(pos_br.x, pos_tl.y, pos_tl.z);
    let uv2 = Vector2::new((rsdata.billboard_mode.to_u16() + 2) as f32, 0.0);
    let mut mesh = ArrayMesh::new();
    let mut verts = Vector3Array::new();
    verts.push(&util::vec3_to_godot(&(pos_tl + rsdata.unk5)));
    verts.push(&util::vec3_to_godot(&(pos_tr + rsdata.unk5)));
    verts.push(&util::vec3_to_godot(&(pos_br + rsdata.unk5)));
    verts.push(&util::vec3_to_godot(&(pos_bl + rsdata.unk5)));
    let mut texcoords = Vector2Array::new();
    texcoords.push(&util::vec2_to_godot(&rsdata.texcoords[0]));
    texcoords.push(&util::vec2_to_godot(&rsdata.texcoords[1]));
    texcoords.push(&util::vec2_to_godot(&rsdata.texcoords[2]));
    texcoords.push(&util::vec2_to_godot(&rsdata.texcoords[3]));
    let mut uv2coords = Vector2Array::new();
    uv2coords.push(&uv2);
    uv2coords.push(&uv2);
    uv2coords.push(&uv2);
    uv2coords.push(&uv2);
    let mut normals = Vector3Array::new();
    normals.push(&Vector3::new(0.0, 0.0, 1.0));
    normals.push(&Vector3::new(0.0, 0.0, 1.0));
    normals.push(&Vector3::new(0.0, 0.0, 1.0));
    normals.push(&Vector3::new(0.0, 0.0, 1.0));
    let mut meshdata = VariantArray::new();
    meshdata.resize(ArrayMesh::ARRAY_MAX as i32);
    meshdata.set(ArrayMesh::ARRAY_VERTEX as i32, &Variant::from(&verts));
    meshdata.set(ArrayMesh::ARRAY_NORMAL as i32, &Variant::from(&normals));
    meshdata.set(ArrayMesh::ARRAY_TEX_UV as i32, &Variant::from(&texcoords));
    meshdata.set(ArrayMesh::ARRAY_TEX_UV2 as i32, &Variant::from(&uv2coords));
    // println!("ROT.unk5: {}", rsdata.unk5);
    // println!("ROT.unk7: {}", rsdata.unk7);
    mesh.add_surface_from_arrays(
        Mesh::PRIMITIVE_TRIANGLE_FAN,
        meshdata,
        VariantArray::new(),
        97280,
    );
    let archiveinstance = file.get_archive_instance();
    archiveinstance
    .map(|archive, res| {
        // let mat = &materials[i];
        // for (i, mat) in materials.iter().enumerate() {
        if let Some(materialfile) = archive.get_file_from_hash(res.new_ref(), rsdata.materialanim_id) {
            let materialdict = reader.read_materialanim_nodeless(materialfile);
            if materialdict.get(&"exists".into()) == true.into() {
                let material: Material = materialdict
                    .get(&"material".into())
                    .try_to_object()
                    .unwrap();
                mesh.surface_set_material(0, Some(material));
            } else {
                godot_warn!("Material {} could not be loaded!", rsdata.materialanim_id);
            }
        } else {
            godot_warn!("Material {} does not exist!", rsdata.materialanim_id);
        }
        // }
    })
    .unwrap();
    data.set(&"mesh".into(), &mesh.to_variant());
    Some(data)
}

pub fn read_rotshape_from_res(data: &ChumFile, reader: &mut ChumReader) -> Dictionary {
    let fmt = data.get_format();
    let mut dict = Dictionary::new();
    match read_rotshape(&data.get_data_as_vec(), fmt, reader, data) {
        Some(mesh) => {
            dict.set(&"exists".into(), &true.into());
            dict.set(&"rotshape".into(), &mesh.to_variant());
        }
        None => {
            godot_print!("read_skin returned None");
            dict.set(&"exists".into(), &false.into());
        }
    }
    dict
}
