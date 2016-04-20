use types::*;
use shaders::*;
use glium::texture::Texture2d;
use glium::backend::glutin_backend::GlutinFacade;
use std::cell::RefCell;

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
      let mut shape = Shape::new(); //TODO
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

//#[derive(Clone)]
pub struct Model {
   pub shape : Option<Shape>,
   color : Option<Color>,
   img_path : Option<&'static str>,
   texture : Option<Texture2d>,
   shader_name : Option<String>
}
impl Model {
   pub fn new() -> Model {
      Model {
         shape : None, color : None, img_path : None,
         texture : None, shader_name : None
      }
   }
   pub fn shape(&mut self, shape_ : Shape) -> &mut Model {
      self.shape = Some(shape_); self
   }
   pub fn color(&mut self, color_ : Color) -> &mut Model {
      self.color = Some(color_); self
   }
   pub fn img_path(&mut self, img_path_ : &'static str) -> &mut Model {
      self.img_path = Some(img_path_); self
   }
   //pub fn shader_name(&mut self, shader_name_ : String) -> &mut Model { self.shader_name = Some(shader_name_); self }

   pub fn finalize(&mut self, shader_manager : &mut ShaderManager, display : &GlutinFacade)
      -> Model
   {
      //calculate missing stuff and make sure we didn't get too many arguments
      let mut has_color = false;
      let mut has_img = false;

      if let Some(color) = self.color {
         has_color = true;

         let name = format!("color-{}{}{}{}", color.0, color.1, color.2, color.3);
         self.shader_name = Some(name.to_string());

         if let Some(_) = shader_manager.shaders.get(&*name) {}
         else {
            let frag_shader_src = frag_shader_color(color);
            shader_manager.add_shader(display, name, VERT_SH_COLOR, &frag_shader_src);
         }
      }
      if let Some(img_path) = self.img_path {
         has_img = true;
         self.shader_name = Some("texture".to_string());
      }
      if (has_color && has_img) || (!has_color && !has_img) {
         panic!("Model needs to either have color or img_path but not both");
      }

      Model {
         shape: self.shape.clone(), color: self.color,
         img_path: self.img_path, texture: None, shader_name: self.shader_name.clone()
      }
   }
   //fn draw() {}
}


