use gdnative::prelude::*;
use libchum::scene;
use libchum::reader;
use libchum::scene::collada;
use crate::chumfile::ChumFile;
use crate::util;
use std::fs::File;

#[derive(NativeClass)]
#[inherit(Reference)]
pub struct SceneData {
    pub scene: scene::Scene
}

#[methods]
impl SceneData {
    fn new(_owner: &Reference) -> Self {
        SceneData {
            scene: scene::Scene::new_empty()
        }
    }

    #[export]
    pub fn add_mesh(&mut self, _owner: &Reference, node_name: GodotString, fh: Instance<ChumFile, Shared>, tx: Transform) {
        unsafe{fh.assume_safe()}.map(|chumfile, _res| {
            let mesh = chumfile.borrow_data(|mut data| {
                match reader::mesh::Mesh::read_data(&mut data, chumfile.get_format()) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("MESH file invalid: {}", err);
                    }
                }
            });
            // mesh.transform(&);
            let name = node_name.to_utf8().to_string();
            let mut trimesh = mesh.create_scene_mesh(name);
            trimesh.transform = util::godot_to_transform3d(&tx);
            self.scene.add_trimesh(trimesh);
        }).unwrap();
    }

    #[export]
    pub fn add_surface(&mut self, _owner: &Reference, node_name: GodotString, fh: Instance<ChumFile, Shared>, tx: Transform, quality: i64) {
        unsafe{fh.assume_safe()}.map(|chumfile, _res| {
            let surface = chumfile.borrow_data(|mut data| {
                match reader::surface::SurfaceObject::read_data(&mut data, chumfile.get_format()) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("SURFACE file invalid: {}", err);
                    }
                }
            });
            let name = node_name.to_utf8().to_string();
            let mut trimesh = surface.generate_trimesh(name, reader::surface::SurfaceGenMode::BezierInterp(quality as usize));
            trimesh.transform = util::godot_to_transform3d(&tx);
            self.scene.add_trimesh(trimesh);
        }).unwrap();
    }

    // #[export]
    // pub fn add_skin(&mut self, _owner: &Resource, node_name: GodotString, fh: Instance<ChumFile, Shared>, tx: Transform, quality: i64, emb)

    #[export]
    pub fn export_to(&self, _owner: &Reference, path: GodotString) {
        let mut buffer = File::create(path.to_utf8().as_str()).unwrap();
        collada::scene_to_writer_dae(&self.scene, &mut buffer).unwrap();
    }
}