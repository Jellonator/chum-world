use crate::chumfile::ChumFile;
use crate::util;
use gdnative::prelude::*;
use libchum::reader::skin;

pub fn read_skin(data: &Vec<u8>, fmt: libchum::format::TotemFormat, file: &ChumFile) -> Option<Dictionary<Unique>> {
    let skin = match skin::Skin::read_data(data, fmt) {
        Ok(x) => x,
        Err(err) => {
            display_err!("Error loading SKIN: {}\n{}", file.get_name_str(), err);
            return None;
        }
    };
    let data = Dictionary::new();
    data.insert(
        "transform",
        util::mat4x4_to_transform(&skin.transform.transform),
    );
    data.insert("meshes", skin.meshes);
    let groups = Dictionary::new();
    for group in skin.vertex_groups.iter() {
        let groupdict = Dictionary::new();
        for section in group.sections.iter() {
            let sectiondata = Dictionary::new();
            let vertices = Dictionary::new();
            let normals = Dictionary::new();
            for vertex in section.vertices.iter() {
                vertices.insert(vertex.vertex_id, vertex.weight);
            }
            for normal in section.normals.iter() {
                normals.insert(normal.normal_id, normal.weight);
            }
            sectiondata.insert("vertices", vertices);
            sectiondata.insert("normals", normals);
            if groupdict.contains(&section.mesh_index.to_variant()) {
                display_warn!(
                    "Group {} already contains mesh {}\n{}",
                    group.group_id,
                    section.mesh_index,
                    file.get_name_str()
                );
            }
            groupdict.insert(section.mesh_index, sectiondata);
        }
        if groups.contains(&group.group_id.to_variant()) {
            display_warn!(
                "Skin already contains group {}\n{}",
                group.group_id,
                file.get_name_str()
            );
        }
        groups.insert(group.group_id, groupdict);
    }
    data.insert("groups", groups);
    Some(data)
}

pub fn read_skin_from_res(data: &ChumFile) -> Dictionary<Unique> {
    let fmt = data.get_format();
    let dict = Dictionary::new();
    match read_skin(&data.get_data_as_vec(), fmt, data) {
        Some(mesh) => {
            dict.insert("exists", true);
            dict.insert("skin", mesh);
        }
        None => {
            godot_print!("read_skin returned None");
            dict.insert("exists", false);
        }
    }
    dict
}
