use crate::reader::material;
use crate::common::*;
use crate::format::TotemFormat;
use std::io::{self, Read};
use std::fmt;
use std::error::Error;

// node union
const T_ROTSHAPEDATA: i32 =   733875652;
const T_MESHDATA: i32 =     -1724712303;
const T_SKEL: i32 =          1985457034;
const T_SURFACEDATAS: i32 =   413080818;
const T_LODDATA: i32 =       -141015160;
const T_PARTICLESDATA: i32 = -241612565;
// extra data union
const E_USERDATA: i32 = -1879206489;

#[derive(Debug, Clone)]
pub enum NodeReadError {
    InvalidNodeData(i32),
    InvalidNodeSkinUnk2ExtraData(i32),
}

impl fmt::Display for NodeReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NodeReadError::*;
        match self {
            InvalidNodeData(i) => write!(f, "Invalid Node Data Type: {}", i),
            InvalidNodeSkinUnk2ExtraData(i) => write!(f, "Invalid Skin Extra Data Type: {}", i),
        }
    }
}

impl Error for NodeReadError {}

pub struct Node {
    pub node_parent_id: i32,
    pub node_unk_ids: [i32; 3],
    pub resource_id: i32,
    pub node_data: NodeDataUnion,
    pub light_id: i32,
    pub hfog_id: i32,
    pub userdefine_id: i32,
    pub floatv1: [f32; 9],
    pub floatv2: [f32; 9],
    pub local_transform: Mat4x4,
    pub local_translation: Vector3,
    pub local_rotation: Quaternion,
    pub local_scale: Vector3,
    pub unk1: [f32; 2],
    pub unk2: [u32; 8],
    pub unk3: [f32; 4],
    pub unk4: [u16; 2],
    pub global_transform: Mat4x4,
    pub global_transform_inverse: Mat4x4
}

impl Node {
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> Result<Node, Box<dyn Error>> {
        Ok(Node {
            node_parent_id: fmt.read_i32(file)?,
            node_unk_ids: [
                fmt.read_i32(file)?,
                fmt.read_i32(file)?,
                fmt.read_i32(file)?,
            ],
            resource_id: fmt.read_i32(file)?,
            node_data: NodeDataUnion::read_from(file, fmt)?,
            light_id: fmt.read_i32(file)?,
            hfog_id: fmt.read_i32(file)?,
            userdefine_id: fmt.read_i32(file)?,
            floatv1: {
                let mut data = [0f32; 9];
                fmt.read_f32_into(file, &mut data)?;
                data
            },
            floatv2: {
                let mut data = [0f32; 9];
                fmt.read_f32_into(file, &mut data)?;
                data
            },
            local_transform: read_mat4(file, fmt)?,
            local_translation: {
                let v = read_vec3(file, fmt)?;
                fmt.skip_n_bytes(file, 4)?;
                v
            },
            local_rotation: read_quat(file, fmt)?,
            local_scale: {
                let v = read_vec3(file, fmt)?;
                fmt.skip_n_bytes(file, 4)?;
                v
            },
            unk1: [
                fmt.read_f32(file)?,
                fmt.read_f32(file)?
            ],
            unk2: {
                let mut data = [0u32; 8];
                fmt.read_u32_into(file, &mut data)?;
                data
            },
            unk3: {
                let mut data = [0f32; 4];
                fmt.read_f32_into(file, &mut data)?;
                data
            },
            unk4: [
                fmt.read_u16(file)?,
                fmt.read_u16(file)?
            ],
            global_transform: read_mat4(file, fmt)?,
            global_transform_inverse: read_mat4(file, fmt)?
        })
    }
}

/// String                Hash | Resource Type
/// ------------------------------------------
///                          0 | (empty)
/// ROTSHAPEDATA     733875652 | ROTSHAPE
/// MESHDATA       -1724712303 | MESH
/// SKEL            1985457034 | SKIN
/// SURFACEDATAS     413080818 | SURFACE
/// LODDATA         -141015160 | LOD
/// PARTICLESDATA   -241612565 | PARTICLES
pub enum NodeDataUnion {
    Empty,
    NodeDataLod {
        path_id: i32,
        subtype_id: i32,
        unk1: [f32; 5],
        data: Vec<NodeDataUnion>,
        unk2: [u8; 100],
        node_id: i32,
        light1_id: i32,
        light2_id: i32,
        nodes: Vec<i32>,
        unk3: Vec<u32>
    },
    NodeDataSkin {
        path_id: i32,
        subtype_id: i32,
        unk1: [f32; 5],
        unk2: Vec<NodeSkinUnk2>,
        unk3_id: i32,
        materials: Vec<NodeSkinMaterial>,
        unk4: Vec<NodeSkinUnk>,
        unk5: Vec<NodeSkinUnk>,
        unk6: Vec<NodeSkinUnk>,
        unk7: Vec<NodeSkinUnk7>
    },
    NodeDataSurface {
        data_id: i32,
        subtype_id: i32,
        data: [f32; 5],
        unk1: Vec<NodeDataSurfaceUnk>,
        unk2: u32,
        unk3: u32
    },
    NodeDataRotshape {
        data_id: i32,
        subtype_id: i32,
        unk1: [u32; 6],
        unk2: u16,
        junk: [u8; 28]
    },
    NodeDataMesh {
        data_id: i32,
        subtype_id: i32,
        data: [f32; 5],
    },
    NodeDataParticles {
        data_id: i32,
        subtype_id: i32,
        unk1: [f32; 5],
        unk2: u16
    }
}

impl NodeDataUnion {
    fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> Result<NodeDataUnion, Box<dyn Error>> {
        use NodeDataUnion::*;
        let datatype = fmt.read_i32(file)?;
        match datatype {
            0 => Ok(Empty),
            T_ROTSHAPEDATA => { // ROTSHAPEDATA
                Ok(NodeDataRotshape {
                    data_id: fmt.read_i32(file)?,
                    subtype_id: fmt.read_i32(file)?,
                    unk1: {
                        let mut data = [0u32; 6];
                        fmt.read_u32_into(file, &mut data)?;
                        data
                    },
                    unk2: fmt.read_u16(file)?,
                    junk: {
                        let mut data = [0u8; 28];
                        fmt.read_u8_into(file, &mut data)?;
                        data
                    }
                })
            },
            T_MESHDATA => { // MESHDATA
                Ok(NodeDataMesh {
                    data_id: fmt.read_i32(file)?,
                    subtype_id: fmt.read_i32(file)?,
                    data: {
                        let mut data = [0f32; 5];
                        fmt.read_f32_into(file, &mut data)?;
                        data
                    }
                })
            },
            T_SKEL => { // SKEL
                Ok(NodeDataSkin{
                    path_id: fmt.read_i32(file)?,
                    subtype_id: fmt.read_i32(file)?,
                    unk1: {
                        let mut data = [0f32; 5];
                        fmt.read_f32_into(file, &mut data)?;
                        data
                    },
                    unk2: {
                        let num = fmt.read_u32(file)?;
                        let mut v = Vec::with_capacity(num as usize);
                        for _ in 0..num {
                            v.push(NodeSkinUnk2::read_from(file, fmt)?);
                        }
                        v
                    },
                    unk3_id: fmt.read_i32(file)?,
                    materials: {
                        let num = fmt.read_u32(file)?;
                        let mut v = Vec::with_capacity(num as usize);
                        for _ in 0..num {
                            v.push(NodeSkinMaterial::read_from(file, fmt)?);
                        }
                        v
                    },
                    unk4: {
                        let num = fmt.read_u32(file)?;
                        let mut v = Vec::with_capacity(num as usize);
                        for _ in 0..num {
                            v.push(NodeSkinUnk::read_from(file, fmt)?);
                        }
                        v
                    },
                    unk6: {
                        let num = fmt.read_u32(file)?;
                        let mut v = Vec::with_capacity(num as usize);
                        for _ in 0..num {
                            v.push(NodeSkinUnk::read_from(file, fmt)?);
                        }
                        v
                    },
                    unk5: {
                        let num = fmt.read_u32(file)?;
                        let mut v = Vec::with_capacity(num as usize);
                        for _ in 0..num {
                            v.push(NodeSkinUnk::read_from(file, fmt)?);
                        }
                        v
                    },
                    unk7: NodeSkinUnk7::read_from(file, fmt)?
                })
            }
            T_SURFACEDATAS => { // SURFACEDATAS
                Ok(NodeDataSurface {
                    data_id: fmt.read_i32(file)?,
                    subtype_id: fmt.read_i32(file)?,
                    data: {
                        let mut data = [0f32; 5];
                        fmt.read_f32_into(file, &mut data)?;
                        data
                    },
                    unk1: {
                        let num = fmt.read_u32(file)?;
                        let mut data = Vec::with_capacity(num as usize);
                        for _ in 0..num {
                            data.push(NodeDataSurfaceUnk::read_from(file, fmt)?);
                        }
                        data
                    },
                    unk2: fmt.read_u32(file)?,
                    unk3: fmt.read_u32(file)?
                })
            },
            T_LODDATA => { // LODDATA
                Ok(NodeDataLod{
                    path_id: fmt.read_i32(file)?,
                    subtype_id: fmt.read_i32(file)?,
                    unk1: {
                        let mut data = [0f32; 5];
                        fmt.read_f32_into(file, &mut data)?;
                        data
                    },
                    data: {
                        let num = fmt.read_u32(file)?;
                        let mut v = Vec::with_capacity(num as usize);
                        for _ in 0..num {
                            v.push(NodeDataUnion::read_from(file, fmt)?);
                        }
                        v
                    },
                    unk2: {
                        let mut data = [0u8; 100];
                        fmt.read_u8_into(file, &mut data)?;
                        data
                    },
                    node_id: fmt.read_i32(file)?,
                    light1_id: fmt.read_i32(file)?,
                    light2_id: fmt.read_i32(file)?,
                    nodes: {
                        let num = fmt.read_u32(file)?;
                        let mut v = Vec::with_capacity(num as usize);
                        for _ in 0..num {
                            v.push(fmt.read_i32(file)?);
                        }
                        v
                    },
                    unk3: {
                        let num = fmt.read_u32(file)?;
                        let mut v = Vec::with_capacity(num as usize);
                        for _ in 0..num {
                            v.push(fmt.read_u32(file)?);
                        }
                        v
                    }
                })
            },
            T_PARTICLESDATA => { // PARTICLESDATA
                Ok(NodeDataParticles{
                    data_id: fmt.read_i32(file)?,
                    subtype_id: fmt.read_i32(file)?,
                    unk1: {
                        let mut data = [0f32; 5];
                        fmt.read_f32_into(file, &mut data)?;
                        data
                    },
                    unk2: fmt.read_u16(file)?
                })
            }
            i => Err(NodeReadError::InvalidNodeData(i))?
        }
    }
}

pub struct NodeDataSurfaceUnk {
    pub data: [u8; 104]
}

impl NodeDataSurfaceUnk {
    fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<NodeDataSurfaceUnk> {
        let mut data = [0u8; 104];
        fmt.read_u8_into(file, &mut data)?;
        Ok(NodeDataSurfaceUnk {
            data
        })
    }
}

pub struct NodeSkinUnk2 {
    pub unk_ids: [i32; 5],
    pub extra_data: NodeSkinUnk2ExtraDataUnion,
    pub local_translation: Vector3,
    pub local_rotation: Quaternion,
    pub local_scale: Vector3,
    pub floatv1: [f32; 9],
    pub floatv2: [f32; 9],
    pub tx1: Mat4x4,
    pub tx2: Mat4x4
}

impl NodeSkinUnk2 {
    fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> Result<NodeSkinUnk2, Box<dyn Error>> {
        let unk_ids = {
            let mut data = [i32::default(); 5];
            fmt.read_i32_into(file, &mut data)?;
            data
        };
        let extra_data = NodeSkinUnk2ExtraDataUnion::read_from(file, fmt)?;
        let local_translation = read_vec3(file, fmt)?;
        fmt.skip_n_bytes(file, 4)?;
        let local_rotation = read_quat(file, fmt)?;
        let local_scale = read_vec3(file, fmt)?;
        let floatv1 = {
            let mut data = [f32::default(); 9];
            fmt.read_f32_into(file, &mut data)?;
            data
        };
        let floatv2 = {
            let mut data = [f32::default(); 9];
            fmt.read_f32_into(file, &mut data)?;
            data
        };
        let tx1 = read_mat4(file, fmt)?;
        let tx2 = read_mat4(file, fmt)?;
        Ok(NodeSkinUnk2 {
            unk_ids,
            extra_data,
            local_translation,
            local_rotation,
            local_scale,
            floatv1,
            floatv2,
            tx1,
            tx2
        })
    }
}

/// String              Hash | Resource Type
/// ------------------------------------------------
///                        0 | (empty)
/// USERDEFINE   -1879206489 | USERDEFINE (embedded)
pub enum NodeSkinUnk2ExtraDataUnion {
    Empty,
    UserDefine {
        type1: i32,
        type2: i32,
        data: Vec<u8>,
    }
}

impl NodeSkinUnk2ExtraDataUnion {
    fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> Result<NodeSkinUnk2ExtraDataUnion, Box<dyn Error>> {
        let datatype = fmt.read_i32(file)?;
        match datatype {
            0 => Ok(NodeSkinUnk2ExtraDataUnion::Empty),
            E_USERDATA => {
                let type1 = fmt.read_i32(file)?;
                let type2 = fmt.read_i32(file)?;
                let length = fmt.read_u32(file)?;
                let mut data = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    data.push(fmt.read_u8(file)?);
                }
                Ok(NodeSkinUnk2ExtraDataUnion::UserDefine {
                    type1,
                    type2,
                    data
                })
            },
            i => Err(NodeReadError::InvalidNodeSkinUnk2ExtraData(i))?
        }
    }
}

pub struct NodeSkinMaterial {
    pub filetype_id: i32,
    pub filename_id: i32,
    pub subtype_id: i32,
    pub material: material::Material,
}

impl NodeSkinMaterial {
    fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<NodeSkinMaterial> {
        Ok(NodeSkinMaterial {
            filetype_id: fmt.read_i32(file)?,
            filename_id: fmt.read_i32(file)?,
            subtype_id: fmt.read_i32(file)?,
            material: material::Material::read_from(file, fmt)?
        })
    }
}

pub struct NodeSkinUnk {
    pub unk1: [f32; 4],
    pub unk2_id: i32,
    pub unk3_id: i32
}

impl NodeSkinUnk {
    fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<NodeSkinUnk> {
        let mut unk1 = [0.0f32; 4];
        fmt.read_f32_into(file, &mut unk1)?;
        Ok(NodeSkinUnk {
            unk1,
            unk2_id: fmt.read_i32(file)?,
            unk3_id: fmt.read_i32(file)?,
        })
    }
}

pub struct NodeSkinUnk7 {
    pub data: NodeDataUnion,
    pub ids: Vec<i32>
}

impl NodeSkinUnk7 {
    fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> Result<Vec<NodeSkinUnk7>, Box<dyn Error>> {
        let num = fmt.read_u32(file)?;
        let mut v: Vec<NodeSkinUnk7> = Vec::with_capacity(num as usize);
        for _ in 0..num {
            v.push(NodeSkinUnk7 {
                data: NodeDataUnion::read_from(file, fmt)?,
                ids: Vec::new()
            });
        }
        for i in 0..num {
            let num_ids = fmt.read_u32(file)?;
            let mut ids = vec![0; num_ids as usize];
            fmt.read_i32_into(file, &mut ids)?;
            v[i as usize].ids = ids;
        }
        Ok(v)
    }
}