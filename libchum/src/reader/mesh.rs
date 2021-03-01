use crate::common::*;
use crate::format::TotemFormat;
use crate::reader::skin;
use crate::scene;
use std::collections::HashMap;
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
    pub sphere_shapes: Vec<SphereShape>,
    pub cuboid_shapes: Vec<CuboidShape>,
    pub cylinder_shapes: Vec<CylinderShape>,
    pub strip_order: Vec<u32>,
}

chum_struct_generate_readwrite! {
    #[derive(Clone, Debug)]
    pub struct SphereShape {
        pub pos: [Vector3],
        pub radius: [f32],
    }
}

impl Default for SphereShape {
    fn default() -> SphereShape {
        SphereShape {
            pos: Vector3::zero(),
            radius: 1.0
        }
    }
}

chum_struct_generate_readwrite! {
    #[derive(Clone, Debug)]
    pub struct CuboidShape {
        pub transform: [Transform3D],
        pub junk: [ignore [fixed array [u8] 16] [0u8; 16]],
    }
}

impl Default for CuboidShape {
    fn default() -> CuboidShape {
        CuboidShape {
            transform: Transform3D::identity(),
            junk: ()
        }
    }
}

chum_struct_generate_readwrite! {
    #[derive(Clone, Debug)]
    pub struct CylinderShape {
        pub position: [Vector3],
        pub height: [f32],
        pub normal: [Vector3],
        pub junk: [ignore [fixed array [u8] 4] [0u8; 4]],
        pub radius: [f32],
    }
}

impl Default for CylinderShape {
    fn default() -> CylinderShape {
        CylinderShape {
            position: Vector3::zero(),
            height: 1.0,
            normal: Vector3::new(0.0, 1.0, 0.0),
            junk: (),
            radius: 1.0
        }
    }
}

chum_struct! {
    /// Structured data for Mesh
    #[derive(Clone)]
    pub struct MeshStruct {
        pub materials: [dynamic array [u32] [reference MATERIAL] 0i32],
        pub sphere_shapes: [dynamic array [u32] [struct SphereShape] SphereShape::default()],
        pub cuboid_shapes: [dynamic array [u32] [struct CuboidShape] CuboidShape::default()],
        pub cylinder_shapes: [dynamic array [u32] [struct CylinderShape] CylinderShape::default()],
    }
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

    /// Write a Mesh to a file
    pub fn write_to<W: Write>(&self, file: &mut W, fmt: TotemFormat) -> io::Result<()> {
        use crate::binary::ChumBinary;
        self.transform.write_to(file, fmt)?;
        fmt.write_u32(file, self.vertices.len() as u32)?;
        for value in self.vertices.iter() {
            write_vec3(value, file, fmt)?;
        }
        fmt.write_u32(file, self.texcoords.len() as u32)?;
        for value in self.texcoords.iter() {
            write_vec2(value, file, fmt)?;
        }
        fmt.write_u32(file, self.normals.len() as u32)?;
        for value in self.normals.iter() {
            write_vec3(value, file, fmt)?;
        }
        fmt.write_u32(file, self.strips.len() as u32)?;
        for strip in self.strips.iter() {
            fmt.write_u32(file, strip.strip.vertex_ids.len() as u32)?;
            for vertid in strip.strip.vertex_ids.iter() {
                fmt.write_u16(file, *vertid)?;
            }
            fmt.write_u32(file, strip.strip.material)?;
            fmt.write_u32(file, strip.strip.tri_order)?;
        }
        if self.transform.item_subtype == 4 {
            for strip in self.strips.iter() {
                fmt.write_i32(file, strip.group.unwrap_or(0i32))?;
            }
        }
        let num_stripext = self.strips.iter().filter(|x| x.ext.is_some()).count();
        fmt.write_u32(file, num_stripext as u32)?;
        for ext in self.strips.iter().filter_map(|x| x.ext.as_ref()) {
            fmt.write_u32(file, ext.elements.len() as u32)?;
            for element in ext.elements.iter() {
                fmt.write_u16(file, element.texcoord_id)?;
                fmt.write_u16(file, element.normal_id)?;
            }
        }
        fmt.write_u32(file, self.materials.len() as u32)?;
        for mat in self.materials.iter() {
            fmt.write_i32(file, *mat)?;
        }
        fmt.write_u32(file, self.sphere_shapes.len() as u32)?;
        for sphere in self.sphere_shapes.iter() {
            write_vec3(&sphere.pos, file, fmt)?;
            fmt.write_f32(file, sphere.radius)?;
        }
        fmt.write_u32(file, self.cuboid_shapes.len() as u32)?;
        for cuboid in self.cuboid_shapes.iter() {
            write_transform3d(&cuboid.transform, file, fmt)?;
            fmt.write_bytes(file, &[0; 16])?;
        }
        fmt.write_u32(file, self.cylinder_shapes.len() as u32)?;
        for cylinder in self.cylinder_shapes.iter() {
            write_vec3(&cylinder.position, file, fmt)?;
            fmt.write_f32(file, cylinder.height)?;
            write_vec3(&cylinder.normal, file, fmt)?;
            fmt.write_bytes(file, &[0; 4])?;
            fmt.write_f32(file, cylinder.radius)?;
        }
        fmt.write_u32(file, 0)?;
        fmt.write_u32(file, self.strip_order.len() as u32)?;
        for value in self.strip_order.iter() {
            fmt.write_u32(file, *value)?;
        }
        Ok(())
    }

    /// Read a Mesh from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<Mesh> {
        use crate::binary::ChumBinary;
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
        // TODO: Handle PS2
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
        let footer1: Vec<SphereShape> = (0..num_unk1)
            .map(|_| {
                Ok(SphereShape {
                    pos: read_vec3(file, fmt)?,
                    radius: fmt.read_f32(file)?,
                })
            })
            .collect::<io::Result<_>>()?;
        let num_unk2: u32 = fmt.read_u32(file)?;
        let footer2: Vec<CuboidShape> = (0..num_unk2)
            .map(|_| {
                let transform = read_transform3d(file, fmt)?;
                fmt.skip_n_bytes(file, 16)?;
                Ok(CuboidShape { transform, junk: () })
            })
            .collect::<io::Result<_>>()?;
        let num_unk3: u32 = fmt.read_u32(file)?;
        let footer3: Vec<CylinderShape> = (0..num_unk3)
            .map(|_| {
                // let mut unk1 = [0.0f32; 4];
                // fmt.read_f32_into(file, &mut unk1)?;
                let position = read_vec3(file, fmt)?;
                let height = fmt.read_f32(file)?;
                let normal = read_vec3(file, fmt)?;
                fmt.skip_n_bytes(file, 4)?;
                let radius = fmt.read_f32(file)?;
                Ok(CylinderShape {
                    position,
                    height,
                    normal,
                    radius,
                    junk: ()
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
            sphere_shapes: footer1,
            cuboid_shapes: footer2,
            cylinder_shapes: footer3,
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

    pub fn generate_mesh_skin(&self, info: skin::SkinInfo) -> Option<scene::MeshSkin> {
        if let None = info.skin.meshes.iter().find(|x| **x == info.mesh_id) {
            return None;
        }
        let mut vertices = vec![scene::SkinVertex::default(); self.vertices.len()];
        for (joint_index, group) in info.skin.vertex_groups.iter().enumerate() {
            for section in group
                .sections
                .iter()
                .filter(|x| info.skin.meshes[x.mesh_index as usize] == info.mesh_id)
            {
                for v in section.vertices.iter() {
                    let elem = scene::SkinVertexElement {
                        joint: joint_index as u16,
                        weight: v.weight,
                    };
                    vertices[v.vertex_id as usize].push_element(elem);
                }
            }
        }
        for vertex in vertices.iter_mut() {
            vertex.normalize();
        }
        let out = scene::MeshSkin { vertices };
        Some(out)
    }

    pub fn create_scene_mesh(&self) -> scene::Mesh {
        let gen = self.gen_triangles();
        let mut triangles = HashMap::<i32, Vec<scene::MeshTriangle>>::new();
        for surface in gen.iter() {
            let mat = self.materials[surface.material_index as usize % self.materials.len()];
            let tridata = triangles.entry(mat).or_default();
            for tri in surface.tris.iter() {
                tridata.push(scene::MeshTriangle {
                    corners: [
                        scene::MeshPoint {
                            vertex_id: tri.points[0].vertex_id as u32,
                            texcoord_id: tri.points[0].texcoord_id as u32,
                            normal_id: tri.points[0].normal_id as u32,
                        },
                        scene::MeshPoint {
                            vertex_id: tri.points[2].vertex_id as u32,
                            texcoord_id: tri.points[2].texcoord_id as u32,
                            normal_id: tri.points[2].normal_id as u32,
                        },
                        scene::MeshPoint {
                            vertex_id: tri.points[1].vertex_id as u32,
                            texcoord_id: tri.points[1].texcoord_id as u32,
                            normal_id: tri.points[1].normal_id as u32,
                        },
                    ],
                });
            }
        }
        scene::Mesh {
            vertices: self.vertices.clone(),
            normals: self.normals.clone(),
            texcoords: self.texcoords.clone(),
            triangles,
            skin: None,
        }
    }

    pub fn import_scene_mesh(&mut self, mesh: &scene::Mesh) {
        // First, consolidate components into Point Vec and create new index buffer
        let mut points: Vec<Point> = Vec::new();
        let mut pointmap: HashMap<Point, u32> = HashMap::new();
        let mut indices: HashMap<i32, Vec<u32>> = HashMap::new();
        for (matid, tris) in mesh.triangles.iter() {
            let mut vec: Vec<u32> = Vec::new();
            for triangle in tris.iter() {
                for ipoint in triangle.corners.iter() {
                    let point = Point {
                        vertex: mesh.vertices[ipoint.vertex_id as usize],
                        texcoord: mesh.texcoords[ipoint.texcoord_id as usize],
                        normal: mesh.normals[ipoint.normal_id as usize],
                    };
                    vec.push(if let Some(i) = pointmap.get(&point) {
                        *i
                    } else {
                        let i = points.len() as u32;
                        points.push(point.clone());
                        pointmap.insert(point, i);
                        i
                    });
                }
            }
            indices.insert(*matid, vec);
        }
        // Optimize index buffers
        for buf in indices.values_mut() {
            let mut result = meshopt::stripify(buf.as_slice(), points.len(), 0xFFFFFFFF).unwrap();
            std::mem::swap(buf, &mut result);
        }
        // Re-build vertex, normal, and texcoord buffers while writing strips
        self.vertices.clear();
        self.texcoords.clear();
        self.normals.clear();
        self.strips.clear();
        self.strip_order.clear();
        self.materials.clear();
        let mut vertex_map: HashMap<[u32; 3], u16> = HashMap::new();
        let mut texcoord_map: HashMap<[u32; 2], u16> = HashMap::new();
        let mut normal_map: HashMap<[u32; 3], u16> = HashMap::new();
        let mut material_map: HashMap<i32, u32> = HashMap::new();
        for (matid, strips) in indices.iter() {
            for strip in strips.split(|x| *x == 0xFFFFFFFF) {
                let mat_index = match material_map.get(matid) {
                    Some(i) => *i,
                    None => {
                        let i = self.materials.len() as u32;
                        self.materials.push(*matid);
                        material_map.insert(*matid, i);
                        i
                    }
                };
                let mut s = Strip {
                    vertex_ids: Vec::new(),
                    tri_order: 1,
                    material: mat_index,
                };
                let mut e = StripExt {
                    elements: Vec::new(),
                };
                for index in strip {
                    let point = &points[*index as usize];
                    let v_id = reinterpret_vec3(&point.vertex);
                    let n_id = reinterpret_vec3(&point.normal);
                    let t_id = reinterpret_vec2(&point.texcoord);
                    s.vertex_ids.push(match vertex_map.get(&v_id) {
                        Some(i) => *i,
                        None => {
                            let i = self.vertices.len() as u16;
                            self.vertices.push(point.vertex);
                            vertex_map.insert(v_id, i);
                            i
                        }
                    });
                    e.elements.push(ElementData {
                        normal_id: match normal_map.get(&n_id) {
                            Some(i) => *i,
                            None => {
                                let i = self.normals.len() as u16;
                                self.normals.push(point.normal);
                                normal_map.insert(n_id, i);
                                i
                            }
                        },
                        texcoord_id: match texcoord_map.get(&t_id) {
                            Some(i) => *i,
                            None => {
                                let i = self.texcoords.len() as u16;
                                self.texcoords.push(point.texcoord);
                                texcoord_map.insert(t_id, i);
                                i
                            }
                        },
                    });
                }
                self.strip_order.push(self.strips.len() as u32);
                self.strips.push(StripData {
                    group: None,
                    strip: s,
                    ext: Some(e),
                });
            }
        }
    }

    pub fn transform(&mut self, tx: &Transform3D) {
        for point in self.vertices.iter_mut() {
            *point = tx.transform_point3d(point.to_point()).unwrap().to_vector();
        }
        for vector in self.normals.iter_mut() {
            *vector = tx.transform_vector3d(*vector);
        }
    }
}
