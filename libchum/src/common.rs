use crate::format::TotemFormat;
use core::fmt::Display;
use std::hash::{Hash, Hasher};
use std::io::{self, Read, Write};
use std::mem;
use std::ops::{Add, Mul, Sub};

/// Basic 3d vector
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Eq for Vector3 {}

impl Hash for Vector3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Rust doesn't let you use floats as hash keys, so I'm just gonna fit some square pegs into round holes
        unsafe {
            state.write_u32(mem::transmute::<f32, u32>(self.x));
            state.write_u32(mem::transmute::<f32, u32>(self.y));
            state.write_u32(mem::transmute::<f32, u32>(self.z));
        }
    }
}

impl Vector3 {
    pub fn read_from<R: Read>(reader: &mut R, fmt: TotemFormat) -> io::Result<Vector3> {
        Ok(Vector3 {
            x: fmt.read_f32(reader)?,
            y: fmt.read_f32(reader)?,
            z: fmt.read_f32(reader)?,
        })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        fmt.write_f32(writer, self.x)?;
        fmt.write_f32(writer, self.y)?;
        fmt.write_f32(writer, self.z)?;
        Ok(())
    }

    pub fn with(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Create a new, empty Vector3
    pub fn new() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Linear interpolation
    pub fn lerp(&self, other: &Vector3, t: f32) -> Vector3 {
        Vector3 {
            x: self.x * (1.0 - t) + other.x * t,
            y: self.y * (1.0 - t) + other.y * t,
            z: self.z * (1.0 - t) + other.z * t,
        }
    }

    /// Linear interpolation between four points
    pub fn qlerp(values: &[[Vector3; 2]; 2], t_x: f32, t_y: f32) -> Vector3 {
        values[0][0] * (1.0 - t_x) * (1.0 - t_y)
            + values[0][1] * (t_x) * (1.0 - t_y)
            + values[1][1] * (t_x) * (t_y)
            + values[1][0] * (1.0 - t_x) * (t_y)
    }

    /// Get the length of this vector
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Display for Vector3 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

/// Basic 2d vector
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Eq for Vector2 {}

impl Hash for Vector2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            state.write_u32(mem::transmute::<f32, u32>(self.x));
            state.write_u32(mem::transmute::<f32, u32>(self.y));
        }
    }
}

impl Vector2 {
    pub fn read_from<R: Read>(reader: &mut R, fmt: TotemFormat) -> io::Result<Vector2> {
        Ok(Vector2 {
            x: fmt.read_f32(reader)?,
            y: fmt.read_f32(reader)?,
        })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        fmt.write_f32(writer, self.x)?;
        fmt.write_f32(writer, self.y)?;
        Ok(())
    }

    pub fn with(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    /// Create a new Vector2
    pub fn new() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }

    /// Linear interpolation
    pub fn lerp(&self, other: &Vector2, t: f32) -> Vector2 {
        Vector2 {
            x: self.x * (1.0 - t) + other.x * t,
            y: self.y * (1.0 - t) + other.y * t,
        }
    }

    /// Two-way linear interpolation
    pub fn qlerp(values: &[[Vector2; 2]; 2], t_x: f32, t_y: f32) -> Vector2 {
        values[0][0] * (1.0 - t_x) * (1.0 - t_y)
            + values[0][1] * (t_x) * (1.0 - t_y)
            + values[1][1] * (t_x) * (t_y)
            + values[1][0] * (1.0 - t_x) * (t_y)
    }

    /// Length of the vector
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Display for Vector2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "({:.3}, {:.3})", self.x, self.y)
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
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

/// A 3x3 Matrix
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Mat3x3 {
    pub mat: [f32; 9],
}

impl Mat3x3 {
    pub fn new_basis() -> Mat3x3 {
        Mat3x3 {
            mat: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn read_from<R: Read>(reader: &mut R, fmt: TotemFormat) -> io::Result<Mat3x3> {
        let mut buf = [0.0f32; 9];
        fmt.read_f32_into(reader, &mut buf)?;
        Ok(Mat3x3 { mat: buf })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        for value in self.mat.iter() {
            fmt.write_f32(writer, *value)?;
        }
        Ok(())
    }

    pub fn swap_order(&self) -> Mat3x3 {
        let m = &self.mat;
        Mat3x3 {
            mat: [m[0], m[3], m[6], m[1], m[4], m[7], m[2], m[5], m[8]],
        }
    }
}

// A 4x4 Matrix
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Mat4x4 {
    pub mat: [f32; 16],
}

impl Mat4x4 {
    pub fn new_basis() -> Mat4x4 {
        Mat4x4 {
            mat: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn read_from<R: Read>(reader: &mut R, fmt: TotemFormat) -> io::Result<Mat4x4> {
        let mut buf = [0.0f32; 16];
        fmt.read_f32_into(reader, &mut buf)?;
        Ok(Mat4x4 { mat: buf })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        for value in self.mat.iter() {
            fmt.write_f32(writer, *value)?;
        }
        Ok(())
    }

    pub fn swap_order(&self) -> Mat4x4 {
        let m = &self.mat;
        Mat4x4 {
            mat: [
                m[0], m[4], m[8], m[12], m[1], m[5], m[9], m[13], m[2], m[6], m[10], m[14], m[3],
                m[7], m[11], m[15],
            ],
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Color {
    pub values: [f32; 4],
}

impl Color {
    pub fn read_from<R: Read>(reader: &mut R, fmt: TotemFormat) -> io::Result<Color> {
        let mut buf = [0.0f32; 4];
        fmt.read_f32_into(reader, &mut buf)?;
        Ok(Color { values: buf })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        for value in self.values.iter() {
            fmt.write_f32(writer, *value)?;
        }
        Ok(())
    }
}

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
    pub fn read_from<R: Read>(
        reader: &mut R,
        fmt: TotemFormat,
    ) -> io::Result<TransformationHeader> {
        let mut floats = [0.0f32; 4];
        fmt.read_f32_into(reader, &mut floats)?;
        let transform = Mat4x4::read_from(reader, fmt)?;
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

    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        for value in self.floats.iter() {
            fmt.write_f32(writer, *value)?;
        }
        self.transform.write_to(writer, fmt)?;
        for value in self.junk.iter() {
            fmt.write_u8(writer, *value)?;
        }
        fmt.write_u16(writer, self.item_type)?;
        fmt.write_u16(writer, self.item_subtype)?;
        Ok(())
    }
}
