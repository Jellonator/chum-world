use crate::common::*;
use crate::scene;
use crate::util;
use bitflags::bitflags;
use gltf;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone)]
struct MeshBuilder {
    pub vertices: Vec<Vector3>,
    pub texcoords: Vec<Vector2>,
    pub normals: Vec<Vector3>,
    pub vertex_ids: HashMap<[u32; 3], usize>,
    pub texcoord_ids: HashMap<[u32; 2], usize>,
    pub normal_ids: HashMap<[u32; 3], usize>,
    pub triangles: HashMap<i32, Vec<scene::MeshTriangle>>,
}

impl MeshBuilder {
    pub fn new() -> MeshBuilder {
        MeshBuilder {
            vertices: Vec::new(),
            texcoords: Vec::new(),
            normals: Vec::new(),
            vertex_ids: HashMap::new(),
            texcoord_ids: HashMap::new(),
            normal_ids: HashMap::new(),
            triangles: HashMap::new(),
        }
    }
    pub fn commit(self) -> scene::Mesh {
        scene::Mesh {
            vertices: self.vertices,
            normals: self.normals,
            texcoords: self.texcoords,
            triangles: self.triangles,
            skin: None,
        }
    }
    pub fn get_vertex_index(&mut self, v: Vector3) -> usize {
        let key = reinterpret_vec3(&v);
        match self.vertex_ids.get(&key) {
            Some(value) => *value,
            None => {
                let value = self.vertices.len();
                self.vertices.push(v);
                self.vertex_ids.insert(key, value);
                value
            }
        }
    }
    pub fn get_normal_index(&mut self, v: Vector3) -> usize {
        let key = reinterpret_vec3(&v);
        match self.normal_ids.get(&key) {
            Some(value) => *value,
            None => {
                let value = self.normals.len();
                self.normals.push(v);
                self.normal_ids.insert(key, value);
                value
            }
        }
    }
    pub fn get_texcoord_index(&mut self, v: Vector2) -> usize {
        let key = reinterpret_vec2(&v);
        match self.texcoord_ids.get(&key) {
            Some(value) => *value,
            None => {
                let value = self.texcoords.len();
                self.texcoords.push(v);
                self.texcoord_ids.insert(key, value);
                value
            }
        }
    }
}

struct PrimitiveData {
    pub vertices: Vec<Vector3>,
    pub texcoords: Vec<Vector2>,
    pub normals: Vec<Vector3>,
    pub indices: Vec<u32>,
}

fn get_data(p: &gltf::Primitive, buffers: &[gltf::buffer::Data]) -> PrimitiveData {
    use gltf::mesh::util::{ReadIndices, ReadTexCoords};
    let reader = p.reader(|buf| buffers.get(buf.index()).map(|x| x.0.as_slice()));
    let positions: Vec<Vector3> = reader
        .read_positions()
        .map(|x| x.map(|v| Vector3::new(v[0], v[1], v[2])).collect())
        .unwrap_or(Vec::new());
    let texcoords: Vec<Vector2> = reader
        .read_tex_coords(0)
        .map(|x| match x {
            ReadTexCoords::U8(data) => data
                .map(|v| Vector2::new(v[0] as f32 / 255.0, v[1] as f32 / 255.0))
                .collect(),
            ReadTexCoords::U16(data) => data
                .map(|v| Vector2::new(v[0] as f32 / 65535.0, v[1] as f32 / 65535.0))
                .collect(),
            ReadTexCoords::F32(data) => data.map(|v| Vector2::new(v[0], v[1])).collect(),
        })
        .unwrap_or(Vec::new());
    let normals: Vec<Vector3> = reader
        .read_normals()
        .map(|x| x.map(|v| Vector3::new(v[0], v[1], v[2])).collect())
        .unwrap_or(Vec::new());
    let indices: Vec<u32> = reader
        .read_indices()
        .map(|x| match x {
            ReadIndices::U8(data) => data.map(|i| i as u32).collect(),
            ReadIndices::U16(data) => data.map(|i| i as u32).collect(),
            ReadIndices::U32(data) => data.collect(),
        })
        .unwrap_or(Vec::new());
    PrimitiveData {
        vertices: positions,
        texcoords,
        normals,
        indices,
    }
}

fn push_primitive_triangles(
    builder: &mut MeshBuilder,
    p: &gltf::Primitive,
    buffers: &[gltf::buffer::Data],
) {
    let data = get_data(p, buffers);
    let mat_id = p
        .material()
        .name()
        .map(|x| util::hash_name_i32(x))
        .unwrap_or(p.material().index().unwrap_or(0) as i32);
    for tri in data.indices.chunks_exact(3) {
        let tri = scene::MeshTriangle {
            corners: [
                scene::MeshPoint {
                    vertex_id: builder.get_vertex_index(data.vertices[tri[0] as usize]) as u32,
                    texcoord_id: builder.get_texcoord_index(data.texcoords[tri[0] as usize]) as u32,
                    normal_id: builder.get_normal_index(data.normals[tri[0] as usize]) as u32,
                },
                scene::MeshPoint {
                    vertex_id: builder.get_vertex_index(data.vertices[tri[1] as usize]) as u32,
                    texcoord_id: builder.get_texcoord_index(data.texcoords[tri[1] as usize]) as u32,
                    normal_id: builder.get_normal_index(data.normals[tri[1] as usize]) as u32,
                },
                scene::MeshPoint {
                    vertex_id: builder.get_vertex_index(data.vertices[tri[2] as usize]) as u32,
                    texcoord_id: builder.get_texcoord_index(data.texcoords[tri[2] as usize]) as u32,
                    normal_id: builder.get_normal_index(data.normals[tri[2] as usize]) as u32,
                },
            ],
        };
        builder.triangles.entry(mat_id).or_default().push(tri);
    }
}

fn push_primitive_strip(
    builder: &mut MeshBuilder,
    p: &gltf::Primitive,
    buffers: &[gltf::buffer::Data],
) {
    let data = get_data(p, buffers);
    let mat_id = p
        .material()
        .name()
        .map(|x| util::hash_name_i32(x))
        .unwrap_or(p.material().index().unwrap_or(0) as i32);
    if data.indices.len() < 3 {
        return;
    }
    let mut c1 = scene::MeshPoint {
        vertex_id: builder.get_vertex_index(data.vertices[data.indices[0] as usize]) as u32,
        texcoord_id: builder.get_texcoord_index(data.texcoords[data.indices[0] as usize]) as u32,
        normal_id: builder.get_normal_index(data.normals[data.indices[0] as usize]) as u32,
    };
    let mut c2 = scene::MeshPoint {
        vertex_id: builder.get_vertex_index(data.vertices[data.indices[1] as usize]) as u32,
        texcoord_id: builder.get_texcoord_index(data.texcoords[data.indices[1] as usize]) as u32,
        normal_id: builder.get_normal_index(data.normals[data.indices[1] as usize]) as u32,
    };
    let mut order = true;
    for index in &data.indices[2..] {
        let c = scene::MeshPoint {
            vertex_id: builder.get_vertex_index(data.vertices[*index as usize]) as u32,
            texcoord_id: builder.get_texcoord_index(data.texcoords[*index as usize]) as u32,
            normal_id: builder.get_normal_index(data.normals[*index as usize]) as u32,
        };
        if order {
            builder
                .triangles
                .entry(mat_id)
                .or_default()
                .push(scene::MeshTriangle {
                    corners: [c1, c2.clone(), c.clone()],
                });
        } else {
            builder
                .triangles
                .entry(mat_id)
                .or_default()
                .push(scene::MeshTriangle {
                    corners: [c2.clone(), c1, c.clone()],
                });
        }
        order = !order;
        c1 = c2;
        c2 = c;
    }
}

fn push_primitive_fan(
    builder: &mut MeshBuilder,
    p: &gltf::Primitive,
    buffers: &[gltf::buffer::Data],
) {
    let data = get_data(p, buffers);
    let mat_id = p
        .material()
        .name()
        .map(|x| util::hash_name_i32(x))
        .unwrap_or(p.material().index().unwrap_or(0) as i32);
    if data.indices.len() < 3 {
        return;
    }
    let c = scene::MeshPoint {
        vertex_id: builder.get_vertex_index(data.vertices[data.indices[0] as usize]) as u32,
        texcoord_id: builder.get_texcoord_index(data.texcoords[data.indices[0] as usize]) as u32,
        normal_id: builder.get_normal_index(data.normals[data.indices[0] as usize]) as u32,
    };
    for index in data.indices[1..].windows(2) {
        let c1 = scene::MeshPoint {
            vertex_id: builder.get_vertex_index(data.vertices[index[0] as usize]) as u32,
            texcoord_id: builder.get_texcoord_index(data.texcoords[index[0] as usize]) as u32,
            normal_id: builder.get_normal_index(data.normals[index[0] as usize]) as u32,
        };
        let c2 = scene::MeshPoint {
            vertex_id: builder.get_vertex_index(data.vertices[index[1] as usize]) as u32,
            texcoord_id: builder.get_texcoord_index(data.texcoords[index[1] as usize]) as u32,
            normal_id: builder.get_normal_index(data.normals[index[1] as usize]) as u32,
        };
        builder
            .triangles
            .entry(mat_id)
            .or_default()
            .push(scene::MeshTriangle {
                corners: [c.clone(), c1, c2],
            });
    }
}

bitflags! {
    pub struct ImportHint: u32 {
        const MESHES = 1<<0;
        /*
        const NODES = 1<<1;
        const TEXTURES = 1<<2;
        const SKINS = 1<<3;
        */
    }
}

pub fn import_scene<P>(path: P, hint: ImportHint) -> gltf::Result<scene::Scene>
where
    P: AsRef<Path>,
{
    let (document, buffers, _images) = gltf::import(path)?;
    let mut scene = scene::Scene::new_empty();
    if hint.contains(ImportHint::MESHES) {
        for doc_mesh in document.meshes() {
            let mut builder = MeshBuilder::new();
            for primitive in doc_mesh.primitives() {
                match primitive.mode() {
                    gltf::mesh::Mode::Triangles => {
                        push_primitive_triangles(&mut builder, &primitive, buffers.as_slice());
                    }
                    gltf::mesh::Mode::TriangleStrip => {
                        push_primitive_strip(&mut builder, &primitive, buffers.as_slice());
                    }
                    gltf::mesh::Mode::TriangleFan => {
                        push_primitive_fan(&mut builder, &primitive, buffers.as_slice());
                    }
                    _ => {}
                }
            }
            let mesh_name = doc_mesh
                .name()
                .map(|x| x.to_owned())
                .unwrap_or_else(|| format!("Mesh {}", doc_mesh.index()));
            scene.meshes.insert(mesh_name, builder.commit());
        }
    }
    // TODO: The rest
    // only care about meshes for now
    Ok(scene)
}
