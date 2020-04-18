use super::depth_buffer::DepthBuffer;
use super::math_utils;
use std::mem::swap;
use std::cmp::{min, max};

pub fn line(
             color: (u8, u8, u8),
             mut p1: (f32, f32, f32), 
             mut p2: (f32, f32, f32), 
             mut buffer: DepthBuffer) -> DepthBuffer {
    if p1.0 > p2.0 { 
        swap(&mut p1, &mut p2); 
    }
    let (width_px, height_px) = buffer.get_size();
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;
    let (dx, dy, dz) = (
        x2 - x1,
        y2 - y1,
        z2 - z1,
    );
    let (dx_px, dy_px) = (
        (dx * width_px as f32),
        (dy * height_px as f32),
    );
    let len_px = (dx_px * dx_px + dy_px * dy_px).sqrt().round();
    let mut t_cur = 0_f32;
    let t_end  = (x2 - x1) / dx;
    let t_step = t_end / len_px;

    loop {
        let (x, y, z) = (
            t_cur * dx + x1,
            t_cur * dy + y1,
            t_cur * dz + z1,
        );
        buffer.plot_f32((x, y, z), color);

        t_cur += t_step;
        if t_cur > t_end {
            break;
        }
    }

    buffer
}

pub fn triagnle(
             color: (u8, u8, u8),
             a: (f32, f32, f32), 
             b: (f32, f32, f32), 
             c: (f32, f32, f32), 
             mut buffer: DepthBuffer) -> DepthBuffer {
    let (width_px, height_px) = buffer.get_size();
    let (xa, ya, a_depth) = a;
    let (xb, yb, b_depth) = b;
    let (xc, yc, c_depth) = c;
    let (xa_px, ya_px) = math_utils::rasterize_dot((xa, ya), (width_px, height_px));
    let (xb_px, yb_px) = math_utils::rasterize_dot((xb, yb), (width_px, height_px));
    let (xc_px, yc_px) = math_utils::rasterize_dot((xc, yc), (width_px, height_px));
    let (x_px_min, y_px_min) = (
        min(xa_px, min(xb_px, xc_px)),
        min(ya_px, min(yb_px, yc_px)),
    );
    let (x_px_max, y_px_max) = (
        max(xa_px, max(xb_px, xc_px)),
        max(ya_px, max(yb_px, yc_px)),
    );
    let vab_px = (
        (xb_px - xa_px) as f32,
        (yb_px - ya_px) as f32,
    );
    let vac_px = (
        (xc_px - xa_px) as f32,
        (yc_px - ya_px) as f32,
    );
    for x in x_px_min..=x_px_max {
        for y in y_px_min..=y_px_max {
            if !math_utils::is_dot_inside_triangle(
                (x, y),
                (xa_px, ya_px),
                (xb_px, yb_px),
                (xc_px, yc_px)) {
                continue;
            }
            let vec_px = (
                (x - xa_px) as f32,
                (y - ya_px) as f32,
            );
            let (k1, k2) = math_utils::as_linear_combination(vec_px, vab_px, vac_px);
            let depth = a_depth + k1 * (b_depth - a_depth) + k2 * (c_depth - a_depth);

            let (r, g, b) = color;
            let color2 = (
                (depth * r as f32).round() as u8,
                (depth * g as f32).round() as u8,
                (depth * b as f32).round() as u8
            );

            buffer.plot_i32((x, y, depth), color2);
        }
    }
    buffer
}

