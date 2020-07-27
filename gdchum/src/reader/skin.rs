use crate::chumfile::ChumFile;
use crate::util;
use gdnative::*;
use libchum::reader::skin;

pub fn read_skin(data: &Vec<u8>, fmt: libchum::format::TotemFormat) -> Option<Dictionary> {
    let skin = match skin::Skin::read_data(data, fmt) {
        Ok(x) => x,
        Err(_) => {
            godot_print!("SKIN file invalid");
            return None;
        }
    };
    let mut data = Dictionary::new();
    data.set(
        &"transform".into(),
        &util::mat4x4_to_transform(&skin.transform.transform).to_variant(),
    );
    data.set(&"meshes".into(), &skin.meshes.to_variant());
    let mut groups = Dictionary::new();
    for group in skin.vertex_groups.iter() {
        let mut groupdict = Dictionary::new();
        for section in group.sections.iter() {
            let mut sectiondata = Dictionary::new();
            let mut vertices = Dictionary::new();
            let mut normals = Dictionary::new();
            for vertex in section.vertices.iter() {
                vertices.set(&vertex.vertex_id.to_variant(), &vertex.weight.to_variant());
            }
            for normal in section.normals.iter() {
                normals.set(&normal.normal_id.to_variant(), &normal.weight.to_variant());
            }
            sectiondata.set(&"vertices".into(), &vertices.to_variant());
            sectiondata.set(&"normals".into(), &normals.to_variant());
            if groupdict.contains(&section.mesh_index.to_variant()) {
                godot_warn!(
                    "Group {} already contains mesh {}",
                    group.group_id,
                    section.mesh_index
                );
            }
            groupdict.set(&section.mesh_index.to_variant(), &sectiondata.to_variant());
        }
        if groups.contains(&group.group_id.to_variant()) {
            godot_warn!("Skin already contains group {}", group.group_id);
        }
        groups.set(&group.group_id.to_variant(), &groupdict.to_variant());
    }
    data.set(&"groups".into(), &groups.to_variant());
    Some(data)
}

pub fn read_skin_from_res(data: &ChumFile) -> Dictionary {
    let fmt = data.get_format();
    let mut dict = Dictionary::new();
    match read_skin(&data.get_data_as_vec(), fmt) {
        Some(mesh) => {
            dict.set(&"exists".into(), &true.into());
            dict.set(&"skin".into(), &mesh.to_variant());
        }
        None => {
            godot_print!("read_skin returned None");
            dict.set(&"exists".into(), &false.into());
        }
    }
    dict
}
