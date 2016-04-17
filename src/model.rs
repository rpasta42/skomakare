use types::*;

#[derive(Debug)]
pub enum BuiltInShape {
   Square, Triangle, Circle
}
#[derive(Clone)]
pub struct Shape {
   pub vertices : Vec<ColorVertex>
}
impl Shape {
   pub fn new() -> Shape { Shape { vertices : Vec::new() } }
   pub fn new_builtin(shape_type : BuiltInShape) -> Shape {
      let mut shape = Shape { vertices : Vec::new() }; //TODO
      match shape_type {
         BuiltInShape::Triangle => {
            shape.add_coords(-0.5, -0.5, 0.0, 0.0);
            shape.add_coords(0.0, 0.5, 0.0, 1.0);
            shape.add_coords(0.5, -0.25, 1.0, 0.0);
         }
         _ => { panic!("not implemented {:?}", shape_type); }
      }
      shape
   }
   pub fn new_vertices(vertices_ : Vec<ColorVertex>) -> Shape {
      Shape { vertices : vertices_ }
   }
   pub fn add(&mut self, v : ColorVertex) {
      self.vertices.push(v);
   }
   pub fn add_coords(&mut self, x1 : Coord, y1 : Coord, x2 : Coord, y2 : Coord) {
      self.vertices.push(ColorVertex::new(x1, y1, x2, y2));
   }
   pub fn print(&self) {
      for vert in &*self.vertices { vert.print(); }
   }
}

#[derive(Clone)]
pub struct Model {
   pub shape : Option<Shape>,
   position: Option<Point>,
   rotation : Option<Coord>,
   color : Option<Color>,
   img_path : Option<&'static str>,
   texture : Option<u8>,
   size : Option<Point> //% of screen width and height
}

impl Model {
   pub fn new() -> Model {
      Model {
         shape : None, position : None, rotation : None,
         color : None, img_path : None, texture : None, size : None
      }
   }
   pub fn shape(&mut self, shape_ : Shape) -> &mut Model { self.shape = Some(shape_); self }
   pub fn position(&mut self, position_ : Point) -> &mut Model { self.position = Some(position_); self }
   pub fn rotation(&mut self, rotation_ : Coord) -> &mut Model { self.rotation = Some(rotation_); self }
   pub fn color(&mut self, color_ : Color) -> &mut Model { self.color = Some(color_); self }
   pub fn img_path(&mut self, img_path_ : &'static str) -> &mut Model { self.img_path = Some(img_path_); self }
   pub fn texture(&mut self, texture_ : u8) -> &mut Model { self.texture = Some(texture_); self }
   pub fn size(&mut self, size_ : Point) -> &mut Model { self.size = Some(size_); self }
   pub fn finalize(&self) -> Model {
      //calculate missing stuff and make sure we didn't get too many arguments
      Model {
         shape: self.shape.clone(), position: self.position, rotation: self.rotation,
         color: self.color, img_path: self.img_path, texture: self.texture, size: self.size
      }
   }
   //fn draw() {}
}


