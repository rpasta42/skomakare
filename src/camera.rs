use types::{Point, Coord, Vector, Matrix, mul_matrices};

//fn null_pt() -> Point { [0.0, 0.0] }

//TODO: also store matrix
pub struct Camera {
   //transform : Matrix
   pos : Point,
   rot : Coord,
   size : Point //% of parent width and height
}
impl Camera {
   pub fn new() -> Camera {
      Camera { pos: [0.0, 0.0], rot : 0.0, size : [0.0, 0.0] }
   }
   pub fn set_position(&mut self, pos_ : &Point) {
      self.pos = [0.0, 0.0];
      //self.transform = Camera::get_identity_m();
      self.move_(pos_);
   }
   pub fn move_(&mut self, pos_ : &Point) {
      self.pos[0] += pos_[0];
      self.pos[1] += pos_[1];
   }
   pub fn set_rotation(&mut self, rot_ : &Coord) {
      self.rot = *rot_;
   }
   pub fn rotate(&mut self, rot_ : &Coord) {
      self.rot += *rot_;
   }
   pub fn set_size(&mut self, size_ : &Point) {
      self.size = *size_;
   }
   pub fn scale(&mut self, size_ : &Coord) {
      self.size[0] *= *size_;
      self.size[1] *= *size_;
   }
   pub fn get_identity_m() -> Matrix {
      [[1.0, 0.0, 0.0, 0.0],
       [0.0, 1.0, 0.0, 0.0],
       [0.0, 0.0, 1.0, 0.0],
       [0.0, 0.0, 0.0, 1.0]]
   }
   pub fn get_pos_m(&self) -> Matrix {
      let v = self.pos;
      [[1.0, 0.0, 0.0, v[0]],
       [0.0, 1.0, 0.0, v[1]],
       [0.0, 0.0, 1.0, 1.0 ],
       [0.0, 0.0, 0.0, 1.0 ]]
      //mul_vector(self.get_v(), pos_m)
   }
   pub fn get_rot_m(&self) -> Matrix {
      let t = self.rot;
      [[t.cos(), -t.sin(), 0.0, 0.0],
       [t.sin(), t.cos(),  0.0, 0.0],
       [0.0,     0.0,      1.0, 0.0],
       [0.0,     0.0,      0.0, 1.0]]
   }
   pub fn get_scale_m(&self) -> Matrix {
      let x = self.size[0];
      let y = self.size[1];
      [[x,   0.0, 0.0, 0.0],
       [0.0, y,   0.0, 0.0],
       [0.0, 0.0, 1.0, 0.0],
       [0.0, 0.0, 0.0, 1.0]]
   }
   //pub fn get_size_m(&self, scale : &Coord) -> Matrix {}
   //pub fn get_v(&self) -> Vector {[self.pos[0], self.pos[1], 0.0, 0.0]}
   pub fn get_m(&self) -> Matrix {
      let m = Camera::get_identity_m();
      let scale_m = mul_matrices(&m, &self.get_scale_m());
      let rot_m = mul_matrices(&scale_m, &self.get_rot_m());
      let translate_m = mul_matrices(&rot_m, &self.get_pos_m());
      translate_m
   }
}

