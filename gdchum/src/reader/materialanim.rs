use crate::chumfile::ChumFile;
use crate::reader::ChumReader;
use gdnative::*;
use libchum::reader::materialanim;

pub fn read_materialanim(
    data: &Vec<u8>,
    fmt: libchum::format::TotemFormat,
    reader: &mut ChumReader,
    file: &ChumFile,
) -> Option<Resource> {
    let matanimdata = match materialanim::MaterialAnimation::read_data(data, fmt) {
        Ok(x) => x,
        Err(_) => {
            godot_print!("MATERIALANIM file invalid");
            return None;
        }
    };
    let archiveinstance = file.get_archive_instance();
    archiveinstance
        .map(|archive, archiveres| {
            if let Some(materialfile) =
                archive.get_file_from_hash(archiveres.clone(), matanimdata.material_id)
            {
                let materialdict = reader.read_material_nodeless_nocache(materialfile);
                if materialdict.get(&"exists".into()) == true.into() {
                    let res: Resource = materialdict
                        .get(&"material".into())
                        .try_to_object()
                        .unwrap();
                    reader.add_materialanim(res.clone(), matanimdata, archive, archiveres.new_ref());
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
    let mut dict = Dictionary::new();
    match read_materialanim(&data.get_data_as_vec(), fmt, reader, data) {
        Some(mat) => {
            dict.set(&"exists".into(), &true.into());
            dict.set(&"material".into(), &mat.to_variant());
            // dict.set(&"texture_reflection".into(), &mesh.1.to_variant());
        }
        None => {
            godot_print!("read_materialanim returned None");
            dict.set(&"exists".into(), &false.into());
        }
    }
    dict
}