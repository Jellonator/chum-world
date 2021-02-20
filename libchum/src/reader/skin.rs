use crate::animsymbol::AnimSymbol;
use crate::common::*;
use crate::format::TotemFormat;
use crate::structure::ChumEnum;
// use std::error::Error;
use crate::scene;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Clone, Debug)]
pub struct Skin {
    pub transform: THeaderTyped,
    pub meshes: Vec<i32>,
    pub vertex_groups: Vec<VertexGroup>,
    pub anims: Option<AnimSection>,
    pub unknown: Vec<UnknownEntry>,
}

#[derive(Clone, Debug)]
pub struct VertexGroup {
    pub group_id: i32,
    pub sections: Vec<VertexGroupSection>,
}

#[derive(Clone, Debug)]
pub struct VertexGroupSection {
    pub mesh_index: u16,
    pub vertices: Vec<VertexGroupVertex>,
    pub normals: Vec<VertexGroupNormal>,
}

#[derive(Clone, Debug)]
pub struct VertexGroupVertex {
    pub vertex_id: u32,
    pub weight: f32,
}

#[derive(Clone, Debug)]
pub struct VertexGroupNormal {
    pub normal_id: u32,
    pub weight: f32,
}

#[derive(Clone, Debug)]
pub struct AnimSection {
    pub entries: Vec<AnimEntry>,
}

#[derive(Clone, Debug)]
pub struct AnimEntry {
    pub symbol: AnimSymbol,
    pub anim_id: i32,
}

#[derive(Clone, Debug)]
pub struct UnknownEntry {
    pub vertices: Vec<u32>,
    pub normals: Vec<u32>,
}

/// specialized stucture used for skin export
#[derive(Clone, Copy)]
pub struct SkinInfo<'a, 'b> {
    pub names: &'a HashMap<i32, String>,
    pub skin: &'b Skin,
    pub skin_id: i32,
    pub mesh_id: i32,
}

impl Skin {
    fn read_section<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<VertexGroupSection> {
        let mesh_index = fmt.read_u16(file)?;
        let num_vertices = fmt.read_u32(file)?;
        let mut vertices = Vec::new();
        for _ in 0..num_vertices {
            vertices.push(VertexGroupVertex {
                vertex_id: fmt.read_u32(file)?,
                weight: fmt.read_f32(file)?,
            });
        }
        let num_normals = fmt.read_u32(file)?;
        let mut normals = Vec::new();
        for _ in 0..num_normals {
            normals.push(VertexGroupNormal {
                normal_id: fmt.read_u32(file)?,
                weight: fmt.read_f32(file)?,
            });
        }
        Ok(VertexGroupSection {
            mesh_index,
            vertices,
            normals,
        })
    }

    fn read_group<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<VertexGroup> {
        let group_id = fmt.read_i32(file)?;
        let num_sections = fmt.read_u32(file)?;
        let mut sections = Vec::new();
        for _ in 0..num_sections {
            sections.push(Skin::read_section(file, fmt)?);
        }
        Ok(VertexGroup { group_id, sections })
    }

    /// Read a TMesh from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<Skin> {
        use crate::binary::ChumBinary;
        let transform = THeaderTyped::read_from(file, fmt).unwrap();
        let num_meshes = fmt.read_u32(file)?;
        let mut meshes = Vec::new();
        for _ in 0..num_meshes {
            meshes.push(fmt.read_i32(file)?);
        }
        fmt.skip_n_bytes(file, 4)?; // skip zero
        let num_groups = fmt.read_u32(file)?;
        let mut groups = Vec::new();
        for _ in 0..num_groups {
            groups.push(Skin::read_group(file, fmt)?);
        }
        let has_anim_entries = fmt.read_u8(file)?;
        let anim_entries = if has_anim_entries != 0 {
            let num_anim_entries = fmt.read_u32(file)?;
            let mut anim_entries = Vec::new();
            for _ in 0..num_anim_entries {
                anim_entries.push(AnimEntry {
                    symbol: AnimSymbol::from_u32(fmt.read_u32(file)?).unwrap(),
                    anim_id: fmt.read_i32(file)?,
                });
            }
            Some(AnimSection {
                entries: anim_entries,
            })
        } else {
            None
        };
        let num_unknown_entries = fmt.read_u32(file)?;
        let mut unknown_entries = Vec::new();
        for _ in 0..num_unknown_entries {
            let num_vertices = fmt.read_u32(file)?;
            let mut vertices = Vec::new();
            for _ in 0..num_vertices {
                vertices.push(fmt.read_u32(file)?);
            }
            let mut normals = Vec::new();
            let num_normals = fmt.read_u32(file)?;
            for _ in 0..num_normals {
                normals.push(fmt.read_u32(file)?);
            }
            unknown_entries.push(UnknownEntry { vertices, normals })
        }
        Ok(Skin {
            transform,
            meshes,
            vertex_groups: groups,
            anims: anim_entries,
            unknown: unknown_entries,
        })
    }

    /// Read a TMesh from data
    pub fn read_data(data: &[u8], fmt: TotemFormat) -> io::Result<Skin> {
        Skin::read_from(&mut data.as_ref(), fmt)
    }

    pub fn generate_scene_skin_joints(
        &self,
        names: &HashMap<i32, String>,
    ) -> Vec<scene::SkinJoint> {
        self.vertex_groups
            .iter()
            .map(|x| scene::SkinJoint {
                transform: Transform3D::identity(),
                name: names
                    .get(&x.group_id)
                    .map(|x| x.to_string())
                    .unwrap_or(format!("{}", x.group_id)),
            })
            .collect()
    }
}
