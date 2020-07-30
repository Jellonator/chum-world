use crate::chumfile::ChumFile;
use crate::reader::ChumReader;
use crate::util;
use gdnative::*;
use libchum::reader::material;

pub fn read_material(
    data: &Vec<u8>,
    fmt: libchum::format::TotemFormat,
    reader: &mut ChumReader,
    file: &ChumFile,
) -> Option<Reference> {
    let matdata = match material::Material::read_data(data, fmt) {
        Ok(x) => x,
        Err(err) => {
            display_err!("Error loading MATERIAL: {}\n{}", file.get_name_str(), err);
            return None;
        }
    };
    let mut material = ShaderMaterial::new();
    let shader: Resource = match ResourceLoader::godot_singleton().load(
        "res://Shader/material.shader".into(),
        "Shader".into(),
        false,
    ) {
        Some(x) => x,
        None => {
            display_err!(
                "Error loading MATERIAL: {}\nCould not read shader.",
                file.get_name_str(),
            );
            return None;
        }
    };
    material.set_shader(Some(shader.cast().unwrap()));
    let archiveinstance = file.get_archive_instance();
    archiveinstance
        .map(|archive, res| {
            if matdata.get_texture() != 0 {
                if let Some(texturefile) =
                    archive.get_file_from_hash(res.clone(), matdata.get_texture())
                {
                    let texturedict = reader.read_bitmap_nodeless(&texturefile);
                    if texturedict.get(&"exists".into()) == true.into() {
                        let image: Image =
                            texturedict.get(&"bitmap".into()).try_to_object().unwrap();
                        let mut texture: ImageTexture = ImageTexture::new();
                        texture.create_from_image(Some(image), 1 | 2 | 4);
                        material.set_shader_param("has_texture".into(), true.into());
                        material.set_shader_param("arg_texture".into(), texture.into());
                    } else {
                        display_warn!(
                            "Could not apply bitmap {} to material {}.",
                            texturefile.script().map(|x| x.get_name_str().to_owned()).unwrap(),
                            file.get_name_str()
                        );
                    }
                } else {
                    display_warn!(
                        "No such bitmap with ID {} to apply to material {}.",
                        matdata.get_texture(),
                        file.get_name_str()
                    );
                }
            }
            if matdata.texture_reflection != 0 {
                if let Some(texturefile) =
                    archive.get_file_from_hash(res, matdata.texture_reflection)
                {
                    let texturedict = reader.read_bitmap_nodeless(&texturefile);
                    if texturedict.get(&"exists".into()) == true.into() {
                        godot_print!("Found material for {}", matdata.texture_reflection);
                        let image: Image =
                            texturedict.get(&"bitmap".into()).try_to_object().unwrap();
                        let mut texture: ImageTexture = ImageTexture::new();
                        texture.create_from_image(Some(image), 2);
                        material.set_shader_param("has_reflection".into(), true.into());
                        material.set_shader_param("arg_reflection".into(), texture.into());
                    } else {
                        display_warn!(
                            "Could not apply bitmap {} to material {}.",
                            texturefile.script().map(|x| x.get_name_str().to_owned()).unwrap(),
                            file.get_name_str()
                        );
                    }
                } else {
                    display_warn!(
                        "No such bitmap with ID {} to apply to material {}.",
                        matdata.get_texture(),
                        file.get_name_str()
                    );
                }
            }
        })
        .unwrap();
    material.set_shader_param(
        "arg_color".into(),
        Vector3::new(matdata.color[0], matdata.color[1], matdata.color[2]).to_variant(),
    );
    material.set_shader_param("arg_alpha".into(), matdata.color[3].to_variant());
    let tx = util::mat3x3_to_transform2d(&matdata.transform);
    let realtx = Transform {
        basis: Basis {
            elements: [
                Vector3::new(tx.m11, tx.m12, 0.0),
                Vector3::new(tx.m21, tx.m22, 0.0),
                Vector3::new(tx.m31, tx.m32, 1.0),
            ],
        },
        origin: Vector3::new(0.0, 0.0, 0.0),
    };
    material.set_shader_param("arg_texcoord_transform".into(), realtx.to_variant());
    Some(material.to_reference())
}

pub fn read_material_from_res(data: &ChumFile, reader: &mut ChumReader) -> Dictionary {
    let fmt = data.get_format();
    let mut dict = Dictionary::new();
    match read_material(&data.get_data_as_vec(), fmt, reader, data) {
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
}
