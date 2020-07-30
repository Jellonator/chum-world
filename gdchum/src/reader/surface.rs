use crate::chumfile::ChumFile;
use crate::reader::ChumReader;
use crate::util;
use gdnative::*;
use libchum::common;
use libchum::reader::surface;

pub struct SurfaceResult {
    // pub surface: Reference,
    pub transform: common::Mat4x4,
    pub surfaces: Vec<Reference>,
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
    let mut meshes: Vec<Reference> = Vec::new();
    let surfaces = surfaceobj.generate_meshes(surface::SurfaceGenMode::BezierInterp(8));
    // let mut materials = Vec::new();
    for surface in surfaces {
        let mut mesh = ArrayMesh::new();
        let mut verts = Vector3Array::new();
        let mut texcoords = Vector2Array::new();
        let mut normals = Vector3Array::new();
        let mut meshdata = VariantArray::new();
        // let mut colordata = ColorArray::new();
        for quad in surface.quads.iter() {
            for tri in &quad.tris() {
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
                }
            }
        }
        // materials.push(surface.material_index);
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
        meshes.push(mesh.to_reference());
        let archiveinstance = file.get_archive_instance();
        archiveinstance
            .map(|archive, res| {
                if let Some(materialfile) =
                    archive.get_file_from_hash(res.new_ref(), surface.material_index)
                {
                    let materialdict = reader.read_materialanim_nodeless(&materialfile);
                    if materialdict.get(&"exists".into()) == true.into() {
                        let material: Material = materialdict
                            .get(&"material".into())
                            .try_to_object()
                            .unwrap();
                        mesh.surface_set_material(0, Some(material));
                    } else {
                        display_warn!(
                            "Could not apply materialanim {} to surface {}.",
                            materialfile.script().map(|x| x.get_name_str().to_owned()).unwrap(),
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

pub fn read_surface_from_res(data: &ChumFile, reader: &mut ChumReader) -> Dictionary {
    let fmt = data.get_format();
    let mut dict = Dictionary::new();
    match read_surface(&data.get_data_as_vec(), fmt, reader, data) {
        Some(mesh) => {
            dict.set(&"exists".into(), &true.into());
            dict.set(&"surfaces".into(), &mesh.surfaces.to_variant());
            dict.set(
                &"transform".into(),
                &util::mat4x4_to_transform(&mesh.transform).to_variant(),
            );
        }
        None => {
            godot_print!("read_tmesh returned None");
            dict.set(&"exists".into(), &false.into());
        }
    }
    dict
}
