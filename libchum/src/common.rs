use core::fmt::Display;
use std::hash::{Hash, Hasher};
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

// A 4x4 Matrix
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Mat4x4 {
    pub mat: [f32; 16],
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Color
{
    pub values: [f32; 4]
}