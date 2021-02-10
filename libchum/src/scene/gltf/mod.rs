use base64;
use gltf_json as json;

use crate::common::*;
use crate::reader;
use crate::scene;

use crate::util::idmap::IdMap;
use json::validation::Checked::Valid;
use std::collections::HashMap;
use std::mem;

fn align(buf: &mut Vec<u8>) {
    while (buf.len() % 4) != 0 {
        buf.push(0);
    }
}

fn generate_primitive(
    buffer: &mut Vec<u8>,
    root: &mut json::Root,
    point_buf: &[Point],
    index_buf: &[u32],
    mode: json::mesh::Mode,
    material: Option<json::Index<json::Material>>,
) -> json::mesh::Primitive {
    // Create point view and add to buffer
    let points_pos = buffer.len();
    let buffer_view_points = json::buffer::View {
        buffer: json::Index::new(0),
        byte_length: point_buf.len() as u32 * mem::size_of::<Point>() as u32,
        byte_offset: Some(points_pos as u32),
        byte_stride: Some(mem::size_of::<Point>() as u32),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
    };
    buffer.extend_from_slice(unsafe {
        let ptr = point_buf.as_ptr() as *const u8;
        std::slice::from_raw_parts(ptr, point_buf.len() * mem::size_of::<Point>())
    });
    let points_index = json::Index::new(root.buffer_views.len() as u32);
    root.buffer_views.push(buffer_view_points);
    // Create index view and add to buffer
    let index_pos = buffer.len();
    let buffer_view_index = json::buffer::View {
        buffer: json::Index::new(0),
        byte_length: index_buf.len() as u32 * mem::size_of::<u32>() as u32,
        byte_offset: Some(index_pos as u32),
        byte_stride: None, //Some(mem::size_of::<u32>() as u32),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ElementArrayBuffer)),
    };
    buffer.extend_from_slice(unsafe {
        let ptr = index_buf.as_ptr() as *const u8;
        std::slice::from_raw_parts(ptr, index_buf.len() * mem::size_of::<u32>())
    });
    let indices_index = json::Index::new(root.buffer_views.len() as u32);
    root.buffer_views.push(buffer_view_index);
    // Create vertex accessor
    let vertices_accessor = json::Accessor {
        buffer_view: Some(points_index),
        byte_offset: 0,
        count: point_buf.len() as u32,
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    };
    let va_index = json::Index::new(root.accessors.len() as u32);
    root.accessors.push(vertices_accessor);
    // Create texcoord accessor
    let texcoord_accessor = json::Accessor {
        buffer_view: Some(points_index),
        byte_offset: mem::size_of::<Vector3>() as u32,
        count: point_buf.len() as u32,
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec2),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    };
    let ta_index = json::Index::new(root.accessors.len() as u32);
    root.accessors.push(texcoord_accessor);
    // Create normal accessor
    let normal_accessor = json::Accessor {
        buffer_view: Some(points_index),
        byte_offset: mem::size_of::<Vector3>() as u32 + mem::size_of::<Vector2>() as u32,
        count: point_buf.len() as u32,
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    };
    let na_index = json::Index::new(root.accessors.len() as u32);
    root.accessors.push(normal_accessor);
    // Create index accessor
    let index_accessor = json::Accessor {
        buffer_view: Some(indices_index),
        byte_offset: 0,
        count: index_buf.len() as u32,
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::U32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Scalar),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    };
    let ia_index = json::Index::new(root.accessors.len() as u32);
    root.accessors.push(index_accessor);
    // Create primitive
    let primitive = json::mesh::Primitive {
        attributes: {
            let mut map = HashMap::new();
            map.insert(Valid(json::mesh::Semantic::Positions), va_index);
            map.insert(Valid(json::mesh::Semantic::TexCoords(0)), ta_index);
            map.insert(Valid(json::mesh::Semantic::Normals), na_index);
            map
        },
        extensions: Default::default(),
        extras: Default::default(),
        indices: Some(ia_index),
        material,
        mode: Valid(mode),
        targets: None,
    };
    primitive
}

fn export_material(
    material: &scene::SMaterial,
    root: &mut json::Root,
    tex: &IdMap<(u32, reader::bitmap::AlphaLevel)>,
) {
    let (amode, info) = match material.texture.as_ref().and_then(|name| tex.get(*name)) {
        Some(value) => {
            let (index, alphalevel) = value.get_value_ref();
            (
                Valid(match alphalevel {
                    reader::bitmap::AlphaLevel::Opaque => json::material::AlphaMode::Opaque,
                    reader::bitmap::AlphaLevel::Bit => json::material::AlphaMode::Mask,
                    reader::bitmap::AlphaLevel::Blend => json::material::AlphaMode::Blend,
                }),
                Some(json::texture::Info {
                    index: json::Index::new(*index),
                    tex_coord: 0,
                    extensions: Default::default(),
                    extras: Default::default(),
                }),
            )
        }
        None => (Valid(json::material::AlphaMode::Opaque), None),
    };
    root.materials.push(json::Material {
        alpha_cutoff: json::material::AlphaCutoff::default(),
        alpha_mode: amode,
        double_sided: false,
        pbr_metallic_roughness: json::material::PbrMetallicRoughness {
            base_color_factor: json::material::PbrBaseColorFactor([
                material.diffuse.x,
                material.diffuse.y,
                material.diffuse.z,
                material.alpha,
            ]),
            base_color_texture: info,
            metallic_factor: json::material::StrengthFactor(0.0),
            roughness_factor: json::material::StrengthFactor(0.5),
            metallic_roughness_texture: None,
            extensions: Default::default(),
            extras: Default::default(),
        },
        normal_texture: None,
        occlusion_texture: None,
        emissive_texture: None,
        emissive_factor: json::material::EmissiveFactor(material.emission.to_array()),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
    });
}

fn export_texture(texture: &scene::STexture, buffer: &mut Vec<u8>, root: &mut json::Root) {
    // add png image to buffer
    let buffer_offset = buffer.len() as u32;
    texture.data.export_png(buffer).expect("Woopsie daisey :)");
    let buffer_length = buffer.len() as u32 - buffer_offset;
    align(buffer);
    // create buffer view
    let index_view = json::Index::<json::buffer::View>::new(root.buffer_views.len() as u32);
    root.buffer_views.push(json::buffer::View {
        buffer: json::Index::new(0),
        byte_length: buffer_length,
        byte_offset: Some(buffer_offset),
        byte_stride: None,
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: None,
    });
    // add image
    let index_image = json::Index::<json::Image>::new(root.images.len() as u32);
    root.images.push(json::Image {
        mime_type: Some(json::image::MimeType("image/png".to_owned())),
        buffer_view: Some(index_view),
        uri: None,
        extensions: None,
        extras: Default::default(),
        name: None,
    });
    // add texture
    root.textures.push(json::Texture {
        sampler: Some(json::Index::new(0)),
        source: index_image,
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
    });
}

fn export_mesh(
    mesh: &scene::Mesh,
    buffer: &mut Vec<u8>,
    root: &mut json::Root,
    mat_indices: &IdMap<u32>,
    materials: &IdMap<scene::SMaterial>,
) -> json::Mesh {
    let mut json_mesh = json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        primitives: Vec::new(),
        weights: None,
    };
    match &mesh.data {
        scene::MeshFormat::Triangles { data } => {
            for (matid, ls) in data.iter() {
                // Each material gets own primitive
                let mut point_buf = Vec::<Point>::new();
                let mut index_buf = Vec::<u32>::new();
                let mut point_indices = HashMap::<Point, u32>::new();
                let tx = materials
                    .get(*matid)
                    .map(|mat| mat.get_value_ref().transform.clone())
                    .unwrap_or(Transform2D::identity());
                for tri in ls {
                    for corner in tri.corners.iter() {
                        let texcoord = mesh.texcoords[corner.texcoord_id as usize];
                        let point = Point {
                            vertex: mesh.vertices[corner.vertex_id as usize],
                            texcoord: tx.transform_point(texcoord.to_point()).to_vector(),
                            normal: mesh.normals[corner.normal_id as usize],
                        };
                        let index = if let Some(index) = point_indices.get(&point) {
                            *index
                        } else {
                            let index = point_buf.len() as u32;
                            point_indices.insert(point.clone(), index);
                            point_buf.push(point);
                            index
                        };
                        index_buf.push(index);
                    }
                }
                let primitive = generate_primitive(
                    buffer,
                    root,
                    point_buf.as_slice(),
                    index_buf.as_slice(),
                    json::mesh::Mode::Triangles,
                    mat_indices
                        .get(*matid)
                        .map(|elem| json::Index::new(*elem.get_value_ref())),
                );
                json_mesh.primitives.push(primitive);
            }
        }
        scene::MeshFormat::Strips { strips } => {
            for strip in strips {
                // each strip gets own primitive
                let mut point_buf = Vec::<Point>::new();
                let mut index_buf = Vec::<u32>::new();
                let mut point_indices = HashMap::<Point, u32>::new();
                for tri in strip.iterate_triangles() {
                    for corner in tri.iter() {
                        let tx = materials
                            .get(strip.material)
                            .map(|mat| mat.get_value_ref().transform.clone())
                            .unwrap_or(Transform2D::identity());
                        let texcoord = mesh.texcoords[corner.texcoord_id as usize];
                        let point = Point {
                            vertex: mesh.vertices[corner.vertex_id as usize],
                            texcoord: tx.transform_point(texcoord.to_point()).to_vector(),
                            normal: mesh.normals[corner.normal_id as usize],
                        };
                        let index = if let Some(index) = point_indices.get(&point) {
                            *index
                        } else {
                            let index = point_buf.len() as u32;
                            point_indices.insert(point.clone(), index);
                            point_buf.push(point);
                            index
                        };
                        index_buf.push(index);
                    }
                }
                let primitive = generate_primitive(
                    buffer,
                    root,
                    point_buf.as_slice(),
                    index_buf.as_slice(),
                    json::mesh::Mode::Triangles,
                    mat_indices
                        .get(strip.material)
                        .map(|elem| json::Index::new(*elem.get_value_ref())),
                );
                json_mesh.primitives.push(primitive);
            }
        }
    }
    json_mesh
}

fn export_node(
    node: &scene::SNode,
    mesh_indices: &HashMap<String, u32>,
    root: &mut json::Root,
) -> json::Index<json::Node> {
    let mut children = Vec::new();
    for c in node.children.iter() {
        children.push(export_node(c, mesh_indices, root));
    }
    if let Some(k) = node.visual_instance.as_ref() {
        println!("Searching for: {}", k);
    }
    let node = json::Node {
        camera: None,
        children: if children.len() > 0 {
            Some(children)
        } else {
            None
        },
        extensions: Default::default(),
        extras: Default::default(),
        matrix: Some(node.transform.to_array()),
        mesh: node
            .visual_instance
            .as_ref()
            .and_then(|id| mesh_indices.get(id).map(|index| json::Index::new(*index))),
        name: Some(node.name.clone()),
        rotation: None,
        scale: None,
        translation: None,
        skin: None,
        weights: None,
    };
    let idx = root.nodes.len() as u32;
    root.nodes.push(node);
    json::Index::new(idx)
}

pub fn export_scene(scn: &scene::Scene, binary: bool) -> (json::Root, Vec<u8>) {
    let mut root = json::Root::default();
    root.samplers.push(json::texture::Sampler {
        name: None,
        mag_filter: Some(Valid(json::texture::MagFilter::Linear)),
        min_filter: Some(Valid(json::texture::MinFilter::Linear)),
        wrap_s: Valid(json::texture::WrappingMode::Repeat),
        wrap_t: Valid(json::texture::WrappingMode::Repeat),
        extensions: None,
        extras: Default::default(),
    });
    let mut buffer = Vec::new();
    let mut tex_indices = IdMap::<(u32, reader::bitmap::AlphaLevel)>::new();
    for (_id, elem) in scn.textures.iter() {
        println!("Adding texture: {}", elem.get_name());
        let tex = elem.get_value_ref();
        let index = root.textures.len() as u32;
        export_texture(tex, &mut buffer, &mut root);
        tex_indices.insert(
            elem.get_name().to_string(),
            (index, tex.data.get_alpha_level()),
        );
    }
    let mut mat_indices = IdMap::<u32>::new();
    for (_id, elem) in scn.materials.iter() {
        println!("Adding material: {}", elem.get_name());
        let mat = elem.get_value_ref();
        let index = root.materials.len() as u32;
        export_material(mat, &mut root, &tex_indices);
        mat_indices.insert(elem.get_name().to_string(), index);
    }
    let mut mesh_indices = HashMap::<String, u32>::new();
    for (_id, elem) in scn.visual_instances.iter() {
        let vis = elem.get_value_ref();
        match vis {
            scene::SVisualInstance::Mesh { mesh } => {
                let json_mesh =
                    export_mesh(&mesh, &mut buffer, &mut root, &mat_indices, &scn.materials);
                let index = root.meshes.len() as u32;
                mesh_indices.insert(elem.get_name().to_string(), index);
                root.meshes.push(json_mesh);
                println!("Inserting: {}", elem.get_name());
            }
        }
    }
    let node_root_idx = export_node(&scn.node, &mesh_indices, &mut root);
    root.scenes.push(json::Scene {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        nodes: vec![node_root_idx],
    });
    align(&mut buffer);
    if binary {
        let buffer = json::Buffer {
            byte_length: buffer.len() as u32,
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            uri: None,
        };
        root.buffers.push(buffer);
    } else {
        let buf_uri = format!(
            "data:application/gltf-buffer;base64,{}",
            base64::encode(buffer.as_slice())
        );
        let buffer = json::Buffer {
            byte_length: buffer.len() as u32,
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            uri: Some(buf_uri),
        };
        root.buffers.push(buffer);
    }
    (root, buffer)
}
