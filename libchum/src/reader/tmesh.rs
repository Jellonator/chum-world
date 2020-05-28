use crate::format::TotemFormat;
use std::io::{self, Read, Write};

/// Basic 3d vector
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Basic 2d vector
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

/// A triangle strip
pub struct Strip {
    pub vertex_ids: Vec<u16>,
    pub tri_order: u32,
}

/// A combination of a normal index and a texture coordinate index
pub struct ElementData {
    pub texcoord_id: u16,
    pub normal_id: u16,
}

/// A triangle strip's extra data
pub struct StripExt {
    pub elements: Vec<ElementData>,
}

/// A point
pub struct Point {
    pub vertex: Vector3,
    pub texcoord: Vector2,
    pub normal: Vector3,
}

/// A triangle (three points)
pub struct Tri {
    pub points: [Point; 3],
}

/// A full triangle mesh
pub struct TMesh {
    // unknown1: [u8; 96]
    // unknown2: u16
    // padding_mult: u16,
    vertices: Vec<Vector3>,
    texcoords: Vec<Vector2>,
    normals: Vec<Vector3>,
    strips: Vec<Strip>,
    // unknown3: [u8; num_strips * padding_mult]
    strips_ext: Vec<StripExt>,
    // unknown4: [u8]
}

/// Read in a triangle strip from a reader
fn read_strip<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<Strip> {
    let num_elements: u32 = fmt.read_u32(file)?;
    let vertex_ids: Vec<u16> = (0..num_elements)
        .map(|_| fmt.read_u16(file))
        .collect::<io::Result<_>>()?;
    let _unknown: u32 = fmt.read_u32(file)?;
    let tri_order: u32 = fmt.read_u32(file)?;
    Ok(Strip {
        vertex_ids,
        tri_order,
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

fn strip_gen_triangle_indices(strip: &Strip, strip_ext: &StripExt) -> Vec<[(u16, u16, u16); 3]> {
    let a = strip.tri_order;
    let b = 3 - a;
    let lists = [[0, b, a], [0, a, b]];
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

/// Generate triangles from a strip
fn strip_gen_triangles(
    strip: &Strip,
    strip_ext: &StripExt,
    vertices: &[Vector3],
    texcoords: &[Vector2],
    normals: &[Vector3],
) -> Vec<Tri> {
    strip_gen_triangle_indices(strip, strip_ext)
        .iter()
        .map(|ls| Tri {
            points: [
                Point {
                    vertex: vertices[ls[0].0 as usize],
                    texcoord: texcoords[ls[0].1 as usize],
                    normal: normals[ls[0].2 as usize],
                },
                Point {
                    vertex: vertices[ls[1].0 as usize],
                    texcoord: texcoords[ls[1].1 as usize],
                    normal: normals[ls[1].2 as usize],
                },
                Point {
                    vertex: vertices[ls[2].0 as usize],
                    texcoord: texcoords[ls[2].1 as usize],
                    normal: normals[ls[2].2 as usize],
                },
            ],
        })
        .collect()
}

impl TMesh {
    fn gen_triangle_indices(&self) -> Vec<Vec<[(u16, u16, u16); 3]>> {
        self.strips
            .iter()
            .zip(&self.strips_ext)
            .map(|(strip, strip_ext)| strip_gen_triangle_indices(strip, strip_ext))
            .collect()
    }

    /// Generate a triangle from a TMesh
    pub fn gen_triangles(&self) -> Vec<Vec<Tri>> {
        self.strips
            .iter()
            .zip(&self.strips_ext)
            .map(|(strip, strip_ext)| {
                strip_gen_triangles(
                    strip,
                    strip_ext,
                    &self.vertices,
                    &self.texcoords,
                    &self.normals,
                )
            })
            .collect()
    }

    /// Read a TMesh from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<TMesh> {
        io::copy(&mut file.take(96), &mut io::sink())?;
        let _unknown2: u16 = fmt.read_u16(file)?;
        let padding_mult: u16 = fmt.read_u16(file)?;
        // Read coordinate data
        let num_vertices: u32 = fmt.read_u32(file)?;
        let vertices: Vec<Vector3> = (0..num_vertices)
            .map(|_| {
                Ok(Vector3 {
                    x: fmt.read_f32(file)?,
                    y: fmt.read_f32(file)?,
                    z: fmt.read_f32(file)?,
                })
            })
            .collect::<io::Result<_>>()?;
        let num_texcoords: u32 = fmt.read_u32(file)?;
        let texcoords: Vec<Vector2> = (0..num_texcoords)
            .map(|_| {
                Ok(Vector2 {
                    x: fmt.read_f32(file)?,
                    y: fmt.read_f32(file)?,
                })
            })
            .collect::<io::Result<_>>()?;
        let num_normals: u32 = fmt.read_u32(file)?;
        let normals: Vec<Vector3> = (0..num_normals)
            .map(|_| {
                Ok(Vector3 {
                    x: fmt.read_f32(file)?,
                    y: fmt.read_f32(file)?,
                    z: fmt.read_f32(file)?,
                })
            })
            .collect::<io::Result<_>>()?;
        // Read strip data
        let num_strips: u32 = fmt.read_u32(file)?;
        let strips: Vec<Strip> = (0..num_strips)
            .map(|_| read_strip(file, fmt))
            .collect::<io::Result<_>>()?;
        // Ignore a few bytes
        io::copy(
            &mut file.take((num_strips as u64) * (padding_mult as u64)),
            &mut io::sink(),
        )?;
        // Read stripext data
        let num_strips_ext: u32 = fmt.read_u32(file)?;
        let strips_ext: Vec<StripExt> = (0..num_strips_ext)
            .map(|_| read_strip_ext(file, fmt))
            .collect::<io::Result<_>>()?;
        // rest of data is unknown
        Ok(TMesh {
            vertices,
            texcoords,
            normals,
            strips,
            strips_ext,
        })
    }

    /// Read a TMesh from data
    pub fn read_data(data: &[u8], fmt: TotemFormat) -> io::Result<TMesh> {
        TMesh::read_from(&mut data.as_ref(), fmt)
    }

    /// Write a TMesh to an OBJ
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
}
