//! See https://github.com/Jellonator/chum-world/wiki/SURFACE for more information

use crate::common::*;
use crate::format::TotemFormat;
use crate::util::bezierpatch;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

/// A surface object; contains entire surface object information
pub struct SurfaceObject {
    pub transform: TransformationHeader,
    pub vertices: Vec<Vector3>,
    pub surfaces: Vec<Surface>,
    pub curves: Vec<Curve>,
    pub normals: Vec<Vector3>,
}

/// A single surface.
pub struct Surface {
    pub texcoords: [Vector2; 4],
    /// indices into normals
    pub normal_ids: [u16; 4],
    /// indices into curves
    pub curve_ids: [u16; 4],
    pub curve_order: u32,
    pub material_id: i32,
}

/// A single curve (indices into vertices)
pub struct Curve {
    pub p1: u16,
    pub p2: u16,
    pub p1_t: u16,
    pub p2_t: u16,
}

/// A point
#[derive(Clone, Debug)]
#[repr(C)]
pub struct SurfPoint {
    pub vertex: Vector3,
    pub texcoord: Vector2,
    pub normal: Vector3,
    pub uv2: Vector2,
}

impl SurfPoint {
    pub fn with(point: &Point, uv2: Vector2) -> SurfPoint {
        SurfPoint {
            uv2,
            vertex: point.vertex.clone(),
            texcoord: point.texcoord.clone(),
            normal: point.normal.clone(),
        }
    }
}

/// A triangle (three points)
#[derive(Clone, Debug)]
#[repr(C)]
pub struct SurfTri {
    pub points: [SurfPoint; 3],
}

/// A quad (four points)
#[derive(Clone, Debug)]
#[repr(C)]
pub struct SurfQuad {
    pub points: [SurfPoint; 4],
}

impl SurfQuad {
    /// Iterate triangles in this quad
    pub fn tris(&self) -> [SurfTri; 2] {
        let a = SurfTri {
            points: [
                self.points[0].clone(),
                self.points[2].clone(),
                self.points[1].clone(),
            ],
        };
        let b = SurfTri {
            points: [
                self.points[0].clone(),
                self.points[3].clone(),
                self.points[2].clone(),
            ],
        };
        [a, b]
    }
}

/// Output mesh
pub struct OutMesh {
    pub material_index: i32,
    pub quads: Vec<SurfQuad>,
}

/// The surface generation mode
#[derive(Copy, Clone)]
pub enum SurfaceGenMode {
    /// Generate each surface as a quad
    SingleQuad,
    /// Generate each surface as 9 quads, with control points as vertices
    ControlPointsAsVertices,
    /// Bezier interpolation
    BezierInterp(usize),
}

/// Generate a single quad
fn generate_surface_singlequad(
    curves: &[[Vector3; 4]; 4],
    normals: &[Vector3; 4],
    texcoords: &[Vector2; 4],
) -> Vec<SurfQuad> {
    let mut quads = Vec::with_capacity(1);
    quads.push(SurfQuad {
        points: [
            SurfPoint {
                vertex: curves[0][0],
                texcoord: texcoords[0],
                normal: normals[0],
                uv2: Vector2::new(0.0, 0.0),
            },
            SurfPoint {
                vertex: curves[1][0],
                texcoord: texcoords[1],
                normal: normals[1],
                uv2: Vector2::new(1.0, 0.0),
            },
            SurfPoint {
                vertex: curves[2][0],
                texcoord: texcoords[2],
                normal: normals[2],
                uv2: Vector2::new(1.0, 1.0),
            },
            SurfPoint {
                vertex: curves[3][0],
                texcoord: texcoords[3],
                normal: normals[3],
                uv2: Vector2::new(0.0, 1.0),
            },
        ],
    });
    quads
}

/// Generate 9 quads
fn generate_surface_quad9(
    curves: &[[Vector3; 4]; 4],
    normals: &[Vector3; 4],
    texcoords: &[Vector2; 4],
) -> Vec<SurfQuad> {
    let mut quads = Vec::with_capacity(9);
    let pts_tl = curves[0][1] - curves[0][0] + curves[3][2];
    let pts_tr = curves[0][2] - curves[0][3] + curves[1][1];
    let pts_bl = curves[2][2] - curves[2][3] + curves[3][1];
    let pts_br = curves[2][1] - curves[1][3] + curves[1][2];
    let points = [
        [curves[0][0], curves[0][1], curves[0][2], curves[0][3]],
        [curves[3][2], pts_tl, pts_tr, curves[1][1]],
        [curves[3][1], pts_bl, pts_br, curves[1][2]],
        [curves[2][3], curves[2][2], curves[2][1], curves[1][3]],
    ];
    let mut ttexc = [[Vector2::new(0.0, 0.0); 4]; 4];
    let mut tnorm = [[Vector3::new(0.0, 0.0, 0.0); 4]; 4];
    let texvals = [[texcoords[0], texcoords[1]], [texcoords[3], texcoords[2]]];
    let normvals = [[normals[0], normals[1]], [normals[3], normals[2]]];
    for ix in 0..4 {
        for iy in 0..4 {
            let u = (ix as f32) / 3.0;
            let v = (iy as f32) / 3.0;
            ttexc[iy][ix] = qlerp_vec2(&texvals, u, v);
            tnorm[iy][ix] = qlerp_vec3(&normvals, u, v);
        }
    }
    for iy in 0..3 {
        for ix in 0..3 {
            quads.push(SurfQuad {
                points: [
                    SurfPoint {
                        vertex: points[iy][ix],
                        texcoord: ttexc[iy][ix],
                        normal: tnorm[iy][ix],
                        uv2: Vector2::new((ix as f32 + 0.0) / 3.0, (iy as f32 + 0.0) / 3.0),
                    },
                    SurfPoint {
                        vertex: points[iy][ix + 1],
                        texcoord: ttexc[iy][ix + 1],
                        normal: tnorm[iy][ix + 1],
                        uv2: Vector2::new((ix as f32 + 0.0) / 3.0, (iy as f32 + 1.0) / 3.0),
                    },
                    SurfPoint {
                        vertex: points[iy + 1][ix + 1],
                        texcoord: ttexc[iy + 1][ix + 1],
                        normal: tnorm[iy + 1][ix + 1],
                        uv2: Vector2::new((ix as f32 + 1.0) / 3.0, (iy as f32 + 1.0) / 3.0),
                    },
                    SurfPoint {
                        vertex: points[iy + 1][ix],
                        texcoord: ttexc[iy + 1][ix],
                        normal: tnorm[iy + 1][ix],
                        uv2: Vector2::new((ix as f32 + 1.0) / 3.0, (iy as f32 + 0.0) / 3.0),
                    },
                ],
            });
        }
    }
    quads
}

/// Generate quads using bezier surface interpolation
fn generate_surface_bezier(
    curves: &[[Vector3; 4]; 4],
    normals: &[Vector3; 4],
    texcoords: &[Vector2; 4],
    steps: usize,
) -> Vec<SurfQuad> {
    let pts_tl = curves[0][1] - curves[0][0] + curves[3][2];
    let pts_tr = curves[0][2] - curves[0][3] + curves[1][1];
    let pts_bl = curves[2][2] - curves[2][3] + curves[3][1];
    let pts_br = curves[2][1] - curves[1][3] + curves[1][2];
    let points = [
        [curves[0][0], curves[0][1], curves[0][2], curves[0][3]],
        [curves[3][2], pts_tl, pts_tr, curves[1][1]],
        [curves[3][1], pts_bl, pts_br, curves[1][2]],
        [curves[2][3], curves[2][2], curves[2][1], curves[1][3]],
    ];
    let vertices = bezierpatch::precompute_surface_texnorm(
        &points,
        steps,
        steps,
        &[[texcoords[0], texcoords[1]], [texcoords[3], texcoords[2]]],
        &[[normals[0], normals[1]], [normals[3], normals[2]]],
    );
    let mut quads = Vec::new();
    for iy in 0..steps {
        for ix in 0..steps {
            quads.push(SurfQuad {
                points: [
                    SurfPoint::with(
                        &vertices[iy][ix],
                        Vector2::new(
                            (ix as f32 + 0.0) / (steps as f32),
                            (iy as f32 + 0.0) / (steps as f32),
                        ),
                    ),
                    SurfPoint::with(
                        &vertices[iy][ix + 1],
                        Vector2::new(
                            (ix as f32 + 0.0) / (steps as f32),
                            (iy as f32 + 1.0) / (steps as f32),
                        ),
                    ),
                    SurfPoint::with(
                        &vertices[iy + 1][ix + 1],
                        Vector2::new(
                            (ix as f32 + 1.0) / (steps as f32),
                            (iy as f32 + 1.0) / (steps as f32),
                        ),
                    ),
                    SurfPoint::with(
                        &vertices[iy + 1][ix],
                        Vector2::new(
                            (ix as f32 + 1.0) / (steps as f32),
                            (iy as f32 + 0.0) / (steps as f32),
                        ),
                    ),
                ],
            });
        }
    }
    quads
}

/// Generate a surface with the given mode
pub fn generate_surface(
    curves: &[[Vector3; 4]; 4],
    normals: &[Vector3; 4],
    texcoords: &[Vector2; 4],
    mode: SurfaceGenMode,
) -> Vec<SurfQuad> {
    match mode {
        SurfaceGenMode::SingleQuad => generate_surface_singlequad(curves, normals, texcoords),
        SurfaceGenMode::ControlPointsAsVertices => {
            generate_surface_quad9(curves, normals, texcoords)
        }
        SurfaceGenMode::BezierInterp(n) => generate_surface_bezier(curves, normals, texcoords, n),
    }
}

impl SurfaceObject {
    /// Generate an entire mesh using the given surface generation mode
    pub fn generate_meshes(&self, mode: SurfaceGenMode) -> Vec<OutMesh> {
        let mut out = Vec::with_capacity(self.surfaces.len());
        for surface in &self.surfaces {
            let normal0 = self.normals[surface.normal_ids[0] as usize];
            let normal1 = self.normals[surface.normal_ids[1] as usize];
            let normal2 = self.normals[surface.normal_ids[2] as usize];
            let normal3 = self.normals[surface.normal_ids[3] as usize];
            let mut curves = [[Vector3::new(0.0, 0.0, 0.0); 4]; 4];
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
        // fmt.skip_n_bytes(file, 96)?;
        // let _unknown2 = fmt.read_u16(file)?;
        // let _unknown3 = fmt.read_u16(file)?;
        let transform = TransformationHeader::read_from(file, fmt)?;
        let num_vertices = fmt.read_u32(file)?;
        let mut vertices = Vec::with_capacity(num_vertices as usize);
        for _ in 0..num_vertices {
            vertices.push(read_vec3(file, fmt)?);
        }
        let num_unk0 = fmt.read_u32(file)?;
        fmt.skip_n_bytes(file, num_unk0 as u64 * 24)?;
        let num_unk1 = fmt.read_u32(file)?;
        fmt.skip_n_bytes(file, num_unk1 as u64 * 24)?;
        let num_surfaces = fmt.read_u32(file)?;
        let mut surfaces = Vec::with_capacity(num_surfaces as usize);
        for _ in 0..num_surfaces {
            let texcoords = [
                read_vec2(file, fmt)?,
                read_vec2(file, fmt)?,
                read_vec2(file, fmt)?,
                read_vec2(file, fmt)?,
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
            normals.push(read_vec3(file, fmt)?);
        }
        Ok(SurfaceObject {
            transform,
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

    /// Create a SurfaceExport object with the given export mode
    pub fn begin_export<'a>(&'a self, mode: SurfaceExportMode) -> SurfaceExport<'a> {
        SurfaceExport {
            surface: self,
            genmode: mode,
        }
    }
}

/// Surface export mode
pub enum SurfaceExportMode {
    Mesh(SurfaceGenMode),
    // Surface
}

/// Surface export object
pub struct SurfaceExport<'a> {
    pub surface: &'a SurfaceObject,
    pub genmode: SurfaceExportMode,
}

/// Insert the key-value pair into the given hashmap if it does not already exist.
/// Returns true if the value was successfully inserted.
fn insert_if_not_exist<T, U>(hashmap: &mut HashMap<T, U>, key: T, value: U) -> bool
where
    T: Eq + std::hash::Hash,
{
    if hashmap.contains_key(&key) {
        false
    } else {
        hashmap.insert(key, value);
        true
    }
}

impl<'a> SurfaceExport<'a> {
    /// Export as a mesh with the given generation mode
    fn export_mesh<W>(&self, writer: &mut W, mode: SurfaceGenMode) -> Result<(), Box<dyn Error>>
    where
        W: Write,
    {
        let mut vertices = HashMap::<[u32; 3], u64>::new();
        let mut normals = HashMap::<[u32; 3], u64>::new();
        let mut texcoords = HashMap::<[u32; 2], u64>::new();
        let gen = self.surface.generate_meshes(mode);
        for mesh in gen.iter() {
            for quad in mesh.quads.iter() {
                for point in quad.points.iter() {
                    let nvert = vertices.len() as u64;
                    if insert_if_not_exist(
                        &mut vertices,
                        reinterpret_vec3(&point.vertex),
                        nvert + 1,
                    ) {
                        writeln!(
                            writer,
                            "v {} {} {}",
                            point.vertex.x, point.vertex.y, point.vertex.z
                        )?;
                    }
                    let nnorm = normals.len() as u64;
                    if insert_if_not_exist(&mut normals, reinterpret_vec3(&point.normal), nnorm + 1)
                    {
                        writeln!(
                            writer,
                            "vn {} {} {}",
                            -point.normal.x, -point.normal.y, -point.normal.z
                        )?;
                    }
                    let ntex = texcoords.len() as u64;
                    if insert_if_not_exist(
                        &mut texcoords,
                        reinterpret_vec2(&point.texcoord),
                        ntex + 1,
                    ) {
                        writeln!(writer, "vt {} {}", point.texcoord.x, point.texcoord.y)?;
                    }
                }
            }
        }
        for mesh in gen.iter() {
            for quad in mesh.quads.iter() {
                write!(writer, "f")?;
                for point in quad.points.iter() {
                    write!(
                        writer,
                        " {}/{}/{}",
                        vertices[&reinterpret_vec3(&point.vertex)],
                        texcoords[&reinterpret_vec2(&point.texcoord)],
                        normals[&reinterpret_vec3(&point.normal)]
                    )?;
                }
                writeln!(writer, "")?;
            }
        }
        Ok(())
    }

    pub fn export_obj<W>(&self, writer: &mut W) -> Result<(), Box<dyn Error>>
    where
        W: Write,
    {
        match self.genmode {
            SurfaceExportMode::Mesh(x) => {
                self.export_mesh(writer, x)?;
            }
        }
        Ok(())
    }
}
