use crate::chumfile::ChumFile;
use crate::util;
use gdnative::prelude::*;
use libchum::reader;
use libchum::scene;
use libchum::scene::collada;
use std::fs::File;

#[derive(NativeClass)]
#[inherit(Reference)]
pub struct SceneData {
    pub scene: scene::Scene,
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
            let mut trimesh = mesh.create_scene_mesh(name);
            trimesh.transform = util::godot_to_transform3d(&tx);
            self.scene.add_trimesh(trimesh);
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
            let name = node_name.to_utf8().to_string();
            let mut trimesh = surface.generate_trimesh(
                name,
                reader::surface::SurfaceGenMode::BezierInterp(quality as usize),
            );
            trimesh.transform = util::godot_to_transform3d(&tx);
            self.scene.add_trimesh(trimesh);
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
        let fh = unsafe { fh.assume_safe() };
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
        .unwrap();
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
    pub fn export_to(&self, _owner: &Reference, path: GodotString) {
        let mut buffer = File::create(path.to_utf8().as_str()).unwrap();
        collada::scene_to_writer_dae(&self.scene, &mut buffer).unwrap();
    }
}
