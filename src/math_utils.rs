use std::ops::{Sub, Mul, Div};
use std::cmp::{PartialOrd};

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

