use crate::format::TotemFormat;
use std::io::{self, Read, Write};
use nalgebra;
use std::mem;

pub type Vector3 = nalgebra::Vector3<f32>;
pub type Vector2 = nalgebra::Vector2<f32>;
pub type Quaternion = nalgebra::Quaternion<f32>;
pub type Mat4x4 = nalgebra::Matrix4<f32>;
pub type Mat3x3 = nalgebra::Matrix3<f32>;
pub type Color = nalgebra::Vector4<f32>;

pub fn read_quat<R: Read>(reader: &mut R, fmt: TotemFormat) -> io::Result<Quaternion> {
    Ok(Quaternion::new(
        fmt.read_f32(reader)?,
        fmt.read_f32(reader)?,
        fmt.read_f32(reader)?,
        fmt.read_f32(reader)?
    ))
}

pub fn write_quat<W: Write>(q: &Quaternion, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
    fmt.write_f32(writer, q[0])?;
    fmt.write_f32(writer, q[1])?;
    fmt.write_f32(writer, q[2])?;
    fmt.write_f32(writer, q[3])?;
    Ok(())
}

/// Read a Vector3 from a file (12 bytes)
pub fn read_vec3<R: Read>(reader: &mut R, fmt: TotemFormat) -> io::Result<Vector3> {
    Ok(Vector3::new(
        fmt.read_f32(reader)?,
        fmt.read_f32(reader)?,
        fmt.read_f32(reader)?
    ))
}

/// Write a Vector3 to a file (12 bytes)
pub fn write_vec3<W: Write>(v: &Vector3, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
    fmt.write_f32(writer, v.x)?;
    fmt.write_f32(writer, v.y)?;
    fmt.write_f32(writer, v.z)?;
    Ok(())
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
            mem::transmute::<f32, u32>(v.y)
        ]
    }
}

/// Linear interpolation between four Vector3
pub fn qlerp_vec3(values: &[[Vector3; 2]; 2], t_x: f32, t_y: f32) -> Vector3 {
    values[0][0] * (1.0 - t_x) * (1.0 - t_y)
    + values[0][1] * (t_x) * (1.0 - t_y)
    + values[1][1] * (t_x) * (t_y)
    + values[1][0] * (1.0 - t_x) * (t_y)
}

/// Read a Vector2 from a file (8 bytes)
pub fn read_vec2<R: Read>(reader: &mut R, fmt: TotemFormat) -> io::Result<Vector2> {
    Ok(Vector2::new(
        fmt.read_f32(reader)?,
        fmt.read_f32(reader)?
    ))
}

/// Write a Vector2 to a file (8 bytes)
pub fn write_vec2<W: Write>(v: &Vector2, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
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
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Point {
    pub vertex: Vector3,
    pub texcoord: Vector2,
    pub normal: Vector3,
}

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
                self.points[2].clone(),
                self.points[1].clone(),
            ],
        };
        let b = Tri {
            points: [
                self.points[0].clone(),
                self.points[3].clone(),
                self.points[2].clone(),
            ],
        };
        [a, b]
    }
}

/// Read a Mat3x3 from a file (36 bytes)
pub fn read_mat3<R: Read>(reader: &mut R, fmt: TotemFormat) -> io::Result<Mat3x3> {
    let mut buf = [0.0f32; 9];
    fmt.read_f32_into(reader, &mut buf)?;
    Ok(Mat3x3::from_row_slice(&buf))
}

/// Write a Mat3x3 to a file (36 bytes)
pub fn write_mat3<W: Write>(mat: &Mat3x3, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
    for value in mat.iter() {
        fmt.write_f32(writer, *value)?;
    }
    Ok(())
}

/// Read a Mat4x4 from a file (64 bytes)
pub fn read_mat4<R: Read>(reader: &mut R, fmt: TotemFormat) -> io::Result<Mat4x4> {
    let mut buf = [0.0f32; 16];
    fmt.read_f32_into(reader, &mut buf)?;
    Ok(Mat4x4::from_row_slice(&buf))
}

/// Write a Mat4x4 to a file (64 bytes)
pub fn write_mat4<W: Write>(mat: &Mat4x4, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
    for value in mat.iter() {
        fmt.write_f32(writer, *value)?;
    }
    Ok(())
}

/// Read an RGBA float-based color from a file (16 bytes)
pub fn read_color<R: Read>(reader: &mut R, fmt: TotemFormat) -> io::Result<Color> {
    let mut buf = [0.0f32; 4];
    fmt.read_f32_into(reader, &mut buf)?;
    Ok(Color::new(buf[0], buf[1], buf[2], buf[3]))
}

/// Write an RGBA float-based color to a file (16 bytes)
pub fn write_color<W: Write>(col: &Color, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
    for value in col.iter() {
        fmt.write_f32(writer, *value)?;
    }
    Ok(())
}

/// Common header used by many different structures
#[derive(Clone, Debug)]
#[repr(C)]
pub struct TransformationHeader {
    pub floats: [f32; 4],
    pub transform: Mat4x4,
    pub junk: [u8; 16],
    pub item_type: u16,
    pub item_subtype: u16,
}

impl TransformationHeader {
    /// Read a transformation header from a file (100 bytes)
    pub fn read_from<R: Read>(
        reader: &mut R,
        fmt: TotemFormat,
    ) -> io::Result<TransformationHeader> {
        let mut floats = [0.0f32; 4];
        fmt.read_f32_into(reader, &mut floats)?;
        let transform = read_mat4(reader, fmt)?;
        let mut junk = [0u8; 16];
        fmt.read_u8_into(reader, &mut junk)?;
        let item_type = fmt.read_u16(reader)?;
        let item_subtype = fmt.read_u16(reader)?;
        Ok(TransformationHeader {
            floats,
            transform,
            junk,
            item_type,
            item_subtype,
        })
    }
    
    /// Write a transformation header to a file (100 bytes)
    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        for value in self.floats.iter() {
            fmt.write_f32(writer, *value)?;
        }
        write_mat4(&self.transform, writer, fmt)?;
        for value in self.junk.iter() {
            fmt.write_u8(writer, *value)?;
        }
        fmt.write_u16(writer, self.item_type)?;
        fmt.write_u16(writer, self.item_subtype)?;
        Ok(())
    }
}