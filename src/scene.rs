use types::*;
use model::*;
use camera::*;

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
      Scene {
         items : Vec::new()
      }
   }
   pub fn draw() {}
}



