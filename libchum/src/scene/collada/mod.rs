use crate::common;
use crate::scene;
use crate::util::xml::{self, XMLAttribute, XMLContent, XMLTag};
use chrono;
use std::error::Error;
use std::io;

pub mod controller;
pub mod geometry;

pub fn make_asset() -> xml::TagStruct {
    let utc: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
    let time_string = utc.format("%Y-%m-%dT%H:%M:%S").to_string();
    xml::TagStruct {
        name: "asset".to_owned(),
        attributes: vec![],
        content: None,
        tags: vec![
            xml::TagStruct {
                name: "contributor".to_owned(),
                attributes: vec![],
                content: None,
                tags: vec![
                    xml::TagStruct {
                        name: "author".to_owned(),
                        attributes: vec![],
                        content: Some("Chum World User".to_owned()),
                        tags: vec![],
                    },
                    xml::TagStruct {
                        name: "authoring_tool".to_owned(),
                        attributes: vec![],
                        content: Some("Chum World Alpha".to_owned()),
                        tags: vec![],
                    },
                ],
            },
            xml::TagStruct {
                name: "created".to_owned(),
                attributes: vec![],
                content: Some(time_string.clone()),
                tags: vec![],
            },
            xml::TagStruct {
                name: "modified".to_owned(),
                attributes: vec![],
                content: Some(time_string.clone()),
                tags: vec![],
            },
            xml::TagStruct {
                name: "unit".to_owned(),
                attributes: vec![
                    ("name".to_owned(), "meter".to_owned()),
                    ("meter".to_owned(), "1".to_owned()),
                ],
                content: None,
                tags: vec![],
            },
            xml::TagStruct {
                name: "up_axis".to_owned(),
                attributes: vec![],
                content: Some("Y_UP".to_owned()),
                tags: vec![],
            },
        ],
    }
}

#[derive(Debug, Clone)]
pub struct COLLADA {
    pub library_geometry: Vec<geometry::LibraryGeometry>,
    pub library_visual_scene: Vec<LibraryVisualScene>,
    pub library_controller: Vec<controller::LibraryControllers>,
    pub scene: Option<Scene>,
    pub asset: xml::TagStruct,
}

impl XMLTag for COLLADA {
    fn get_name(&self) -> &str {
        "COLLADA"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![
            ("xmlns", &"http://www.collada.org/2005/11/COLLADASchema"),
            ("version", &"1.4.1"),
            ("xmlns:xsi", &"http://www.w3.org/2001/XMLSchema-instance"),
        ]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        let mut v = Vec::new();
        v.push(&self.asset as &dyn XMLTag);
        v.extend(self.library_geometry.iter().map(|x| x as &dyn XMLTag));
        v.extend(self.library_controller.iter().map(|x| x as &dyn XMLTag));
        v.extend(self.library_visual_scene.iter().map(|x| x as &dyn XMLTag));
        v.extend(self.scene.iter().map(|x| x as &dyn XMLTag));
        v
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    // pub instance_physics_scene
    pub instance_visual_scene: Option<InstanceVisualScene>, // pub instance_kinematics_scene
}

impl_tag_tree!(
    Scene,
    "scene",
    attr => [],
    tags => [instance_visual_scene]
);

#[derive(Debug, Clone)]
pub struct InstanceVisualScene {
    pub sid: Option<String>,
    pub name: Option<String>,
    pub url: String,
}

impl_tag_tree!(
    InstanceVisualScene,
    "instance_visual_scene",
    attr => [
        ("sid", sid),
        ("name", name),
        ("url", url)
    ],
    tags => []
);

#[derive(Debug, Clone)]
pub enum Array {
    BoolArray {
        count: usize,
        id: Option<String>,
        name: Option<String>,
        data: Vec<bool>,
    },
    FloatArray {
        count: usize,
        id: Option<String>,
        name: Option<String>,
        digits: Option<u8>,
        magnitude: Option<u16>,
        data: Vec<f32>,
    },
    IDREFArray {
        count: usize,
        id: Option<String>,
        name: Option<String>,
        data: Vec<String>,
    },
    IntArray {
        count: usize,
        id: Option<String>,
        name: Option<String>,
        min_inclusive: Option<i64>,
        max_inclusive: Option<i64>,
        data: Vec<i64>,
    },
    NameArray {
        count: usize,
        id: Option<String>,
        name: Option<String>,
        data: Vec<String>,
    },
    SIDREFArray {
        count: usize,
        id: Option<String>,
        name: Option<String>,
        data: Vec<String>,
    },
}

impl_tag_enum!(
    Array,
    BoolArray => (
        "bool_array",
        attr => [("count", count), ("id", id), ("name", name)],
        tags => [],
        content => data
    ),
    FloatArray => (
        "float_array",
        attr => [("count", count), ("id", id), ("name", name), ("digits", digits), ("magnitude", magnitude)],
        tags => [],
        content => data
    ),
    IDREFArray => (
        "IDREF_array",
        attr => [("count", count), ("id", id), ("name", name)],
        tags => [],
        content => data
    ),
    IntArray => (
        "int_array",
        attr => [("count", count), ("id", id), ("name", name), ("minInclusive", min_inclusive), ("maxInclusive", max_inclusive)],
        tags => [],
        content => data
    ),
    NameArray => (
        "Name_array",
        attr => [("count", count), ("id", id), ("name", name)],
        tags => [],
        content => data
    ),
    SIDREFArray => (
        "SIDREF_array",
        attr => [("count", count), ("id", id), ("name", name)],
        tags => [],
        content => data
    )
);

#[derive(Debug, Clone)]
pub struct LibraryVisualScene {
    pub visual_scene: Vec<VisualScene>,
    pub id: Option<String>,
    pub name: Option<String>,
}

impl_tag_tree!(
    LibraryVisualScene,
    "library_visual_scenes",
    attr => [
        ("id", id),
        ("name", name)
    ],
    tags => [visual_scene]
);

#[derive(Debug, Clone)]
pub struct VisualScene {
    pub id: Option<String>,
    pub name: Option<String>,
    pub node: Vec<Node>,
    // pub evaluate_scene: Vec<EvaluateScene>,
}

impl_tag_tree!(
    VisualScene,
    "visual_scene",
    attr => [
        ("id", id),
        ("name", name)
    ],
    tags => [node]
);

#[derive(Debug, Clone)]
pub enum NodeType {
    NODE,
    JOINT,
}

impl XMLAttribute for NodeType {
    fn serialize_attribute(&self) -> Option<String> {
        match self {
            NodeType::NODE => Some("NODE".to_owned()),
            NodeType::JOINT => Some("JOINT".to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Option<String>,
    pub name: Option<String>,
    pub sid: Option<String>,
    pub datatype: NodeType,
    pub layer: Vec<String>,
    pub transform: Vec<NodeTransform>,
    // pub camera: Vec<InstanceCamera>
    pub controller: Vec<controller::InstanceController>,
    pub geometry: Vec<geometry::InstanceGeometry>,
    // pub light: Vec<InstanceLight>
    pub node: Vec<Node>,
}

impl_tag_tree!(
    Node,
    "node",
    attr => [
        ("id", id),
        ("name", name),
        ("sid", sid),
        ("type", datatype),
        ("layer", layer)
    ],
    tags => [transform, node, geometry, controller]
);

#[derive(Clone, Debug)]
pub enum NodeTransform {
    // LookAt,
    Matrix {
        sid: Option<String>,
        data: common::Mat4x4,
    }, // Rotate
       // Scale
       // Skew
       // Translate
}

impl XMLTag for NodeTransform {
    fn get_name(&self) -> &str {
        use NodeTransform::*;
        match self {
            Matrix { .. } => "matrix",
        }
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        use NodeTransform::*;
        match self {
            Matrix { sid, .. } => vec![("sid", sid)],
        }
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        use NodeTransform::*;
        match self {
            Matrix { data, .. } => Some(data),
        }
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct InputShared {
    offset: usize,
    semantic: String,
    source: String,
    set: Option<usize>,
}

impl_tag_tree!(
    InputShared,
    "input",
    attr => [("offset", offset), ("semantic", semantic), ("source", source), ("set", set)],
    tags => []
);

#[derive(Debug, Clone)]
pub struct InputUnshared {
    semantic: String,
    source: String,
}

impl_tag_tree!(
    InputUnshared,
    "input",
    attr => [("semantic", semantic), ("source", source)],
    tags => []
);

#[derive(Debug, Clone)]
pub struct Source {
    pub id: String,
    pub name: Option<String>,
    pub array: Array,
    pub technique_common: Option<SourceTechnique>,
}

impl_tag_tree!(
    Source,
    "source",
    attr => [("id", id), ("name", name)],
    tags => [array, technique_common]
);

#[derive(Debug, Clone)]
pub struct SourceTechnique {
    pub accessor: Accessor,
}

impl_tag_tree!(
    SourceTechnique,
    "technique_common",
    attr => [],
    tags => [accessor]
);

#[derive(Debug, Clone)]
pub struct Accessor {
    pub count: usize,
    pub offset: Option<usize>,
    pub source: String,
    pub stride: Option<usize>,
    pub param: Vec<ParamAccessor>,
}

impl_tag_tree!(
    Accessor,
    "accessor",
    attr => [("count", count), ("offset", offset), ("source", source), ("stride", stride)],
    tags => [param]
);

#[derive(Debug, Clone)]
pub struct ParamAccessor {
    pub name: Option<String>,
    pub sid: Option<String>,
    pub datatype: String,
    pub semantic: Option<String>,
}

impl_tag_tree!(
    ParamAccessor,
    "param",
    attr => [("name", name), ("sid", sid), ("type", datatype), ("semantic", semantic)],
    tags => []
);

pub fn trimesh_to_geometry_node(mesh: &scene::SceneTriMesh) -> (geometry::Geometry, Node) {
    let id_mesh = format!("{}-mesh", mesh.name);
    (
        geometry::trimesh_to_geometry(mesh),
        Node {
            id: Some(mesh.name.clone()),
            name: Some(mesh.name.clone()),
            sid: None,
            datatype: NodeType::NODE,
            layer: vec![],
            transform: vec![NodeTransform::Matrix {
                sid: Some("transform".to_owned()),
                data: mesh.transform.clone(),
            }],
            geometry: vec![geometry::InstanceGeometry {
                url: format!("#{}", id_mesh),
                name: Some(mesh.name.clone()),
                sid: None,
            }],
            node: vec![],
            controller: vec![]
        },
    )
}

pub fn trimesh_to_skin_node(
    mesh: &scene::SceneTriMesh,
    skin: &scene::SceneSkin,
) -> (controller::Controller, geometry::Geometry, Node) {
    // let id_mesh = format!("{}-mesh", mesh.name);
    let id_armature = format!("{}-armature", mesh.name);
    let id_skin = format!("{}-skin", mesh.name);
    // let id_skeleton = format!("{}-skeleton", mesh.name);
    let controller = controller::skin_to_controller(skin, &mesh.name);
    let geometry = geometry::trimesh_to_geometry(mesh);
    let mut nodes = Vec::new();
    for group in skin.groups.iter() {
        nodes.push(Node {
            name: Some(group.clone()),
            sid: Some(group.clone()),
            id: Some(format!("{}-{}", id_armature, group)),
            datatype: NodeType::JOINT,
            geometry: vec![],
            controller: vec![],
            layer: vec![],
            transform: vec![NodeTransform::Matrix {
                sid: Some("transform".to_owned()),
                data: mesh.transform.clone(),
            }],
            node: vec![]
        });
    }
    let meshnode = Node {
        name: Some(mesh.name.clone()),
        sid: None,
        id: Some(mesh.name.clone()),
        datatype: NodeType::NODE,
        geometry: vec![],
        controller: vec![controller::InstanceController {
            sid: None,
            url: format!("#{}", id_skin),
            name: None,
            skeleton: vec![controller::Skeleton {
                data: format!("#{}-base", id_armature)
            }]
        }],
        layer: vec![],
        transform: vec![NodeTransform::Matrix {
            sid: Some("transform".to_owned()),
            data: mesh.transform.clone(),
        }],
        node: vec![]
    };
    (
        controller,
        geometry,
        Node {
            id: Some(id_armature.clone()),
            name: Some(id_armature.clone()),
            sid: None,
            datatype: NodeType::NODE,
            layer: vec![],
            transform: vec![NodeTransform::Matrix {
                sid: Some("transform".to_owned()),
                data: common::Mat4x4::new_basis()
            }],
            node: vec![
                Node {
                    id: Some(format!("{}-base", id_armature)),
                    name: Some("Base".to_owned()),
                    sid: Some("Base".to_owned()),
                    datatype: NodeType::JOINT,
                    layer: vec![],
                    node: nodes,
                    geometry: vec![],
                    controller: vec![],
                    transform: vec![NodeTransform::Matrix {
                        sid: Some("transform".to_owned()),
                        data: mesh.transform.clone(),
                    }],
                },
                meshnode
            ],
            geometry: vec![],
            controller: vec![],
        }
    )
}

fn scene_to_collada(scene: &scene::Scene) -> COLLADA {
    let mut geometries = Vec::new();
    let mut nodes = Vec::new();
    let mut controllers = Vec::new();
    for trimesh in scene.trimeshes.iter() {
        if let Some(skin) = &trimesh.skin {
            let (controller, geometry, node) = trimesh_to_skin_node(trimesh, skin);
            controllers.push(controller);
            geometries.push(geometry);
            nodes.push(node);
        } else {
            let (geometry, node) = trimesh_to_geometry_node(trimesh);
            geometries.push(geometry);
            nodes.push(node);
        }
    }
    COLLADA {
        asset: make_asset(),
        library_controller: vec![controller::LibraryControllers {
            id: None,
            name: None,
            controller: controllers,
        }],
        library_geometry: vec![geometry::LibraryGeometry {
            geometry: geometries,
            id: None,
            name: None,
        }],
        library_visual_scene: vec![LibraryVisualScene {
            id: None,
            name: None,
            visual_scene: vec![VisualScene {
                id: Some("Scene".to_owned()),
                name: Some("Scene".to_owned()),
                node: nodes,
            }],
        }],
        scene: Some(Scene {
            instance_visual_scene: Some(InstanceVisualScene {
                name: None,
                sid: None,
                url: "#Scene".to_owned(),
            }),
        }),
    }
}

pub fn scene_to_writer_dae<W>(scene: &scene::Scene, writer: &mut W) -> Result<(), Box<dyn Error>>
where
    W: io::Write,
{
    xml::write_to(&scene_to_collada(scene), writer, true)
}

pub fn scene_to_string_dae(scene: &scene::Scene) -> Result<String, Box<dyn Error>> {
    xml::write_to_str(&scene_to_collada(scene), true)
}

mod test {
    #[test]
    pub fn test_to_string_empty() {
        use crate::scene;
        use crate::scene::collada;
        let scene = scene::Scene { trimeshes: vec![] };
        println!("{}", collada::scene_to_string_dae(&scene).unwrap());
    }
    // #[test]
    // pub fn test_to_string_single_mesh() {
    //     use crate::common::*;
    //     use crate::scene;
    //     use crate::scene::collada;
    //     let scene = scene::Scene {
    //         trimeshes: vec![scene::SceneTriMesh {
    //             name: "Wee".to_owned(),
    //             transform: Mat4x4::new_basis(),
    //             tris: vec![Tri {
    //                 points: [
    //                     Point {
    //                         vertex: Vector3::with(0.0, 0.0, 0.0),
    //                         texcoord: Vector2::with(0.0, 0.0),
    //                         normal: Vector3::with(1.0, 0.0, 0.0),
    //                     },
    //                     Point {
    //                         vertex: Vector3::with(-1.0, 0.0, 0.0),
    //                         texcoord: Vector2::with(0.0, 0.0),
    //                         normal: Vector3::with(1.0, 0.0, 0.0),
    //                     },
    //                     Point {
    //                         vertex: Vector3::with(0.0, 1.0, 1.0),
    //                         texcoord: Vector2::with(0.0, 0.0),
    //                         normal: Vector3::with(1.0, 0.0, 0.0),
    //                     },
    //                 ],
    //             }],
    //         }],
    //     };
    //     println!("{}", collada::scene_to_string_dae(&scene).unwrap());
    // }
}
