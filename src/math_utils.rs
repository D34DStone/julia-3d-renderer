use std::ops::{Sub, Mul, Div};
use std::cmp::{PartialOrd, Ordering};
use nalgebra as na;

pub fn rasterize_dot_1(
    dot             : na::Vector2<f32>,
    dim             : na::Vector2<u32>) -> na::Vector2<i32> {
    assert!(dot.x.abs() <= 1.0, "X must be in range [-1, 1]");
    assert!(dot.y.abs() <= 1.0, "Y must be in range [-1, 1]");
    let dim = dim / 2_u32;
    na::Vector2::<i32>::new(
        (dot.x * dim.x as f32).round() as i32,
        (dot.y * dim.y as f32).round() as i32)
}

pub fn oriented_area_1(
    a: &na::Vector2<f32>,
    b: &na::Vector2<f32>,
    c: &na::Vector2<f32>) -> f32 {
    (b.x - a.x) * (c.y - b.y) - (c.x - b.x) * (b.y - a.y)
}

pub fn is_dot_inside_triangle_1(
    a: na::Vector2<f32>,
    b: na::Vector2<f32>,
    c: na::Vector2<f32>,
    d: na::Vector2<f32>) -> bool {
    let s1 = self::oriented_area_1(&a, &d, &b); 
    let s2 = self::oriented_area_1(&b, &d, &c); 
    let s3 = self::oriented_area_1(&c, &d, &a); 
    (s1 >= 0.0 && s2 >= 0.0 && s3 >= 0.0) ||
    (s1 <= 0.0 && s2 <= 0.0 && s3 <= 0.0)
}


// find alpha and lambda so 
// a = alpha * b + lambda * c.
pub fn as_linear_combination_1(
    a: na::Vector2<f32>,
    b: na::Vector2<f32>,
    c: na::Vector2<f32>) -> na::Vector2<f32> {
    na::Matrix2::<f32>::from_columns(&[b, c]).lu().solve(&a).expect("Cant solve linear equation.")
}


// Build the minimal rect hull around given dots so it containts
// all of them.
pub fn rect_hull(dots: &[na::Vector2<f32>]) -> (na::Vector2<f32>,
                                                na::Vector2<f32>) {
    let mut ox: Vec<f32> = dots
        .iter()
        .map(|v| v.x)
        .into_iter()
        .collect();

    let mut oy: Vec<f32> = dots
        .iter()
        .map(|v| v.y)
        .into_iter()
        .collect();

    ox.sort_by(|a, b| a.partial_cmp(b).unwrap());
    oy.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (
        na::Vector2::<f32>::new(
            ox[0],
            oy[0],
        ),
        na::Vector2::<f32>::new(
            ox.last().unwrap().clone(),
            oy.last().unwrap().clone(),
        ),
    )
}

// THERE GOES LEGACY...


pub fn rasterize_dot(
    d: (f32, f32),
    canvas_size: (u32, u32)) -> (i32, i32) {
    let (x, y) = d;
    let (width, height) = canvas_size;
    if -1.0 > x || x > 1.0 {
        panic!("X is out of range (must be in [-1, 1])");
    }

    if -1.0 > y || y > 1.0 {
        panic!("Y is out of range (must be in [-1, 1])");
    }

    let (width_2, height_2) = (
        width  as f32 / 2_f32,
        height as f32 / 2_f32,
    );

    (
        (x * width_2).round() as i32,
        (y * height_2).round() as i32,
    )
}

pub fn oriented_area<T: Sub<Output=T> + Mul<Output=T> + Copy>(
    a: (T, T),
    b: (T, T),
    c: (T, T)) -> T {
    let (x1, y1) = a;
    let (x2, y2) = b;
    let (x3, y3) = c;
    (x2 - x1) * (y3 - y2) - (x3 - x2) * (y2 - y1)
}

pub fn is_dot_inside_triangle<T: 
    Sub<Output=T> 
    + Mul<Output=T> 
    + PartialOrd 
    + From<i32> 
    + Copy>(
    d: (T, T),
    a: (T, T),
    b: (T, T),
    c: (T, T)) -> bool { 
    let (xd, yd) = d; 
    let (xa, ya) = a;
    let (xb, yb) = b;
    let (xc, yc) = c;
    let s1 = self::oriented_area((xa, ya), (xd, yd), (xb, yb)); 
    let s2 = self::oriented_area((xb, yb), (xd, yd), (xc, yc)); 
    let s3 = self::oriented_area((xc, yc), (xd, yd), (xa, ya)); 
    let zero = T::from(0_i32);
    (s1 >= zero && s2 >= zero && s3 >= zero)
    || (s1 <= zero && s2 <= zero && s3 <= zero)
}

/// Returns alpha and lambda such that 
/// ``` alpha * basis1 + lambda * basis2 = vec ```
pub fn as_linear_combination<T:
    Sub<Output=T> + 
    Mul<Output=T> + 
    Div<Output=T> + 
    Copy> (
    vec: (T, T),
    basis1: (T, T),
    basis2: (T, T)) -> (T, T) {
    let (x0, y0) = vec;
    let (x1, y1) = basis1;
    let (x2, y2) = basis2;
    (
        (x0 * y2 - x2 * y0) / (x1 * y2 - x2 * y1),
        (x0 * y1 - x1 * y0) / (x2 * y1 - x1 * y2),
    )
}

