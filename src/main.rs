#![feature(type_ascription)]
#[macro_use]
extern crate glium;
extern crate image;

use std::io::Cursor;

type Coord = f32;
type Point = [Coord; 2];
type Color = (Coord, Coord, Coord, Coord);
/*#[derive(Copy, Clone)]
struct Point {
   x : Coord, y : Coord
}*/

#[derive(Copy, Clone)]
struct ColorVertex {
   pos : Point, tex_pos : Point
}
impl ColorVertex {
   fn new_coord(x1 : Coord, y1 : Coord, x2 : Coord, y2: Coord) -> ColorVertex {
      ColorVertex {
         pos      : [x1, y1],
         tex_pos  : [x2,y2]
      }
   }

   fn new(x1 : Coord, y1 : Coord, x2 : Coord, y2: Coord) -> ColorVertex {
      ColorVertex::new_coord(x1, y1, x2, y2)
   }

   fn new_point(pos_ : Point, tex_pos_ : Point) -> ColorVertex {
      ColorVertex {
         pos : pos_, tex_pos : tex_pos_
      }
   }
   fn print(&self) {
      println!("(x:{}, y:{}, tex_pos x: {}, tex_pos y: {})", self.pos[0], self.pos[1], self.tex_pos[0], self.tex_pos[1]);
   }
}

implement_vertex!(ColorVertex, pos, tex_pos);

#[derive(Debug)]
enum BuiltInShape {
   Square, Triangle, Circle
}
#[derive(Clone)]
struct Shape {
   vertices : Vec<ColorVertex>
}
impl Shape {
   fn new() -> Shape { Shape { vertices : Vec::new() } }
   fn new_builtin(shape_type : BuiltInShape) -> Shape {

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
   fn new_vertices(vertices_ : Vec<ColorVertex>) -> Shape {
      Shape { vertices : vertices_ }
   }
   fn add(&mut self, v : ColorVertex) {
      self.vertices.push(v);
   }
   fn add_coords(&mut self, x1 : Coord, y1 : Coord, x2 : Coord, y2 : Coord) {
      self.vertices.push(ColorVertex::new(x1, y1, x2, y2));
   }
   fn print(&self) {
      for vert in &*self.vertices { vert.print(); }
   }
}

#[derive(Clone)]
struct Model {
   shape : Option<Shape>,
   position: Option<Point>,
   rotation : Option<Coord>,
   color : Option<Color>,
   img_path : Option<&'static str>,
   texture : Option<u8>,
   size : Option<Point> //% of screen width and height
}
impl Model {
   fn new() -> Model {
      Model {
         shape : None, position : None, rotation : None,
         color : None, img_path : None, texture : None, size : None
      }
   }
   fn shape(&mut self, shape_ : Shape) -> &mut Model { self.shape = Some(shape_); self }
   fn position(&mut self, position_ : Point) -> &mut Model { self.position = Some(position_); self }
   fn rotation(&mut self, rotation_ : Coord) -> &mut Model { self.rotation = Some(rotation_); self }
   fn color(&mut self, color_ : Color) -> &mut Model { self.color = Some(color_); self }
   fn img_path(&mut self, img_path_ : &'static str) -> &mut Model { self.img_path = Some(img_path_); self }
   fn texture(&mut self, texture_ : u8) -> &mut Model { self.texture = Some(texture_); self }
   fn size(&mut self, size_ : Point) -> &mut Model { self.size = Some(size_); self }
   fn finalize(&self) -> Model {
      //calculate missing stuff and make sure we didn't get too many arguments
      Model {
         shape: self.shape.clone(), position: self.position, rotation: self.rotation,
         color: self.color, img_path: self.img_path, texture: self.texture, size: self.size
      }
   }

   fn draw() {}
}

struct Camera {}
impl Camera {
}

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

const vertex_shader_src : &'static str = r#"
   #version 140
   in vec2 pos;
   in vec2 tex_pos;
   out vec2 v_tex_coords;

   uniform mat4 matrix;

   void main() {
      v_tex_coords = tex_pos; 
      gl_Position = matrix * vec4(pos, 0.0, 1.0); }
"#;

const fragment_shader_src : &'static str = r#"
   #version 140
   in vec2 v_tex_coords;
   out vec4 color;

   uniform sampler2D tex;

   void main() {
      color = texture(tex, v_tex_coords);
   }
"#;

fn draw(m : &Shape, img_path : &str) {
   use glium::{DisplayBuild, Surface};
   let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

   let image = image::load(Cursor::new(&include_bytes!("../data/opengl.png")[..]), image::PNG)
               .unwrap().to_rgba();

   let image_dimensions = image.dimensions();
   let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
   use glium::Texture2d;
   let texture = glium::texture::Texture2d::new(&display, image).unwrap(); //kk

   let vertex_buffer = glium::VertexBuffer::new(&display, &m.vertices).unwrap();
   let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


   let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

   let mut i = 0.0f32;
   let mut t = -0.5;

   loop {
      i += 2.0*/*f32::consts::PI*/3.1415/1000.0;
      t += 0.0002;
      if t > 0.5 { t = -0.5; }

      let mut target = display.draw();
      target.clear_color(0.0, 0.0, 1.0, 1.0);

      let uniforms = uniform! {
         matrix: [
            [i.cos(), 0.0,  i.sin(),  0.0],
            [0.0,  1.0, 0.0,  0.0],
            [-i.sin(),  0.0,  i.cos(), 0.0],
            [0.0,  0.0,  0.0,  1.0f32]
            /*[1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ t , 0.0, 0.0, 1.0f32],*/
         ],
         tex: &texture
      };

      //target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
      //target.draw(&vertex_buffer, &indices, &program, &uniform! { matrix: matrix }, &Default::default()).unwrap();
      target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();

      target.finish().unwrap();

      for ev in display.poll_events() {
         match ev {
            glium::glutin::Event::Closed => return,
            _ => ()
         }
      }
   }
}

fn main_old() {
   use glium::{DisplayBuild, Surface};
   let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

   let mut m = Shape::new();
   m.add_coords(-0.5, -0.5, 0.0, 0.0);
   m.add_coords(0.0, 0.5, 0.0, 1.0);
   m.add_coords(0.5, -0.25, 1.0, 0.0);
   draw(&m, "data/opengl.png");
}

fn main() {
   use glium::{DisplayBuild, Surface};
   let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

   let m = Model::new().shape(Shape::new_builtin(BuiltInShape::Triangle)).finalize();
}

