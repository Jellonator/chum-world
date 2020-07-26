//! A module for defining scenes for file export.
//! Ideally, this will be a generic interface for export (and eventually import)
//! of 3D scenes.

use crate::common;
// use std::collections::HashMap;

pub mod collada;

#[derive(Clone, Debug)]
pub struct SceneTriMesh {
    pub name: String,
    pub transform: common::Mat4x4,
    pub vertices: Vec<common::Vector3>,
    pub texcoords: Vec<common::Vector2>,
    pub normals: Vec<common::Vector3>,
    pub elements: Vec<[(usize, usize, usize); 3]>, // pub tris: Vec<common::Tri>,
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub trimeshes: Vec<SceneTriMesh>,
}

impl Scene {
    pub fn new_empty() -> Scene {
        Scene {
            trimeshes: Vec::new(),
        }
    }

    pub fn add_trimesh(&mut self, mesh: SceneTriMesh) {
        self.trimeshes.push(mesh)
    }
}
