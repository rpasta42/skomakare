#[macro_use]
extern crate glium;

/*
build camera with matrix

face = triangle
Open object file. Look for f (face)
vertex index/uv index/normal index *3
vn = vertex normal x y z
s = specular
vt = uv = texture coordinates. mapping
*/

fn read_file(path_str : &str) -> Option<String> {
   use std::io::prelude::*;
   use std::fs::File;
   use std::path::Path;
   use std::error::Error;

   println!("loading file {}", path_str);
   let path = Path::new(path_str);
   match File::open(&path) {
      Ok(mut file) => {
         let mut file_content = String::new();
         match file.read_to_string(&mut file_content) {
            Ok(_) => Some(file_content.to_string()),
            Err(why) => { panic!("{}", Error::description(&why)); None }
         }
      }
      Err(why) => { panic!("{}", Error::description(&why)); None }
   }
}

type Coord = f32;

#[derive(Copy, Clone)]
struct Vertex {
   //position: [Coord; 3]
   position: (f32, f32, f32)
}
impl Vertex {
   fn new(x : Coord, y : Coord, z : Coord) -> Vertex { Vertex { position : (x, y, z) } }
   fn print(&self) {
      println!("(x:{}, y:{}, z:{})", self.position.0, self.position.1, self.position.2);
   }
}

struct Model {
   vertices : Vec<Vertex>
}
impl Model {
   fn new() -> Model {
      let mut m = Model { vertices : Vec::new() };
      m.add_coords(0.0, 0.0, 0.0);
      m
   }
   fn add(&mut self, v : Vertex) {
      self.vertices.push(v);
   }
   fn add_coords(&mut self, x : Coord, y : Coord, z : Coord) {
      //self.vertices.push(Vertex::new(x * 5.0, y * 5.0, z * 5.0));
      self.vertices.push(Vertex::new(x, y , z ));

   }
   fn print(&self) {
      for vert in &self.vertices {
         vert.print();
      }
   }
}

fn s_to_f(s : &str) -> f32 {
   s.parse::<f32>().unwrap()
}
fn char_at(s : &str, n : usize) -> Option<char> {
   for (i, c) in s.chars().enumerate() {
      if i == n { return Some(c) }
   } return None
}

fn parse_obj(data : &str) -> Model {
   let mut m = Model::new();

   let lines = data.split("\n").collect::<Vec<&str>>();
   for line in lines.iter() {
      let words = line.split(" ").collect::<Vec<&str>>();
      //TODO: check words >= 1
      if words[0] == "#" { continue; }  //let first = char_at(line, 0).unwrap(); if (first == '#') continue;
      else if words[0] == "v" {
         if words.len() != 4 {
            println!("bad line: {}", line)
         } else {
            m.add_coords(s_to_f(words[1]), s_to_f(words[2]), s_to_f(words[3]));
         }
      }
      else { /*println!("known line type {}", words[0]);*/ }
   }
   m
}

fn display_file(path : &str) {
   let data = read_file(path).unwrap();
   let m = parse_obj(&data);
   m.print();
   draw(&m);
}

fn draw(m : &Model) {
   implement_vertex!(Vertex, position);

   use glium::{DisplayBuild, Surface};
   let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

   //let vertex_buffer = glium::VertexBuffer::new(&display, &m.vertices).unwrap();
   let positions = glium::VertexBuffer::new(&display, &m.vertices).unwrap();
   //let normals = None; //glium::n;

   let i_type = glium::index::PrimitiveType::TrianglesList; //doesnt work
   //let i_type = glium::index::PrimitiveType::LinesListAdjacency; //bad
   //let i_type = glium::index::PrimitiveType::LinesList; //bad
   //let i_type = glium::index::PrimitiveType::LineStrip; //bad
   //let i_type = glium::index::PrimitiveType::LineStripAdjacency; //bad'
   //let i_type = glium::index::PrimitiveType::Points; //ok-ish
   let indices = glium::index::NoIndices(i_type);


   let vertex_shader_src = r#"
      #version 140
      in vec3 position;
      uniform mat4 matrix;
      //void main() { gl_Position = matrix * vec4(position, 1.0); }
      void main() { gl_Position = matrix * vec4(position/500.0, 1.0); }
   "#;

   let fragment_shader_src = r#"
      #version 140
      out vec4 color;
      //void main() { color = vec4(1.0, 0.0, 0.0, 1.0); }
      void main() { color = vec4(0.0, 1.0, 0.0, 1.0); }

   "#;

   let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

   let mut i = 0.0f32;

   loop {
      i += 2.0*/*f32::consts::PI*/3.1415/1000.0;

      let mut target = display.draw();
      target.clear_color(0.0, 0.0, 1.0, 1.0);

      /*let matrix = [
         [0.01, 0.0,  0.0,  0.0],
         [0.0,  0.01, 0.0,  0.0],
         [0.0,  0.0,  0.01, 0.0],
         [0.0,  0.0,  0.0,  1.0f32]
      ];*/

      /*let matrix = [
         [1.0, 0.0,  0.0,  0.0],
         [0.0,  1.0, 0.0,  0.0],
         [0.0,  0.0,  1.0, 0.0],
         [0.0,  0.0,  0.0,  1.0f32]
      ];*/

      let matrix = [
         [i.cos(), 0.0,  i.sin(),  0.0],
         [0.0,  1.0, 0.0,  0.0],
         [-i.sin(),  0.0,  i.cos(), 0.0],
         [0.0,  0.0,  0.0,  1.0f32]
      ];
      //target.draw(&positions, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
      target.draw(&positions, &indices, &program, &uniform! { matrix: matrix }, &Default::default()).unwrap();
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
   //display_file("data/square/square.obj");
   //display_file("data/sub/subdiv_square.obj");
   display_file("data/female02.obj");

}
