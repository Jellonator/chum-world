use crate::scene;
use crate::scene::collada::*;
use crate::util::xml::{XMLAttribute, XMLContent, XMLTag};

#[derive(Debug, Clone)]
pub struct LibraryGeometry {
    pub geometry: Vec<Geometry>,
    pub id: Option<String>,
    pub name: Option<String>,
}

impl_tag_tree!(
    LibraryGeometry,
    "library_geometries",
    attr => [("id", id), ("name", name)],
    tags => [geometry]
);

#[derive(Debug, Clone)]
pub struct Geometry {
    pub id: Option<String>,
    pub name: Option<String>,
    pub data: GeometryData,
}

impl_tag_tree!(
    Geometry,
    "geometry",
    attr => [("id", id), ("name", name)],
    tags => [data]
);

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

impl_tag_enum! (
    GeometryData,
    Mesh => (
        "mesh",
        attr => [],
        tags => [source, vertices, triangles]
    )
);

#[derive(Debug, Clone)]
pub struct Vertices {
    pub id: String,
    pub name: Option<String>,
    pub input: Vec<InputUnshared>,
}

impl_tag_tree!(
    Vertices,
    "vertices",
    attr => [("id", id), ("name", name)],
    tags => [input]
);

#[derive(Debug, Clone)]
pub struct Triangles {
    pub count: usize,
    pub name: Option<String>,
    // material
    pub input: Vec<InputShared>,
    pub p: Option<TriangleData>,
}

impl_tag_tree!(
    Triangles,
    "triangles",
    attr => [("name", name), ("count", count)],
    tags => [input, p]
);

#[derive(Debug, Clone)]
pub struct TriangleData {
    pub data: Vec<usize>,
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

impl_tag_content!(TriangleData, "p", data);

pub fn trimesh_to_geometry(mesh: &scene::SceneTriMesh) -> Geometry {
    let id_mesh = format!("{}-mesh", mesh.name);
    let id_positions = format!("{}-positions", id_mesh);
    let id_positions_array = format!("{}-array", id_positions);
    let id_normals = format!("{}-normals", id_mesh);
    let id_normals_array = format!("{}-array", id_normals);
    let id_texcoords = format!("{}-texcoords", id_mesh);
    let id_texcoords_array = format!("{}-array", id_texcoords);
    let id_vertices = format!("{}-vertices", id_mesh);
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
                        count: mesh.vertices.len() * 3,
                        name: None,
                        digits: None,
                        magnitude: None,
                        data: mesh
                            .vertices
                            .iter()
                            .flat_map(|x| vec![x.x, x.y, x.z])
                            .collect(),
                    },
                    technique_common: Some(SourceTechnique {
                        accessor: Accessor {
                            count: mesh.vertices.len(),
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
                        count: mesh.texcoords.len() * 2,
                        name: None,
                        digits: None,
                        magnitude: None,
                        data: mesh.texcoords.iter().flat_map(|x| vec![x.x, x.y]).collect(),
                    },
                    technique_common: Some(SourceTechnique {
                        accessor: Accessor {
                            count: mesh.texcoords.len(),
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
                        count: mesh.normals.len() * 3,
                        name: None,
                        digits: None,
                        magnitude: None,
                        data: mesh
                            .normals
                            .iter()
                            .flat_map(|x| vec![x.x, x.y, x.z])
                            .collect(),
                    },
                    technique_common: Some(SourceTechnique {
                        accessor: Accessor {
                            count: mesh.normals.len(),
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
                count: mesh.elements.len(),
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
                        set: Some(0),
                    },
                    InputShared {
                        semantic: "NORMAL".to_owned(),
                        source: format!("#{}", id_normals),
                        offset: 2,
                        set: None,
                    },
                ],
                p: Some(TriangleData {
                    data: mesh
                        .elements
                        .iter()
                        .flat_map(|x| {
                            vec![
                                x[0].0, x[0].1, x[0].2, x[1].0, x[1].1, x[1].2, x[2].0, x[2].1,
                                x[2].2,
                            ]
                        })
                        .collect(),
                }),
            }],
        },
    }
}
