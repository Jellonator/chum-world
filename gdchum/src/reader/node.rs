use crate::chumfile::ChumFile;
use crate::util;
use gdnative::prelude::*;
use libchum::reader::node;

pub fn read_node(data: &Vec<u8>, fmt: libchum::format::TotemFormat, file: &ChumFile) -> Option<Dictionary<Unique>> {
    use libchum::binary::ChumBinary;
    let node = match node::Node::read_from(&mut data.as_slice(), fmt) {
        Ok(x) => x,
        Err(err) => {
            display_err!("Error loading NODE: {}\n{}", file.get_name_str(), err);
            return None;
        }
    };
    let data = Dictionary::new();
    data.insert("parent_id", node.node_parent_id);
    data.insert("resource_id", node.resource_id);
    data.insert(
        "global_transform",
        util::transform3d_to_godot(&node.global_transform),
    );
    data.insert(
        "local_transform",
        util::transform3d_to_godot(&node.local_transform),
    );
    data.insert(
        "local_translation",
        node.local_translation,
    );
    data.insert(
        "local_scale",
        node.local_scale,
    );
    // data.insert(
    //     "local_rotation",
    //     util::quat_to_godot(&node.local_rotation),
    // );
    Some(data)
}

pub fn read_node_from_res(data: &ChumFile) -> Dictionary<Unique> {
    let fmt = data.get_format();
    let dict = Dictionary::new();
    match read_node(&data.get_data_as_vec(), fmt, data) {
        Some(node) => {
            dict.insert("exists", true);
            dict.insert("node", node);
        }
        None => {
            godot_print!("read_node returned None");
            dict.insert("exists", false);
        }
    }
    dict
}
