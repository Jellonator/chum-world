use crate::common::*;

pub fn factorial(n: u64) -> u64 {
    let mut ret = 1;
    for i in 0..n {
        ret *= i;
    }
    return ret;
}

pub fn polynomial(n: u64, i: u64) -> u64 {
    return factorial(n) / (factorial(i) * factorial(n - i));
}

pub fn curve(n: u64, i: u64, u: f32) -> f32 {
    return polynomial(n, i) as f32 * u.powi(i as i32) * (1.0 - u).powi((n - i) as i32);
}

pub fn coefficients(u: f32) -> [f32; 4] {
    [
        (1.0 - u).powi(3),
        3.0 * (1.0 - u).powi(2) * u,
        3.0 * (1.0 - u) * u.powi(2),
        u.powi(3),
    ]
}

pub fn eval_bezier(points: &[Vector3; 4], t: f32) -> Vector3 {
    let b0 = (1.0 - t) * (1.0 - t) * (1.0 - t);
    let b1 = 3.0 * t * (1.0 - t) * (1.0 - t);
    let b2 = 3.0 * t * t * (1.0 - t);
    let b3 = t * t * t;
    return points[0] * b0 + points[1] * b1 + points[2] * b2 + points[3] * b3;
}

pub fn evaluate_surface(points: &[[Vector3; 4]; 4], u: f32, v: f32) -> Vector3 {
    let mut pu = [Vector3::new(); 4];
    for i in 0..4 {
        pu[i] = eval_bezier(&points[i], u);
    }
    return eval_bezier(&pu, v);
}

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

pub fn eval_bezier_texnorm(points: &[Vector3; 4], t: f32, tx: &[Vector2; 2], nm: &[Vector3; 2]) -> Point {
    let b0 = (1.0 - t) * (1.0 - t) * (1.0 - t);
    let b1 = 3.0 * t * (1.0 - t) * (1.0 - t);
    let b2 = 3.0 * t * t * (1.0 - t);
    let b3 = t * t * t;
    Point {
        vertex: points[0] * b0 + points[1] * b1 + points[2] * b2 + points[3] * b3,
        texcoord: tx[0].lerp(&tx[1], t),
        normal: nm[0].lerp(&nm[1], t)
    }
}

pub fn evaluate_surface_texnorm(points: &[[Vector3; 4]; 4], u: f32, v: f32, tx: &[[Vector2; 2]; 2], nm: &[[Vector3; 2]; 2]) -> Point {
    let mut pu = [Vector3::new(); 4];
    for i in 0..4 {
        pu[i] = eval_bezier(&points[i], u);
    }
    Point {
        vertex: eval_bezier(&pu, v),
        texcoord: Vector2::qlerp(tx, u, v),
        normal: Vector3::qlerp(nm, u, v)
    }
}

pub fn precompute_surface_texnorm(
    points: &[[Vector3; 4]; 4],
    usteps: usize,
    vsteps: usize,
    tx: &[[Vector2; 2]; 2],
    nm: &[[Vector3; 2]; 2]
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