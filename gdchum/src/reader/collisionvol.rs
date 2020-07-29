use crate::chumfile::ChumFile;
use crate::util;
use gdnative::*;
use libchum::reader::collisionvol;

pub fn read_collisionvol(data: &Vec<u8>, fmt: libchum::format::TotemFormat) -> Option<Dictionary> {
    let volume = match collisionvol::CollisionVol::read_from(&mut data.as_slice(), fmt) {
        Ok(x) => x,
        Err(_) => {
            godot_print!("COLLISIONVOL file invalid");
            return None;
        }
    };
    let mut data = Dictionary::new();
    data.set(&"local_transform".into(), &util::mat4x4_to_transform(&volume.local_transform).to_variant());
    data.set(&"local_transform_inv".into(), &util::mat4x4_to_transform(&volume.local_transform_inv).to_variant());
    Some(data)
}

pub fn read_collisionvol_from_res(data: &ChumFile) -> Dictionary {
    let fmt = data.get_format();
    let mut dict = Dictionary::new();
    match read_collisionvol(&data.get_data_as_vec(), fmt) {
        Some(vol) => {
            dict.set(&"exists".into(), &true.into());
            dict.set(&"collisionvol".into(), &vol.to_variant());
        }
        None => {
            godot_print!("read_collisionvol returned None");
            dict.set(&"exists".into(), &false.into());
        }
    }
    dict
}
