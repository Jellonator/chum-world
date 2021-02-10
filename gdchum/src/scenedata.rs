use crate::chumfile::ChumFile;
use crate::util;
use crate::ChumArchive;
use gdnative::prelude::*;
use libchum::binary::ChumBinary;
use libchum::reader;
use libchum::scene;

#[derive(NativeClass)]
#[inherit(Reference)]
pub struct SceneData {
    pub scene: scene::Scene,
}

pub fn get_node_name(mut name: &str) -> &str {
    if name.starts_with("DB:>") {
        name = &name[4..];
    }
    if let Some(pos) = name.rfind('>') {
        let (_name, e) = name.split_at(pos);
        let e = &e[1..];
        e
    } else {
        name
    }
}

pub fn get_node_path(mut name: &str) -> (Vec<&str>, &str) {
    if name.starts_with("DB:>") {
        name = &name[4..];
    }
    if let Some(pos) = name.rfind('>') {
        let (name, e) = name.split_at(pos);
        let e = &e[1..];
        (name.split('>').collect(), e)
    } else {
        (Vec::new(), name)
    }
}

#[methods]
impl SceneData {
    fn new(_owner: &Reference) -> Self {
        SceneData {
            scene: scene::Scene::new_empty(),
        }
    }

    #[export]
    pub fn add_mesh(
        &mut self,
        _owner: &Reference,
        node_name: GodotString,
        fh: Instance<ChumFile, Shared>,
        tx: Transform,
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
            let name = node_name.to_utf8().to_string();
            let mesh_name = chumfile.get_name_str();
            let (node_path, node_name) = get_node_path(&name);
            let parent_node = self.scene.node.make_parent_node(&node_path);
            let trimesh = mesh.create_scene_mesh();
            let node = scene::SNode {
                name: node_name.to_string(),
                children: Vec::new(),
                transform: util::godot_to_transform3d(&tx),
                visual_instance: Some(mesh_name.to_string()),
            };
            parent_node.children.push(node);
            self.scene.visual_instances.insert(
                mesh_name.to_string(),
                scene::SVisualInstance::Mesh { mesh: trimesh },
            );
        })
        .unwrap();
    }

    #[export]
    pub fn add_surface(
        &mut self,
        _owner: &Reference,
        node_name: GodotString,
        fh: Instance<ChumFile, Shared>,
        tx: Transform,
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
            let name = node_name.to_utf8().to_string();
            let (node_path, node_name) = get_node_path(&name);
            let parent_node = self.scene.node.make_parent_node(&node_path);
            let node = scene::SNode {
                name: node_name.to_string(),
                children: Vec::new(),
                transform: util::godot_to_transform3d(&tx),
                visual_instance: Some(surface_name.to_string()),
            };
            parent_node.children.push(node);
            self.scene.visual_instances.insert(
                surface_name.to_string(),
                scene::SVisualInstance::Mesh { mesh: trimesh },
            );
            /*let mut trimesh = surface.generate_trimesh(
                name,
                reader::surface::SurfaceGenMode::BezierInterp(quality as usize),
            );
            trimesh.transform = util::godot_to_transform3d(&tx);
            self.scene.add_trimesh(trimesh);*/
        })
        .unwrap();
    }

    #[export]
    pub fn add_skin(
        &mut self,
        _owner: &Reference,
        node_name: GodotString,
        fh: Instance<ChumFile, Shared>,
        tx: Transform,
        _quality: i64,
        include_skin: bool,
    ) {
        /*let fh = unsafe { fh.assume_safe() };
        fh.map(|chumfile, _res| {
            let skin = chumfile.borrow_data(|mut data| {
                match reader::skin::Skin::read_data(&mut data, chumfile.get_format()) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("SKIN file invalid: {}", err);
                    }
                }
            });
            let name = node_name.to_utf8().to_string();
            let archiveinstance = unsafe { chumfile.get_archive_instance().assume_safe() };
            let mut trimeshes = Vec::new();
            archiveinstance
                .map(|archive, archive_res| {
                    let names: Vec<String> = skin
                        .vertex_groups
                        .iter()
                        .map(|group| archive.maybe_get_name_from_hash_str(group.group_id))
                        .collect();
                    for meshid in skin.meshes.iter() {
                        if let Some(meshfile) = archive.get_file_from_hash(&archive_res, *meshid) {
                            let meshfile = unsafe { meshfile.assume_safe() };
                            meshfile
                                .map(|meshscript, _meshres| match meshscript.get_type_str() {
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
                                        let mut trimesh = mesh.create_scene_mesh(
                                            util::get_basename(meshscript.get_name_str())
            .to_owned(),
                                        );
                                        if include_skin {
                                            trimesh.skin = Some(skin.generate_scene_skin_for_mesh(
                                                names.as_slice(),
                                                *meshid,
                                                mesh.vertices.len(),
                                            ));
                                        }
                                        trimeshes.push(trimesh);
                                    }
                                    _ => {}
                                })
                                .unwrap();
                        } else {
                            godot_warn!("Mesh {} does not exist!", meshid);
                        }
                    }
                })
                .unwrap();
            if let Some(mut realmodel) = scene::merge_mesh_vec(trimeshes) {
                scene::try_determine_group_transforms(&mut realmodel);
                realmodel.transform = util::godot_to_transform3d(&tx);
                realmodel.name = name;
                self.scene.add_trimesh(realmodel);
            }
        })
        .unwrap();*/
    }

    #[export]
    pub fn add_lod(
        &mut self,
        owner: &Reference,
        node_name: GodotString,
        fh: Instance<ChumFile, Shared>,
        tx: Transform,
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
                            "SKIN" => self.add_skin(
                                owner,
                                node_name.clone(),
                                skin_fh,
                                tx,
                                quality,
                                include_skin,
                            ),
                            "MESH" => self.add_mesh(owner, node_name.clone(), skin_fh, tx),
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
        let archive = unsafe { archive.assume_safe() };
        archive
            .map(|inner, res| {
                for matid in self.scene.get_required_materials().iter() {
                    if self.scene.materials.contains_key(*matid) {
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
                                    self.scene.materials.insert(
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
                                            let material =
                                                match reader::material::Material::read_from(
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
                                            self.scene.materials.insert(
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
                for texture_id in self.scene.get_required_textures().iter() {
                    if self.scene.materials.contains_key(*texture_id) {
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
                            self.scene.textures.insert(
                                texture_data.get_name_str().to_owned(),
                                scene::STexture { data: texture },
                            );
                        })
                        .unwrap();
                }
            })
            .unwrap();
    }

    #[export]
    pub fn export_to(&self, _owner: &Reference, path: GodotString) {
        // add required materials
        // let mut buffer = File::create(path.to_utf8().as_str()).unwrap();
        // collada::scene_to_writer_dae(&self.scene, &mut buffer).unwrap();
        //let gltfroot = chumgltf::export_scene(&self.scene);
        // let writer = File::create(path.to_utf8().as_str()).expect("I/O Error");
        self.scene
            .export_to(path.to_utf8().as_str())
            .expect("Serialization Error");
        // gltf_json::serialize::to_writer_pretty(writer, &gltfroot).expect("Serialization Error");
    }
}
