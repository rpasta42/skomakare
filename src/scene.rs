
/*trait GameObject {
   fn set_position(x : Point);
   fn move(x : Point);
   fn set_rotation(x : Coord);
   fn rotate(degrees : Coord);
   fn set_size(scale : Coord);
   fn resize(scale_factor : Coord);
}*/

struct ObjectData {
   location : Point,
   rotation : Coord,
   size : Coord
}
enum ObjectType {
   Model(Model), Scene(Scene), Camera(Camera) //Physics(u32)
}
struct GameObject {
   object_type : ObjectType,
   object_data : ObjectData
}
impl GameObject {
   fn set_position(x : Point) {}
   fn move_(x : Point) {}
   fn set_rotation(x : Coord) {}
   fn rotate(degrees : Coord) {}
   fn set_size(scale : Coord) {}
   fn resize(scale_factor : Coord) {}
}
struct Scene {
   items : Vec<GameObject>
}

