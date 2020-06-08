use crate::bytedata::ByteData;
use crate::chumfile::ChumFile;
use crate::reader::ChumReader;
use gdnative::*;
use libchum::reader::surface;

pub fn read_surface(
    data: &ByteData,
    fmt: libchum::format::TotemFormat,
    reader: &mut ChumReader,
    file: &ChumFile,
) -> Option<Reference> {
    let surfaceobj = match surface::SurfaceObject::read_data(data.get_data(), fmt) {
        Ok(x) => x,
        Err(_) => {
            godot_print!("SURFACE file invalid");
            return None;
        }
    };
    let mut mesh = ArrayMesh::new();
    let surfaces = surfaceobj.generate_meshes(surface::SurfaceGenMode::SingleQuad);
    let mut materials = Vec::new();
    for surface in surfaces {
        let mut verts = Vector3Array::new();
        let mut texcoords = Vector2Array::new();
        let mut normals = Vector3Array::new();
        let mut meshdata = VariantArray::new();
        // let mut colordata = ColorArray::new();
        for quad in surface.quads {
            for point in &quad.points {
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
            }
        }
        materials.push(surface.material_index);
        meshdata.resize(ArrayMesh::ARRAY_MAX as i32);
        meshdata.set(ArrayMesh::ARRAY_VERTEX as i32, &Variant::from(&verts));
        meshdata.set(ArrayMesh::ARRAY_NORMAL as i32, &Variant::from(&normals));
        meshdata.set(ArrayMesh::ARRAY_TEX_UV as i32, &Variant::from(&texcoords));
        mesh.add_surface_from_arrays(
            Mesh::PRIMITIVE_TRIANGLE_FAN,
            meshdata,
            VariantArray::new(),
            97280,
        )
    }
    let archiveinstance = file.get_archive_instance();
    archiveinstance
        .map(|archive, res| {
            for (i, mat) in materials.iter().enumerate() {
                if let Some(materialfile) = archive.get_file_from_hash(res.new_ref(), *mat) {
                    let materialdict = reader.read_materialanim_nodeless(materialfile);
                    if materialdict.get(&"exists".into()) == true.into() {
                        let material: Material = materialdict
                            .get(&"material".into())
                            .try_to_object()
                            .unwrap();
                        mesh.surface_set_material(i as i64, Some(material));
                    } else {
                        godot_warn!("Material {}/{:08X} could not be loaded!", i, mat);
                    }
                } else {
                    godot_warn!("Material {}/{:08X} does not exist!", i, mat);
                }
            }
        })
        .unwrap();
    Some(mesh.to_reference())
}

pub fn read_surface_from_res(data: &ChumFile, reader: &mut ChumReader) -> Dictionary {
    let fmt = data.get_format();
    data.get_bytedata()
        .script()
        .map(|x| {
            let mut dict = Dictionary::new();
            match read_surface(x, fmt, reader, data) {
                Some(mesh) => {
                    dict.set(&"exists".into(), &true.into());
                    dict.set(&"mesh".into(), &mesh.to_variant());
                }
                None => {
                    godot_print!("read_tmesh returned None");
                    dict.set(&"exists".into(), &false.into());
                }
            }
            dict
        })
        .unwrap()
}
