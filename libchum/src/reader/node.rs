use crate::reader::material;
use std::error::Error;
use std::fmt;
use crate::format::TotemFormat;
use crate::util::error::*;
use std::io::Read;

// node union
const T_ROTSHAPEDATA: i32 = 733875652;
const T_MESHDATA: i32 = -1724712303;
const T_SKEL: i32 = 1985457034;
const T_SURFACEDATAS: i32 = 413080818;
const T_LODDATA: i32 = -141015160;
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

chum_struct_generate_readwrite! {
    #[derive(Clone, Default)]
    pub struct Node {
        pub node_parent_id: [reference NODE],
        pub node_unk_ids: [fixed array [reference NODE] 3],
        pub resource_id: [reference],
        pub node_data: [struct NodeDataUnion],
        pub light_id: [reference LIGHT],
        pub hfog_id: [reference HFOG],
        pub userdefine_id: [reference USERDEFINE],
        pub floatv1: [fixed array [f32] 9],
        pub floatv2: [fixed array [f32] 9],
        pub local_transform: [Mat4x4],
        pub local_translation: [Vector3],
        pub junk1: [ignore [fixed array [u8] 4] [0u8; 4]],
        pub local_rotation: [Quaternion],
        pub local_scale: [Vector3],
        pub junk2: [ignore [fixed array [u8] 4] [0u8; 4]],
        pub unk1: [fixed array [f32] 2],
        pub unk2: [fixed array [u32] 8],
        pub unk3: [fixed array [f32] 4],
        pub unk4: [fixed array [u16] 2],
        pub global_transform: [Mat4x4],
        pub global_transform_inverse: [Mat4x4],
    }
}

// String                Hash | Resource Type
// ------------------------------------------
//                          0 | (empty)
// ROTSHAPEDATA     733875652 | ROTSHAPE
// MESHDATA       -1724712303 | MESH
// SKEL            1985457034 | SKIN
// SURFACEDATAS     413080818 | SURFACE
// LODDATA         -141015160 | LOD
// PARTICLESDATA   -241612565 | PARTICLES
chum_struct_enum! {
    #[derive(Clone)]
    pub enum NodeDataUnion [i32] {
        Empty: 0 => {},
        NodeDataLod: T_LODDATA => {
            path_id: [i32] = 0i32,
            subtype_id: [i32] = 0i32,
            unk1: [fixed array [f32] 5] = [0f32;5],
            data: [dynamic array [u32] [struct NodeDataUnion] NodeDataUnion::Empty{}] = Vec::new(),
            unk2: [fixed array [u8] 100] = [0u8;100],
            node_id: [reference NODE] = 0i32,
            light1_id: [reference LIGHT] = 0i32,
            light2_id: [reference LIGHT] = 0i32,
            nodes: [dynamic array [u32] [reference NODE] 0i32] = Vec::new(),
            unk3: [dynamic array [u32] [u32] 0u32] = Vec::new(),
        },
        NodeDataSkin: T_SKEL => {
            path_id: [i32] = 0i32,
            subtype_id: [i32] = 0i32,
            unk1: [fixed array [f32] 5] = [0f32; 5],
            unk2: [dynamic array [u32] [struct NodeSkinUnk2] NodeSkinUnk2::default()] = Vec::new(),
            unk3_id: [i32] = 0i32,
            materials: [dynamic array [u32] [struct NodeSkinMaterial] NodeSkinMaterial::default()] = Vec::new(),
            unk4: [dynamic array [u32] [struct NodeSkinUnk] NodeSkinUnk::default()] = Vec::new(),
            unk5: [dynamic array [u32] [struct NodeSkinUnk] NodeSkinUnk::default()] = Vec::new(),
            unk6: [dynamic array [u32] [struct NodeSkinUnk] NodeSkinUnk::default()] = Vec::new(),
            unk7: [custom_binary
                [dynamic array [u32] [struct NodeSkinUnk7] NodeSkinUnk7::default()]
                read: |_skel: &Inner, file: &mut dyn Read, fmt: TotemFormat| -> StructUnpackResult<Vec<NodeSkinUnk7>> {
                    // Another one of Joker's tricks
                    // also while you're here, look at the amount of code that
                    // is required to read a few integers with proper error handling.
                    // THIS is why I am using macros.
                    let len = fmt.read_u32(file).map_err(|e| StructUnpackError {
                        structname: "NodeDataUnion::NodeDataSkin".to_owned(),
                        structpath: "unk7".to_owned(),
                        error: Box::new(e)
                    })? as usize;
                    let mut value = vec![NodeSkinUnk7::default(); len];
                    for i in 0..len {
                        value[i].data = NodeDataUnion::read_from(file, fmt).map_err(|e|
                            e.structuralize("NodeDataUnion::NodeDataSkin", &format!("unk7[{}].data", i))
                        )?;
                    }
                    for i in 0..len {
                        let inner_len = fmt.read_u32(file).map_err(|e| StructUnpackError {
                            structname: "NodeDataUnion::NodeDataSkin".to_owned(),
                            structpath: format!("unk7[{}].ids", i),
                            error: Box::new(e)
                        })? as usize;
                        let mut ids = Vec::with_capacity(inner_len.min(1000usize));
                        for j in 0..inner_len {
                            ids.push(fmt.read_i32(file).map_err(|e| StructUnpackError {
                                structname: "NodeDataUnion::NodeDataSkin".to_owned(),
                                structpath: format!("unk7[{}].ids[{}]", i, j),
                                error: Box::new(e)
                            })?);
                        }
                        value[i].ids = ids;
                    }
                    Ok(value)
                };
                write: |value: &Option<LodSoundData>, file, fmt| -> io::Result<()> {
                    unimplemented!()
                };
            ] = Vec::new(),
        },
        NodeDataSurface: T_SURFACEDATAS => {
            data_id: [i32] = 0i32,
            subtype_id: [i32] = 0i32,
            data: [fixed array [f32] 5] = [0f32; 5],
            unk1: [dynamic array [u32] [struct NodeDataSurfaceUnk] NodeDataSurfaceUnk::default()] = Vec::new(),
            unk2: [u32] = 0u32,
            unk3: [u32] = 0u32,
        },
        NodeDataRotshape: T_ROTSHAPEDATA => {
            data_id: [i32] = 0i32,
            subtype_id: [i32] = 0i32,
            unk1: [fixed array [u32] 6] = [0u32; 6],
            unk2: [u16] = 0u16,
            junk: [fixed array [u8] 28] = [0u8;28],
        },
        NodeDataMesh: T_MESHDATA => {
            data_id: [i32] = 0i32,
            subtype_id: [i32] = 0i32,
            data: [fixed array [f32] 5] = [0f32; 5],
        },
        NodeDataParticles: T_PARTICLESDATA => {
            data_id: [i32] = 0i32,
            subtype_id: [i32] = 0i32,
            unk1: [fixed array [f32] 5] = [0f32; 5],
            unk2: [u16] = 0u16,
        },
    }
}

impl Default for NodeDataUnion {
    fn default() -> Self {
        NodeDataUnion::Empty {}
    }
}

chum_struct_generate_readwrite! {
    #[derive(Clone)]
    pub struct NodeDataSurfaceUnk {
        pub data: [fixed array [u8] 104],
    }
}

impl Default for NodeDataSurfaceUnk {
    fn default() -> Self {
        Self {
            data: [0u8; 104]
        }
    }
}

chum_struct_generate_readwrite! {
    #[derive(Default, Clone)]
    pub struct NodeSkinUnk2 {
        pub unk_ids: [fixed array [i32] 5],
        pub extra_data: [struct NodeSkinUnk2ExtraDataUnion],
        pub local_translation: [Vector3],
        pub junk1: [ignore [fixed array [u8] 4] [0u8; 4]],
        pub local_rotation: [Quaternion],
        pub local_scale: [Vector3],
        pub floatv1: [fixed array [f32] 9],
        pub floatv2: [fixed array [f32] 9],
        pub tx1: [Mat4x4],
        pub tx2: [Mat4x4],
    }
}

chum_struct_enum! {
    /// String              Hash | Resource Type
    /// ------------------------------------------------
    ///                        0 | (empty)
    /// USERDEFINE   -1879206489 | USERDEFINE (embedded)
    #[derive(Clone)]
    pub enum NodeSkinUnk2ExtraDataUnion [i32] {
        Empty: 0 => {},
        UserDefine: E_USERDATA => {
            type1: [i32] = 0i32,
            type2: [i32] = 0i32,
            data: [dynamic array [u32] [u8] 0u8] = Vec::new(),
        },
    }
}

impl Default for NodeSkinUnk2ExtraDataUnion {
    fn default() -> Self {
        NodeSkinUnk2ExtraDataUnion::Empty {}
    }
}

chum_struct_generate_readwrite! {
    #[derive(Default, Clone)]
    pub struct NodeSkinMaterial {
        pub filetype_id: [i32],
        pub filename_id: [i32],
        pub subtype_id: [i32],
        pub material: [struct material::Material],
    }
}

chum_struct_generate_readwrite! {
    #[derive(Default, Clone)]
    pub struct NodeSkinUnk {
        pub unk1: [fixed array [f32] 4],
        pub unk2_id: [i32],
        pub unk3_id: [i32],
    }
}

chum_struct_generate_readwrite! {
    #[derive(Default, Clone)]
    pub struct NodeSkinUnk7 {
        pub data: [struct NodeDataUnion],
        pub ids: [dynamic array [u32] [i32] 0i32],
    }
}