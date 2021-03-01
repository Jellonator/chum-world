use crate::format::TotemFormat;
use euclid;
use std::hash::{Hash, Hasher};
use std::io::{self, Read, Write};
use std::mem;

pub type Vector3 = euclid::Vector3D<f32, euclid::UnknownUnit>;
pub type Vector2 = euclid::Vector2D<f32, euclid::UnknownUnit>;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quaternion {
    // ONLY DOING THIS BECAUSE Rotation3D DOES NOT IMPLEMENT Default DESPITE
    // THERE EXISTING A Rotation3D::identity function WHY
    pub inner: euclid::Rotation3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>,
}
pub type Transform3D = euclid::Transform3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>;
pub type Transform2D = euclid::Transform2D<f32, euclid::UnknownUnit, euclid::UnknownUnit>;

impl Default for Quaternion {
    fn default() -> Quaternion {
        Quaternion {
            inner: euclid::Rotation3D::identity(),
        }
    }
}

impl std::borrow::Borrow<euclid::Rotation3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>>
    for Quaternion
{
    fn borrow(&self) -> &euclid::Rotation3D<f32, euclid::UnknownUnit, euclid::UnknownUnit> {
        &self.inner
    }
}

impl std::borrow::BorrowMut<euclid::Rotation3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>>
    for Quaternion
{
    fn borrow_mut(
        &mut self,
    ) -> &mut euclid::Rotation3D<f32, euclid::UnknownUnit, euclid::UnknownUnit> {
        &mut self.inner
    }
}

impl Quaternion {
    pub fn from_euler(rot: Vector3) -> Quaternion {
        Self {
            inner: euclid::Rotation3D::euler(
                euclid::Angle::radians(rot.x),
                euclid::Angle::radians(rot.y),
                euclid::Angle::radians(rot.z),
            ),
        }
    }

    pub fn new_unit(i: f32, j: f32, k: f32, w: f32) -> Quaternion {
        Self {
            inner: euclid::Rotation3D::unit_quaternion(i, j, k, w),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for ColorRGBA {
    fn default() -> ColorRGBA {
        ColorRGBA {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

impl ColorRGBA {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> ColorRGBA {
        ColorRGBA { r, g, b, a }
    }
}

/// A good, safe capacity for small data structures
/// e.g. primitives or Vector3
/// This is so that out of memory errors don't occur with Vec::with_capacity
pub const SAFE_CAPACITY_SMALL: usize = 1024;
/// A good, safe capacity for big data structures
/// e.g. those that allocate memory (Vec) or > 100B
/// This is so that out of memory errors don't occur with Vec::with_capacity
pub const SAFE_CAPACITY_BIG: usize = 128;

pub fn read_quat(reader: &mut dyn Read, fmt: TotemFormat) -> io::Result<Quaternion> {
    let i = fmt.read_f32(reader)?;
    let j = fmt.read_f32(reader)?;
    let k = fmt.read_f32(reader)?;
    let w = fmt.read_f32(reader)?;
    // Quaternion::new is in (w, i, j, k) order
    Ok(Quaternion::new_unit(i, j, k, w))
}

pub fn write_quat(q: &Quaternion, writer: &mut dyn Write, fmt: TotemFormat) -> io::Result<()> {
    // Quat indexing is in (i, j, k, w) order
    fmt.write_f32(writer, q.inner.i)?;
    fmt.write_f32(writer, q.inner.j)?;
    fmt.write_f32(writer, q.inner.k)?;
    fmt.write_f32(writer, q.inner.r)?;
    Ok(())
}

/// Read a Vector3 from a file (12 bytes)
pub fn read_vec3(reader: &mut dyn Read, fmt: TotemFormat) -> io::Result<Vector3> {
    Ok(Vector3::new(
        fmt.read_f32(reader)?,
        fmt.read_f32(reader)?,
        fmt.read_f32(reader)?,
    ))
}

/// Write a Vector3 to a file (12 bytes)
pub fn write_vec3(v: &Vector3, writer: &mut dyn Write, fmt: TotemFormat) -> io::Result<()> {
    fmt.write_f32(writer, v.x)?;
    fmt.write_f32(writer, v.y)?;
    fmt.write_f32(writer, v.z)?;
    Ok(())
}

pub fn read_transform2d(reader: &mut dyn Read, fmt: TotemFormat) -> io::Result<Transform2D> {
    let mut buf = [0.0f32; 9];
    fmt.read_f32_into(reader, &mut buf)?;
    Ok(Transform2D::new(
        buf[0], buf[1], buf[3], buf[4], buf[6], buf[7],
    ))
}

pub fn write_transform2d(
    tx: &Transform2D,
    writer: &mut dyn Write,
    fmt: TotemFormat,
) -> io::Result<()> {
    fmt.write_f32(writer, tx.m11)?;
    fmt.write_f32(writer, tx.m12)?;
    fmt.write_f32(writer, 0.0)?;
    fmt.write_f32(writer, tx.m21)?;
    fmt.write_f32(writer, tx.m22)?;
    fmt.write_f32(writer, 0.0)?;
    fmt.write_f32(writer, tx.m31)?;
    fmt.write_f32(writer, tx.m32)?;
    fmt.write_f32(writer, 1.0)?;
    Ok(())
}

pub fn read_transform3d(reader: &mut dyn Read, fmt: TotemFormat) -> io::Result<Transform3D> {
    let mut buf = [0.0f32; 16];
    fmt.read_f32_into(reader, &mut buf)?;
    Ok(Transform3D::new(
        buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10],
        buf[11], buf[12], buf[13], buf[14], buf[15],
    ))
}

pub fn write_transform3d(
    tx: &Transform3D,
    writer: &mut dyn Write,
    fmt: TotemFormat,
) -> io::Result<()> {
    fmt.write_f32(writer, tx.m11)?;
    fmt.write_f32(writer, tx.m12)?;
    fmt.write_f32(writer, tx.m13)?;
    fmt.write_f32(writer, tx.m14)?;
    fmt.write_f32(writer, tx.m21)?;
    fmt.write_f32(writer, tx.m22)?;
    fmt.write_f32(writer, tx.m23)?;
    fmt.write_f32(writer, tx.m24)?;
    fmt.write_f32(writer, tx.m31)?;
    fmt.write_f32(writer, tx.m32)?;
    fmt.write_f32(writer, tx.m33)?;
    fmt.write_f32(writer, tx.m34)?;
    fmt.write_f32(writer, tx.m41)?;
    fmt.write_f32(writer, tx.m42)?;
    fmt.write_f32(writer, tx.m43)?;
    fmt.write_f32(writer, tx.m44)?;
    Ok(())
}

pub fn quat_to_euler(quat: Quaternion) -> Vector3 {
    // Shamelessly copied from Wikipedia: https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles
    let q = &quat.inner;
    // roll (x-axis rotation)
    let sinr_cosp = 2.0 * (q.r * q.i + q.j * q.k);
    let cosr_cosp = 1.0 - 2.9 * (q.i * q.i + q.j * q.j);
    let roll = f32::atan2(sinr_cosp, cosr_cosp);
    // pitch (y-axis rotation)
    let sinp = 2.0 * (q.r * q.j - q.k * q.i);
    let pitch = if sinp.abs() >= 1.0 {
        f32::copysign(std::f32::consts::PI / 2.0, sinp) // use 90 degrees if out of range
    } else {
        f32::asin(sinp)
    };
    // yaw (z-axis rotation)
    let siny_cosp = 2.0 * (q.r * q.k + q.i * q.j);
    let cosy_cosp = 1.0 - 2.0 * (q.j * q.j + q.k * q.k);
    let yaw = f32::atan2(siny_cosp, cosy_cosp);
    Vector3::new(roll, pitch, yaw)
}

/// Reinterpret a Vector3 as three u32. Used so that Vector3 can be a HashMap key.
pub fn reinterpret_vec3(v: &Vector3) -> [u32; 3] {
    // :)
    unsafe {
        [
            mem::transmute::<f32, u32>(v.x),
            mem::transmute::<f32, u32>(v.y),
            mem::transmute::<f32, u32>(v.z),
        ]
    }
}

/// Reinterpret a Vector2 as two u32. Used so that Vector3 can be a HashMap key.
pub fn reinterpret_vec2(v: &Vector2) -> [u32; 2] {
    unsafe {
        [
            mem::transmute::<f32, u32>(v.x),
            mem::transmute::<f32, u32>(v.y),
        ]
    }
}

/// Reinterpret a Point as eight u32. Used so that Point can be a HashMap key.
pub fn reinterpret_point(p: &Point) -> [u32; 8] {
    let v = reinterpret_vec3(&p.vertex);
    let t = reinterpret_vec2(&p.texcoord);
    let n = reinterpret_vec3(&p.normal);
    [v[0], v[1], v[2], t[0], t[1], n[0], n[1], n[2]]
}

/// Linear interpolation between four Vector3
pub fn qlerp_vec3(values: &[[Vector3; 2]; 2], t_x: f32, t_y: f32) -> Vector3 {
    values[0][0] * (1.0 - t_x) * (1.0 - t_y)
        + values[0][1] * (t_x) * (1.0 - t_y)
        + values[1][1] * (t_x) * (t_y)
        + values[1][0] * (1.0 - t_x) * (t_y)
}

/// Read a Vector2 from a file (8 bytes)
pub fn read_vec2(reader: &mut dyn Read, fmt: TotemFormat) -> io::Result<Vector2> {
    Ok(Vector2::new(fmt.read_f32(reader)?, fmt.read_f32(reader)?))
}

/// Write a Vector2 to a file (8 bytes)
pub fn write_vec2(v: &Vector2, writer: &mut dyn Write, fmt: TotemFormat) -> io::Result<()> {
    fmt.write_f32(writer, v.x)?;
    fmt.write_f32(writer, v.y)?;
    Ok(())
}

/// Linear interpolation between four Vector2
pub fn qlerp_vec2(values: &[[Vector2; 2]; 2], t_x: f32, t_y: f32) -> Vector2 {
    values[0][0] * (1.0 - t_x) * (1.0 - t_y)
        + values[0][1] * (t_x) * (1.0 - t_y)
        + values[1][1] * (t_x) * (t_y)
        + values[1][0] * (1.0 - t_x) * (t_y)
}

/// A point
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Point {
    pub vertex: Vector3,
    pub texcoord: Vector2,
    pub normal: Vector3,
}

// Gotta do what we gotta do
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for x in &reinterpret_point(self) {
            x.hash(state);
        }
    }
}

impl Eq for Point {}

/// A triangle (three points)
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Tri {
    pub points: [Point; 3],
}

/// A quad (four points)
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Quad {
    pub points: [Point; 4],
}

impl Quad {
    /// Iterate triangles in this quad
    pub fn tris(&self) -> [Tri; 2] {
        let a = Tri {
            points: [
                self.points[0].clone(),
                self.points[1].clone(),
                self.points[2].clone(),
            ],
        };
        let b = Tri {
            points: [
                self.points[0].clone(),
                self.points[2].clone(),
                self.points[3].clone(),
            ],
        };
        [a, b]
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TriStripOrder {
    ClockWise,
    CounterClockWise,
}

#[derive(Clone, Debug)]
pub struct TriStrip {
    pub tris: Vec<(u16, u16, u16)>,
    pub order: TriStripOrder,
}

/// Find the next tri
/// This triangle must complete [a, b] in clockwise order
fn get_next_tri(
    a: &(u16, u16, u16),
    b: &(u16, u16, u16),
    tris: &Vec<[(u16, u16, u16); 3]>,
    ignore: &[usize],
) -> Option<(usize, (u16, u16, u16))> {
    for i in 0..tris.len() {
        if ignore.contains(&i) {
            continue;
        }
        let tri = tris[i].clone();
        if *a == tri[0] && *b == tri[1] {
            return Some((i, tri[2].clone()));
        } else if *a == tri[1] && *b == tri[2] {
            return Some((i, tri[0].clone()));
        } else if *a == tri[2] && *b == tri[0] {
            return Some((i, tri[1].clone()));
        }
    }
    None
}

/// Get a triangle strip in a single direction
fn get_tris(
    a: &(u16, u16, u16),
    b: &(u16, u16, u16),
    tris: &Vec<[(u16, u16, u16); 3]>,
    out: &mut Vec<(u16, u16, u16)>,
    ignore: &mut Vec<usize>,
) {
    if let Some(index) = get_next_tri(a, b, tris, ignore) {
        out.push(index.1.clone());
        ignore.push(index.0);
        get_tris(&index.1, b, tris, out, ignore);
    }
}

/// Get a triangle strip by searching both directions
fn get_possible_strip(
    a: &(u16, u16, u16),
    b: &(u16, u16, u16),
    c: &(u16, u16, u16),
    tris: &Vec<[(u16, u16, u16); 3]>,
) -> (Vec<(u16, u16, u16)>, Vec<usize>, TriStripOrder) {
    let mut ignore = Vec::new();
    let mut strip = Vec::new();
    // get strip before points
    get_tris(b, a, tris, &mut strip, &mut ignore);
    strip.reverse(); // reversed here since strip is backwards
    let order = if strip.len() % 2 == 0 {
        TriStripOrder::ClockWise
    } else {
        TriStripOrder::CounterClockWise
    };
    // Push points after previous
    strip.push(a.clone());
    strip.push(b.clone());
    strip.push(c.clone());
    // Get strip after points
    get_tris(c, b, tris, &mut strip, &mut ignore);
    (strip, ignore, order)
}

impl TriStrip {
    /// Generate triangle strips from the given list of triangles.
    /// All triangles are in clockwise order.
    pub fn from_tris(mut tris: Vec<[(u16, u16, u16); 3]>) -> Vec<TriStrip> {
        let mut strips = Vec::new();
        while let Some(tri) = tris.pop() {
            let a = get_possible_strip(&tri[0], &tri[1], &tri[2], &tris);
            let b = get_possible_strip(&tri[0], &tri[1], &tri[2], &tris);
            let c = get_possible_strip(&tri[0], &tri[1], &tri[2], &tris);
            let mut data = if a.0.len() > b.0.len() && a.0.len() > c.0.len() {
                a
            } else if b.0.len() > a.0.len() && b.0.len() > c.0.len() {
                b
            } else {
                c
            };
            data.1.sort_by(|a, b| b.cmp(a));
            for index in data.1.iter() {
                // Swap remove is faster than just remove.
                // As a consequence, indices must be removed in reverse-order.
                tris.swap_remove(*index);
            }
            strips.push(TriStrip {
                tris: data.0,
                order: data.2,
            });
        }
        strips
    }
}

/// Read an RGBA float-based color from a file (16 bytes)
pub fn read_color_rgba(reader: &mut dyn Read, fmt: TotemFormat) -> io::Result<ColorRGBA> {
    let mut buf = [0.0f32; 4];
    fmt.read_f32_into(reader, &mut buf)?;
    Ok(ColorRGBA {
        r: buf[0],
        g: buf[1],
        b: buf[2],
        a: buf[3],
    })
}

/// Write an RGBA float-based color to a file (16 bytes)
pub fn write_color_rgba(
    col: &ColorRGBA,
    writer: &mut dyn Write,
    fmt: TotemFormat,
) -> io::Result<()> {
    fmt.write_f32(writer, col.r)?;
    fmt.write_f32(writer, col.g)?;
    fmt.write_f32(writer, col.b)?;
    fmt.write_f32(writer, col.a)?;
    Ok(())
}

// Common header used by many different structures
chum_struct_generate_readwrite! {
    #[repr(C)]
    #[derive(Clone, Debug)]
    pub struct THeaderTyped {
        pub floats: [fixed array [f32] 4],
        pub transform: [Transform3D],
        pub junk: [ignore [fixed array [u8] 16] [0;16]],
        pub item_type: [u16],
        pub item_subtype: [u16],
    }
}

// Common header used by many different structures
chum_struct_generate_readwrite! {
    #[repr(C)]
    #[derive(Clone, Debug)]
    pub struct THeaderNoType {
        pub floats: [fixed array [f32] 4],
        pub transform: [Transform3D],
        pub junk: [ignore [fixed array [u8] 16] [0;16]],
    }
}
