use types::*;
use shaders::*;
use utils::img_path_to_texture;
use glium::texture::Texture2d;
use glium::backend::glutin_backend::GlutinFacade;
use std::cell::RefCell;
use std::rc::Rc;

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
   pub fn add_coords(&mut self, x1 : Coord, y1 : Coord,
                     x2 : Coord, y2 : Coord)
   {
      self.vertices.push(ColorVertex::new(x1, y1, x2, y2));
   }
   pub fn print(&self) {
      for vert in &*self.vertices { vert.print(); }
   }
}

#[derive(PartialEq)]
pub enum TextureType {
   None, Image, Color
}

//#[derive(Clone)]
pub struct Model {
   pub shape : Option<Shape>,
   pub texture_type : TextureType,
   pub color : Option<Color>,
   pub img_path : Option<String>,
   pub texture : Option<Texture2d>,
   pub shader_name : Option<String>
}

impl Model {
   pub fn new() -> Model {
      Model {
         shape : None, color : None, img_path : None,
         texture : None, shader_name : None,
         texture_type : TextureType::None
      }
   }
   pub fn shape(&mut self, shape_ : Shape) -> &mut Model {
      self.shape = Some(shape_); self
   }
   pub fn color(&mut self, color_ : Color) -> &mut Model {
      self.color = Some(color_); self
   }
   pub fn img_path(&mut self, img_path_ : String) -> &mut Model {
      self.img_path = Some(img_path_); self
   }
   //pub fn shader_name(&mut self, shader_name_ : String) -> &mut Model { self.shader_name = Some(shader_name_); self }

   pub fn get_texture(&self) -> &Option<Texture2d> {
      &self.texture
   }
   pub fn finalize(&mut self, sm : &mut ShaderManager, display : &GlutinFacade)
      -> Model
   {
      //calculate missing stuff and make sure we didn't get too many arguments
      let mut texture_type = TextureType::None;

      if let Some(c) = self.color {
         texture_type = TextureType::Color;
         let name = format!("color-{}{}{}{}", c.0, c.1, c.2, c.3);
         self.shader_name = Some(name.to_string());

         if let Some(_) = sm.shaders.get(&*name) {}
         else {
            let frag_sh_src = frag_shader_color(c);
            sm.add_shader(display, name, VERT_SH_COLOR, &frag_sh_src);
         }
      }
      let texture = if let Some(img_path) = self.img_path.clone() {
         if texture_type == TextureType::Color {
            panic!("Cannot have texture image and color");
         }
         texture_type = TextureType::Image;
         self.shader_name = Some("texture".to_string());
         Some(img_path_to_texture(img_path, display))
      } else { None };

      if texture_type == TextureType::None {
         panic!("Model needs to either have color or img_path");
      }

      Model {
         shape: self.shape.clone(), color: self.color,
         img_path: self.img_path.clone(), texture: texture,
         shader_name: self.shader_name.clone(),
         texture_type: texture_type
      }
   }
   //fn draw() {}
}


