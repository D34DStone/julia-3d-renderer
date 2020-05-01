use nalgebra as na;
use super::math_utils as utils;

type RGB = (u8, u8, u8);

pub struct Rasterizer_ {
    color_buffer: Vec<RGB>,
    depth_buffer: Vec<f32>,
    width       : u32,
    height      : u32,
}

pub struct Vertex {
    pub coords  : na::Vector3<f32>,
    pub color   : na::Vector3<u8>,
}

pub struct LineRaster<'a> {
    pub coords      : (i32, i32),
    pub baricentric : [(f32, &'a Vertex); 2],
}

pub struct PolygonRaster<'a> {
    pub coords      : (i32, i32),
    pub baricentric : [(f32, &'a Vertex); 3],
}

// deprecated
pub struct Raster {
    pub coords      : (i32, i32),
    pub baricentric : (f32, f32),
}

impl Rasterizer_ {
    pub fn new(width: u32, height: u32) -> Self {
        let buff_sz = (width * height) as usize;
        Rasterizer_ {
            color_buffer: vec![(0_u8, 0_u8, 0_u8); buff_sz],
            depth_buffer: vec![0.0; buff_sz],
            width       : width,
            height      : height,
        }
    }

    pub fn size(&self) -> na::Vector2<u32> {
        na::Vector2::<u32>::new(self.width, self.height)
    }

    // Map xy integral coords to corresponding buffer index.
    fn buff_offset(&self, coords: na::Vector2<i32>) -> usize {
        let sz = self.size();
        let size_2 = na::Vector2::<i32>::new((sz.x / 2_u32) as i32,(sz.y / 2_u32) as i32);
        let coords_u: na::Vector2<u32> = na::try_convert(size_2 + coords)
            .expect("Can't convert to unsigned coords");
        (coords_u.x + coords_u.y * self.width) as usize
    }

    pub fn rasterize_triangle_color(
        &mut self, 
        a       : na::Vector3<f32>,
        b       : na::Vector3<f32>,
        c       : na::Vector3<f32>,
        color   : RGB) {
        let ai = utils::rasterize_dot_1(a.xy(), self.size());
        let bi = utils::rasterize_dot_1(b.xy(), self.size());
        let ci = utils::rasterize_dot_1(c.xy(), self.size());
        let (x10, l1) = Self::line_hull_x((ai.x, ai.y), (bi.x, bi.y));
        let (x20, l2) = Self::line_hull_x((bi.x, bi.y), (ci.x, ci.y));
        let (x30, l3) = Self::line_hull_x((ai.x, ai.y), (ci.x, ci.y));
        let xl = std::cmp::min(x10, std::cmp::min(x20, x30));
        let xr = std::cmp::max(x10 + l1.len() as i32, 
                 std::cmp::max(x20 + l2.len() as i32, 
                               x30 + l3.len() as i32));

        let dx = (xr - xl) as usize;
        let mut hull = vec![(std::i32::MAX, std::i32::MIN); dx];
        for i in 0..l1.len() {
            let x = (i as i32 + x10 - xl) as usize;
            let y = l1[i];
            hull[x].0 = std::cmp::min(hull[x].0, y.0);
            hull[x].1 = std::cmp::max(hull[x].1, y.1);
        }
        for i in 0..l2.len() {
            let x = (i as i32 + x20 - xl) as usize;
            let y = l2[i];
            hull[x].0 = std::cmp::min(hull[x].0, y.0);
            hull[x].1 = std::cmp::max(hull[x].1, y.1);
        }
        for i in 0..l3.len() {
            let x = (i as i32 + x30 - xl) as usize;
            let y = l3[i];
            hull[x].0 = std::cmp::min(hull[x].0, y.0);
            hull[x].1 = std::cmp::max(hull[x].1, y.1);
        }

        for x_ctr in 0..dx {
            let x = x_ctr as i32 + xl;
            let y_b = hull[x_ctr].0;
            let y_t = hull[x_ctr].1;
            for y in y_b..=y_t {
                let index = self.buff_offset(na::Vector2::new(x, y));
                self.color_buffer[index] = color;
            }
        }
    }

    pub fn rasterize_line_vertex(
        &mut self,
        a: Vertex,
        b: Vertex) {
        let rasters = self.line_baricentric(a.coords, b.coords);
        for Raster{
            coords:         (x, y), 
            baricentric:    (ka, kb),
        } in rasters {
            let color = (
                (a.color.x as f32 * ka + b.color.x as f32 * kb).round() as u8,
                (a.color.y as f32 * ka + b.color.y as f32 * kb).round() as u8,
                (a.color.z as f32 * ka + b.color.z as f32 * kb).round() as u8,
            );

            let index = self.buff_offset(na::Vector2::new(x, y));
            self.color_buffer[index] = color;
        }
    }

    pub fn rasterize_polygon<'a>(
        &self, 
        a: &'a Vertex,
        b: &'a Vertex,
        c: &'a Vertex) -> Vec<PolygonRaster<'a>> {
        unimplemented!()
    }

    // TODO #1: I couldn't write types for comparator so made it ugly and 
    // let compiler to do it for me. When you'll better in rust, come and fix it. 
    //
    // TODO #2: Wrap return type into struct and doc it because it might be confusing 
    // 4 u in the future.
    pub fn build_segment_hull_x<'a>(rasters: &'a Vec<LineRaster<'a>>) -> (i32, Vec<(&'a LineRaster<'a>, &'a LineRaster<'a>)>) {
        let xl = rasters.iter().min_by(|rast1, rast2| match rast1.coords.0.cmp(&rast2.coords.0) {
            std::cmp::Ordering::Equal => rast1.coords.0.cmp(&rast2.coords.1),
            eq@_                      => eq,
        }).unwrap().coords.0;
        let xr = rasters.iter().max_by(|rast1, rast2| match rast1.coords.0.cmp(&rast2.coords.0) {
            std::cmp::Ordering::Equal => rast1.coords.0.cmp(&rast2.coords.1),
            eq@_                      => eq,
        }).unwrap().coords.0;
        let dx = (xl - xr).abs();
        unimplemented!()
    }

    pub fn rasterize_segment<'a>(
        &self,
        a: &'a Vertex,
        b: &'a Vertex) -> Vec<LineRaster<'a>> {
        let ar = utils::rasterize_dot_1(a.coords.xy(), self.size());
        let br = utils::rasterize_dot_1(b.coords.xy(), self.size());
        let coords = Self::line((ar.x, ar.y), (br.x, br.y));
        let mut result = Vec::with_capacity(coords.len());
        for (x, y) in coords {
            let da = (ar.x - x).abs();
            let db = (br.x - x).abs();
            let sum = (da + db) as f32;
            let ka = db as f32 / sum;
            let kb = da as f32 / sum;
            result.push(LineRaster {
                coords:         (x, y),
                baricentric:    [(ka, a), (kb, b)],
            });
        }
        result
    }

    pub fn render_segment(
        &mut self,
        a: Vertex,
        b: Vertex,
        ) {
        for LineRaster {
            coords: (x, y),
            baricentric: [(k1, v1), (k2, v2)],
        } in self.rasterize_segment(&a, &b) {
            let index = self.buff_offset(na::Vector2::new(x, y));
            let vertex = Self::merge_vertices_line(v1, v2, k1, k2);
            let depth = vertex.coords.z;
            let color = (vertex.color.x, vertex.color.y, vertex.color.z);
            self.color_buffer[index] = color;
        }
    }

    pub fn merge_vertices_line(
        a: &Vertex,
        b: &Vertex,
        ka: f32,
        kb: f32) -> Vertex {
        Vertex {
            coords  : ka * a.coords + kb * b.coords,
            color   : Self::merge_rgb(a.color, b.color, ka, kb),
        }
    }

    pub fn merge_rgb(
        a: na::Vector3<u8>,
        b: na::Vector3<u8>,
        ka: f32, 
        kb: f32) -> na::Vector3<u8> {
        let af: na::Vector3<f32> = na::convert(a);
        let bf: na::Vector3<f32> = na::convert(b);
        na::try_convert(ka * af + kb * bf).unwrap()
    }

    // Return type is vector of rasters and their baricentric coords.
    //
    // TODO: Wrap return type in a normal data-structure.
    pub fn line_baricentric(
        &self,
        a: na::Vector3<f32>,
        b: na::Vector3<f32>) -> Vec<Raster> {
        let ar = utils::rasterize_dot_1(a.xy(), self.size());
        let br = utils::rasterize_dot_1(b.xy(), self.size());
        let coords = Self::line((ar.x, ar.y), (br.x, br.y));
        let mut result = Vec::with_capacity(coords.len());
        for (x, y) in coords {
            let da = (ar.x - x).abs();
            let db = (br.x - x).abs();
            let sum = (da + db) as f32;
            let ka = db as f32 / sum;
            let kb = da as f32 / sum;
            result.push(Raster {
                coords:         (x, y),
                baricentric:    (ka, kb),
            });
        }
        result
    }

    pub fn provide_with_baricentric(
        a: (i32, i32),
        b: (i32, i32),
        l: Vec<(i32, i32)>) -> Vec<Raster> {
        let mut result = Vec::with_capacity(l.len());
        for (x, y) in l {
            let da = (a.0 - x).abs();
            let db = (b.0 - x).abs();
            let sum = (da + db) as f32;
            let ka = db as f32 / sum;
            let kb = da as f32 / sum;
            result.push(Raster {
                coords:         (x, y),
                baricentric:    (ka, kb),
            })
        }
        result
    }
    
    pub fn rasterize_line_color(
        &mut self,
        a: na::Vector3<f32>,
        b: na::Vector3<f32>,
        color: RGB) { 
        let ai = utils::rasterize_dot_1(a.xy(), self.size());
        let bi = utils::rasterize_dot_1(b.xy(), self.size());
        let coords = Self::line((ai.x, ai.y), (bi.x, bi.y));
        for (x, y) in coords {
            let index = self.buff_offset(na::Vector2::new(x, y));
            self.color_buffer[index] = color;
        }
    }

    fn line_hull_x(
        a: (i32, i32),
        b: (i32, i32)) -> (i32, Vec<(i32, i32)>) {
        let coords = Self::line(a, b);
        let xl = *coords.iter().map(|(x, _y)| x).min().unwrap();
        let xr = *coords.iter().map(|(x, _y)| x).max().unwrap();
        let dx = (xr - xl + 1) as usize;
        let mut result = vec![(std::i32::MAX, std::i32::MIN); dx];
        for (x, y) in coords {
            let index = (x - xl) as usize;
            result[index].0 = std::cmp::min(y, result[index].0);
            result[index].1 = std::cmp::max(y, result[index].1);
        } 
        (xl, result)
    }

    fn line(
        a: (i32, i32),
        b: (i32, i32)) -> Vec<(i32, i32)> { 
        let dx = b.0 - a.0;
        let dy = b.1 - a.1;
        if dx.abs() >= dy.abs() {
            if dx >= 0 {
                Self::line_unit_x(a, dx, dy)
            } else {
                Self::line_unit_x(b, -dx, -dy)
            }
        } else {
            if dy >= 0 {
                Self::line_unit_y(a, dx, dy)
            } else {
                Self::line_unit_y(b, -dx, -dy)
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

    pub fn get_color_buffer_ptr(&self) -> *const (u8, u8, u8) {
        self.color_buffer.as_ptr()
    }
}
