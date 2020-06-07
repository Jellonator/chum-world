/// Basic 3d vector
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// Basic 2d vector
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

/// A point
#[derive(Clone)]
#[repr(C)]
pub struct Point {
    pub vertex: Vector3,
    pub texcoord: Vector2,
    pub normal: Vector3,
}

/// A triangle (three points)
#[derive(Clone)]
#[repr(C)]
pub struct Tri {
    pub points: [Point; 3],
}

/// A triangle (three points)
#[derive(Clone)]
#[repr(C)]
pub struct Quad {
    pub points: [Point; 4],
}

/// A 3x3 Matrix
#[derive(Clone)]
#[repr(C)]
pub struct Mat3x3 {
    pub mat: [f32; 9],
}

// A 4x4 Matrix
#[derive(Clone)]
#[repr(C)]
pub struct Mat4x4 {
    pub mat: [f32; 16],
}
