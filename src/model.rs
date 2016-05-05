use types::*;
use shaders::*;
use utils::img_path_to_texture;
use glium::texture::Texture2d;
use glium::backend::glutin_backend::GlutinFacade;
use std::cell::RefCell;
use std::rc::Rc;

use glium::index::PrimitiveType;


#[derive(Debug)]
pub enum BuiltInShape {
   Rectangle, Triangle, Circle
}
#[derive(Clone)]
pub struct Shape {
   pub vertices : Vec<ColorVertex>,
   pub primitive_type : Option<PrimitiveType>

}
impl Shape {
   pub fn new() -> Shape {
      Shape {
         vertices : Vec::new(),
         primitive_type : None
      }
   }
   pub fn new_builtin(shape_type : BuiltInShape) -> Shape {
      let mut shape = Shape::new(); //TODO
      match shape_type {
         BuiltInShape::Triangle => {
            /*shape.add_coords(-0.5, -0.5, 0.0, 0.0);
            shape.add_coords(0.0, 0.5, 0.0, 1.0);
            shape.add_coords(0.5, -0.5, 1.0, 0.0);*/
            shape.add_coords(-0.5, -0.4330127, 0.0, 0.0);
            shape.add_coords(0.0, 0.4330127, 0.0, 1.0);
            shape.add_coords(0.5, -0.4330127, 1.0, 0.0);
         },
         BuiltInShape::Rectangle => {
            shape.add_coords(0.5, -0.5, 1.0, 0.0);
            shape.add_coords(-0.5, -0.5, 0.0, 0.0);
            shape.add_coords(-0.5, 0.5, 0.0, 1.0);
            shape.add_coords(0.5, -0.5, 1.0, 0.0);
            shape.add_coords(0.5, 0.5, 1.0, 1.0);
            shape.add_coords(-0.5, 0.5, 0.0, 1.0);
         },
         BuiltInShape::Circle => {
            //so far, only have triangle strip, in lisp actually draw circle
            /* for rot = pi/4, n = 8
            let x = 0.35355339;
            shape.add_coords(0.0, 0.0, 0.0, 0.0);
            shape.add_coords(-x, -x, 0.0, 0.0);
            shape.add_coords(0.0, -0.5, 0.0, 0.0);*/
            let x = 0.121267813;
            let y = 0.515388203;
            let z = 0.48507;
            //rot = pi/4/2/2, n = 8*2*2
            shape.add_coords(0.0, 0.0, 0.0, 0.0);
            shape.add_coords(-x, -z, 0.0, 0.0);
            shape.add_coords(0.0, -0.5, 0.0, 0.0);

            /*shape.add_coords(0.0, 0.0, 0.0, 0.0);
            shape.add_coords(-0.5, -0.5, 0.0, 0.0);
            shape.add_coords(0.0, -0.5, 0.0, 0.0);*/
         },
         //_ => { panic!("not implemented {:?}", shape_type); }
      }
      shape.primitive_type = Some(PrimitiveType::TrianglesList);
      shape
   }
   pub fn new_vertices(vertices_ : Vec<ColorVertex>,
                       primitive_type : PrimitiveType)
   -> Shape
   {
      Shape { vertices : vertices_, primitive_type : Some(primitive_type) }
   }
   //primitivetype::Points;
   pub fn from_obj_file(&mut self, path : &str) {
      use utils::{read_file, s_to_f};

      let data = read_file(path).unwrap();

      let mut rect_verts = Vec::new();

      let lines = data.split("\n").collect::<Vec<&str>>();
      for line in lines.iter() {
         let words = line.split(" ").collect::<Vec<&str>>();
         //TODO: check words >= 1
         if words[0] == "#" { continue; }
         else if words[0] == "v" {
            if words.len() != 4 {
               println!("bad line: {}", line);
            } else {
               let v = ColorVertex {
                  pos: [s_to_f(words[1]), s_to_f(words[2])],  //s_to_f(words[3]));
                  tex_pos : [0.0, 0.0]
               };
               rect_verts.push(v);
            }
         }
         else { /*println!("known line type {}", words[0]);*/ }
      }

      let mut rect_ctr = 0;
      let mut rect = Vec::new();
      for vert in rect_verts {
         //self.add(rectangle);

         rect.push(vert);
         rect_ctr += 1;
         if rect_ctr == 4 {
            self.add(rect[0]);
            self.add(rect[1]);
            self.add(rect[2]);
            self.add(rect[0]);
            self.add(rect[1]);
            self.add(rect[3]);
            rect = Vec::new();
            rect_ctr = 0;
         }
      }
      //self.primitive_type = Some(PrimitiveType::Points);
      //self.primitive_type = Some(PrimitiveType::TriangleFan);
      self.primitive_type = Some(PrimitiveType::TrianglesList);


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


