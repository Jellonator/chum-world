use crate::chumfile::ChumFile;
use crate::reader::ChumReader;
use gdnative::prelude::*;
use gdnative::api::ShaderMaterial;
use libchum::reader::materialanim;

pub fn read_materialanim(
    data: &Vec<u8>,
    fmt: libchum::format::TotemFormat,
    reader: &mut ChumReader,
    file: &ChumFile,
) -> Option<Ref<ShaderMaterial, Shared>> {
    use libchum::binary::ChumBinary;
    let matanimdata = match materialanim::MaterialAnimation::read_from(&mut data.as_slice(), fmt) {
        Ok(x) => x,
        Err(err) => {
            display_err!("Error loading MATERIALANIM: {}\n{}", file.get_name_str(), err);
            return None;
        }
    };
    let unsafe_archive_instance = file.get_archive_instance();
    let archiveinstance = unsafe { unsafe_archive_instance.assume_safe() };
    archiveinstance
        .map(|archive, archiveres| {
            if let Some(materialfile) =
                archive.get_file_from_hash(archiveres, matanimdata.material_id)
            {
                let materialdict = reader.read_material_nodeless_nocache(materialfile.clone());
                if materialdict.get("exists").to_bool() == true {
                    let res: Ref<ShaderMaterial, Shared> = materialdict
                        .get("material")
                        .try_to_object()
                        .unwrap();
                    reader.add_materialanim(
                        res.clone(),
                        matanimdata,
                        archive,
                        archiveres,
                    );
                    Some(res)
                } else {
                    display_warn!(
                        "Could not apply material {} to materialanim {}.",
                        unsafe { materialfile.assume_safe() }.map(|x,_| x.get_name_str().to_owned()).unwrap(),
                        file.get_name_str()
                    );
                    None
                }
            } else {
                display_warn!(
                    "No such material with ID {} to apply to material {}.",
                    matanimdata.material_id,
                    file.get_name_str()
                );
                None
            }
        })
        .unwrap()
}

pub fn read_materialanim_from_res(data: &ChumFile, reader: &mut ChumReader) -> Dictionary<Unique> {
    let fmt = data.get_format();
    let dict = Dictionary::new();
    match read_materialanim(&data.get_data_as_vec(), fmt, reader, data) {
        Some(mat) => {
            dict.insert("exists", true);
            dict.insert("material", mat);
            // dict.set(&"texture_reflection".into(), &mesh.1.to_variant());
        }
        None => {
            godot_print!("read_materialanim returned None");
            dict.insert("exists", false);
        }
    }
    dict
}
