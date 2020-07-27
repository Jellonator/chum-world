//! A module for defining scenes for file export.
//! Ideally, this will be a generic interface for export (and eventually import)
//! of 3D scenes.

use crate::common;
// use std::collections::HashMap;

pub mod collada;

/// A single influence on a vertex.
/// `joint` refers to a group in `SceneSkin.groups`.
/// `weight` is the weighted influence that this influence has.
#[derive(Clone, Debug)]
pub struct SceneSkinInfluence {
    pub joint: usize,
    pub weight: f32,
}

/// Every vertex has a list of influences.
#[derive(Clone, Debug)]
pub struct SceneSkinVertex {
    pub influences: Vec<SceneSkinInfluence>,
}

impl SceneSkinVertex {
    pub fn new_empty() -> SceneSkinVertex {
        SceneSkinVertex {
            influences: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SceeneGroup {
    pub name: String,
    pub transform: common::Mat4x4
}

/// Skin for a scene object.
/// `groups` is the name of each group that this skin uses.
/// `vertices` corresponds to each of the vertices in the mesh.
#[derive(Clone, Debug)]
pub struct SceneSkin {
    pub groups: Vec<String>,
    pub vertices: Vec<SceneSkinVertex>,
}

#[derive(Clone, Debug)]
pub struct SceneTriMesh {
    pub name: String,
    pub transform: common::Mat4x4,
    pub vertices: Vec<common::Vector3>,
    pub texcoords: Vec<common::Vector2>,
    pub normals: Vec<common::Vector3>,
    pub elements: Vec<[(usize, usize, usize); 3]>,
    pub skin: Option<SceneSkin>,
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
