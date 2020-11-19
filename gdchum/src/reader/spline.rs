use crate::chumfile::ChumFile;
use crate::util;
use gdnative::prelude::*;
use libchum::reader::spline;

pub fn read_spline(data: &Vec<u8>, fmt: libchum::format::TotemFormat, file: &ChumFile) -> Option<Dictionary<Unique>> {
    use libchum::binary::ChumBinary;
    let spline = match spline::Spline::read_from(&mut data.as_slice(), fmt) {
        Ok(x) => x,
        Err(err) => {
            display_err!("Error loading SPLINE: {}\n{}", file.get_name_str(), err);
            return None;
        }
    };
    let data = Dictionary::new();
    data.insert("unk4", spline.unk4.to_vec().to_variant());
    data.insert(
        "vertices",
        spline
            .get_vertices_as_vec()
            .into_iter()
            .map(|x| util::vec3_to_godot(&x))
            .collect::<Vec<Vector3>>()
            .to_variant(),
    );
    data.insert(
        "stops",
        &spline
            .get_section_stops_as_vec()
            .into_iter()
            .map(|x| util::vec3_to_godot(&x))
            .collect::<Vec<Vector3>>()
            .to_variant(),
    );
    Some(data)
}

pub fn read_spline_from_res(data: &ChumFile) -> Dictionary<Unique> {
    let fmt = data.get_format();
    let dict = Dictionary::new();
    match read_spline(&data.get_data_as_vec(), fmt, data) {
        Some(mesh) => {
            dict.insert("exists", true);
            dict.insert("spline", mesh);
        }
        None => {
            godot_print!("read_spline returned None");
            dict.insert("exists", false);
        }
    }
    dict
}
