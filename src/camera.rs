pub struct Camera {
   pos : Point,
   rot : Coord,
   size : Point //% of parent width and height
}
impl Camera {
   pub fn new() -> Camera {
      Camera {
         pos: [0.0, 0.0], rot : 0.0, size : [0.0, 0.0]
      }
   }
   pub fn set_position(&mut self, pos_ : Point) {
      self.pos = pos_;
   }
   pub fn move_(&mut self, pos_ : Point) {
      //self.pos[0] = self.pos + pos_[0]
   }
   pub fn set_rotation(&mut self, rot_ : Coord) {
      self.rot = rot_;
   }
   pub fn rotate(&mut self, rot_ : Coord) {
      self.rot = rot_;
   }
   pub fn set_size(&mut self, size_ : Point) {
      self.size = size_;
   }
   pub fn scale(&mut self, size_ : Point) {
      self.size = size_;
   }
}

