use crate::chumfile::ChumFile;
use crate::util;
use crate::ChumArchive;
use gdnative::prelude::*;
use libchum::binary::ChumBinary;
use libchum::reader;
use libchum::scene;
use libchum::util as chumutil;
use std::collections::HashMap;

#[derive(NativeClass)]
#[inherit(Reference)]
pub struct SceneData {
    pub scene: scene::Scene,
}

pub fn add_required_materials(scene: &mut scene::Scene, archive: Instance<ChumArchive, Shared>) {
    let archive = unsafe { archive.assume_safe() };
    archive
        .map(|inner, res| {
            for matid in scene.get_required_materials().iter() {
                if scene.materials.contains_key(*matid) {
                    continue;
                }
                let materialanim_fh = match inner.get_file_from_hash(res.as_ref(), *matid) {
                    Some(x) => x,
                    None => continue,
                };
                // Can be MATERIAL or MATERIALANIM
                let materialanim_fh = unsafe { materialanim_fh.assume_safe() };
                materialanim_fh
                    .map(|materialanim_data, _materialanim_res| {
                        match materialanim_data.get_type_str() {
                            "MATERIAL" => {
                                let material = match reader::material::Material::read_from(
                                    &mut materialanim_data.get_data_as_vec().as_slice(),
                                    materialanim_data.get_format(),
                                ) {
                                    Ok(x) => x,
                                    Err(err) => {
                                        panic!("MATERIAL file invalid: {}", err);
                                    }
                                };
                                scene.materials.insert(
                                    materialanim_data.get_name_str().to_owned(),
                                    scene::SMaterial::from_material(&material),
                                );
                            }
                            "MATERIALANIM" => {
                                let materialanim =
                                    match reader::materialanim::MaterialAnimation::read_from(
                                        &mut materialanim_data.get_data_as_vec().as_slice(),
                                        materialanim_data.get_format(),
                                    ) {
                                        Ok(x) => x,
                                        Err(err) => {
                                            panic!("MATERIALANIM file invalid: {}", err);
                                        }
                                    };
                                let material_id = materialanim.material_id;
                                let material_fh =
                                    match inner.get_file_from_hash(res.as_ref(), material_id) {
                                        Some(x) => x,
                                        None => return,
                                    };
                                let material_fh = unsafe { material_fh.assume_safe() };
                                material_fh
                                    .map(|material_data, _material_res| {
                                        let material = match reader::material::Material::read_from(
                                            &mut material_data.get_data_as_vec().as_slice(),
                                            material_data.get_format(),
                                        ) {
                                            Ok(x) => x,
                                            Err(err) => {
                                                panic!("MATERIAL file invalid: {}", err);
                                            }
                                        };
                                        // have to add it under the MATERIALANIM's name
                                        // for it to be recognized
                                        scene.materials.insert(
                                            materialanim_data.get_name_str().to_owned(),
                                            scene::SMaterial::from_material(&material),
                                        );
                                    })
                                    .unwrap();
                            }
                            o => panic!("INVALID: {}", o),
                        }
                    })
                    .unwrap();
            }
            for texture_id in scene.get_required_textures().iter() {
                if scene.materials.contains_key(*texture_id) {
                    continue;
                }
                let texture_fh = match inner.get_file_from_hash(res.as_ref(), *texture_id) {
                    Some(x) => x,
                    None => return,
                };
                let texture_fh = unsafe { texture_fh.assume_safe() };
                texture_fh
                    .map(|texture_data, _texture_res| {
                        let texture = match reader::bitmap::Bitmap::read_from(
                            &mut texture_data.get_data_as_vec().as_slice(),
                            texture_data.get_format(),
                        ) {
                            Ok(x) => x,
                            Err(err) => {
                                panic!("BITMAP file invalid: {}", err);
                            }
                        };
                        scene.textures.insert(
                            texture_data.get_name_str().to_owned(),
                            scene::STexture { data: texture },
                        );
                    })
                    .unwrap();
            }
        })
        .unwrap();
}

#[methods]
impl SceneData {
    fn new(_owner: &Reference) -> Self {
        SceneData {
            scene: scene::Scene::new_empty(),
        }
    }

    #[export]
    pub fn has_node(&self, _owner: &Reference, path: Vec<String>) -> bool {
        self.scene.root.get_node_ref(path.as_slice()).is_some()
    }

    #[export]
    pub fn add_node(
        &mut self,
        _owner: &Reference,
        path: Vec<String>,
        name: String,
        local_transform: Transform,
    ) {
        let base = self.scene.root.get_node_mut(path.as_slice()).unwrap();
        base.tree.insert(
            name,
            scene::SNode {
                tree: HashMap::new(),
                transform: util::godot_to_transform3d(&local_transform),
                graphic: scene::NodeGraphic::None,
            },
        );
    }

    #[export]
    pub fn set_node_mesh(
        &mut self,
        _owner: &Reference,
        node_path: Vec<String>,
        fh: Instance<ChumFile, Shared>,
    ) {
        let fh = unsafe { fh.assume_safe() };
        fh.map(|chumfile, _res| {
            let mesh = chumfile.borrow_data(|mut data| {
                match reader::mesh::Mesh::read_data(&mut data, chumfile.get_format()) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("MESH file invalid: {}", err);
                    }
                }
            });
            let mesh_name = chumfile.get_name_str();
            let trimesh = mesh.create_scene_mesh();
            let node = self.scene.root.get_node_mut(node_path.as_slice()).unwrap();
            node.graphic = scene::NodeGraphic::Mesh {
                mesh: mesh_name.to_string(),
            };
            self.scene.meshes.insert(mesh_name.to_string(), trimesh);
        })
        .unwrap();
    }

    #[export]
    pub fn set_node_surface(
        &mut self,
        _owner: &Reference,
        node_path: Vec<String>,
        fh: Instance<ChumFile, Shared>,
        quality: i64,
    ) {
        let fh = unsafe { fh.assume_safe() };
        fh.map(|chumfile, _res| {
            let surface = chumfile.borrow_data(|mut data| {
                match reader::surface::SurfaceObject::read_data(&mut data, chumfile.get_format()) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("SURFACE file invalid: {}", err);
                    }
                }
            });
            let trimesh = surface.generate_simple_mesh(
                reader::surface::SurfaceGenMode::BezierInterp(quality as usize),
            );
            let surface_name = chumfile.get_name_str();
            let node = self.scene.root.get_node_mut(node_path.as_slice()).unwrap();
            node.graphic = scene::NodeGraphic::Mesh {
                mesh: surface_name.to_string(),
            };
            self.scene.meshes.insert(surface_name.to_string(), trimesh);
        })
        .unwrap();
    }

    #[export]
    pub fn set_node_skin(
        &mut self,
        _owner: &Reference,
        node_path: Vec<String>,
        fh: Instance<ChumFile, Shared>,
        _quality: i64,
        _include_skin: bool,
    ) {
        let fh = unsafe { fh.assume_safe() };
        let node = self.scene.root.get_node_mut(node_path.as_slice()).unwrap();
        // let visual_instances = &mut self.scene.visual_instances;
        // let skins = &mut self.scene.skins;
        let meshes = &mut self.scene.meshes;
        fh.map(|chumfile, _res| {
            let skin = chumfile.borrow_data(|mut data| {
                match reader::skin::Skin::read_data(&mut data, chumfile.get_format()) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("SKIN file invalid: {}", err);
                    }
                }
            });
            // add skin
            let archiveinstance = unsafe { chumfile.get_archive_instance().assume_safe() };
            //let mut trimeshes = Vec::new();
            archiveinstance
                .map(|archive, archive_res| {
                    let mut skin_meshes = Vec::new();
                    // create a child for each mesh in skin
                    for mesh_id in skin.meshes.iter() {
                        if let Some(meshfile) = archive.get_file_from_hash(&archive_res, *mesh_id) {
                            let meshfile = unsafe { meshfile.assume_safe() };
                            meshfile
                                .map(|meshscript, _meshres| {
                                    match meshscript.get_type_str() {
                                        "MESH" => {
                                            let mesh = match reader::mesh::Mesh::read_data(
                                                &mut meshscript.get_data_as_vec(),
                                                meshscript.get_format(),
                                            ) {
                                                Ok(x) => x,
                                                Err(err) => {
                                                    panic!("MESH file invalid: {}", err);
                                                }
                                            };
                                            let new_name = format!(
                                                "{}.skin{}",
                                                meshscript.get_name_str(),
                                                chumfile.get_hash_id_ownerless()
                                            );
                                            let new_name_hash = chumutil::hash_name_i32(&new_name);
                                            // add mesh if not exists
                                            if !meshes.contains_key(new_name_hash) {
                                                let mut trimesh = mesh.create_scene_mesh();
                                                trimesh.skin = mesh.generate_mesh_skin(
                                                    reader::skin::SkinInfo {
                                                        names: archive.get_name_map(),
                                                        skin: &skin,
                                                        skin_id: chumfile.get_hash_id_ownerless(),
                                                        mesh_id: *mesh_id,
                                                    },
                                                );
                                                meshes.insert(new_name.to_string(), trimesh);
                                            }
                                            skin_meshes.push(new_name);
                                        }
                                        _ => {}
                                    }
                                })
                                .unwrap();
                        } else {
                            godot_warn!("Mesh {} does not exist!", mesh_id);
                        }
                    }
                    let mut scene_skin = scene::Skin {
                        joints: skin.generate_scene_skin_joints(archive.get_name_map()),
                    };
                    scene_skin.auto_set_joint_transforms(skin_meshes.iter().filter_map(|id| {
                        meshes
                            .get(chumutil::hash_name_i32(&id))
                            .map(|x| x.get_value_ref())
                    }));
                    // add node
                    node.graphic = scene::NodeGraphic::Skin {
                        skin: scene_skin,
                        meshes: skin_meshes,
                    };
                })
                .unwrap();
        })
        .unwrap();
    }

    #[export]
    pub fn set_node_lod(
        &mut self,
        owner: &Reference,
        node_path: Vec<String>,
        fh: Instance<ChumFile, Shared>,
        quality: i64,
        include_skin: bool,
    ) {
        use libchum::binary::ChumBinary;
        let fh = unsafe { fh.assume_safe() };
        fh.map(|chumfile, _res| {
            let lod = chumfile.borrow_data(|mut data| {
                match reader::lod::Lod::read_from(&mut data, chumfile.get_format()) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("LOD file invalid: {}", err);
                    }
                }
            });
            let archive = unsafe { chumfile.get_archive_instance().assume_safe() };
            archive
                .map(|inner, res| {
                    for skin_id in lod.skin_ids.iter() {
                        let skin_fh = inner.get_file_from_hash(res.as_ref(), *skin_id).unwrap();
                        let skin_type: String = unsafe {
                            skin_fh
                                .assume_safe()
                                .map(|skin_inner, _res| skin_inner.get_type_str().to_owned())
                        }
                        .unwrap();
                        match skin_type.as_str() {
                            "SKIN" => self.set_node_skin(
                                owner,
                                node_path.clone(),
                                skin_fh,
                                quality,
                                include_skin,
                            ),
                            "MESH" => self.set_node_mesh(owner, node_path.clone(), skin_fh),
                            _ => {}
                        };
                    }
                })
                .unwrap()
        })
        .unwrap();
    }

    #[export]
    pub fn add_required_materials(
        &mut self,
        _owner: &Reference,
        archive: Instance<ChumArchive, Shared>,
    ) {
        add_required_materials(&mut self.scene, archive);
    }

    #[export]
    pub fn export_to(&self, _owner: &Reference, path: GodotString) {
        self.scene
            .export_to(path.to_utf8().as_str())
            .expect("Serialization Error");
    }
}
