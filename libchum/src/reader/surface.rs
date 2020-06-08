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

#[derive(Copy, Clone)]
pub enum SurfaceGenMode {
    SingleQuad, // Generates a single quad
                // ControlPointsAsVertices, // Generates 9 quads
                // SplineInterp(usize), // Spline interpolation
                // NURBSInterp(usize), // NURBS interpolation
}

fn generate_surface_singlequad(
    curves: &[[Vector3; 4]; 4],
    normals: &[Vector3; 4],
    texcoords: &[Vector2; 4],
) -> Vec<Quad> {
    let mut quads = Vec::with_capacity(1);
    quads.push(Quad {
        points: [
            Point {
                vertex: curves[0][0],
                texcoord: texcoords[0],
                normal: normals[0],
            },
            Point {
                vertex: curves[1][0],
                texcoord: texcoords[1],
                normal: normals[1],
            },
            Point {
                vertex: curves[2][0],
                texcoord: texcoords[2],
                normal: normals[2],
            },
            Point {
                vertex: curves[3][0],
                texcoord: texcoords[3],
                normal: normals[3],
            },
        ],
    });
    quads
}

/*fn generate_surface_quad9(curves: &[[Vector3; 4]; 4], normals: &[Vector3; 4], texcoords: &[Vector2; 4]) -> Vec<Quad> {
    let mut quads = Vec::with_capacity(9);
    let pts_tl = curves[0][1] - curves[0][0] + curves[3][2];
    let pts_tr = curves[0][2] - curves[0][3] + curves[1][1];
    let pts_bl = curves[2][2] - curves[2][3] + curves[3][1];
    let pts_br = curves[2][1] - curves[1][3] + curves[1][2];
    let points = [
        [curves[0][0], curves[0][1], curves[0][2], curves[0][3]],
        [curves[3][2],       pts_tl,       pts_tr, curves[1][1]],
        [curves[3][1],       pts_bl,       pts_br, curves[1][2]],
        [curves[2][3], curves[2][2], curves[2][1], curves[1][3]],
    ];
    let mut ttexc = [[Vector2::new(); 4]; 4];
    let mut tnorm = [[Vector3::new(); 4]; 4];
    for ix in 0..4 {
        for iy in 0..4 {
            let totaldisx = (points[iy][0] - points[iy][ix]).len() + (points[iy][ix] - points[iy][3]).len();
            let totaldisy = (points[0][ix] - points[iy][ix]).len() + (points[iy][ix] - points[3][ix]).len();
            let t_x = (points[iy][0] - points[iy][ix]).len() / totaldisx;
            let t_y = (points[0][ix] - points[iy][ix]).len() / totaldisy;
            ttexc[ix][iy] =
                texcoords[0] * (1.0 - t_x) * (1.0 - t_y) +
                texcoords[1] * (      t_x) * (1.0 - t_y) +
                texcoords[2] * (      t_x) * (      t_y) +
                texcoords[3] * (1.0 - t_x) * (      t_y);
            tnorm[ix][iy] =
                normals[0] * (1.0 - t_x) * (1.0 - t_y) +
                normals[1] * (      t_x) * (1.0 - t_y) +
                normals[2] * (      t_x) * (      t_y) +
                normals[3] * (1.0 - t_x) * (      t_y);
        }
    }
    for ix in 0..3 {
        for iy in 0..3 {
            quads.push(Quad {
                points: [
                    Point {
                        vertex: points[iy][ix],
                        texcoord: ttexc[iy][ix],
                        normal: tnorm[iy][ix],
                    },
                    Point {
                        vertex: points[iy][ix+1],
                        texcoord: ttexc[iy][ix+1],
                        normal: tnorm[iy][ix+1],
                    },
                    Point {
                        vertex: points[iy+1][ix+1],
                        texcoord: ttexc[iy+1][ix+1],
                        normal: tnorm[iy+1][ix+1],
                    },
                    Point {
                        vertex: points[iy+1][ix],
                        texcoord: ttexc[iy+1][ix],
                        normal: tnorm[iy+1][ix],
                    },
                ],
            });
        }
    }
    quads
}*/

pub fn generate_surface(
    curves: &[[Vector3; 4]; 4],
    normals: &[Vector3; 4],
    texcoords: &[Vector2; 4],
    mode: SurfaceGenMode,
) -> Vec<Quad> {
    match mode {
        SurfaceGenMode::SingleQuad => generate_surface_singlequad(curves, normals, texcoords),
        // SurfaceGenMode::ControlPointsAsVertices => generate_surface_quad9(curves, normals, texcoords),
    }
}

impl SurfaceObject {
    pub fn generate_meshes(&self, mode: SurfaceGenMode) -> Vec<OutMesh> {
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
            let normals = [normal0, normal1, normal2, normal3];
            out.push(OutMesh {
                material_index: surface.material_id,
                quads: generate_surface(&curves, &normals, &surface.texcoords, mode),
            })
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
