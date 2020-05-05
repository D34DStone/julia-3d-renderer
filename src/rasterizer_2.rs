use nalgebra as na;
use na::Vector4 as v4;
use na::Vector3 as v3;
use na::Vector2 as v2;
use na::Matrix4 as mat4;

use std::convert::TryInto;

pub struct IVertex {
    pub coords  : v3<f32>,
    pub color   : v3<f32>,
}

#[derive(Clone, Copy)]
struct Vertex {
    coords  : v4<f32>,
    color   : v3<f32>,
}

type Basis<'a> = [&'a Vertex; 3];

#[derive(Clone, Copy)]
struct Raster<'a> {
    window_coords   : v2<i32>,
    baricentric     : v3<f32>,
    basis           : &'a Basis<'a>,
}

struct Fragment {
    window_coords   : v2<i32>,
    depth           : f32,
    color           : v3<f32>,
}

pub struct Julia3D {
    color_buffer    : Vec<(u8, u8, u8)>,
    depth_buffer    : Vec<f32>,
    shape           : v2<i32>,
}

impl Julia3D {
    pub fn new(width: i32, height: i32) -> Self {
        let buff_sz = (width * height) as usize;
        Self {
            color_buffer    : vec![(0_u8, 0_u8, 0_u8); buff_sz],
            depth_buffer    : vec![-1.; buff_sz],
            shape           : v2::new(width, height),
        }
    }

    pub fn buff_ptr(&self) -> *const (u8, u8, u8) { self.color_buffer.as_ptr() }

    pub fn render_polygon<'a>(
        &mut self,
        a: IVertex,
        b: IVertex,
        c: IVertex) {
        let ivertices   = vec![a, b, c];
        let vertices    = self.geometry(ivertices);
        let basis       = [&vertices[0], &vertices[1], &vertices[2]];
        let rasters     = self.rasterize_polygon(&basis);   // mock (stroke only)
        let fragments   = self.fragment(rasters);           // mock (no depth interpolation)
        for frag in fragments {
            let index = self.buff_offset(frag.window_coords);
            if frag.depth > self.depth_buffer[index] {
                self.depth_buffer[index];
                self.color_buffer[index] = (
                    frag.color.x.round() as u8,
                    frag.color.y.round() as u8,
                    frag.color.z.round() as u8);
            }
        }
    }

    pub fn geometry(&self, ivertices: Vec<IVertex>) -> Vec<Vertex> {
        ivertices.into_iter().map(|v| Self::shader(v)).collect()
    }

    fn shader(v: IVertex) -> Vertex {
        let coords4 = v4::new(v.coords.x / v.coords.z, 
                              v.coords.y / v.coords.z, 
                              v.coords.z,
                              v.coords.z);
        Vertex {
            coords  : coords4,
            color   : v.color,
        }
    }

    pub fn rasterize_polygon<'a>(&self, basis: &'a Basis) -> Vec<Raster<'a>> {
        let mut rasters = vec![];
        rasters.extend(self.rasterize_line(basis, (0, 1)).into_iter());
        rasters.extend(self.rasterize_line(basis, (1, 2)).into_iter());
        rasters.extend(self.rasterize_line(basis, (0, 2)).into_iter());

        let mut scanline = vec![None; self.shape.x as usize];
        let w_2 = self.shape.x / 2;
        for raster in rasters {
            let index = (raster.window_coords.x + w_2) as usize;
            scanline[index] = match scanline[index] {
                None => Some((raster, raster)),
                Some((rast_min, rast_max)) => Some((
                    if rast_min.window_coords.y > raster.window_coords.y { raster } else { rast_min },
                    if rast_max.window_coords.y < raster.window_coords.y { raster } else { rast_max }))
            }
        }

        let mut result = vec![];
        for mb_rasters in scanline {
            match mb_rasters {
                None => {
                    // no-op
                },
                Some((rast_min, rast_max)) => {
                    let x = rast_min.window_coords.x;
                    let y_min = rast_min.window_coords.y;
                    let y_max = rast_max.window_coords.y;
                    for y in y_min..=y_max {
                        let win_coords = v2::new(x, y);
                        let (k1, k2) = utils::linear_interpolation(win_coords, rast_min.window_coords, rast_max.window_coords);
                        result.push(Raster {
                            window_coords   : win_coords,
                            baricentric     : k1 * rast_min.baricentric + k2 * rast_max.baricentric,
                            basis           : basis,
                        });
                    }
                }
            }
        }

        result
    }

    fn rasterize_line<'a>(&self, basis: &'a Basis, map: (usize, usize)) -> Vec<Raster<'a>>{
        let fst = utils::rasterize_dot(basis[map.0].coords.xy(), self.shape);
        let snd = utils::rasterize_dot(basis[map.1].coords.xy(), self.shape);
        utils::line((fst.x, fst.y), (snd.x, snd.y))
            .into_iter()
            .map(|(x, y)| {
                let mut baricentric = v3::new(0., 0., 0.);
                let (k1, k2) = utils::linear_interpolation(v2::new(x, y), fst, snd);
                baricentric[map.0] = k1;
                baricentric[map.1] = k2;
                Raster {
                    window_coords   : v2::new(x, y),
                    baricentric     : baricentric,
                    basis           : basis,
                }
            })
            .collect()
    }

    pub fn fragment(&self, rs: Vec<Raster>) -> Vec<Fragment> {
        rs.into_iter().map(|r| {
            let v1 = r.basis[0];
            let v2 = r.basis[1];
            let v3 = r.basis[2];
            let v1_w = r.baricentric.x / v1.coords.w;
            let v2_w = r.baricentric.y / v2.coords.w;
            let v3_w = r.baricentric.z / v3.coords.w;
            let color = (v1.color * v1_w + v2.color * v2_w + v3.color * v3_w) / (v1_w + v2_w + v3_w);
            Fragment {
                window_coords   : r.window_coords,
                color           : color,
                depth           : 0.,
            }
        }).collect()
    }

    pub fn buff_offset(&self, window_coords: v2<i32>) -> usize {
        let coords_u = window_coords + self.shape / 2;
        (coords_u.x + coords_u.y * self.shape.x) as usize
    }
}

mod utils {
    use super::*;

    pub fn linear_interpolation(d: v2<i32>, d1: v2<i32>, d2: v2<i32>) -> (f32, f32) {
        let dx = (d1.x - d2.x).abs();
        if dx != 0 {
            let q = (d1.x - d.x).abs() as f32 / dx as f32;
            (1. - q, q)
        } else {
            let dy = (d1.y - d2.y).abs();
            let q = (d1.y - d.y).abs() as f32 / dy as f32;
            (1. - q, q)
        }
    }

    pub fn rasterize_dot(dot: v2<f32>, shape: v2<i32>) -> v2<i32> {
        v2::new(
            (dot.x * shape.x as f32 / 2.).round() as i32,
            (dot.y * shape.y as f32 / 2.).round() as i32)
    }

    pub fn line(
        a: (i32, i32),
        b: (i32, i32)) -> Vec<(i32, i32)> { 
        let dx = b.0 - a.0;
        let dy = b.1 - a.1;
        if dx.abs() >= dy.abs() {
            if dx >= 0 {
                line_unit_x(a, dx, dy)
            } else {
                line_unit_x(b, -dx, -dy)
            }
        } else {
            if dy >= 0 {
                line_unit_y(a, dx, dy)
            } else {
                line_unit_y(b, -dx, -dy)
            }
        }
    }

    fn line_unit_y(
        a: (i32, i32),
        dx: i32,
        dy: i32) -> Vec<(i32, i32)> {
        let (dx, xinc) = if dx >= 0 {
            (dx, 1)
        } else {
            (-dx, -1)
        };
        let mut x = a.0;
        let mut d = 2*dx - dy;
        let mut result = Vec::with_capacity((dx + 1) as usize);
        for y in a.1..a.1+dy {
            result.push((x, y,));
            if d > 0 {
                d -= 2 * dy;
                x += xinc
            }
            d += 2 * dx;
        }
        result
    }

    fn line_unit_x(
        a: (i32, i32),
        dx: i32,
        dy: i32)-> Vec<(i32, i32)> {
        let (dy, yinc) = if dy >= 0 {
            (dy, 1)
        } else {
            (-dy, -1)
        };
        let mut y = a.1;
        let mut d = 2*dy - dx;
        let mut result = Vec::with_capacity((dx + 1) as usize);
        for x in a.0..a.0+dx {
            result.push((x, y,));
            if d > 0 {
                d -= 2 * dx;
                y += yinc;
            }
            d += 2 * dy;
        }
        result
    }
}
