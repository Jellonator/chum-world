//! A module for defining scenes for file export.
//! Ideally, this will be a generic interface for export (and eventually import)
//! of 3D scenes.

use crate::common;
use crate::reader;
use crate::util::idmap::IdMap;
use std::collections::HashMap;

pub mod gltf;

/// A simple triangle mesh that strips away non-exportable data
#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<common::Vector3>,
    pub texcoords: Vec<common::Vector2>,
    pub normals: Vec<common::Vector3>,
    pub data: MeshFormat,
}

/// The internal format of the Mesh
#[derive(Clone, Debug)]
pub enum MeshFormat {
    Triangles {
        data: HashMap<i32, Vec<MeshTriangle>>,
    },
    Strips {
        strips: Vec<MeshStrip>,
    },
}

/// A single triangle in a mesh
#[derive(Clone, Debug)]
pub struct MeshTriangle {
    pub corners: [MeshPoint; 3],
}

/// A simpler version of StripData
#[derive(Clone, Debug)]
pub struct MeshStrip {
    pub elements: Vec<MeshPoint>,
    pub material: i32,
    pub tri_order: common::TriStripOrder,
}

impl MeshStrip {
    pub fn iterate_triangles<'a>(
        &'a self,
    ) -> std::iter::Map<
        std::iter::Zip<
            std::ops::Range<usize>,
            std::iter::Repeat<(&Vec<MeshPoint>, common::TriStripOrder)>,
        >,
        fn((usize, (&Vec<MeshPoint>, common::TriStripOrder))) -> [&MeshPoint; 3],
    > {
        let n = self.elements.len();
        let v = &self.elements;
        let o = self.tri_order;
        // have to start at two because 0..(n-2) has the (very unlikely) chance to overflow
        (2..n).zip(std::iter::repeat((v, o))).map(|(i, (v, o))| {
            let i1 = i-2;
            let a = (i1 % 2) + 1;
            let b = 3 - a;
            let (i2, i3) = match o {
                common::TriStripOrder::ClockWise => (i1 + a, i1 + b),
                common::TriStripOrder::CounterClockWise => (i1 + b, i1 + a),
            };
            [&v[i1], &v[i2], &v[i3]]
        })
    }
}

/// A single point in a Mesh
#[derive(Clone, Debug)]
pub struct MeshPoint {
    pub vertex_id: u32,
    pub texcoord_id: u32,
    pub normal_id: u32,
}

/// A single texture
#[derive(Clone)]
pub struct STexture {
    pub data: reader::bitmap::Bitmap,
}

/// A single material
#[derive(Clone, Debug)]
pub struct SMaterial {
    pub texture: Option<String>,
    pub alpha: f32,
    pub diffuse: common::Vector3,
    pub emission: common::Vector3,
    pub transform: common::Transform2D,
}

/// A single visual instance
#[derive(Clone)]
pub enum SVisualInstance {
    Mesh { mesh: Mesh },
    /*Surface {
        surface: reader::surface::SurfaceObject
    }*/
}

/// A single node
#[derive(Clone, Debug)]
pub struct SNode {
    pub name: String,
    pub children: Vec<SNode>,
    pub transform: common::Transform3D,
    pub visual_instance: Option<String>,
}

impl SNode {
    pub fn new(name: String) -> SNode {
        SNode {
            name,
            children: Vec::new(),
            transform: common::Transform3D::identity(),
            visual_instance: None,
        }
    }
}

/// A full scene
#[derive(Clone)]
pub struct Scene {
    pub textures: IdMap<STexture>,
    pub materials: IdMap<SMaterial>,
    pub visual_instances: IdMap<SVisualInstance>,
    pub node: SNode,
}

impl Scene {
    pub fn new_empty() -> Scene {
        Scene {
            textures: IdMap::new(),
            materials: IdMap::new(),
            visual_instances: IdMap::new(),
            node: SNode::new("".to_string()),
        }
    }
}

/*

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
    pub transform: common::Transform3D,
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
pub struct SceneTriMeshElement {
    pub vertex: usize,
    pub texcoord: usize,
    pub normal: usize
}

#[derive(Clone, Debug)]
pub struct SceneTriMeshMaterial {
    pub material: String,
    pub elements: Vec<[SceneTriMeshElement; 3]>
}

#[derive(Clone, Debug)]
pub struct SceneTriMesh {
    pub name: String,
    pub transform: common::Transform3D,
    pub vertices: Vec<common::Vector3>,
    pub texcoords: Vec<common::Vector2>,
    pub normals: Vec<common::Vector3>,
    pub materials: Vec<SceneTriMeshMaterial>,
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
    for material in b.materials.iter_mut() {
        for element in material.elements.iter_mut() {
            for e in element.iter_mut() {
                e.vertex += averts;
                e.texcoord += atex;
                e.normal += anorm;
            }
        }
    }
    // Sort in reverse order that way popping is in normal order
    a.materials.sort_unstable_by(|a, b| b.material.cmp(&a.material));
    b.materials.sort_unstable_by(|a, b| b.material.cmp(&a.material));
    let mut materials = Vec::new();
    while a.materials.len() > 0 && b.materials.len() > 0 {
        let a_last = a.materials.last().unwrap();
        let b_last = b.materials.last().unwrap();
        if a_last.material == b_last.material {
            let mut a_value = a.materials.pop().unwrap();
            let b_value = b.materials.pop().unwrap();
            a_value.elements.extend(b_value.elements.into_iter());
            materials.push(
                SceneTriMeshMaterial {
                    material: a_value.material,
                    elements: a_value.elements
                }
            );
        } else if a_last.material < b_last.material {
            // A comes first, so push that
            materials.push(a.materials.pop().unwrap());
        } else {
            materials.push(b.materials.pop().unwrap());
        }
    }
    materials.append(&mut a.materials);
    materials.append(&mut b.materials);
    SceneTriMesh {
        name: a.name,
        transform: common::Transform3D::identity(),
        vertices: a.vertices,
        texcoords: a.texcoords,
        normals: a.normals,
        materials,
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
                        sum += *meshv * influence.weight;
                        sum_weight += influence.weight;
                    }
                }
            }
            if sum_weight > 1e-5 {
                let offset = sum / sum_weight;
                group.transform = common::Transform3D::translation(offset.x, offset.y, offset.z);
                println!("{:?}", group.transform);
                println!("{:?}", group.transform.to_array());
            } else {
                group.transform = common::Transform3D::identity();
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub trimeshes: Vec<SceneTriMesh>,
    pub materials: Vec<SceneMaterial>
}

impl Scene {
    pub fn new_empty() -> Scene {
        Scene {
            trimeshes: Vec::new(),
            materials: Vec::new()
        }
    }

    pub fn add_trimesh(&mut self, mesh: SceneTriMesh) {
        self.trimeshes.push(mesh)
    }
}

#[derive(Clone, Debug)]
pub struct SceneMaterial {
    pub texture: Option<String>,
    pub alpha: f32,
    pub diffuse: common::Vector3,
    pub emission: common::Vector3,
    pub transform: common::Transform2D
}
*/
