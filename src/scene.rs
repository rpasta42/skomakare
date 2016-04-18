use types::*;
use model::*;
use camera::*;

/*trait GameObject {
   fn set_position(x : Point);
   fn move(x : Point);
   fn set_rotation(x : Coord);
   fn rotate(degrees : Coord);
   fn set_size(scale : Coord);
   fn resize(scale_factor : Coord);
}*/

pub struct ObjectData {
   position : Option<Point>,
   rotation : Option<Coord>,
   size : Option<Point> //% of parent width and height
}
impl ObjectData {
   fn new() -> ObjectData {
      ObjectData {
         position: None, rotation : None, size : None
      }
   }
   pub fn position(&mut self, position_ : Point) -> &mut ObjectData {
      self.position = Some(position_); self
   }
   pub fn rotation(&mut self, rotation_ : Coord) -> &mut ObjectData {
      self.rotation = Some(rotation_); self
   }
   pub fn size(&mut self, size_ : Point) -> &mut ObjectData {
      self.size = Some(size_); self
   }
   pub fn finalize(&self) -> ObjectData {
      //calculate missing stuff and make sure we didn't get too many arguments
      ObjectData {
         position: self.position, rotation: self.rotation, size : self.size
      }
   }
}

pub enum ObjectType {
   Model(Model), Scene(Scene), Camera(Camera) //Physics(u32), Undecided
}
pub struct GameObject {
   pub object_type : ObjectType,
   pub object_data : ObjectData
}
impl GameObject {
   pub fn new(object_type_ : ObjectType) -> GameObject {
      GameObject {
         object_type : object_type_, //ObjectType::Undecided,
         object_data : ObjectData::new()
      }
   }
   pub fn set_position(x : Point) {}
   pub fn move_(x : Point) {}
   pub fn set_rotation(x : Coord) {}
   pub fn rotate(degrees : Coord) {}
   pub fn set_size(scale : Coord) {}
   pub fn resize(scale_factor : Coord) {}
}

pub struct Scene {
   pub items : Vec<GameObject>
}

