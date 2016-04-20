//TODO: also store matrix
pub struct Camera {
   //transform : Matrix
   pos : Point,
   rot : Coord,
   size : Point //% of parent width and height
}
impl Camera {
   pub fn new() -> Camera {
      Camera {
         pos: [0.0, 0.0], rot : 0.0, size : [0.0, 0.0],
         //transform: Camera::get_identity_m()
      }
   }
   pub fn set_position(&mut self, pos_ : &Point) {
      self.pos = [0.0, 0.0];
      self.transform = Camera::get_identity_m();
      self.move(pos_);
   }
   pub fn move_(&mut self, pos_ : Point) {
      self.pos[0] += pos_[0];
      self.pos[1] += pos_[1];
   }
   pub fn set_rotation(&mut self, rot_ : Coord) {
      self.rot = rot_;
   }
   pub fn rotate(&mut self, rot_ : Coord) {
      self.rot += rot_;
   }
   pub fn set_size(&mut self, size_ : Point) {
      self.size = size_;
   }
   pub fn scale(&mut self, size_ : Coord) {
      self.size[0] *= size_;
      self.size[1] *= size_;
   }
   pub fn get_v(&self) -> Vector {
      [pos[0], pos[1], 0 0]
   }
   pub fn get_identity_m() -> Matrix {
      [[1.0, 0.0, 0.0, 0.0],
       [0.0, 1.0, 0.0, 0.0],
       [0.0, 0.0, 1.0, 0.0],
       [0.0, 0.0, 0.0, 1.0]]
   }
   pub fn get_pos_m(&self, v : &Vector) -> Matrix {
      [[1.0, 0.0, 0.0, v[0]],
       [0.0, 1.0, 0.0, v[1]],
       [0.0, 0.0, 1.0, v[2]],
       [0.0, 0.0, 0.0, v[3]]]
      //mul_vector(self.get_v(), pos_m)
   }
   pub fn get_rot_m(&self, rot : &Coord) -> Matrix {
      [[rot.cos(), -rot.sin(), 0.0, 0.0],
       [rot.sin(), rot.cos(),  0.0, 0.0],
       [0.0,       0.0,        1.0, 0.0],
       [0.0,       0.0,        0.0, 1.0]]
   }
   pub fn get_size_m(&self, scale : &Coord) -> Matrix {}
   pub fn get_m(&self) -> Matrix {

   }
}

