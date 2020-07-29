use crate::chumfile::ChumFile;
// use crate::util;
use gdnative::*;
use libchum::reader::lod;

pub fn read_lod(data: &Vec<u8>, fmt: libchum::format::TotemFormat) -> Option<Dictionary> {
    let loddata = match lod::Lod::read_from(&mut data.as_slice(), fmt) {
        Ok(x) => x,
        Err(e) => {
            godot_print!("LOD file invalid: {}", e);
            return None;
        }
    };
    let mut data = Dictionary::new();
    data.set(&"skins".into(), &loddata.skin_ids.to_variant());
    Some(data)
}

pub fn read_lod_from_res(data: &ChumFile) -> Dictionary {
    let fmt = data.get_format();
    let mut dict = Dictionary::new();
    match read_lod(&data.get_data_as_vec(), fmt) {
        Some(mesh) => {
            dict.set(&"exists".into(), &true.into());
            dict.set(&"lod".into(), &mesh.to_variant());
        }
        None => {
            godot_print!("read_skin returned None");
            dict.set(&"exists".into(), &false.into());
        }
    }
    dict
}
