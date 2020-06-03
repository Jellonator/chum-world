use crate::bytedata::ByteData;
use crate::chumfile::ChumFile;
use gdnative::*;
use libchum::reader::material;
use crate::reader::ChumReader;

pub fn read_material(data: &ByteData, fmt: libchum::format::TotemFormat, reader: &mut ChumReader, file: &ChumFile) -> Option<Reference> {
    let matdata = match material::Material::read_data(data.get_data(), fmt) {
        Ok(x) => x,
        Err(_) => {
            godot_print!("TMESH file invalid");
            return None;
        }
    };
    let mut material = SpatialMaterial::new();
    let archiveinstance = file.get_archive_instance();
    archiveinstance.map(|archive, res| {
        if let Some(texturefile) = archive.get_file_from_hash(res, matdata.get_texture()) {
            let texturedict = reader.read_bitmap_nodeless(texturefile);
            if texturedict.get(&"exists".into()) == true.into() {
                godot_print!("Found material for {}", matdata.get_texture());
                let image: Image = texturedict.get(&"bitmap".into()).try_to_object().unwrap();
                let mut texture: ImageTexture = ImageTexture::new();
                texture.create_from_image(Some(image), 0);
                material.set_texture(SpatialMaterial::TEXTURE_ALBEDO, Some(texture.cast().unwrap()));
                material.set_feature(SpatialMaterial::FEATURE_TRANSPARENT, texturedict.get(&"hasalpha".into()).to_bool());
            } else {
                godot_warn!("Material {} has invalid bitmap", matdata.get_texture());
            }
        } else {
            godot_warn!("Material {} could not be found", matdata.get_texture());
        }
    }).unwrap();
    Some(material.to_reference())
}

pub fn read_material_from_res(data: &ChumFile, reader: &mut ChumReader) -> Dictionary {
    let fmt = data.get_format();
    godot_print!("FORMAT: {:?}", fmt);
    data.get_bytedata()
        .script()
        .map(|x| {
            let mut dict = Dictionary::new();
            match read_material(x, fmt, reader, data) {
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