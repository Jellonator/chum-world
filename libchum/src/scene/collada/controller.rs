use crate::scene::collada::*;
use crate::util::xml::{XMLAttribute, XMLContent, XMLTag};

#[derive(Clone, Debug)]
pub struct LibraryControllers {
    pub id: Option<String>,
    pub name: Option<String>,
    pub controller: Vec<Controller>,
}

impl_tag_tree!(
    LibraryControllers,
    "library_controllers",
    attr => [("id", id), ("name", name)],
    tags => [controller]
);

#[derive(Clone, Debug)]
pub struct Controller {
    pub id: Option<String>,
    pub name: Option<String>,
    pub data: ControllerData,
}

impl_tag_tree!(
    Controller,
    "controller",
    attr => [("id", id), ("name", name)],
    tags => [data]
);

#[derive(Clone, Debug)]
pub enum ControllerData {
    Skin {
        source: String,
        bind_shape_matrix: Option<BindShapeMatrix>,
        source_elements: Vec<Source>,
        joints: Joints,
        vertex_weights: VertexWeights,
    },
    // Morph
}

impl_tag_enum!(
    ControllerData,
    Skin => (
        "skin",
        attr => [("source", source)],
        tags => [
            bind_shape_matrix,
            source_elements,
            joints,
            vertex_weights
        ]
    )
);

#[derive(Clone, Debug)]
pub struct BindShapeMatrix {
    data: common::Mat4x4,
}

impl_tag_content!(BindShapeMatrix, "bind_shape_matrix", data);

#[derive(Clone, Debug)]
pub struct Joints {
    pub input: Vec<InputUnshared>,
}

impl_tag_tree!(
    Joints,
    "joints",
    attr => [],
    tags => [input]
);

#[derive(Clone, Debug)]
pub struct VertexWeights {
    pub count: usize,
    pub input: Vec<InputShared>,
    pub vcount: Option<VertexWeightCounts>,
    pub vindices: Option<VertexWeightIndices>,
}

impl_tag_tree!(
    VertexWeights,
    "vertex_weights",
    attr => [("count", count)],
    tags => [input, vcount, vindices]
);

#[derive(Clone, Debug)]
pub struct VertexWeightCounts {
    pub data: Vec<usize>,
}

impl_tag_content!(VertexWeightCounts, "vcount", data);

#[derive(Clone, Debug)]
pub struct VertexWeightIndices {
    pub data: Vec<usize>,
}

impl_tag_content!(VertexWeightIndices, "v", data);

// scene stuff

#[derive(Clone, Debug)]
pub struct InstanceController {
    pub sid: Option<String>,
    pub name: Option<String>,
    pub url: String,
    pub skeleton: Vec<Skeleton>,
    // pub bind_material: BindMaterial
}

impl_tag_tree!(
    InstanceController,
    "instance_controller",
    attr => [("sid", sid), ("name", name), ("url", url)],
    tags => [skeleton]
);

#[derive(Clone, Debug)]
pub struct Skeleton {
    pub data: String,
}

impl_tag_content!(Skeleton, "skeleton", data);

pub fn skin_to_controller(skin: &scene::SceneSkin, meshname: &str) -> Controller {
    let id_skin = format!("{}-skin", meshname);
    let id_skin_joints = format!("{}-joints", id_skin);
    let id_skin_joints_array = format!("{}-array", id_skin_joints);
    let id_skin_bind_poses = format!("{}-bind_poses", id_skin);
    let id_skin_bind_poses_array = format!("{}-array", id_skin_bind_poses);
    let id_skin_weights = format!("{}-weights", id_skin);
    let id_skin_weights_array = format!("{}-array", id_skin_weights);
    let id_mesh = format!("{}-mesh", meshname);
    Controller {
        id: Some(id_skin.clone()),
        name: Some(id_skin.clone()),
        data: ControllerData::Skin {
            bind_shape_matrix: Some(BindShapeMatrix {
                data: common::Mat4x4::identity(),
            }),
            source: format!("#{}", id_mesh),
            source_elements: vec![
                Source {
                    id: id_skin_joints.clone(),
                    name: None,
                    array: Array::NameArray {
                        count: skin.groups.len(),
                        id: Some(id_skin_joints_array.clone()),
                        name: None,
                        data: skin.groups.iter().map(|group| group.name.clone()).collect(),
                    },
                    technique_common: Some(SourceTechnique {
                        accessor: Accessor {
                            count: skin.groups.len(),
                            stride: Some(1),
                            source: format!("#{}", id_skin_joints_array),
                            offset: None,
                            param: vec![ParamAccessor {
                                name: Some("JOINT".to_owned()),
                                datatype: "name".to_owned(),
                                sid: None,
                                semantic: None,
                            }],
                        },
                    }),
                },
                Source {
                    id: id_skin_bind_poses.clone(),
                    name: None,
                    array: Array::FloatArray {
                        count: skin.groups.len() * 16,
                        id: Some(id_skin_bind_poses_array.clone()),
                        name: None,
                        digits: None,
                        magnitude: None,
                        data: skin
                            .groups
                            .iter()
                            .flat_map(|group| {
                                group
                                    .transform
                                    .try_inverse()
                                    .unwrap_or(common::Mat4x4::identity())
                                    .as_slice()
                                    .to_vec()
                            })
                            .collect(),
                    },
                    technique_common: Some(SourceTechnique {
                        accessor: Accessor {
                            count: skin.groups.len(),
                            stride: Some(16),
                            source: format!("#{}", id_skin_bind_poses_array),
                            offset: None,
                            param: vec![ParamAccessor {
                                name: Some("TRANSFORM".to_owned()),
                                datatype: "float4x4".to_owned(),
                                sid: None,
                                semantic: None,
                            }],
                        },
                    }),
                },
                Source {
                    id: id_skin_weights.clone(),
                    name: None,
                    array: Array::FloatArray {
                        count: skin
                            .vertices
                            .iter()
                            .flat_map(|x| x.influences.iter())
                            .count(),
                        id: Some(id_skin_weights_array.clone()),
                        name: None,
                        digits: None,
                        magnitude: None,
                        data: skin
                            .vertices
                            .iter()
                            .flat_map(|x| x.influences.iter())
                            .map(|x| x.weight)
                            .collect(),
                    },
                    technique_common: Some(SourceTechnique {
                        accessor: Accessor {
                            count: skin
                                .vertices
                                .iter()
                                .flat_map(|x| x.influences.iter())
                                .count(),
                            stride: Some(1),
                            source: format!("#{}", id_skin_weights_array),
                            offset: None,
                            param: vec![ParamAccessor {
                                name: Some("WEIGHT".to_owned()),
                                datatype: "float".to_owned(),
                                sid: None,
                                semantic: None,
                            }],
                        },
                    }),
                },
            ],
            joints: Joints {
                input: vec![
                    InputUnshared {
                        semantic: "JOINT".to_owned(),
                        source: format!("#{}", id_skin_joints),
                    },
                    InputUnshared {
                        semantic: "INV_BIND_MATRIX".to_owned(),
                        source: format!("#{}", id_skin_bind_poses),
                    },
                ],
            },
            vertex_weights: VertexWeights {
                count: skin.vertices.len(),
                input: vec![
                    InputShared {
                        semantic: "JOINT".to_owned(),
                        source: format!("#{}", id_skin_joints),
                        offset: 0,
                        set: None,
                    },
                    InputShared {
                        semantic: "WEIGHT".to_owned(),
                        source: format!("#{}", id_skin_weights),
                        offset: 1,
                        set: None,
                    },
                ],
                vcount: Some(VertexWeightCounts {
                    data: skin.vertices.iter().map(|x| x.influences.len()).collect(),
                }),
                vindices: Some(VertexWeightIndices {
                    data: skin
                        .vertices
                        .iter()
                        .flat_map(|x| x.influences.iter())
                        .enumerate()
                        .flat_map(|(i, x)| vec![x.joint, i])
                        .collect(),
                }),
            },
        },
    }
}
