use crate::bytedata::ByteData;
use crate::chumfile::ChumFile;
use gdnative::*;
use libchum::reader::tmesh;

pub fn read_tmesh(data: &ByteData, fmt: libchum::format::TotemFormat) -> Option<(Reference, Vec<i32>)> {
    let tmesh = match tmesh::TMesh::read_data(data.get_data(), fmt) {
        Ok(x) => x,
        Err(_) => {
            godot_print!("TMESH file invalid");
            return None;
        }
    };
    let mut mesh = ArrayMesh::new();
    let generated_tris = tmesh.gen_triangles();
    let num = generated_tris.len();
    let mesh_materials = tmesh.get_materials();
    let mut materials = Vec::new();
    // let colors = [
    //     Color::rgb(0.05, 0.05, 0.05),
    //     Color::rgb(0.95, 0.95, 0.95),
    //     Color::rgb(0.95, 0.05, 0.05),
    //     Color::rgb(0.05, 0.95, 0.05),
    //     Color::rgb(0.05, 0.05, 0.95),
    //     Color::rgb(0.95, 0.95, 0.05),
    //     Color::rgb(0.05, 0.95, 0.95),
    //     Color::rgb(0.95, 0.05, 0.95),
    //     Color::rgb(0.50, 0.50, 0.50),
    //     Color::rgb(0.95, 0.50, 0.05),
    //     Color::rgb(0.05, 0.50, 0.95),
    //     Color::rgb(0.95, 0.05, 0.50),
    //     Color::rgb(0.05, 0.95, 0.50),
    //     Color::rgb(0.50, 0.95, 0.05),
    //     Color::rgb(0.50, 0.05, 0.95),
    //     Color::rgb(0.50, 0.50, 0.95),
    //     Color::rgb(0.50, 0.50, 0.05),
    //     Color::rgb(0.50, 0.05, 0.50),
    //     Color::rgb(0.50, 0.95, 0.50),
    //     Color::rgb(0.05, 0.50, 0.50),
    //     Color::rgb(0.95, 0.50, 0.50),
    // ];
    godot_print!("There are {} colors", num);
    for (i, mat) in tmesh.get_materials().iter().enumerate() {
        godot_print!("Material {:>2} is 0x{:08X}", i, mat);
    }
    for (i, trivec) in generated_tris.into_iter().enumerate() {
        let mut verts = Vector3Array::new();
        let mut texcoords = Vector2Array::new();
        let mut normals = Vector3Array::new();
        let mut meshdata = VariantArray::new();
        // let mut colordata = ColorArray::new();
        godot_print!("Strip {:>2} has material {:>4}", i, trivec.material_index);
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
            }
        }
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
        )
    }
    Some((mesh.to_reference(), materials))
}

pub fn read_tmesh_from_res(data: &ChumFile) -> Dictionary {
    let fmt = data.get_format();
    godot_print!("FORMAT: {:?}", fmt);
    data.get_bytedata()
        .script()
        .map(|x| {
            let mut dict = Dictionary::new();
            match read_tmesh(x, fmt) {
                Some(mesh) => {
                    dict.set(&"exists".into(), &true.into());
                    dict.set(&"mesh".into(), &mesh.0.to_variant());
                    dict.set(&"materials".into(), &mesh.1.to_variant());
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
