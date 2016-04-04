#[macro_use]
extern crate glium;

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
      println!("(x:{}, y:{}, z:{})", self.position[0], self.position[1], self.position[2]);
   }
}

struct Model {
   vertices : Vec<Vertex>
}
impl Model {
   fn new() -> Model { Model { vertices : Vec::new() } }
   fn add(&mut self, v : Vertex) {
      self.vertices.push(v);
   }
   fn add_coords(&mut self, x : Coord, y : Coord, z : Coord) {
      self.vertices.push(Vertex::new(x, y, z));
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
   use glium::{DisplayBuild, Surface};
   let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
   implement_vertex!(Vertex, position);

   let vertex_buffer = glium::VertexBuffer::new(&display, &m.vertices).unwrap();
   let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

   let vertex_shader_src = r#"
      #version 140
      in vec2 position;
      void main() { gl_Position = vec4(position, 0.0, 1.0); }
   "#;

   let fragment_shader_src = r#"
      #version 140
      out vec4 color;
      void main() { color = vec4(1.0, 0.0, 0.0, 1.0); }
   "#;

   let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

   loop {
      let mut target = display.draw();
      target.clear_color(0.0, 0.0, 1.0, 1.0);
      target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
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
    display_file("data/square/square.obj");
}
