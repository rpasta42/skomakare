
pub type Coord = f32;
pub type Point = [Coord; 2];
pub type Color = (Coord, Coord, Coord, Coord);

#[derive(Copy, Clone)]
pub struct ColorVertex {
   pub pos : Point, pub tex_pos : Point
}
impl ColorVertex {
   pub fn new_coord(x1 : Coord, y1 : Coord, x2 : Coord, y2: Coord) -> ColorVertex {
      ColorVertex {
         pos      : [x1, y1],
         tex_pos  : [x2,y2]
      }
   }
   pub fn new(x1 : Coord, y1 : Coord, x2 : Coord, y2: Coord) -> ColorVertex {
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


