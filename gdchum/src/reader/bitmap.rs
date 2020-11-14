use crate::chumfile::ChumFile;
use gdnative::prelude::*;
use libchum::reader::bitmap;

pub fn read_bitmap(
    data: &Vec<u8>,
    fmt: libchum::format::TotemFormat,
    chumfile: &ChumFile,
) -> Option<(Ref<Image,Unique>, bool)> {
    let bitmap = match bitmap::Bitmap::read_data(data, fmt) {
        Ok(x) => x,
        Err(err) => {
            display_err!("Error loading BITMAP: {}\n{}", chumfile.get_name_str(), err);
            return None;
        }
    };
    let mut image = Ref::<Image,Unique>::new();
    let mut data = ByteArray::new();
    for color in bitmap.get_data_as_vec().into_iter() {
        data.push(color.r);
        data.push(color.g);
        data.push(color.b);
        data.push(color.a);
    }
    image.create_from_data(
        bitmap.get_width() as i64,
        bitmap.get_height() as i64,
        false,
        Image::FORMAT_RGBA8,
        data,
    );
    Some((
        image,
        bitmap.get_alpha_level() != bitmap::AlphaLevel::Opaque,
    ))
}

pub fn read_bitmap_from_res(data: &ChumFile) -> Dictionary<Unique> {
    let fmt = data.get_format();
    let mut dict = Dictionary::new();
    match read_bitmap(&data.get_data_as_vec(), fmt, data) {
        Some((mesh, hasalpha)) => {
            dict.insert("exists", true);
            dict.insert("bitmap", mesh);
            dict.insert("hasalpha", hasalpha);
        }
        None => {
            godot_print!("read_tmesh returned None");
            dict.insert("exists", false);
        }
    }
    dict
}
