use crate::bytedata::ByteData;
use crate::chumfile::ChumFile;
use gdnative::*;
use libchum::reader::bitmap;

pub fn read_bitmap(data: &ByteData, fmt: libchum::format::TotemFormat) -> Option<Reference> {
    let bitmap = match bitmap::Bitmap::read_data(data.get_data(), fmt) {
        Ok(x) => x,
        Err(err) => {
            godot_print!("BITMAP file invalid: {}", err);
            return None;
        }
    };
    let mut image = Image::new();
    let mut data = ByteArray::new();
    for color in bitmap.get_data() {
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
    Some(image.to_reference())
}

pub fn read_bitmap_from_res(data: &ChumFile) -> Dictionary {
    let fmt = data.get_format();
    godot_print!("FORMAT: {:?}", fmt);
    data.get_bytedata()
        .script()
        .map(|x| {
            let mut dict = Dictionary::new();
            match read_bitmap(x, fmt) {
                Some(mesh) => {
                    dict.set(&"exists".into(), &true.into());
                    dict.set(&"bitmap".into(), &mesh.to_variant());
                }
                None => {
                    godot_print!("read_tmesh returned None");
                    dict.set(&"exists".into(), &false.into());
                    dict.set(&"bitmap".into(), &Variant::new());
                }
            }
            dict
        })
        .unwrap()
}
