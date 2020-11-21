use crate::chumfile::ChumFile;
use crate::reader::ChumReader;
use crate::util;
use gdnative::prelude::*;
use gdnative::api::{ShaderMaterial, ImageTexture};
use libchum::reader::material;

pub fn read_material(
    data: &Vec<u8>,
    fmt: libchum::format::TotemFormat,
    reader: &mut ChumReader,
    file: &ChumFile,
) -> Option<Ref<ShaderMaterial,Unique>> {
    use libchum::binary::ChumBinary;
    let matdata = match material::Material::read_from(&mut data.as_slice(), fmt) {
        Ok(x) => x,
        Err(err) => {
            display_err!("Error loading MATERIAL: {}\n{}", file.get_name_str(), err);
            return None;
        }
    };
    let material = Ref::<ShaderMaterial,Unique>::new();
    let shader: Ref::<Shader,Shared> = match ResourceLoader::godot_singleton().load(
        "res://Shader/material.shader",
        "Shader",
        false,
    ) {
        Some(x) => x.cast().unwrap(),
        None => {
            display_err!(
                "Error loading MATERIAL: {}\nCould not read shader.",
                file.get_name_str(),
            );
            return None;
        }
    };
    material.set_shader(shader);
    let unsafe_archive_instance = file.get_archive_instance();
    let archiveinstance = unsafe { unsafe_archive_instance.assume_safe() };
    archiveinstance
        .map(|archive, res| {
            if matdata.get_texture() != 0 {
                if let Some(texturefile) =
                    archive.get_file_from_hash(&res, matdata.get_texture())
                {
                    let texturedict = reader.read_bitmap_nodeless(texturefile.clone());
                    if texturedict.get("exists").to_bool() == true {
                        let image: Ref<Image,Shared> =
                            texturedict.get("bitmap").try_to_object().unwrap();
                        let texture = Ref::<ImageTexture,Unique>::new();
                        texture.create_from_image(image, 1 | 2 | 4);
                        material.set_shader_param("has_texture", true);
                        material.set_shader_param("arg_texture", texture);
                    } else {
                        display_warn!(
                            "Could not apply bitmap {} to material {}.",
                            unsafe { texturefile.assume_safe() }.map(|x,_| x.get_name_str().to_owned()).unwrap(),
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
                    archive.get_file_from_hash(&res, matdata.texture_reflection)
                {
                    let texturedict = reader.read_bitmap_nodeless(texturefile.clone());
                    if texturedict.get("exists").to_bool() == true {
                        godot_print!("Found material for {}", matdata.texture_reflection);
                        let image: Ref<Image, Shared> =
                            texturedict.get("bitmap").try_to_object().unwrap();
                        let texture = Ref::<ImageTexture, Unique>::new();
                        texture.create_from_image(image, 1 | 2 | 4);
                        material.set_shader_param("has_reflection", true);
                        material.set_shader_param("arg_reflection", texture);
                    } else {
                        display_warn!(
                            "Could not apply bitmap {} to material {}.",
                            unsafe {texturefile.assume_safe() }.map(|x,_| x.get_name_str().to_owned()).unwrap(),
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
        "arg_color",
        Vector3::new(matdata.color[0], matdata.color[1], matdata.color[2]),
    );
    material.set_shader_param("arg_alpha", matdata.color[3]);
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
    material.set_shader_param("arg_texcoord_transform", realtx);
    material.set_shader_param("arg_emission", util::vec3_to_godot(&matdata.emission));
    Some(material)
}

pub fn read_material_from_res(data: &ChumFile, reader: &mut ChumReader) -> Dictionary<Unique> {
    let fmt = data.get_format();
    let dict = Dictionary::new();
    match read_material(&data.get_data_as_vec(), fmt, reader, data) {
        Some(mat) => {
            dict.insert("exists", true);
            dict.insert("material", mat);
            // dict.set(&"texture_reflection".into(), &mesh.1.to_variant());
        }
        None => {
            godot_print!("read_tmesh returned None");
            dict.insert("exists", false);
        }
    }
    dict
}
