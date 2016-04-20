use glium::backend::glutin_backend::GlutinFacade;

pub type Display = GlutinFacade;
pub type Coord = f32;
pub type Point = [Coord; 2];
pub type Vector = [Coord; 4];
pub type Matrix = [Vector; 4];
pub type Color = (Coord, Coord, Coord, Coord);

pub fn mul_vector(v : &Vector, m : &Matrix) -> Vector {
   let mut ret = Vec::new();
   for i in 0..3 {
      let mut sum = 0;
      for j in 0..3 {
         sum += v[j] * m[i][j];
      }
      ret.push(sum);
   }
   [ret[0], ret[1], ret[2], ret[3]]
}
pub fn mul_matrices(m1 : &Matrix, m2 : &Matrix) -> Matrix {
   let mut ret = Vec::new();
   for i in 0..3 {
     ret.push(mul_vector(m1, m2[i]));
   }
   [ret[0], ret[1], ret[2], ret[3]]
}

//fn add_vec(v1 : &Vector, v2 : &Vector) {}

#[derive(Copy, Clone)]
pub struct ColorVertex {
   pub pos : Point, pub tex_pos : Point
}
impl ColorVertex {
   pub fn new_coord(x1 : Coord, y1 : Coord,
                    x2 : Coord, y2 : Coord)
   -> ColorVertex
   {
      ColorVertex {
         pos      : [x1, y1],
         tex_pos  : [x2,y2]
      }
   }
   pub fn new(x1 : Coord, y1 : Coord,
              x2 : Coord, y2: Coord)
   -> ColorVertex
   {
      ColorVertex::new_coord(x1, y1, x2, y2)
   }
   pub fn new_point(pos_ : Point, tex_pos_ : Point) -> ColorVertex {
      ColorVertex {
         pos : pos_, tex_pos : tex_pos_
      }
   }
   pub fn print(&self) {
      println!("(x:{}, y:{}, tex_pos x: {}, tex_pos y: {})", self.pos[0], self.pos[1], self.tex_pos[0], self.tex_pos[1]);
   }
}


