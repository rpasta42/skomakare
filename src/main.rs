#![feature(type_ascription)]
#[macro_use]
extern crate glium;
extern crate image;

use std::io::Cursor;

type Coord = f32;

#[derive(Copy, Clone)]
struct Vertex {
   pos: [Coord; 2],
   tex: [Coord; 2]
}

impl Vertex {
   fn new(x1 : Coord, y1 : Coord, x2 : Coord, y2: Coord) -> Vertex {
      Vertex {
         pos : [x1, y1],
         tex : [x2, y2]
      }
   }
   fn print(&self) {
      println!("(x:{}, y:{}, tex1: {}, tex2: {})", self.pos[0], self.pos[1], self.tex[0], self.tex[1]);
   }
}

implement_vertex!(Vertex, pos, tex);

struct Model {
   vertices : Vec<Vertex>
}
impl Model {
   fn new() -> Model {
      let mut m = Model { vertices : Vec::new() };
      m
   }
   fn add(&mut self, v : Vertex) {
      self.vertices.push(v);
   }
   fn add_coords(&mut self, x1 : Coord, y1 : Coord, x2 : Coord, y2 : Coord) {
      self.vertices.push(Vertex::new(x1, y1, x2, y2));
   }
   fn print(&self) {
      for vert in &self.vertices { vert.print(); }
   }
}

fn draw(m : &Model, img_path : &str) {
   use glium::{DisplayBuild, Surface};
   let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

   let image = image::load(Cursor::new(&include_bytes!("../data/opengl.png")[..]), image::PNG)
               .unwrap().to_rgba();

   let image_dimensions = image.dimensions();
   let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
   let texture = glium::texture::Texture2d::new(&display, image).unwrap();

   let vertex_buffer = glium::VertexBuffer::new(&display, &m.vertices).unwrap();
   let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

   let vertex_shader_src = r#"
      #version 140
      in vec2 pos;
      in vec2 tex;
      out vec2 v_tex_coords;

      uniform mat4 matrix;

      void main() {
         v_tex_coords = tex; 
         gl_Position = matrix * vec4(pos, 0.0, 1.0); }
   "#;

   let fragment_shader_src = r#"
      #version 140
      in vec2 v_tex_coords;
      out vec4 color;

      uniform sampler2D tex;

      void main() {
         color = texture(tex, v_tex_coords);
      }
   "#;

   let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

   //let mut i = 0.0f32;
   let mut t = -0.5;

   loop {
      //i += 2.0*/*f32::consts::PI*/3.1415/1000.0;
      t += 0.0002;
      if t > 0.5 { t = -0.5; }

      let mut target = display.draw();
      target.clear_color(0.0, 0.0, 1.0, 1.0);

      let uniforms = uniform! {
         matrix: [
            /*[i.cos(), 0.0,  i.sin(),  0.0],
            [0.0,  1.0, 0.0,  0.0],
            [-i.sin(),  0.0,  i.cos(), 0.0],
            [0.0,  0.0,  0.0,  1.0f32]*/
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ t , 0.0, 0.0, 1.0f32],
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
   let mut m = Model::new();
   m.add_coords(-0.5, -0.5, 0.0, 0.0);
   m.add_coords(0.0, 0.5, 0.0, 1.0);
   m.add_coords(0.5, -0.25, 1.0, 0.0);
   draw(&m, "data/opengl.png");
}

