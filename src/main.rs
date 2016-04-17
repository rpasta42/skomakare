#![feature(type_ascription)]
#[macro_use]
extern crate glium;

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

fn draw(m : &Model) {
   implement_vertex!(Vertex, pos, tex);
   use glium::{DisplayBuild, Surface};
   let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

   let vertex_buffer = glium::VertexBuffer::new(&display, &m.vertices).unwrap();

   let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

   let vertex_shader_src = r#"
      #version 140
      in vec2 pos;
      uniform mat4 matrix;
      void main() { gl_Position = matrix * vec4(pos, 0.0, 1.0); }
   "#;

   let fragment_shader_src = r#"
      #version 140
      out vec4 color;
      void main() { color = vec4(1.0, 0.0, 0.0, 1.0); }
   "#;

   let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

   let mut i = 0.0f32;

   loop {
      i += 2.0*/*f32::consts::PI*/3.1415/1000.0;

      let mut target = display.draw();
      target.clear_color(0.0, 0.0, 1.0, 1.0);

      let matrix = [
         [i.cos(), 0.0,  i.sin(),  0.0],
         [0.0,  1.0, 0.0,  0.0],
         [-i.sin(),  0.0,  i.cos(), 0.0],
         [0.0,  0.0,  0.0,  1.0f32]
      ];

      //target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
      target.draw(&vertex_buffer, &indices, &program, &uniform! { matrix: matrix }, &Default::default()).unwrap();

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
   m.add_coords(0.0, 0.5, 0.0, 0.0);
   m.add_coords(0.5, -0.25, 0.0, 0.0);
   draw(&m);
}

