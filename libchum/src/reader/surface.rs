use crate::common::*;
use crate::format::TotemFormat;
use std::io::{self, Read};

pub struct SurfaceObject {
    pub vertices: Vec<Vector3>,
    pub surfaces: Vec<Surface>,
    pub curves: Vec<Curve>,
    pub normals: Vec<Vector3>,
}

pub struct Surface {
    pub texcoords: [Vector2; 4],
    pub normal_ids: [u16; 4],
    pub curve_ids: [u16; 4],
    pub curve_order: u32,
    pub material_id: i32,
}

pub struct Curve {
    pub p1: u16,
    pub p2: u16,
    pub p1_t: u16,
    pub p2_t: u16,
}

pub struct OutMesh {
    pub material_index: i32,
    pub quads: Vec<Quad>,
}

impl SurfaceObject {
    pub fn generate_meshes(&self) -> Vec<OutMesh> {
        let mut out = Vec::with_capacity(self.surfaces.len());
        for surface in &self.surfaces {
            let normal0 = self.normals[surface.normal_ids[0] as usize];
            let normal1 = self.normals[surface.normal_ids[1] as usize];
            let normal2 = self.normals[surface.normal_ids[2] as usize];
            let normal3 = self.normals[surface.normal_ids[3] as usize];
            let mut curves = [[Vector3::new(); 4]; 4];
            for i in 0..4 {
                let curve = &self.curves[surface.curve_ids[i] as usize];
                if surface.curve_order & (0b10 << i) == 0 {
                    curves[i][0] = self.vertices[curve.p1 as usize];
                    curves[i][1] = self.vertices[curve.p1_t as usize];
                    curves[i][2] = self.vertices[curve.p2_t as usize];
                    curves[i][3] = self.vertices[curve.p2 as usize];
                } else {
                    curves[i][3] = self.vertices[curve.p1 as usize];
                    curves[i][2] = self.vertices[curve.p1_t as usize];
                    curves[i][1] = self.vertices[curve.p2_t as usize];
                    curves[i][0] = self.vertices[curve.p2 as usize];
                }
            }
            let mut quads = Vec::with_capacity(1);
            quads.push(Quad {
                points: [
                    Point {
                        vertex: curves[0][0],
                        texcoord: surface.texcoords[0],
                        normal: normal0,
                    },
                    Point {
                        vertex: curves[1][0],
                        texcoord: surface.texcoords[0],
                        normal: normal1,
                    },
                    Point {
                        vertex: curves[2][0],
                        texcoord: surface.texcoords[0],
                        normal: normal2,
                    },
                    Point {
                        vertex: curves[3][0],
                        texcoord: surface.texcoords[0],
                        normal: normal3,
                    },
                ],
            });
            println!("MATERIALINDEX: {}", surface.material_id);
            out.push(OutMesh {
                material_index: surface.material_id,
                quads,
            })
            // let curve0 = &self.curves[surface.curve_ids[0] as usize];
            // let curve1 = &self.curves[surface.curve_ids[1] as usize];
            // let curve2 = &self.curves[surface.curve_ids[2] as usize];
            // let curve3 = &self.curves[surface.curve_ids[3] as usize];
            // let curves = [
            //     [self.vertices[curve0.p1 as usize], self.vertices[curve0.p1_t as usize], self.vertices[curve0.p2_t as usize]]
            // ];
        }
        out
    }

    /// Read a SurfaceObject from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<SurfaceObject> {
        fmt.skip_n_bytes(file, 96)?;
        let _unknown2 = fmt.read_u16(file)?;
        let _unknown3 = fmt.read_u16(file)?;
        let num_vertices = fmt.read_u32(file)?;
        let mut vertices = Vec::with_capacity(num_vertices as usize);
        for _ in 0..num_vertices {
            vertices.push(Vector3 {
                x: fmt.read_f32(file)?,
                y: fmt.read_f32(file)?,
                z: fmt.read_f32(file)?,
            });
        }
        let num_unk0 = fmt.read_u32(file)?;
        fmt.skip_n_bytes(file, num_unk0 as u64 * 24)?;
        let num_unk1 = fmt.read_u32(file)?;
        fmt.skip_n_bytes(file, num_unk1 as u64 * 24)?;
        let num_surfaces = fmt.read_u32(file)?;
        let mut surfaces = Vec::with_capacity(num_surfaces as usize);
        for _ in 0..num_surfaces {
            let texcoords = [
                Vector2 {
                    x: fmt.read_f32(file)?,
                    y: fmt.read_f32(file)?,
                },
                Vector2 {
                    x: fmt.read_f32(file)?,
                    y: fmt.read_f32(file)?,
                },
                Vector2 {
                    x: fmt.read_f32(file)?,
                    y: fmt.read_f32(file)?,
                },
                Vector2 {
                    x: fmt.read_f32(file)?,
                    y: fmt.read_f32(file)?,
                },
            ];
            fmt.skip_n_bytes(file, 12 * 4)?;
            let mut normal_ids = [0; 4];
            fmt.read_u16_into(file, &mut normal_ids)?;
            let mut curve_ids = [0; 4];
            fmt.read_u16_into(file, &mut curve_ids)?;
            let curve_order = fmt.read_u32(file)?;
            fmt.skip_n_bytes(file, 32 + 4)?;
            let material_id = fmt.read_i32(file)?;
            println!("MATERIAL: {}", material_id);
            surfaces.push(Surface {
                texcoords,
                normal_ids,
                curve_ids,
                curve_order,
                material_id,
            });
        }
        let num_curves = fmt.read_u32(file)?;
        let mut curves = Vec::with_capacity(num_curves as usize);
        for _ in 0..num_curves {
            curves.push(Curve {
                p1: fmt.read_u16(file)?,
                p2: fmt.read_u16(file)?,
                p1_t: fmt.read_u16(file)?,
                p2_t: fmt.read_u16(file)?,
            });
        }
        let num_normals = fmt.read_u32(file)?;
        let mut normals = Vec::with_capacity(num_normals as usize);
        for _ in 0..num_normals {
            normals.push(Vector3 {
                x: fmt.read_f32(file)?,
                y: fmt.read_f32(file)?,
                z: fmt.read_f32(file)?,
            });
        }
        Ok(SurfaceObject {
            vertices,
            surfaces,
            curves,
            normals,
        })
    }

    /// Read a SurfaceObject from data
    pub fn read_data(data: &[u8], fmt: TotemFormat) -> io::Result<SurfaceObject> {
        SurfaceObject::read_from(&mut data.as_ref(), fmt)
    }
}
