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
    pub tris: Vec<common::Tri>,
}

pub struct Scene {
    pub trimeshes: Vec<SceneTriMesh>,
}
