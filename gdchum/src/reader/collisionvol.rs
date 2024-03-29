use crate::chumfile::ChumFile;
use crate::util;
use gdnative::prelude::*;
use libchum::reader::collisionvol;

pub fn read_collisionvol(
    data: &Vec<u8>,
    fmt: libchum::format::TotemFormat,
    chumfile: &ChumFile,
) -> Option<Dictionary<Unique>> {
    use libchum::binary::ChumBinary;
    let volume = match collisionvol::CollisionVol::read_from(&mut data.as_slice(), fmt) {
        Ok(x) => x,
        Err(e) => {
            display_err!(
                "Error loading COLLISIONVOL: {}\n{}",
                chumfile.get_name_str(),
                e
            );
            return None;
        }
    };
    let data = Dictionary::new();
    data.insert(
        "local_transform",
        util::transform3d_to_godot(&volume.local_transform),
    );
    data.insert(
        "local_transform_inv",
        util::transform3d_to_godot(&volume.local_transform_inv),
    );
    Some(data)
}

pub fn read_collisionvol_from_res(data: &ChumFile) -> Dictionary<Unique> {
    let fmt = data.get_format();
    let dict = Dictionary::new();
    match read_collisionvol(&data.get_data_as_vec(), fmt, data) {
        Some(vol) => {
            dict.insert("exists", true);
            dict.insert("collisionvol", vol);
        }
        None => {
            godot_print!("read_collisionvol returned None");
            dict.insert("exists", false);
        }
    }
    dict
}
