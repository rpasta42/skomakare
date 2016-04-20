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

pub enum GameObjectType {
   Model(Model), Scene(Scene), //Camera(Camera) //Physics(u32), Undecided
}

pub struct GameObject {
   pub data : GameObjectType,
   pub cam  : Camera
}
impl GameObject {
   pub fn new(object_type : GameObjectType) -> GameObject {
      GameObject {
         data : object_type, //ObjectType::Undecided,
         cam  : Camera::new()
      }
   }
}

pub struct Scene {
   pub items : Vec<GameObject>
}
impl Scene {
   pub fn new() -> Scene {
      items : Vec::new()
   }
}
