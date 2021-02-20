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

fn vec4_to_value(value: [f32; 4]) -> json::Value {
    json::Value::Array(vec![
        value[0].into(),
        value[1].into(),
        value[2].into(),
        value[3].into(),
    ])
}

fn vec3_to_value(value: Vector3) -> json::Value {
    json::Value::Array(vec![value.x.into(), value.y.into(), value.z.into()])
}

fn vec2_to_value(value: Vector2) -> json::Value {
    json::Value::Array(vec![value.x.into(), value.y.into()])
}

fn min_vec4<'a, I>(mut values: I) -> [f32; 4]
where
    I: std::iter::Iterator<Item = &'a [f32; 4]>,
{
    let mut ret = values.next().map(|x| x.to_owned()).unwrap_or([0.0f32; 4]);
    for v in values {
        ret[0] = ret[0].min(v[0]);
        ret[1] = ret[1].min(v[1]);
        ret[2] = ret[2].min(v[2]);
        ret[3] = ret[3].min(v[3]);
    }
    ret
}

fn max_vec4<'a, I>(mut values: I) -> [f32; 4]
where
    I: std::iter::Iterator<Item = &'a [f32; 4]>,
{
    let mut ret = values.next().map(|x| x.to_owned()).unwrap_or([0.0f32; 4]);
    for v in values {
        ret[0] = ret[0].max(v[0]);
        ret[1] = ret[1].max(v[1]);
        ret[2] = ret[2].max(v[2]);
        ret[3] = ret[3].max(v[3]);
    }
    ret
}

fn min_vec3<'a, I>(mut values: I) -> Vector3
where
    I: std::iter::Iterator<Item = &'a Vector3>,
{
    let mut ret = values
        .next()
        .map(|x| x.to_owned())
        .unwrap_or(Vector3::zero());
    for v in values {
        ret.x = ret.x.min(v.x);
        ret.y = ret.y.min(v.y);
        ret.z = ret.z.min(v.z);
    }
    ret
}

fn max_vec3<'a, I>(mut values: I) -> Vector3
where
    I: std::iter::Iterator<Item = &'a Vector3>,
{
    let mut ret = values
        .next()
        .map(|x| x.to_owned())
        .unwrap_or(Vector3::zero());
    for v in values {
        ret.x = ret.x.max(v.x);
        ret.y = ret.y.max(v.y);
        ret.z = ret.z.max(v.z);
    }
    ret
}

fn min_vec2<'a, I>(mut values: I) -> Vector2
where
    I: std::iter::Iterator<Item = &'a Vector2>,
{
    let mut ret = values
        .next()
        .map(|x| x.to_owned())
        .unwrap_or(Vector2::zero());
    for v in values {
        ret.x = ret.x.min(v.x);
        ret.y = ret.y.min(v.y);
    }
    ret
}

fn max_vec2<'a, I>(mut values: I) -> Vector2
where
    I: std::iter::Iterator<Item = &'a Vector2>,
{
    let mut ret = values
        .next()
        .map(|x| x.to_owned())
        .unwrap_or(Vector2::zero());
    for v in values {
        ret.x = ret.x.max(v.x);
        ret.y = ret.y.max(v.y);
    }
    ret
}

fn generate_primitive(
    buffer: &mut Vec<u8>,
    root: &mut json::Root,
    point_buf: &[Point],
    index_buf: &[u32],
    skin_buf: Option<(&[[f32; 4]], &[[u16; 4]])>,
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
        min: Some(vec3_to_value(min_vec3(point_buf.iter().map(|x| &x.vertex)))),
        max: Some(vec3_to_value(max_vec3(point_buf.iter().map(|x| &x.vertex)))),
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
        min: Some(vec2_to_value(min_vec2(
            point_buf.iter().map(|x| &x.texcoord),
        ))),
        max: Some(vec2_to_value(max_vec2(
            point_buf.iter().map(|x| &x.texcoord),
        ))),
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
        min: Some(vec3_to_value(min_vec3(point_buf.iter().map(|x| &x.normal)))),
        max: Some(vec3_to_value(max_vec3(point_buf.iter().map(|x| &x.normal)))),
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
    // handle skin
    let skin_indices = if let Some((weights, joints)) = skin_buf {
        let weights_pos = buffer.len() as u32;
        buffer.extend_from_slice(unsafe {
            let ptr = weights.as_ptr() as *const u8;
            std::slice::from_raw_parts(ptr, weights.len() * mem::size_of::<[f32; 4]>())
        });
        let joints_pos = buffer.len() as u32;
        buffer.extend_from_slice(unsafe {
            let ptr = joints.as_ptr() as *const u8;
            std::slice::from_raw_parts(ptr, joints.len() * mem::size_of::<[u16; 4]>())
        });
        let weights_view_index = root.buffer_views.len() as u32;
        root.buffer_views.push(json::buffer::View {
            buffer: json::Index::new(0),
            byte_length: weights.len() as u32 * mem::size_of::<[f32; 4]>() as u32,
            byte_offset: Some(weights_pos as u32),
            byte_stride: None,
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
        });
        let joints_view_index = root.buffer_views.len() as u32;
        root.buffer_views.push(json::buffer::View {
            buffer: json::Index::new(0),
            byte_length: joints.len() as u32 * mem::size_of::<[u16; 4]>() as u32,
            byte_offset: Some(joints_pos as u32),
            byte_stride: None,
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
        });
        let weight_accessor_index = root.accessors.len() as u32;
        root.accessors.push(json::Accessor {
            buffer_view: Some(json::Index::new(weights_view_index)),
            byte_offset: 0,
            count: weights.len() as u32,
            component_type: Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::F32,
            )),
            extensions: Default::default(),
            extras: Default::default(),
            type_: Valid(json::accessor::Type::Vec4),
            min: Some(vec4_to_value(min_vec4(weights.iter()))),
            max: Some(vec4_to_value(max_vec4(weights.iter()))),
            name: None,
            normalized: false,
            sparse: None,
        });
        let joint_accessor_index = root.accessors.len() as u32;
        root.accessors.push(json::Accessor {
            buffer_view: Some(json::Index::new(joints_view_index)),
            byte_offset: 0,
            count: joints.len() as u32,
            component_type: Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::U16,
            )),
            extensions: Default::default(),
            extras: Default::default(),
            type_: Valid(json::accessor::Type::Vec4),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None,
        });
        Some((weight_accessor_index, joint_accessor_index))
    } else {
        None
    };
    // Create primitive
    let primitive = json::mesh::Primitive {
        attributes: {
            let mut map = HashMap::new();
            map.insert(Valid(json::mesh::Semantic::Positions), va_index);
            map.insert(Valid(json::mesh::Semantic::TexCoords(0)), ta_index);
            map.insert(Valid(json::mesh::Semantic::Normals), na_index);
            if let Some((weight_i, joints_i)) = skin_indices {
                map.insert(
                    Valid(json::mesh::Semantic::Weights(0)),
                    json::Index::new(weight_i),
                );
                map.insert(
                    Valid(json::mesh::Semantic::Joints(0)),
                    json::Index::new(joints_i),
                );
            }
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

fn export_skin(
    skin: &scene::Skin,
    buffer: &mut Vec<u8>,
    root: &mut json::Root,
) -> (json::Index<json::Skin>, json::Index<json::Node>) {
    let index = json::Index::new(root.skins.len() as u32);
    let buffer_pos = buffer.len() as u32;
    // Create node for each joint
    let mut joints: Vec<json::Index<json::Node>> = Vec::new();
    for joint in skin.joints.iter() {
        let joint_index = json::Index::new(root.nodes.len() as u32);
        joints.push(joint_index);
        root.nodes.push(json::Node {
            camera: None,
            children: None,
            extensions: Default::default(),
            extras: Default::default(),
            matrix: if joint.transform == Transform3D::identity() {
                None
            } else {
                Some(joint.transform.to_array())
            },
            mesh: None,
            skin: None,
            name: None,
            rotation: None,
            scale: None,
            translation: None,
            weights: None,
        });
        let txinv = joint.transform.inverse().unwrap();
        let invbindmat = txinv.to_array();
        buffer.extend_from_slice(unsafe {
            let ptr = invbindmat.as_ptr() as *const u8;
            std::slice::from_raw_parts(ptr, invbindmat.len() * mem::size_of::<f32>())
        });
    }
    // Create root node
    let skeleton_index = json::Index::new(root.nodes.len() as u32);
    let txinv = Transform3D::identity();
    let invbindmat = txinv.to_array();
    buffer.extend_from_slice(unsafe {
        let ptr = invbindmat.as_ptr() as *const u8;
        std::slice::from_raw_parts(ptr, invbindmat.len() * mem::size_of::<f32>())
    });
    root.nodes.push(json::Node {
        camera: None,
        children: Some(joints.clone()),
        extensions: Default::default(),
        extras: Default::default(),
        matrix: None,
        translation: None,
        mesh: None,
        skin: None,
        name: None,
        rotation: None,
        scale: None,
        weights: None,
    });
    joints.push(skeleton_index);
    // Create buffer and accessor for inverse bind matrices
    let view_index = root.buffer_views.len() as u32;
    root.buffer_views.push(json::buffer::View {
        buffer: json::Index::new(0),
        byte_length: buffer.len() as u32 - buffer_pos,
        byte_offset: Some(buffer_pos as u32),
        byte_stride: None,
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: None,
    });
    let accessor_index = root.accessors.len() as u32;
    root.accessors.push(json::Accessor {
        buffer_view: Some(json::Index::new(view_index)),
        byte_offset: 0,
        count: joints.len() as u32,
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Mat4),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });
    root.skins.push(json::Skin {
        inverse_bind_matrices: Some(json::Index::new(accessor_index)),
        joints,
        name: None,
        skeleton: Some(skeleton_index),
        extensions: None,
        extras: Default::default(),
    });
    (index, skeleton_index)
}

fn export_mesh(
    mesh: &scene::Mesh,
    buffer: &mut Vec<u8>,
    root: &mut json::Root,
    mat_indices: &IdMap<u32>,
    materials: &IdMap<scene::SMaterial>,
) -> MeshIndex {
    let mut json_mesh = json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        primitives: Vec::new(),
        weights: None,
    };
    for (matid, ls) in mesh.triangles.iter() {
        // Each material gets own primitive
        let mut point_buf = Vec::<Point>::new();
        let mut index_buf = Vec::<u32>::new();
        let mut point_indices = HashMap::<Point, u32>::new();
        let mut skin_info = mesh
            .skin
            .as_ref()
            .map(|skin| (skin, Vec::<[f32; 4]>::new(), Vec::<[u16; 4]>::new()));
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
                    if let Some((skin, weights, joints)) = &mut skin_info {
                        weights.push(skin.vertices[corner.vertex_id as usize].get_weight_array());
                        joints.push(skin.vertices[corner.vertex_id as usize].get_joint_array());
                    }
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
            skin_info.as_ref().map(|x| (x.1.as_slice(), x.2.as_slice())),
            json::mesh::Mode::Triangles,
            mat_indices
                .get(*matid)
                .map(|elem| json::Index::new(*elem.get_value_ref())),
        );
        json_mesh.primitives.push(primitive);
    }
    // json_mesh
    let mesh_index = root.meshes.len() as u32;
    root.meshes.push(json_mesh);
    MeshIndex { mesh: mesh_index }
}

struct MeshIndex {
    pub mesh: u32,
}

fn export_node(
    name: &str,
    node: &scene::SNode,
    mesh_indices: &HashMap<String, MeshIndex>,
    root: &mut json::Root,
    buffer: &mut Vec<u8>,
) -> json::Index<json::Node> {
    let mut children = Vec::new();
    for (name, c) in node.tree.iter() {
        children.push(export_node(name, c, mesh_indices, root, buffer));
    }
    let mesh = match &node.graphic {
        scene::NodeGraphic::Mesh { mesh } => {
            mesh_indices.get(mesh).map(|id| json::Index::new(id.mesh))
        }
        scene::NodeGraphic::Skin { skin, meshes } => {
            let (skin_index, skeleton_index) = export_skin(&skin, buffer, root);
            for mesh in meshes.iter() {
                let mesh_node_index = json::Index::new(root.nodes.len() as u32);
                root.nodes.push(json::Node {
                    camera: None,
                    children: None,
                    extensions: Default::default(),
                    extras: Default::default(),
                    matrix: None,
                    mesh: mesh_indices.get(mesh).map(|id| json::Index::new(id.mesh)),
                    skin: Some(skin_index),
                    name: Some(name.to_string()),
                    rotation: None,
                    scale: None,
                    translation: None,
                    weights: None,
                });
                children.push(mesh_node_index);
            }
            children.push(skeleton_index);
            None
        }
        scene::NodeGraphic::None => None,
    };
    let node = json::Node {
        camera: None,
        children: if children.len() > 0 {
            Some(children)
        } else {
            None
        },
        extensions: Default::default(),
        extras: Default::default(),
        matrix: if node.transform == Transform3D::identity() {
            None
        } else {
            Some(node.transform.to_array())
        },
        mesh,
        skin: None,
        name: Some(name.to_string()),
        rotation: None,
        scale: None,
        translation: None,
        weights: None,
    };
    let idx = json::Index::new(root.nodes.len() as u32);
    root.nodes.push(node);
    idx
}

pub fn export_scene(scn: &scene::Scene, binary: bool) -> (json::Root, Vec<u8>) {
    let mut root = json::Root::default();
    // add basic, common sampler
    root.samplers.push(json::texture::Sampler {
        name: None,
        mag_filter: Some(Valid(json::texture::MagFilter::Linear)),
        min_filter: Some(Valid(json::texture::MinFilter::Linear)),
        wrap_s: Valid(json::texture::WrappingMode::Repeat),
        wrap_t: Valid(json::texture::WrappingMode::Repeat),
        extensions: None,
        extras: Default::default(),
    });
    // export textures
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
    // export materials
    let mut mat_indices = IdMap::<u32>::new();
    for (_id, elem) in scn.materials.iter() {
        println!("Adding material: {}", elem.get_name());
        let mat = elem.get_value_ref();
        let index = root.materials.len() as u32;
        export_material(mat, &mut root, &tex_indices);
        mat_indices.insert(elem.get_name().to_string(), index);
    }
    // export meshes
    let mut mesh_indices = HashMap::<String, MeshIndex>::new();
    for (_id, elem) in scn.meshes.iter() {
        let mesh = elem.get_value_ref();
        let index = export_mesh(mesh, &mut buffer, &mut root, &mat_indices, &scn.materials);
        mesh_indices.insert(elem.get_name().to_string(), index);
        println!("Inserting: {}", elem.get_name());
    }
    // export nodes
    let node_root_idx = export_node("root", &scn.root, &mesh_indices, &mut root, &mut buffer);
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

#[cfg(test)]
mod test {
    use crate::common::*;
    use crate::scene::gltf::*;
    #[test]
    fn minmax_vec2() {
        let a = vec![
            Vector2::new(0.5, 1.5),
            Vector2::new(-0.1, 2.5),
            Vector2::new(3.0, 0.1),
        ];
        let min = min_vec2(a.iter());
        let max = max_vec2(a.iter());
        assert_eq!(min, Vector2::new(-0.1, 0.1));
        assert_eq!(max, Vector2::new(3.0, 2.5));
    }
}
