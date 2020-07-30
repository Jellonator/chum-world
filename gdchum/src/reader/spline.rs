use crate::chumfile::ChumFile;
use crate::util;
use gdnative::*;
use libchum::reader::spline;

pub fn read_spline(data: &Vec<u8>, fmt: libchum::format::TotemFormat, file: &ChumFile) -> Option<Dictionary> {
    let spline = match spline::Spline::read_from(&mut data.as_slice(), fmt) {
        Ok(x) => x,
        Err(err) => {
            display_err!("Error loading SPLINE: {}\n{}", file.get_name_str(), err);
            return None;
        }
    };
    let mut data = Dictionary::new();
    data.set(&"unk4".into(), &spline.unk4.to_vec().to_variant());
    data.set(
        &"vertices".into(),
        &spline
            .get_vertices_as_vec()
            .into_iter()
            .map(|x| util::vec3_to_godot(&x))
            .collect::<Vec<Vector3>>()
            .to_variant(),
    );
    data.set(
        &"stops".into(),
        &spline
            .get_section_stops_as_vec()
            .into_iter()
            .map(|x| util::vec3_to_godot(&x))
            .collect::<Vec<Vector3>>()
            .to_variant(),
    );
    Some(data)
}

pub fn read_spline_from_res(data: &ChumFile) -> Dictionary {
    let fmt = data.get_format();
    let mut dict = Dictionary::new();
    match read_spline(&data.get_data_as_vec(), fmt, data) {
        Some(mesh) => {
            dict.set(&"exists".into(), &true.into());
            dict.set(&"spline".into(), &mesh.to_variant());
        }
        None => {
            godot_print!("read_spline returned None");
            dict.set(&"exists".into(), &false.into());
        }
    }
    dict
}
