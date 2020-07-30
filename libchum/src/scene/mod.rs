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
pub struct SceneGroup {
    pub name: String,
    pub transform: common::Mat4x4,
}

/// Skin for a scene object.
/// `groups` is the name of each group that this skin uses.
/// `vertices` corresponds to each of the vertices in the mesh.
#[derive(Clone, Debug)]
pub struct SceneSkin {
    pub groups: Vec<SceneGroup>,
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

pub fn merge_scene_skins(
    a: Option<SceneSkin>,
    b: Option<SceneSkin>,
    averts: usize,
    bverts: usize,
) -> Option<SceneSkin> {
    if a.is_none() && b.is_none() {
        return None;
    }
    let a = if let Some(mut skin) = a {
        skin.vertices.resize(averts, SceneSkinVertex::new_empty());
        skin
    } else {
        SceneSkin {
            groups: Vec::new(),
            vertices: vec![SceneSkinVertex::new_empty(); averts],
        }
    };
    let b = if let Some(mut skin) = b {
        skin.vertices.resize(bverts, SceneSkinVertex::new_empty());
        skin
    } else {
        SceneSkin {
            groups: Vec::new(),
            vertices: vec![SceneSkinVertex::new_empty(); bverts],
        }
    };
    let mut b_indices: Vec<usize> = Vec::new();
    let mut groups = a.groups;
    for group in b.groups {
        if let Some((i, _x)) = groups
            .iter()
            .enumerate()
            .find(|(_i, x)| x.name == group.name)
        {
            b_indices.push(i);
        } else {
            b_indices.push(groups.len());
            groups.push(group.clone());
        }
    }
    let mut vertices = a.vertices;
    for vert in b.vertices {
        vertices.push(SceneSkinVertex {
            influences: vert
                .influences
                .into_iter()
                .map(|inf| SceneSkinInfluence {
                    joint: b_indices[inf.joint],
                    weight: inf.weight,
                })
                .collect(),
        })
    }
    Some(SceneSkin { groups, vertices })
}

pub fn merge_meshes(mut a: SceneTriMesh, mut b: SceneTriMesh) -> SceneTriMesh {
    let averts = a.vertices.len();
    let bverts = b.vertices.len();
    let atex = a.texcoords.len();
    let anorm = a.normals.len();
    let skin = merge_scene_skins(a.skin, b.skin, averts, bverts);
    a.vertices.append(&mut b.vertices);
    a.texcoords.append(&mut b.texcoords);
    a.normals.append(&mut b.normals);
    a.elements.extend(b.elements.into_iter().map(|elem| {
        [
            (elem[0].0 + averts, elem[0].1 + atex, elem[0].2 + anorm),
            (elem[1].0 + averts, elem[1].1 + atex, elem[1].2 + anorm),
            (elem[2].0 + averts, elem[2].1 + atex, elem[2].2 + anorm),
        ]
    }));
    SceneTriMesh {
        name: a.name,
        transform: common::Mat4x4::identity(),
        vertices: a.vertices,
        texcoords: a.texcoords,
        normals: a.normals,
        elements: a.elements,
        skin,
    }
}

pub fn merge_mesh_vec(mut meshes: Vec<SceneTriMesh>) -> Option<SceneTriMesh> {
    if let Some(mut value) = meshes.pop() {
        while let Some(newvalue) = meshes.pop() {
            value = merge_meshes(newvalue, value);
        }
        Some(value)
    } else {
        None
    }
}

pub fn try_determine_group_transforms(mesh: &mut SceneTriMesh) {
    if let Some(ref mut skin) = mesh.skin {
        for (groupi, group) in skin.groups.iter_mut().enumerate() {
            let mut sum = common::Vector3::new(0.0, 0.0, 0.0);
            let mut sum_weight = 0.0;
            for (skinv, meshv) in skin.vertices.iter().zip(mesh.vertices.iter()) {
                for influence in skinv.influences.iter() {
                    if influence.joint == groupi {
                        sum += meshv * influence.weight;
                        sum_weight += influence.weight;
                    }
                }
            }
            if sum_weight > 1e-5 {
                group.transform = common::Mat4x4::new_translation(&(sum / sum_weight)).transpose();
                println!("{:?}", group.transform);
                println!("{:?}", group.transform.as_slice());
            } else {
                group.transform = common::Mat4x4::identity();
            }
        }
    }
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
