use crate::common;
use crate::scene;
use crate::util::xml::{self, XMLAttribute, XMLContent, XMLTag};
use std::error::Error;
use std::io;

#[derive(Debug, Clone)]
pub struct COLLADA {
    pub library_geometry: Vec<LibraryGeometry>,
    pub library_visual_scene: Vec<LibraryVisualScene>,
    pub scene: Option<Scene>,
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
        v.extend(self.library_geometry.iter().map(|x| x as &dyn XMLTag));
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

impl XMLTag for Scene {
    fn get_name(&self) -> &str {
        "scene"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        self.instance_visual_scene
            .iter()
            .map(|x| x as &dyn XMLTag)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct InstanceVisualScene {
    pub sid: Option<String>,
    pub name: Option<String>,
    pub url: String,
}

impl XMLTag for InstanceVisualScene {
    fn get_name(&self) -> &str {
        "instance_visual_scene"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![("url", &self.url), ("name", &self.name), ("sid", &self.sid)]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct LibraryGeometry {
    pub geometry: Vec<Geometry>,
    pub id: Option<String>,
    pub name: Option<String>,
}

impl XMLTag for LibraryGeometry {
    fn get_name(&self) -> &str {
        "library_geometries"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![("id", &self.id), ("name", &self.name)]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        self.geometry.iter().map(|x| x as &dyn XMLTag).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Geometry {
    pub id: Option<String>,
    pub name: Option<String>,
    // #[serde(flatten)]
    pub data: GeometryData,
}

impl XMLTag for Geometry {
    fn get_name(&self) -> &str {
        "geometry"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![("id", &self.id), ("name", &self.name)]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        vec![&self.data]
    }
}

#[derive(Debug, Clone)]
pub enum GeometryData {
    Mesh {
        source: Vec<Source>,
        vertices: Vertices,
        // vertices: 1
        // lines
        // linestrips
        // polygons
        // polylist
        triangles: Vec<Triangles>, // trifans
                                   // tristrips
    },
}

impl XMLTag for GeometryData {
    fn get_name(&self) -> &str {
        use GeometryData::*;
        match self {
            Mesh { .. } => "mesh",
        }
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        use GeometryData::*;
        match self {
            Mesh {
                source,
                vertices,
                triangles,
            } => {
                let mut v: Vec<&dyn XMLTag> = vec![vertices];
                v.extend(source.iter().map(|x| x as &dyn XMLTag));
                v.extend(triangles.iter().map(|x| x as &dyn XMLTag));
                v
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vertices {
    pub id: String,
    pub name: Option<String>,
    pub input: Vec<InputUnshared>,
}

impl XMLTag for Vertices {
    fn get_name(&self) -> &str {
        "vertices"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![("id", &self.id), ("name", &self.name)]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        self.input.iter().map(|x| x as &dyn XMLTag).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Source {
    pub id: String,
    pub name: Option<String>,
    pub array: Array,
    pub technique_common: Option<SourceTechnique>,
}

impl XMLTag for Source {
    fn get_name(&self) -> &str {
        "source"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![("id", &self.id), ("name", &self.name)]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        let mut v: Vec<&dyn XMLTag> = vec![&self.array];
        if let Some(ref t) = self.technique_common {
            v.push(t);
        }
        v
    }
}

#[derive(Debug, Clone)]
pub struct SourceTechnique {
    pub accessor: Accessor,
}

impl XMLTag for SourceTechnique {
    fn get_name(&self) -> &str {
        "technique_common"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        vec![&self.accessor]
    }
}

#[derive(Debug, Clone)]
pub struct Accessor {
    pub count: usize,
    pub offset: Option<usize>,
    pub source: String,
    pub stride: Option<usize>,
    pub param: Vec<ParamAccessor>,
}

impl XMLTag for Accessor {
    fn get_name(&self) -> &str {
        "accessor"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![
            ("count", &self.count),
            ("offset", &self.offset),
            ("source", &self.source),
            ("stride", &self.stride),
        ]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        self.param.iter().map(|x| x as &dyn XMLTag).collect()
    }
}

#[derive(Debug, Clone)]
pub struct ParamAccessor {
    pub name: Option<String>,
    pub sid: Option<String>,
    pub datatype: String,
    pub semantic: Option<String>,
}

impl XMLTag for ParamAccessor {
    fn get_name(&self) -> &str {
        "param"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![
            ("name", &self.name),
            ("sid", &self.sid),
            ("type", &self.datatype),
            ("semantic", &self.semantic),
        ]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        vec![]
    }
}

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

impl XMLTag for Array {
    fn get_name(&self) -> &str {
        use Array::*;
        match self {
            BoolArray { .. } => "bool_array",
            FloatArray { .. } => "float_array",
            IDREFArray { .. } => "IDREF_array",
            IntArray { .. } => "int_array",
            NameArray { .. } => "Name_array",
            SIDREFArray { .. } => "SIDREF_array",
        }
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        use Array::*;
        match self {
            BoolArray {
                count,
                id,
                name,
                data: _,
            } => vec![("count", count), ("id", id), ("name", name)],
            FloatArray {
                count,
                id,
                name,
                digits,
                magnitude,
                data: _,
            } => vec![
                ("count", count),
                ("id", id),
                ("name", name),
                ("magnitude", magnitude),
                ("digits", digits),
            ],
            IDREFArray {
                count,
                id,
                name,
                data: _,
            } => vec![("count", count), ("id", id), ("name", name)],
            IntArray {
                count,
                id,
                name,
                min_inclusive,
                max_inclusive,
                data: _,
            } => vec![
                ("count", count),
                ("id", id),
                ("name", name),
                ("minInclusive", min_inclusive),
                ("maxInclusive", max_inclusive),
            ],
            NameArray {
                count,
                id,
                name,
                data: _,
            } => vec![("count", count), ("id", id), ("name", name)],
            SIDREFArray {
                count,
                id,
                name,
                data: _,
            } => vec![("count", count), ("id", id), ("name", name)],
        }
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        use Array::*;
        match self {
            BoolArray { data, .. } => Some(data),
            FloatArray { data, .. } => Some(data),
            IDREFArray { data, .. } => Some(data),
            IntArray { data, .. } => Some(data),
            NameArray { data, .. } => Some(data),
            SIDREFArray { data, .. } => Some(data),
        }
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct Triangles {
    pub count: usize,
    pub name: Option<String>,
    // material
    pub input: Vec<InputShared>,
    pub p: Option<TriangleData>,
}

impl XMLTag for Triangles {
    fn get_name(&self) -> &str {
        "triangles"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![("count", &self.count), ("name", &self.name)]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
        // self.p.as_ref().map(|x| x as &dyn XMLContent)
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        let mut v: Vec<&dyn XMLTag> = self.input.iter().map(|x| x as &dyn XMLTag).collect();
        if let Some(ref p) = self.p {
            v.push(p);
        }
        v
    }
}

#[derive(Debug, Clone)]
pub struct TriangleData {
    pub data: Vec<usize>,
}

impl XMLTag for TriangleData {
    fn get_name(&self) -> &str {
        "p"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        Some(&self.data)
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

impl XMLTag for InputShared {
    fn get_name(&self) -> &str {
        "input"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![
            ("semantic", &self.semantic),
            ("source", &self.source),
            ("offset", &self.offset),
            ("set", &self.set),
        ]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        Vec::new()
    }
}

#[derive(Debug, Clone)]
pub struct InputUnshared {
    semantic: String,
    source: String,
}

impl XMLTag for InputUnshared {
    fn get_name(&self) -> &str {
        "input"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![("semantic", &self.semantic), ("source", &self.source)]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        Vec::new()
    }
}

#[derive(Debug, Clone)]
pub struct LibraryVisualScene {
    pub visual_scene: Vec<VisualScene>,
    pub id: Option<String>,
    pub name: Option<String>,
}

impl XMLTag for LibraryVisualScene {
    fn get_name(&self) -> &str {
        "library_visual_scenes"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![("id", &self.id), ("name", &self.name)]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        self.visual_scene.iter().map(|x| x as &dyn XMLTag).collect()
    }
}

#[derive(Debug, Clone)]
pub struct VisualScene {
    pub id: Option<String>,
    pub name: Option<String>,
    pub node: Vec<Node>,
    // pub evaluate_scene: Vec<EvaluateScene>,
}

impl XMLTag for VisualScene {
    fn get_name(&self) -> &str {
        "visual_scene"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![("id", &self.id), ("name", &self.name)]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        self.node.iter().map(|x| x as &dyn XMLTag).collect()
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    NODE,
    JOINT,
}

impl XMLAttribute for NodeType {
    fn serialize_attribute(&self) -> Option<String> {
        match self {
            NodeType::NODE => None, // NODE is default
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
    // pub controller: Vec<InstanceController>
    pub geometry: Vec<InstanceGeometry>,
    // pub light: Vec<InstanceLight>
    pub node: Vec<Node>,
}

impl XMLTag for Node {
    fn get_name(&self) -> &str {
        "node"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![
            ("id", &self.id),
            ("name", &self.name),
            ("sid", &self.sid),
            ("type", &self.datatype),
            ("layer", &self.layer),
        ]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        let mut v = Vec::new();
        v.extend(self.transform.iter().map(|x| x as &dyn XMLTag));
        v.extend(self.geometry.iter().map(|x| x as &dyn XMLTag));
        v.extend(self.node.iter().map(|x| x as &dyn XMLTag));
        v
    }
}

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

#[derive(Clone, Debug)]
pub struct InstanceGeometry {
    pub sid: Option<String>,
    pub name: Option<String>,
    pub url: String,
    // pub bind_material: Option<BindMaterial>
}

impl XMLTag for InstanceGeometry {
    fn get_name(&self) -> &str {
        "instance_geometry"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        vec![("name", &self.name), ("sid", &self.sid), ("url", &self.url)]
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        None
    }
    fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
        vec![]
    }
}

fn trimesh_to_geometry(mesh: &scene::SceneTriMesh) -> (Geometry, Node) {
    let id_mesh = format!("{}-mesh", mesh.name);
    let id_positions = format!("{}-positions", id_mesh);
    let id_positions_array = format!("{}-array", id_positions);
    let id_normals = format!("{}-normals", id_mesh);
    let id_normals_array = format!("{}-array", id_normals);
    let id_texcoords = format!("{}-texcoords", id_mesh);
    let id_texcoords_array = format!("{}-array", id_texcoords);
    let id_vertices = format!("{}-vertices", id_mesh);
    (
        Geometry {
            name: Some(mesh.name.clone()),
            id: Some(id_mesh.clone()),
            data: GeometryData::Mesh {
                source: vec![
                    Source {
                        id: id_positions.clone(),
                        name: None,
                        array: Array::FloatArray {
                            id: Some(id_positions_array.clone()),
                            count: mesh.tris.len() * 9,
                            name: None,
                            digits: None,
                            magnitude: None,
                            data: mesh
                                .tris
                                .iter()
                                .flat_map(|x| x.points.iter())
                                .flat_map(|x| vec![x.vertex.x, x.vertex.y, x.vertex.z])
                                .collect(),
                        },
                        technique_common: Some(SourceTechnique {
                            accessor: Accessor {
                                count: mesh.tris.len() * 3,
                                offset: None,
                                source: format!("#{}", id_positions_array.clone()),
                                stride: Some(3),
                                param: vec![
                                    ParamAccessor {
                                        name: Some("X".to_owned()),
                                        sid: None,
                                        datatype: "float".to_owned(),
                                        semantic: None,
                                    },
                                    ParamAccessor {
                                        name: Some("Y".to_owned()),
                                        sid: None,
                                        datatype: "float".to_owned(),
                                        semantic: None,
                                    },
                                    ParamAccessor {
                                        name: Some("Z".to_owned()),
                                        sid: None,
                                        datatype: "float".to_owned(),
                                        semantic: None,
                                    },
                                ],
                            },
                        }),
                    },
                    Source {
                        id: id_texcoords.clone(),
                        name: None,
                        array: Array::FloatArray {
                            id: Some(id_texcoords_array.clone()),
                            count: mesh.tris.len() * 6,
                            name: None,
                            digits: None,
                            magnitude: None,
                            data: mesh
                                .tris
                                .iter()
                                .flat_map(|x| x.points.iter())
                                .flat_map(|x| vec![x.texcoord.x, x.texcoord.y])
                                .collect(),
                        },
                        technique_common: Some(SourceTechnique {
                            accessor: Accessor {
                                count: mesh.tris.len() * 3,
                                offset: None,
                                source: format!("#{}", id_texcoords_array.clone()),
                                stride: Some(2),
                                param: vec![
                                    ParamAccessor {
                                        name: Some("S".to_owned()),
                                        sid: None,
                                        datatype: "float".to_owned(),
                                        semantic: None,
                                    },
                                    ParamAccessor {
                                        name: Some("T".to_owned()),
                                        sid: None,
                                        datatype: "float".to_owned(),
                                        semantic: None,
                                    },
                                ],
                            },
                        }),
                    },
                    Source {
                        id: id_normals.clone(),
                        name: None,
                        array: Array::FloatArray {
                            id: Some(id_normals_array.clone()),
                            count: mesh.tris.len() * 9,
                            name: None,
                            digits: None,
                            magnitude: None,
                            data: mesh
                                .tris
                                .iter()
                                .flat_map(|x| x.points.iter())
                                .flat_map(|x| vec![x.normal.x, x.normal.y, x.normal.z])
                                .collect(),
                        },
                        technique_common: Some(SourceTechnique {
                            accessor: Accessor {
                                count: mesh.tris.len() * 3,
                                offset: None,
                                source: format!("#{}", id_normals_array.clone()),
                                stride: Some(3),
                                param: vec![
                                    ParamAccessor {
                                        name: Some("X".to_owned()),
                                        sid: None,
                                        datatype: "float".to_owned(),
                                        semantic: None,
                                    },
                                    ParamAccessor {
                                        name: Some("Y".to_owned()),
                                        sid: None,
                                        datatype: "float".to_owned(),
                                        semantic: None,
                                    },
                                    ParamAccessor {
                                        name: Some("Z".to_owned()),
                                        sid: None,
                                        datatype: "float".to_owned(),
                                        semantic: None,
                                    },
                                ],
                            },
                        }),
                    },
                ],
                vertices: Vertices {
                    id: id_vertices.clone(),
                    name: None,
                    input: vec![InputUnshared {
                        semantic: "POSITION".to_owned(),
                        source: format!("#{}", id_positions),
                    }],
                },
                triangles: vec![Triangles {
                    count: mesh.tris.len(),
                    name: None,
                    input: vec![
                        InputShared {
                            semantic: "VERTEX".to_owned(),
                            source: format!("#{}", id_vertices),
                            offset: 0,
                            set: None,
                        },
                        InputShared {
                            semantic: "TEXCOORD".to_owned(),
                            source: format!("#{}", id_texcoords),
                            offset: 1,
                            set: None,
                        },
                        InputShared {
                            semantic: "NORMAL".to_owned(),
                            source: format!("#{}", id_normals),
                            offset: 2,
                            set: Some(0),
                        },
                    ],
                    p: Some(TriangleData {
                        data: (0..mesh.tris.len()*3)
                            .into_iter()
                            .flat_map(|i| vec![i, i, i])
                            .collect(),
                    }),
                }],
            },
        },
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
            geometry: vec![InstanceGeometry {
                url: format!("#{}", id_mesh),
                name: Some(mesh.name.clone()),
                sid: None,
            }],
            node: vec![],
        },
    )
}

fn scene_to_collada(scene: &scene::Scene) -> COLLADA {
    let mut geometries = Vec::new();
    let mut nodes = Vec::new();
    for (geometry, node) in scene.trimeshes.iter().map(|x| trimesh_to_geometry(x)) {
        geometries.push(geometry);
        nodes.push(node);
    }
    COLLADA {
        library_geometry: vec![LibraryGeometry {
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
    #[test]
    pub fn test_to_string_single_mesh() {
        use crate::common::*;
        use crate::scene;
        use crate::scene::collada;
        let scene = scene::Scene {
            trimeshes: vec![scene::SceneTriMesh {
                name: "Wee".to_owned(),
                transform: Mat4x4::new_basis(),
                tris: vec![Tri {
                    points: [
                        Point {
                            vertex: Vector3::with(0.0, 0.0, 0.0),
                            texcoord: Vector2::with(0.0, 0.0),
                            normal: Vector3::with(1.0, 0.0, 0.0),
                        },
                        Point {
                            vertex: Vector3::with(-1.0, 0.0, 0.0),
                            texcoord: Vector2::with(0.0, 0.0),
                            normal: Vector3::with(1.0, 0.0, 0.0),
                        },
                        Point {
                            vertex: Vector3::with(0.0, 1.0, 1.0),
                            texcoord: Vector2::with(0.0, 0.0),
                            normal: Vector3::with(1.0, 0.0, 0.0),
                        },
                    ],
                }],
            }],
        };
        println!("{}", collada::scene_to_string_dae(&scene).unwrap());
    }
}
