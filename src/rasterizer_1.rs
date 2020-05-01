use nalgebra as na;
use super::math_utils as utils;

type RGB = (u8, u8, u8);

pub struct Rasterizer_ {
    color_buffer: Vec<RGB>,
    depth_buffer: Vec<f32>,
    width       : u32,
    height      : u32,
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub coords  : na::Vector3<f32>,
    pub color   : na::Vector3<u8>,
}

pub struct VertexBasis2 {
    x: Vertex,
    y: Vertex,
}

pub struct LineRaster<'a> {
    pub coords      : (i32, i32),
    pub baricentric : (f32, f32),
    pub basis       : &'a VertexBasis2,
}

pub struct VertexBasis3 {
    pub x: Vertex,
    pub y: Vertex,
    pub z: Vertex,
}

#[derive(Clone, Copy)]
pub struct PolygonRaster<'a> {
    pub coords      : (i32, i32),
    pub baricentric : (f32, f32, f32),
    pub basis       : &'a VertexBasis3,
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

    /*
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
    */

    pub fn rasterize_polygon<'a>(
        &mut self,
        basis: &'a VertexBasis3) /* -> Vec<PolygonRaster<'a>> */ {
        let (a, b, c) = (&basis.x, &basis.y, &basis.z);
        let ab_basis = VertexBasis2{ x: *a, y: *b };
        let bc_basis = VertexBasis2{ x: *b, y: *c };
        let ac_basis = VertexBasis2{ x: *a, y: *c };
        let ab = Self::advance_basis_many(self.rasterize_segment_(&ab_basis), basis, (0, 1));
        let bc = Self::advance_basis_many(self.rasterize_segment_(&bc_basis), basis, (1, 2));
        let ac = Self::advance_basis_many(self.rasterize_segment_(&ac_basis), basis, (0, 2));
        let width = self.size().y as i32;
        let width_2 = width / 2;
        let mut hull: Vec<Option<(PolygonRaster, PolygonRaster)>> = vec![None; width as usize];
        for raster in ab {
            let cur_y = raster.coords.1;
            let index = (raster.coords.0 + width_2) as usize;
            hull[index] = match hull[index] {
                None => Some((raster, raster)),
                Some((rast_min, rast_max)) => Some((
                    if rast_min.coords.1 > cur_y { raster } else { rast_min }, 
                    if rast_max.coords.1 < cur_y { raster } else { rast_max })),
            }
        }
        for raster in bc {
            let cur_y = raster.coords.1;
            let index = (raster.coords.0 + width_2) as usize;
            hull[index] = match hull[index] {
                None => Some((raster, raster)),
                Some((rast_min, rast_max)) => Some((
                    if rast_min.coords.1 > cur_y { raster } else { rast_min }, 
                    if rast_max.coords.1 < cur_y { raster } else { rast_max })),
            }
        }
        for raster in ac {
            let cur_y = raster.coords.1;
            let index = (raster.coords.0 + width_2) as usize;
            hull[index] = match hull[index] {
                None => Some((raster, raster)),
                Some((rast_min, rast_max)) => Some((
                    if rast_min.coords.1 > cur_y { raster } else { rast_min }, 
                    if rast_max.coords.1 < cur_y { raster } else { rast_max })),
            }
        }

        for maybe_raster in hull {
            match maybe_raster {
                None => { 
                    // no-op
                },
                Some((rast_min, rast_max)) => {
                    let x = rast_min.coords.0;
                    let y0 = rast_min.coords.1;
                    let y1 = rast_max.coords.1;
                    for y in  y0..=y1 {
                        let index = self.buff_offset(na::Vector2::new(x, y));
                        let d0 = y - y0;
                        let d1 = y1 - y;
                        let sum = (d0 + d1) as f32;
                        let k0 = d1 as f32 / sum;
                        let k1 = d0 as f32 / sum;
                        let vertex = PolygonRaster {
                            coords      : (x, y),
                            basis       : basis,
                            baricentric : (k0 * rast_min.baricentric.0 + k1 * rast_max.baricentric.0,
                                            k0 * rast_min.baricentric.1 + k1 * rast_max.baricentric.1,
                                            k0 * rast_min.baricentric.2 + k1 * rast_max.baricentric.2),
                        };

                        let cx = basis.x.color;
                        let cy = basis.y.color;
                        let cz = basis.z.color;
                        let (kx, ky, kz) = vertex.baricentric;
                        let color = (
                            (cx.x as f32 * kx + cy.x as f32 * ky + cz.x as f32 * kz).round() as u8,
                            (cx.y as f32 * kx + cy.y as f32 * ky + cz.y as f32 * kz).round() as u8,
                            (cx.z as f32 * kx + cy.z as f32 * ky + cz.z as f32 * kz).round() as u8,
                        );
                        self.color_buffer[index] = color;
                    }
                }
            }
        }
    }

    // TODO #1: I couldn't write types for comparator so made it ugly and 
    // let compiler to do it for me. When you'll better in rust, come and fix it. 
    //
    // TODO #2: Wrap return type into struct and doc it because it might be confusing 
    // 4 u in the future.
    pub fn build_segment_hull_x<'a>(
        rasters : &'a Vec<LineRaster<'a>>,
        basis   : &'a VertexBasis3,
        mapping : (usize, usize)) -> (i32, Vec<(PolygonRaster<'a>, PolygonRaster<'a>)>) {
        let xl = rasters.iter().min_by(|rast1, rast2| match rast1.coords.0.cmp(&rast2.coords.0) {
            std::cmp::Ordering::Equal => rast1.coords.0.cmp(&rast2.coords.1),
            ord@_                      => ord,
        }).unwrap().coords.0;
        let xr = rasters.iter().max_by(|rast1, rast2| match rast1.coords.0.cmp(&rast2.coords.0) {
            std::cmp::Ordering::Equal => rast1.coords.0.cmp(&rast2.coords.1),
            ord@_                      => ord,
        }).unwrap().coords.0;
        let dx = (xl - xr).abs();
        let mut hull = Vec::with_capacity((dx + 1) as usize);
        let mut ptr = 0;
        while ptr < rasters.len() {
            let cur_x = rasters[ptr].coords.0;
            let mut right = ptr;
            while right < rasters.len() + 1 && rasters[right + 1].coords.0 == cur_x {
                right += 1;
            }
            let offset = (cur_x - xl) as usize;
            /*
            hull[offset] = (Self::advance_basis(&rasters[ptr], basis, mapping),
                            Self::advance_basis(&rasters[ptr], basis, mapping));
            */
            ptr = right + 1;
        }
        (xl, hull)
    }

    pub fn advance_basis<'a>(
        rast        : LineRaster<'a>,
        basis       : &'a VertexBasis3, 
        mapping     : (usize, usize)) -> PolygonRaster<'a> {
        println!("Mapping: {:?}", mapping);
        assert!(mapping.0 <= 2 && mapping.1 <= 2 && mapping.0 != mapping.1, 
                "Wrong mapping, any element must be unqiue and in [0, 2]");
        let mut map = [0.; 3];
        map[mapping.0] = rast.baricentric.0;
        map[mapping.1] = rast.baricentric.1;
        PolygonRaster {
            coords      : rast.coords,
            baricentric : (map[0], map[1], map[2]),
            basis       : basis,
        }
    }

    pub fn advance_basis_many<'a>(
        rasters     : Vec<LineRaster<'a>>,
        basis       : &'a VertexBasis3,
        mapping     : (usize, usize)) -> Vec<PolygonRaster<'a>> {
        rasters
            .into_iter()
            .map(|rast2| Self::advance_basis(rast2, basis, mapping))
            .collect()
    }

    pub fn rasterize_segment_<'a>(
        &self,
        basis: &'a VertexBasis2) -> Vec<LineRaster<'a>> {
        let ar = utils::rasterize_dot_1(basis.x.coords.xy(), self.size());
        let br = utils::rasterize_dot_1(basis.y.coords.xy(), self.size());
        let coords = Self::line((ar.x, ar.y), (br.x, br.y));
        let mut result = Vec::with_capacity(coords.len());
        for (x, y) in coords {
            let da = (ar.x - x).abs();
            let db = (br.x - x).abs();
            let sum = (da + db) as f32;
            let ka = db as f32 / sum;
            let kb = da as f32 / sum;
            result.push(LineRaster {
                coords          : (x, y),
                baricentric     : (ka, kb),
                basis           : basis,
            });
        }
        result
    }

    pub fn render_segment_(
        &mut self,
        a: Vertex,
        b: Vertex) {
        let basis = VertexBasis2 {
            x: a,
            y: b,
        };
        for LineRaster {
            coords: (x, y),
            baricentric: coeffs,
            basis: cur_basis
        } in self.rasterize_segment_(&basis) {
            let vertex = Self::produce2(&basis, coeffs);
            let index = self.buff_offset(na::Vector2::new(x, y));
            self.color_buffer[index] = (
                vertex.color.x,
                vertex.color.y, 
                vertex.color.z);
        }
    }

    // THINK #1: Maybe it is a good idea to require from vertex 
    // to have mult operator with f32 and sum operator with itself?
    pub fn produce2(
        basis   : &VertexBasis2,
        coeffs  : (f32, f32)) -> Vertex {
        Vertex {
            coords  : na::Vector3::new(1., 1., 1.),
            color   : Self::merge_rgb(basis.x.color, basis.y.color, coeffs.0, coeffs.1),
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
