use byteorder::{BigEndian, ReadBytesExt};
use std::io::{self, Read};

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

pub struct Strip {
    pub vertex_ids: Vec<u16>,
    pub tri_order: u32,
}

pub struct ElementData {
    pub texcoord_id: u16,
    pub normal_id: u16,
}

pub struct StripExt {
    pub elements: Vec<ElementData>,
}

pub struct Point {
    pub vertex: Vector3,
    pub texcoord: Vector2,
    pub normal: Vector3,
}

pub struct Tri {
    pub points: [Point; 3],
}

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

fn read_strip<R: Read>(file: &mut R) -> io::Result<Strip> {
    let num_elements: u32 = file.read_u32::<BigEndian>()?;
    let vertex_ids: Vec<u16> = (0..num_elements)
        .map(|_| file.read_u16::<BigEndian>())
        .collect::<io::Result<_>>()?;
    let _unknown: u32 = file.read_u32::<BigEndian>()?;
    let tri_order: u32 = file.read_u32::<BigEndian>()?;
    Ok(Strip {
        vertex_ids,
        tri_order,
    })
}

fn read_strip_ext<R: Read>(file: &mut R) -> io::Result<StripExt> {
    let num_elements: u32 = file.read_u32::<BigEndian>()?;
    let elements: Vec<ElementData> = (0..num_elements)
        .map(|_| {
            Ok(ElementData {
                texcoord_id: file.read_u16::<BigEndian>()?,
                normal_id: file.read_u16::<BigEndian>()?,
            })
        })
        .collect::<io::Result<_>>()?;
    Ok(StripExt { elements })
}

fn strip_gen_triangles(
    strip: &Strip,
    strip_ext: &StripExt,
    vertices: &[Vector3],
    texcoords: &[Vector2],
    normals: &[Vector3],
) -> Vec<Tri> {
    let a = strip.tri_order;
    let b = 3 - a;
    let lists = [[0, b, a], [0, a, b]];
    // Rust doesn't prevent you from writing bad code
    strip
        .vertex_ids
        .windows(3)
        .zip(strip_ext.elements.windows(3).into_iter())
        .zip(lists.iter().cycle())
        .enumerate()
        .map(|(i, ((vertex_ids, elements), cycle))| {
            let index0 = i + cycle[0] as usize;
            let index1 = i + cycle[1] as usize;
            let index2 = i + cycle[2] as usize;
            Tri {
                points: [
                    Point {
                        vertex: vertices[vertex_ids[index0] as usize],
                        texcoord: texcoords[elements[index0].texcoord_id as usize],
                        normal: normals[elements[index0].normal_id as usize],
                    },
                    Point {
                        vertex: vertices[vertex_ids[index1] as usize],
                        texcoord: texcoords[elements[index1].texcoord_id as usize],
                        normal: normals[elements[index1].normal_id as usize],
                    },
                    Point {
                        vertex: vertices[vertex_ids[index2] as usize],
                        texcoord: texcoords[elements[index2].texcoord_id as usize],
                        normal: normals[elements[index2].normal_id as usize],
                    },
                ],
            }
        })
        .collect()
}

impl TMesh {
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

    pub fn read_from<R: Read>(file: &mut R) -> io::Result<TMesh> {
        io::copy(&mut file.take(96), &mut io::sink())?;
        let _unknown2: u16 = file.read_u16::<BigEndian>()?;
        let padding_mult: u16 = file.read_u16::<BigEndian>()?;
        let num_vertices: u32 = file.read_u32::<BigEndian>()?;
        // Read coordinate data
        let vertices: Vec<Vector3> = (0..num_vertices)
            .map(|_| {
                Ok(Vector3 {
                    x: file.read_f32::<BigEndian>()?,
                    y: file.read_f32::<BigEndian>()?,
                    z: file.read_f32::<BigEndian>()?,
                })
            })
            .collect::<io::Result<_>>()?;
        let num_texcoords: u32 = file.read_u32::<BigEndian>()?;
        let texcoords: Vec<Vector2> = (0..num_texcoords)
            .map(|_| {
                Ok(Vector2 {
                    x: file.read_f32::<BigEndian>()?,
                    y: file.read_f32::<BigEndian>()?,
                })
            })
            .collect::<io::Result<_>>()?;
        let num_normals: u32 = file.read_u32::<BigEndian>()?;
        let normals: Vec<Vector3> = (0..num_normals)
            .map(|_| {
                Ok(Vector3 {
                    x: file.read_f32::<BigEndian>()?,
                    y: file.read_f32::<BigEndian>()?,
                    z: file.read_f32::<BigEndian>()?,
                })
            })
            .collect::<io::Result<_>>()?;
        // Read strip data
        let num_strips: u32 = file.read_u32::<BigEndian>()?;
        let strips: Vec<Strip> = (0..num_strips)
            .map(|_| read_strip(file))
            .collect::<io::Result<_>>()?;
        // Ignore a few bytes
        io::copy(
            &mut file.take((num_strips as u64) * (padding_mult as u64)),
            &mut io::sink(),
        )?;
        // Read stripext data
        let num_strips_ext: u32 = file.read_u32::<BigEndian>()?;
        let strips_ext: Vec<StripExt> = (0..num_strips_ext)
            .map(|_| read_strip_ext(file))
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

    pub fn read_data(data: &[u8]) -> io::Result<TMesh> {
        TMesh::read_from(&mut data.as_ref())
    }
}
