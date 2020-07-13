use crate::common::*;

/// Evaluate bezier through points at t.
pub fn eval_bezier(points: &[Vector3; 4], t: f32) -> Vector3 {
    let b0 = (1.0 - t) * (1.0 - t) * (1.0 - t);
    let b1 = 3.0 * t * (1.0 - t) * (1.0 - t);
    let b2 = 3.0 * t * t * (1.0 - t);
    let b3 = t * t * t;
    return points[0] * b0 + points[1] * b1 + points[2] * b2 + points[3] * b3;
}

/// Evaluate a surface patch through points at the position (u, v).
/// u and v should be in the range [0, 1]
pub fn evaluate_surface(points: &[[Vector3; 4]; 4], u: f32, v: f32) -> Vector3 {
    let mut pu = [Vector3::new(); 4];
    for i in 0..4 {
        pu[i] = eval_bezier(&points[i], u);
    }
    return eval_bezier(&pu, v);
}

/// Precompute a surface patch so that (usteps, vsteps) quads can be generated.
/// The resulting Vec<Vec<Vector3>> will have a size of usteps+1, and each
/// Vec<Vector3> will have a size of vsteps+1
pub fn precompute_surface(
    points: &[[Vector3; 4]; 4],
    usteps: usize,
    vsteps: usize,
) -> Vec<Vec<Vector3>> {
    let mut vec = Vec::with_capacity(usteps);
    for iu in 0..=usteps {
        let mut subvec = Vec::new();
        for iv in 0..=vsteps {
            let u = (iu as f32) / (usteps as f32);
            let v = (iv as f32) / (vsteps as f32);
            subvec.push(evaluate_surface(&points, u, v));
        }
        vec.push(subvec);
    }
    vec
}

pub fn eval_bezier_texnorm(
    points: &[Vector3; 4],
    t: f32,
    tx: &[Vector2; 2],
    nm: &[Vector3; 2],
) -> Point {
    let b0 = (1.0 - t) * (1.0 - t) * (1.0 - t);
    let b1 = 3.0 * t * (1.0 - t) * (1.0 - t);
    let b2 = 3.0 * t * t * (1.0 - t);
    let b3 = t * t * t;
    Point {
        vertex: points[0] * b0 + points[1] * b1 + points[2] * b2 + points[3] * b3,
        texcoord: tx[0].lerp(&tx[1], t),
        normal: nm[0].lerp(&nm[1], t),
    }
}

pub fn evaluate_surface_texnorm(
    points: &[[Vector3; 4]; 4],
    u: f32,
    v: f32,
    tx: &[[Vector2; 2]; 2],
    nm: &[[Vector3; 2]; 2],
) -> Point {
    let mut pu = [Vector3::new(); 4];
    for i in 0..4 {
        pu[i] = eval_bezier(&points[i], u);
    }
    Point {
        vertex: eval_bezier(&pu, v),
        texcoord: Vector2::qlerp(tx, u, v),
        normal: Vector3::qlerp(nm, u, v),
    }
}

pub fn precompute_surface_texnorm(
    points: &[[Vector3; 4]; 4],
    usteps: usize,
    vsteps: usize,
    tx: &[[Vector2; 2]; 2],
    nm: &[[Vector3; 2]; 2],
) -> Vec<Vec<Point>> {
    let mut vec = Vec::with_capacity(usteps);
    for iu in 0..=usteps {
        let mut subvec = Vec::new();
        for iv in 0..=vsteps {
            let u = (iu as f32) / (usteps as f32);
            let v = (iv as f32) / (vsteps as f32);
            subvec.push(evaluate_surface_texnorm(&points, u, v, tx, nm));
        }
        vec.push(subvec);
    }
    vec
}
