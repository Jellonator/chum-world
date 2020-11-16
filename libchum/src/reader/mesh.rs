use crate::common::*;
use crate::format::TotemFormat;
use crate::scene;
use std::io::{self, Read, Write};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct MeshPoint {
    pub vertex: Vector3,
    pub texcoord: Vector2,
    pub normal: Vector3,
    pub vertex_id: u16,
    pub texcoord_id: u16,
    pub normal_id: u16,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct MeshTri {
    pub points: [MeshPoint; 3],
}

/// A triangle strip
#[derive(Clone, Debug)]
pub struct Strip {
    pub vertex_ids: Vec<u16>,
    pub tri_order: u32,
    pub material: u32,
}

/// A combination of a normal index and a texture coordinate index
#[derive(Clone, Debug)]
pub struct ElementData {
    pub texcoord_id: u16,
    pub normal_id: u16,
}

/// A triangle strip's extra data
#[derive(Clone, Debug)]
pub struct StripExt {
    pub elements: Vec<ElementData>,
}

#[derive(Clone, Debug)]
pub struct StripData {
    pub strip: Strip,
    pub group: Option<i32>,    // None if subtype is 0
    pub ext: Option<StripExt>, // None on PS2
}

/// A full triangle mesh
#[derive(Clone, Debug)]
pub struct Mesh {
    pub transform: THeaderTyped,
    pub vertices: Vec<Vector3>,
    pub texcoords: Vec<Vector2>,
    pub normals: Vec<Vector3>,
    pub strips: Vec<StripData>,
    pub materials: Vec<i32>,
    pub unk1: Vec<Footer1>,
    pub unk2: Vec<Footer2>,
    pub unk3: Vec<Footer3>,
    pub strip_order: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct Footer1 {
    pub pos: Vector3,
    pub radius: f32,
}

#[derive(Clone, Debug)]
pub struct Footer2 {
    pub transform: Mat4x4,
}

#[derive(Clone, Debug)]
pub struct Footer3 {
    pub unk1: [f32; 4],
    pub normal: Vector3,
    pub junk: u32,
    pub unk2: f32,
}

/// Read in a triangle strip from a reader
fn read_strip<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<Strip> {
    let num_elements: u32 = fmt.read_u32(file)?;
    let vertex_ids: Vec<u16> = (0..num_elements)
        .map(|_| fmt.read_u16(file))
        .collect::<io::Result<_>>()?;
    let material: u32 = fmt.read_u32(file)?;
    let tri_order: u32 = fmt.read_u32(file)?;
    Ok(Strip {
        vertex_ids,
        tri_order,
        material,
    })
}

/// Read in a triangle strip's extra data from a reader
fn read_strip_ext<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<StripExt> {
    let num_elements: u32 = fmt.read_u32(file)?;
    let elements: Vec<ElementData> = (0..num_elements)
        .map(|_| {
            Ok(ElementData {
                texcoord_id: fmt.read_u16(file)?,
                normal_id: fmt.read_u16(file)?,
            })
        })
        .collect::<io::Result<_>>()?;
    Ok(StripExt { elements })
}

/// Get a vector of triangle indices in (vertex, texcoord, normal) order.
fn strip_gen_triangle_indices(strip: &Strip, strip_ext: &StripExt) -> Vec<[(u16, u16, u16); 3]> {
    let a = strip.tri_order;
    let b = 3 - a;
    let lists = [[0, a, b], [0, b, a]];
    // Rust doesn't prevent you from writing bad code
    strip
        .vertex_ids
        .windows(3)
        .zip(strip_ext.elements.windows(3).into_iter())
        .zip(lists.iter().cycle())
        .map(|((vertex_ids, elements), cycle)| {
            let index0 = cycle[0] as usize;
            let index1 = cycle[1] as usize;
            let index2 = cycle[2] as usize;
            [
                (
                    vertex_ids[index0],
                    elements[index0].texcoord_id,
                    elements[index0].normal_id,
                ),
                (
                    vertex_ids[index1],
                    elements[index1].texcoord_id,
                    elements[index1].normal_id,
                ),
                (
                    vertex_ids[index2],
                    elements[index2].texcoord_id,
                    elements[index2].normal_id,
                ),
            ]
        })
        .collect()
}

/// A triangle surface, contains a material and a list of triangles
pub struct TriangleSurface {
    pub material_index: u32,
    pub tris: Vec<MeshTri>,
}

/// Generate triangles from a strip
fn strip_gen_triangles(
    strip: &Strip,
    strip_ext: &StripExt,
    vertices: &[Vector3],
    texcoords: &[Vector2],
    normals: &[Vector3],
) -> Vec<MeshTri> {
    strip_gen_triangle_indices(strip, strip_ext)
        .iter()
        .map(|ls| MeshTri {
            points: [
                MeshPoint {
                    vertex: vertices[ls[0].0 as usize],
                    texcoord: texcoords[ls[0].1 as usize],
                    normal: normals[ls[0].2 as usize],
                    vertex_id: ls[0].0,
                    texcoord_id: ls[0].1,
                    normal_id: ls[0].2,
                },
                MeshPoint {
                    vertex: vertices[ls[1].0 as usize],
                    texcoord: texcoords[ls[1].1 as usize],
                    normal: normals[ls[1].2 as usize],
                    vertex_id: ls[1].0,
                    texcoord_id: ls[1].1,
                    normal_id: ls[1].2,
                },
                MeshPoint {
                    vertex: vertices[ls[2].0 as usize],
                    texcoord: texcoords[ls[2].1 as usize],
                    normal: normals[ls[2].2 as usize],
                    vertex_id: ls[2].0,
                    texcoord_id: ls[2].1,
                    normal_id: ls[2].2,
                },
            ],
        })
        .collect()
}

impl Mesh {
    /// Get the materials that this mesh uses
    pub fn get_materials(&self) -> &[i32] {
        &self.materials
    }

    /// Generate triangle indices
    fn gen_triangle_indices(&self) -> Vec<Vec<[(u16, u16, u16); 3]>> {
        self.strips
            .iter()
            .map(|strip| strip_gen_triangle_indices(&strip.strip, strip.ext.as_ref().unwrap()))
            .collect()
    }

    /// Generate triangle surfaces from a Mesh
    pub fn gen_triangles(&self) -> Vec<TriangleSurface> {
        let mut values: Vec<(u32, Vec<MeshTri>)> = self
            .strips
            .iter()
            .map(|strip| {
                (
                    strip.strip.material,
                    strip_gen_triangles(
                        &strip.strip,
                        strip.ext.as_ref().unwrap(),
                        &self.vertices,
                        &self.texcoords,
                        &self.normals,
                    ),
                )
            })
            .collect();
        values.sort_by_key(|x| x.0);
        if values.len() == 0 {
            Vec::new()
        } else {
            let mut material = values[0].0;
            let mut ret: Vec<TriangleSurface> = Vec::new();
            ret.push(TriangleSurface {
                material_index: material,
                tris: Vec::new(),
            });
            for value in values.iter_mut() {
                if value.0 != material {
                    material = value.0;
                    ret.push(TriangleSurface {
                        material_index: material,
                        tris: Vec::new(),
                    });
                }
                ret.last_mut().unwrap().tris.append(&mut value.1);
            }
            ret
        }
    }

    /// Read a Mesh from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<Mesh> {
        use crate::structure::ChumBinary;
        let transform = THeaderTyped::read_from(file, fmt).unwrap();
        // Read coordinate data
        let num_vertices: u32 = fmt.read_u32(file)?;
        let vertices: Vec<Vector3> = (0..num_vertices)
            .map(|_| read_vec3(file, fmt))
            .collect::<io::Result<_>>()?;
        let num_texcoords: u32 = fmt.read_u32(file)?;
        let texcoords: Vec<Vector2> = (0..num_texcoords)
            .map(|_| read_vec2(file, fmt))
            .collect::<io::Result<_>>()?;
        let num_normals: u32 = fmt.read_u32(file)?;
        let normals: Vec<Vector3> = (0..num_normals)
            .map(|_| read_vec3(file, fmt))
            .collect::<io::Result<_>>()?;
        // Read strip data
        let num_strips: u32 = fmt.read_u32(file)?;
        let strips: Vec<Strip> = (0..num_strips)
            .map(|_| read_strip(file, fmt))
            .collect::<io::Result<_>>()?;
        // Ignore a few bytes
        let groups = match transform.item_subtype {
            4 => {
                let mut data = vec![0i32; strips.len()];
                fmt.read_i32_into(file, &mut data)?;
                Some(data)
            }
            0 => None,
            _ => panic!(),
        };
        // Read stripext data
        let num_strips_ext: u32 = fmt.read_u32(file)?;
        let mut strips_ext = if num_strips_ext == 0 {
            None
        } else if num_strips_ext == num_strips {
            let strips_ext: Vec<StripExt> = (0..num_strips_ext)
                .map(|_| read_strip_ext(file, fmt))
                .collect::<io::Result<_>>()?;
            Some(strips_ext)
        } else {
            panic!()
        };
        // read material data
        let num_materials: u32 = fmt.read_u32(file)?;
        let materials: Vec<i32> = (0..num_materials)
            .map(|_| fmt.read_i32(file))
            .collect::<io::Result<_>>()?;
        // read unknown data
        let num_unk1: u32 = fmt.read_u32(file)?;
        let footer1: Vec<Footer1> = (0..num_unk1)
            .map(|_| {
                Ok(Footer1 {
                    pos: read_vec3(file, fmt)?,
                    radius: fmt.read_f32(file)?,
                })
            })
            .collect::<io::Result<_>>()?;
        let num_unk2: u32 = fmt.read_u32(file)?;
        let footer2: Vec<Footer2> = (0..num_unk2)
            .map(|_| {
                let transform = read_mat4(file, fmt)?;
                fmt.skip_n_bytes(file, 16)?;
                Ok(Footer2 { transform })
            })
            .collect::<io::Result<_>>()?;
        let num_unk3: u32 = fmt.read_u32(file)?;
        let footer3: Vec<Footer3> = (0..num_unk3)
            .map(|_| {
                let mut unk1 = [0.0f32; 4];
                fmt.read_f32_into(file, &mut unk1)?;
                Ok(Footer3 {
                    unk1,
                    normal: read_vec3(file, fmt)?,
                    junk: fmt.read_u32(file)?,
                    unk2: fmt.read_f32(file)?,
                })
            })
            .collect::<io::Result<_>>()?;
        let num_unk4: u32 = fmt.read_u32(file)?; // always 0?
        if num_unk4 != 0 {
            panic!();
        }
        // pack strips together (they all should have the same length from earlier checks)
        let num_strip_order: u32 = fmt.read_u32(file)?;
        let mut strip_order = vec![0u32; num_strip_order as usize];
        fmt.read_u32_into(file, &mut strip_order)?;
        let strips = strips
            .into_iter()
            .enumerate()
            .map(|(i, value)| {
                let ext = if let Some(ref mut stripext) = strips_ext {
                    let mut fake = StripExt {
                        elements: Vec::new(),
                    };
                    std::mem::swap(&mut fake, &mut stripext[i]);
                    Some(fake)
                } else {
                    None
                };
                StripData {
                    strip: value,
                    group: groups.as_ref().map(|groupdata| groupdata[i]),
                    ext,
                }
            })
            .collect();
        Ok(Mesh {
            transform,
            vertices,
            texcoords,
            normals,
            strips,
            materials,
            unk1: footer1,
            unk2: footer2,
            unk3: footer3,
            strip_order,
        })
    }

    /// Read a Mesh from data
    pub fn read_data(data: &[u8], fmt: TotemFormat) -> io::Result<Mesh> {
        Mesh::read_from(&mut data.as_ref(), fmt)
    }

    /// Write a Mesh to an OBJ
    pub fn export_obj<W: Write>(&self, obj: &mut W) -> io::Result<()> {
        for vert in &self.vertices {
            writeln!(obj, "v {} {} {}", vert.x, vert.y, vert.z)?;
        }
        for texcoord in &self.texcoords {
            writeln!(obj, "vt {} {}", texcoord.x, texcoord.y)?;
        }
        for normal in &self.normals {
            writeln!(obj, "vn {} {} {}", normal.x, normal.y, normal.z)?;
        }
        for strip in self.gen_triangle_indices() {
            for tri in strip {
                write!(obj, "f")?;
                for (vert, texc, norm) in tri.iter() {
                    write!(obj, "{}/{}/{} ", vert, texc, norm)?;
                }
            }
            write!(obj, "\n")?;
        }
        Ok(())
    }

    pub fn create_scene_mesh(&self, name: String) -> scene::SceneTriMesh {
        scene::SceneTriMesh {
            name,
            // Mesh.transform.transform is NOT actually applied to this mesh
            transform: Mat4x4::identity(),
            vertices: self.vertices.clone(),
            normals: self.normals.clone(),
            texcoords: self.texcoords.clone(),
            elements: self
                .gen_triangle_indices()
                .into_iter()
                .flat_map(|x| x)
                .map(|x| {
                    [
                        (x[0].0 as usize, x[0].1 as usize, x[0].2 as usize),
                        (x[2].0 as usize, x[2].1 as usize, x[2].2 as usize),
                        (x[1].0 as usize, x[1].1 as usize, x[1].2 as usize),
                    ]
                })
                .collect(),
            skin: None,
        }
    }
}