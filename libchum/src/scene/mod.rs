//! A module for defining scenes for file export.
//! Ideally, this will be a generic interface for export (and eventually import)
//! of 3D scenes.

use crate::common;
use crate::gltf as gltf_rs;
use crate::reader;
use crate::util::idmap::IdMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub mod gltf;

/// Skin join
#[derive(Clone, Debug)]
pub struct SkinJoint {
    pub transform: common::Transform3D, // inverse bind matrix is calculated on write
    pub name: String,
}

/// Skinning data per vertex
#[derive(Clone, Debug)]
pub struct SkinVertex {
    pub elements: [SkinVertexElement; 4],
}

impl Default for SkinVertex {
    fn default() -> SkinVertex {
        SkinVertex {
            elements: [
                SkinVertexElement::default(),
                SkinVertexElement::default(),
                SkinVertexElement::default(),
                SkinVertexElement::default(),
            ],
        }
    }
}

impl SkinVertex {
    pub fn length(&self) -> f32 {
        self.elements.iter().map(|x| x.weight).sum()
    }

    pub fn normalize(&mut self) {
        let sum_weight: f32 = self.elements.iter().map(|x| x.weight).sum();
        if sum_weight <= 1e-10 {
            return;
        }
        for element in self.elements.iter_mut() {
            element.weight = element.weight / sum_weight;
        }
    }

    fn find_insert_index(&self, weight: f32) -> Option<usize> {
        for i in 0..4 {
            if weight > self.elements[i].weight {
                return Some(i);
            }
        }
        None
    }

    pub fn push_element(&mut self, element: SkinVertexElement) {
        if let Some(idx) = self.find_insert_index(element.weight) {
            // push elements out of the way
            for i in (idx..3).rev() {
                self.elements[i + 1] = self.elements[i];
            }
            // add new element
            self.elements[idx] = element;
        }
    }

    pub fn get_weight_array(&self) -> [f32; 4] {
        [
            self.elements[0].weight,
            self.elements[1].weight,
            self.elements[2].weight,
            self.elements[3].weight,
        ]
    }

    pub fn get_joint_array(&self) -> [u16; 4] {
        [
            self.elements[0].joint,
            self.elements[1].joint,
            self.elements[2].joint,
            self.elements[3].joint,
        ]
    }
}

/// A single element in a skin vertex
#[derive(Clone, Debug, Copy)]
pub struct SkinVertexElement {
    pub joint: u16,
    pub weight: f32,
}

impl Default for SkinVertexElement {
    fn default() -> SkinVertexElement {
        SkinVertexElement {
            joint: 0,
            weight: 0.0,
        }
    }
}

/// A skin
#[derive(Clone, Debug)]
pub struct Skin {
    pub joints: Vec<SkinJoint>, // skins in TotemTech have no hierarchy
}

impl Skin {
    pub fn auto_set_joint_transforms<'a, I>(&mut self, meshes: I)
    where
        I: Iterator<Item = &'a Mesh>,
    {
        let mut centers = vec![common::Vector3::zero(); self.joints.len()];
        let mut weight_sums = vec![0.0f32; self.joints.len()];
        for mesh in meshes {
            if let Some(mesh_skin) = mesh.skin.as_ref() {
                for (vert_i, vert) in mesh_skin.vertices.iter().enumerate() {
                    for elem in vert.elements.iter() {
                        centers[elem.joint as usize] += mesh.vertices[vert_i] * elem.weight;
                        weight_sums[elem.joint as usize] += elem.weight;
                    }
                }
            }
        }
        for (i, joint) in self.joints.iter_mut().enumerate() {
            let mut c = centers[i];
            if weight_sums[i] > 1e-5 {
                c = c / weight_sums[i];
            }
            joint.transform = common::Transform3D::translation(c.x, c.y, c.z);
        }
    }
}

/// A mesh's skinning information
#[derive(Clone, Debug)]
pub struct MeshSkin {
    pub vertices: Vec<SkinVertex>,
}

/// A simple triangle mesh that strips away non-exportable data
#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<common::Vector3>,
    pub texcoords: Vec<common::Vector2>,
    pub normals: Vec<common::Vector3>,
    pub triangles: HashMap<i32, Vec<MeshTriangle>>,
    pub skin: Option<MeshSkin>,
}

impl Mesh {
    pub fn new_empty() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            texcoords: Vec::new(),
            normals: Vec::new(),
            triangles: HashMap::new(),
            skin: None,
        }
    }
}

/// A single triangle in a mesh
#[derive(Clone, Debug)]
pub struct MeshTriangle {
    pub corners: [MeshPoint; 3],
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
    pub texture: Option<i32>,
    pub alpha: f32,
    pub diffuse: common::Vector3,
    pub emission: common::Vector3,
    pub transform: common::Transform2D,
}

impl SMaterial {
    pub fn from_material(mat: &reader::material::Material) -> SMaterial {
        SMaterial {
            texture: match mat.texture {
                0 => None,
                value => Some(value),
            },
            alpha: mat.color.a,
            diffuse: common::Vector3::new(mat.color.r, mat.color.g, mat.color.b),
            emission: mat.emission,
            transform: mat.transform,
        }
    }
}

/// A single visual instance
#[derive(Clone, Debug)]
pub enum NodeGraphic {
    Mesh { mesh: String },
    Skin { skin: Skin, meshes: Vec<String> },
    None,
}

/// A single node
#[derive(Clone, Debug)]
pub struct SNode {
    pub tree: HashMap<String, SNode>,
    pub transform: common::Transform3D,
    pub graphic: NodeGraphic,
}

impl SNode {
    pub fn new() -> SNode {
        SNode {
            tree: HashMap::new(),
            transform: common::Transform3D::identity(),
            graphic: NodeGraphic::None,
        }
    }

    pub fn get_node_ref<S: AsRef<str>>(&self, path: &[S]) -> Option<&SNode> {
        if let Some(first) = path.first() {
            if let Some(child) = self.tree.get(first.as_ref()) {
                child.get_node_ref(&path[1..])
            } else {
                None
            }
        } else {
            Some(self)
        }
    }

    pub fn get_node_mut<S: AsRef<str>>(&mut self, path: &[S]) -> Option<&mut SNode> {
        if let Some(first) = path.first() {
            if let Some(child) = self.tree.get_mut(first.as_ref()) {
                child.get_node_mut(&path[1..])
            } else {
                None
            }
        } else {
            Some(self)
        }
    }
}

/// A full scene
#[derive(Clone)]
pub struct Scene {
    pub textures: IdMap<STexture>,
    pub materials: IdMap<SMaterial>,
    pub meshes: IdMap<Mesh>,
    pub root: SNode,
}

impl Scene {
    pub fn new_empty() -> Scene {
        Scene {
            textures: IdMap::new(),
            materials: IdMap::new(),
            meshes: IdMap::new(),
            root: SNode::new(),
        }
    }

    pub fn export_to(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::borrow::Cow;
        let writer = fs::File::create(name)?;
        if name.to_lowercase().ends_with("gltf") {
            let (gltfroot, _buffer) = gltf::export_scene(&self, false);
            gltf_json::serialize::to_writer_pretty(writer, &gltfroot)?;
        } else {
            let (gltfroot, buffer) = gltf::export_scene(&self, true);
            let json_string = gltf_json::serialize::to_string(&gltfroot)?;
            let mut json_offset = json_string.len() as u32;
            json_offset = (json_offset + 3) & !3; // align to multiple of 3
            let glb = gltf_rs::binary::Glb {
                header: gltf_rs::binary::Header {
                    magic: b"glTF".clone(),
                    version: 2,
                    length: json_offset + buffer.len() as u32,
                },
                bin: Some(Cow::Owned(buffer)),
                json: Cow::Owned(json_string.into_bytes()),
            };
            glb.to_writer(writer)?;
        }
        Ok(())
    }

    pub fn get_required_materials(&self) -> HashSet<i32> {
        let mut v = HashSet::new();
        for (_i, value) in self.meshes.iter() {
            v.extend(value.get_value_ref().triangles.keys());
        }
        v
    }

    pub fn get_required_textures(&self) -> HashSet<i32> {
        let mut v = HashSet::new();
        for (_i, mat) in self.materials.iter() {
            if let Some(id) = mat.get_value_ref().texture.as_ref() {
                v.insert(*id);
            }
        }
        v
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
