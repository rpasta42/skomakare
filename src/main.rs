#![feature(type_ascription)]
#[macro_use]
extern crate glium;
extern crate image;

use model::*;
use shaders::*;
use types::*;
use scene::*;
use utils::*;
use camera::Camera;

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
   root : Scene,
   shader_manager : ShaderManager,
   cam : Camera
}
impl Game {
   fn new() -> Game {
      use glium::{DisplayBuild, Surface};
      use glium::glutin::WindowBuilder;

      let display_ = WindowBuilder::new().build_glium().unwrap();
      let mut game = Game {
         display : display_,
         cam : Camera::new(),
         root : Scene::new(),
         shader_manager : ShaderManager::new()
      };
      game.shader_manager.add_defaults(&game.display);
      game
   }
   fn draw(&self) {
      use glium::Surface;
      //self.root.draw();
      let init_m = self.cam.get_m();
      let mut target = self.display.draw();
      target.clear_color(0.0, 0.0, 1.0, 1.0);

      for game_obj in &self.root.items {
         let obj_m = game_obj.cam.get_m();
         let final_m = mul_matrices(&init_m, &obj_m);

         if let GameObjectType::Model(ref m) = game_obj.data {
            let shape = m.shape.clone().unwrap();

            //draw(&shape, "data/opengl.png");
            use glium::VertexBuffer as VB;
            let vert_buff = VB::new(&self.display,
                                    &shape.vertices)
                                   .unwrap();

            use glium::index::{NoIndices, PrimitiveType};
            let indices = NoIndices(PrimitiveType::TrianglesList);
            let ref shaders = self.shader_manager.shaders;
            let shader_name = m.shader_name.clone().unwrap();
            let program = shaders.get(&*shader_name).unwrap();


            /*let i = 0.0f32;
            let ma = [[1.0, 0.0, 0.0, 0.0],
                      [0.0, 1.0, 0.0, 0.0],
                      [0.0, 0.0, 1.0, 0.0],
                      [0.0, 0.0, 0.0, 1.0f32]];*/
            let ma = final_m;

            match m.texture_type {
               TextureType::Image => {
                  let t = match *m.get_texture() {
                     Some(ref x) => x, None => panic!("z")
                  };
                  /*let u = uniform! {
                     matrix : //final_m,
                     tex : t
                  };*/
                  let i = 0.0f32;
                  let u = uniform! {
                     matrix: ma,
                     tex: t
                  };

                  target.draw(&vert_buff, &indices, program, &u,
                              &Default::default()).unwrap();

               },
               TextureType::Color => {
                  //let u = uniform! { matrix : final_m };
                  let u = uniform! {
                      matrix: ma
                  };

                  target.draw(&vert_buff, &indices, program, &u,
                              &Default::default()).unwrap();
               }
               _ => { panic!("unknown texture type"); }
            };

         } else { panic!("unsupported object"); }
         /*match game_obj.data {
            Model(ref m) => {
               let shape = m.shape.clone();
               draw(&shape.unwrap(), "data/opengl.png");
            }
            _ => panic!("unsupported object")
         };*/
      }
      target.finish().unwrap();
      use glium::glutin::Event;
      for ev in self.display.poll_events() {
         match ev {
            Event::Closed => panic!("exiting application"), //return,
            _ => ()
         }
      }
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
   //use glium::{DisplayBuild, Surface};
   //let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

   let mut game = Game::new();

   let shape = Shape::new_builtin(BuiltInShape::Triangle);
   let color = (1.0, 0.0, 0.0, 1.0);

   let m = Model::new()
            .shape(shape).color(color) //.img_path("data/opengl.png")
            .finalize(&mut game.shader_manager, &game.display);

   let triangle = GameObject::new(GameObjectType::Model(m));
   game.root.items.push(triangle);

   use std::time::Duration;
   use std::time::SystemTime;
   let mut start = SystemTime::now();

   loop {
      let elapsed = now.duration_since(start).unwrap().subsec_nanos();
      if elapsed > 1000000000 {
         println!("{}", elapsed);
         start = now;
      }
      game.draw();
   }

   //draw(&m.shape.unwrap(), "data/opengl.png");
}

