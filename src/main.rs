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

enum BuiltInShape {
   Square, Triangle, Circle
}
enum Shape {
   Vertices(Vec<ColorVertex>), BuiltIn(BuiltInShape)
}

struct Shape {
   vertices : Option<Vec<ColorVertex>>,
}
impl Shape {
   fn new() -> Shape {
      let mut m = Shape { vertices : Vec::new() };
      m
   }
   fn add(&mut self, v : ColorVertex) {
      self.vertices.push(v);
   }
   fn add_coords(&mut self, x1 : Coord, y1 : Coord, x2 : Coord, y2 : Coord) {
      self.vertices.push(ColorVertex::new(x1, y1, x2, y2));
   }
   fn print(&self) {
      for vert in &self.vertices { vert.print(); }
   }
}


struct GameObject {
   shape : Option<Shape>,
   position: Option<Point>,
   rotation : Option<Coord>,
   color : Option<Color>,
   img_path : Option<String>,
   texture : u8,
}
impl GameObject {
   fn new() -> GameObject { shape : None, position : None, rotation : None, color : None, img_path : None, texture : None }

   fn shape(&mut self, s : Shape) -> &mut GameObject { 
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

fn main() {
   let mut m = Shape::new();
   m.add_coords(-0.5, -0.5, 0.0, 0.0);
   m.add_coords(0.0, 0.5, 0.0, 1.0);
   m.add_coords(0.5, -0.25, 1.0, 0.0);
   draw(&m, "data/opengl.png");
}

