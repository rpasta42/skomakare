#![feature(type_ascription)]
#[macro_use]
extern crate glium;
extern crate image;

use model::*;
use shaders::*;
use types::*;
use scene::*;
use utils::*;

use glium::backend::glutin_backend::GlutinFacade;

mod types;
mod shaders;
mod model;
mod scene;
mod camera;
mod utils;

implement_vertex!(ColorVertex, pos, tex_pos);

struct Game {
   display : Display,
   scene : Scene,
   shader_manager : ShaderManager
}
impl Game {
   fn new() -> Game {
      use glium::{DisplayBuild, Surface};
      let display_ = glium::glutin::WindowBuilder::new().build_glium().unwrap();
      let mut game = Game {
         display : display_,
         scene : Scene { items: Vec::new() },
         shader_manager : ShaderManager::new()
      };
      game.shader_manager.add_defaults(&game.display);
      game
   }
}

fn draw(m : &Shape, img_path : &str) {
   use glium::{DisplayBuild, Surface};
   let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

   //let image = img_path_to_image(img_path);
   //let texture = Texture2d::new(&display, image).unwrap(); 
   let texture = img_path_to_texture(img_path, &display);

   let vertex_buffer = glium::VertexBuffer::new(&display, &m.vertices).unwrap();
   let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

   use glium::Program;
   let program = Program::from_source(&display, VERT_SH_TEXTURE, FRAG_SH_TEXTURE, None).unwrap();

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

fn main_very_old() {
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

   let mut game = Game::new();

   let shape = Shape::new_builtin(BuiltInShape::Triangle);
   let color = (1.0, 0.0, 1.0, 0.0);
   let m = Model::new()
            .shape(shape)
            .color(color)
            .finalize(&mut game.shader_manager, &game.display);

   //draw(&m.shape.unwrap(), "data/opengl.png");

   let triangle = GameObject::new(ObjectType::Model(m));
   game.scene.items.push(triangle);

   match game.scene.items[0].object_type {
      ObjectType::Model(ref m) => {
         let shape = m.shape.clone();
         draw(&shape.unwrap(), "data/opengl.png");
      }
      _ => panic!("unsupported object")
   };
}

