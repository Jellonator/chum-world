use crate::chumfile::ChumFile;
use gdnative::prelude::*;
use libchum::reader::lod;

pub fn read_lod(
    data: &Vec<u8>,
    fmt: libchum::format::TotemFormat,
    chumfile: &ChumFile,
) -> Option<Dictionary<Unique>> {
    let loddata = match lod::Lod::read_from(&mut data.as_slice(), fmt) {
        Ok(x) => x,
        Err(e) => {
            display_err!("Error loading LOD: {}\n{}", chumfile.get_name_str(), e);
            return None;
        }
    };
    let mut data = Dictionary::new();
    data.insert("skins", loddata.skin_ids);
    Some(data)
}

pub fn read_lod_from_res(data: &ChumFile) -> Dictionary<Unique> {
    let fmt = data.get_format();
    let mut dict = Dictionary::new();
    match read_lod(&data.get_data_as_vec(), fmt, data) {
        Some(mesh) => {
            dict.insert("exists", true);
            dict.insert("lod", mesh);
        }
        None => {
            godot_print!("read_skin returned None");
            dict.insert("exists", false);
        }
    }
    dict
}
