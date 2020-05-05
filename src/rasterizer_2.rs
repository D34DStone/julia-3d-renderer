use nalgebra as na;
use na::Vector4 as v4;
use na::Vector3 as v3;
use na::Vector2 as v2;
use na::Matrix4 as mat4;

use std::convert::TryInto;

struct IVertex {
    pub coords  : v3<f32>,
    pub color   : v3<f32>,
}

#[derive(Clone, Copy)]
struct Vertex {
    coords  : v4<f32>,
    color   : v3<f32>,
}

struct Raster2<'a> {
    window_coords   : v2<i32>,
    baricentric     : v2<f32>,
    basis           : &'a [Vertex; 2], 
}

type Basis<'a> = [&'a Vertex; 3];

struct Raster3<'a> {
    window_coords   : v2<i32>,
    baricentric     : v3<f32>,
    basis           : &'a Basis<'a>,
}

struct Fragment {
    window_coords   : v2<i32>,
    depth           : f32,
    color           : v3<f32>,
}

struct Julia3D {
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
        let rasters     = self.rasterize_polygon(&basis); // unimpl
        let fragments   = self.fragment(rasters);           // mock
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
        let coords4 = v4::new(v.coords.x, v.coords.y, v.coords.z, 1.);
        let proj_op = mat4::new_perspective(1., 3.14 / 4., 0.1, 1000.0);
        Vertex {
            coords  : proj_op * coords4,
            color   : v.color,
        }
    }

    pub fn rasterize_polygon<'a>(&self, basis: &'a Basis) -> Vec<Raster3<'a>> {
        let ab = self.rasterize_line(basis, (0, 1));
        let bc = self.rasterize_line(basis, (1, 2));
        let ac = self.rasterize_line(basis, (0, 2));
        let mut result = vec![];
        result.extend(ab.into_iter());
        result.extend(bc.into_iter());
        result.extend(ac.into_iter());
        result
    }

    fn rasterize_line<'a>(&self, basis: &'a Basis, map: (usize, usize)) -> Vec<Raster3<'a>>{
        let fst = utils::rasterize_dot(basis[map.0].coords.xy(), self.shape);
        let snd = utils::rasterize_dot(basis[map.1].coords.xy(), self.shape);
        unimplemented!()
    }

    pub fn fragment(&self, rs: Vec<Raster3>) -> Vec<Fragment> {
        rs.into_iter().map(|r| Fragment {
            window_coords   : r.window_coords,
            depth           : 0.,
            color           : v3::new(255., 0., 0.),
        }).collect()
    }

    pub fn buff_offset(&self, window_coords: v2<i32>) -> usize {
        let coords_u = window_coords + self.shape / 2;
        (coords_u.x + coords_u.y * self.shape.x) as usize
    }
}

mod utils {
    use super::*;

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
