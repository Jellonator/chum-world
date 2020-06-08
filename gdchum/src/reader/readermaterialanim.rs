use crate::bytedata::ByteData;
use crate::chumfile::ChumFile;
use crate::reader::ChumReader;
use gdnative::*;
use libchum::reader::materialanim;

pub fn read_materialanim(
    data: &ByteData,
    fmt: libchum::format::TotemFormat,
    reader: &mut ChumReader,
    file: &ChumFile,
) -> Option<Resource> {
    let matanimdata = match materialanim::MaterialAnimation::read_data(data.get_data(), fmt) {
        Ok(x) => x,
        Err(_) => {
            godot_print!("TMESH file invalid");
            return None;
        }
    };
    let archiveinstance = file.get_archive_instance();
    archiveinstance
        .map(|archive, res| {
            if let Some(materialfile) =
                archive.get_file_from_hash(res.clone(), matanimdata.material_id)
            {
                let materialdict = reader.read_material_nodeless_nocache(materialfile);
                if materialdict.get(&"exists".into()) == true.into() {
                    let res: Resource = materialdict
                        .get(&"material".into())
                        .try_to_object()
                        .unwrap();
                    reader.add_materialanim(res.clone(), matanimdata, archive, res.new_ref());
                    Some(res)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap()
}

pub fn read_materialanim_from_res(data: &ChumFile, reader: &mut ChumReader) -> Dictionary {
    let fmt = data.get_format();
    data.get_bytedata()
        .script()
        .map(|x| {
            let mut dict = Dictionary::new();
            match read_materialanim(x, fmt, reader, data) {
                Some(mat) => {
                    dict.set(&"exists".into(), &true.into());
                    dict.set(&"material".into(), &mat.to_variant());
                    // dict.set(&"texture_reflection".into(), &mesh.1.to_variant());
                }
                None => {
                    godot_print!("read_tmesh returned None");
                    dict.set(&"exists".into(), &false.into());
                }
            }
            dict
        })
        .unwrap()
}
