#![feature(type_ascription, libc)]
#[macro_use]
extern crate glium;
extern crate image;
extern crate lambda_oxide;
extern crate libc;

use std::collections::HashMap;

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
   cam : Camera,
   clear_color : Color,
   //since we can't store game objects in lisp, we track everything by name
   script_objs : HashMap<String, usize>
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
         shader_manager : ShaderManager::new(),
         clear_color : (0.0, 0.0, 0.0, 1.0), //white
         script_objs : HashMap::new()
      };
      game.shader_manager.add_defaults(&game.display);
      game
   }
   fn draw(&self) {
      use glium::Surface;
      //self.root.draw();
      let init_m = self.cam.get_m();
      let mut target = self.display.draw();
      let cc = self.clear_color;
      target.clear_color(cc.0, cc.1, cc.2, cc.3);

      for game_obj in &self.root.items {
         let obj_m = game_obj.cam.get_m();
         //let final_m = mul_matrices(&init_m, &obj_m);
         //let final_m = obj_m;
         let final_m = mul_matrices(&obj_m, &init_m);

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

            match m.texture_type {
               TextureType::Image => {
                  let t = match *m.get_texture() {
                     Some(ref x) => x, None => panic!("z")
                  };
                  let u = uniform! {
                     matrix: final_m,
                     tex: t
                  };

                  target.draw(&vert_buff, &indices, program, &u,
                              &Default::default()).unwrap();

               },
               TextureType::Color => {
                  let u = uniform! { matrix : final_m };
                  target.draw(&vert_buff, &indices, program, &u,
                              &Default::default()).unwrap();
               }
               _ => { panic!("unknown texture type"); }
            };

         } else { panic!("unsupported object"); }
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

fn draw(m : &Shape, img_path : String) {
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

#[allow(dead_code)]
fn main_very_old() {
   use glium::{DisplayBuild, Surface};
   let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

   let mut m = Shape::new();
   m.add_coords(-0.5, -0.5, 0.0, 0.0);
   m.add_coords(0.0, 0.5, 0.0, 1.0);
   m.add_coords(0.5, -0.25, 1.0, 0.0);
   draw(&m, "data/opengl.png".to_string());
}

fn engine_main() {
   let mut game = Game::new();

   let shape = Shape::new_builtin(BuiltInShape::Triangle);
   let red = (1.0, 0.0, 0.0, 1.0);

   let m = Model::new()
            .shape(shape).color(red) //.img_path("data/opengl.png")
            .finalize(&mut game.shader_manager, &game.display);

   let triangle = GameObject::new(GameObjectType::Model(m));
   game.root.items.push(triangle);

   use std::time::Duration;
   use std::time::SystemTime;
   let mut start = SystemTime::now();
   let one_tenth_sec = 100000000;
   let mut moved = false;
   let mut rotd = false;

   loop {
      let elapsed = start.elapsed().unwrap().as_secs();
      if elapsed > 1 && !moved {
         game.root.items[0].cam.translate(&[1.0, 0.0]);
         moved = true;
      }
      //if elapsed > 2 && !rotd {game.root.items[0].cam.rotate(90.0); rotd = true;}
      game.draw();
   }
   //draw(&m.shape.unwrap(), "data/opengl.png");
}

use std::sync::mpsc::{Sender, Receiver, channel};
use lambda_oxide::types::Sexps;
use lambda_oxide::main::{Env, arg_extract_str, eval};
use std::cell::RefCell;

type ObjName = String;
//Obj(name, <triangle|square|circle>, <color|pic_path>)
#[derive(Clone)]
enum GameCmd {
   Obj(String, String, String), Move(String, Point),
   Rotate(String, Coord), Scale(String, Point), Exit
}
type CmdSender = Sender<GameCmd>;
type CmdReceiver = Receiver<GameCmd>;

pub fn arg_extract_float(args : &Vec<Sexps>, index : usize) -> Option<f64> {
   if let Sexps::Float(ref s) = args[index] {
      Some(s.clone())
   } else { None }
}

#[allow(unused_variables)]
fn setup_game_script_env(sender : CmdSender) -> RefCell<Env> {
   use lambda_oxide::main::{Callable, Root};
   use lambda_oxide::types::{Sexps, arg_extractor, EnvId, err, print_compact_tree};

   let env = lambda_oxide::main::setup_env();

   let sender_shape = sender.clone();
   let shape_ = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      if let Sexps::Err(ref s) = args_ { return err(s); }

      let args = arg_extractor(&args_).unwrap();
      if args.len() < 3 { return err("shape needs 3 arguments"); }

      let shape_name = arg_extract_str(&args, 0).unwrap();
      let shape_type = arg_extract_str(&args, 1).unwrap(); //TODO: check types
      let color_type = arg_extract_str(&args, 2).unwrap(); //and notify user if wrong

      let cmd = GameCmd::Obj(shape_name.clone(), shape_type, color_type);
      sender_shape.send(cmd.clone()).unwrap();

      Sexps::Str(shape_name.clone())
   };
   env.borrow_mut().table_add(0, "shape", Callable::BuiltIn(0, Box::new(shape_)));

   let sender_move = sender.clone();
   //object_name, x, y
   let move_ = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      if let Sexps::Err(ref s) = args_ { return err(s); }

      let args = arg_extractor(&args_).unwrap();
      if args.len() != 3 { return err(&*format!("move needs 3 arguments but {} were given", args.len())); }

      let shape_name = arg_extract_str(&args, 0).unwrap();
      let x = arg_extract_float(&args, 1).unwrap(); //TODO: check types
      let y = arg_extract_float(&args, 2).unwrap(); //and notify user if wrong

      let cmd = GameCmd::Move(shape_name, [x as f32, y as f32]);
      sender_move.send(cmd.clone()).unwrap();

      Sexps::Str("success".to_string())
   };
   env.borrow_mut().table_add(0, "move", Callable::BuiltIn(0, Box::new(move_)));

   let sender_rotate = sender.clone();
   //object_name, degrees
   let rotate = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      if let Sexps::Err(ref s) = args_ { return err(s); }

      let args = arg_extractor(&args_).unwrap();
      if args.len() != 2 { return err("rotate needs 2 arguments"); }

      let shape_name = arg_extract_str(&args, 0).unwrap(); //TODO: check types
      let degrees = arg_extract_float(&args, 1).unwrap();  //and notify user if wrong

      let cmd = GameCmd::Rotate(shape_name, degrees as Coord);
      sender_rotate.send(cmd.clone());

      Sexps::Str("success".to_string())
   };
   env.borrow_mut().table_add(0, "rotate", Callable::BuiltIn(0, Box::new(rotate)));

   let sender_scale = sender.clone();
   //object_name, degrees
   let resize = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      if let Sexps::Err(ref s) = args_ { return err(s); }

      let args = arg_extractor(&args_).unwrap();
      if args.len() != 3 { return err("resize needs 3 arguments"); }

      let shape_name = arg_extract_str(&args, 0).unwrap(); //TODO: check types
      let x = arg_extract_float(&args, 1).unwrap(); //and notify user if wrong
      let y = arg_extract_float(&args, 2).unwrap();

      let cmd = GameCmd::Scale(shape_name, [x as Coord, y as Coord]);
      sender_scale.send(cmd.clone()).unwrap();

      Sexps::Str("success".to_string())
   };
   env.borrow_mut().table_add(0, "resize", Callable::BuiltIn(0, Box::new(resize)));

   let halt_ = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      if let Sexps::Err(ref s) = args_ { return err(s); }

      let cmd = GameCmd::Exit;
      //kkerr sender.send(cmd).unwrap();
      err("halting")
   };
   env.borrow_mut().table_add(0, "exit", Callable::BuiltIn(0, Box::new(halt_)));

   //TODO: move this to core language
   let sleep = |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      if let Sexps::Err(ref s) = args_ { return err(s); }

      let args = arg_extractor(&args_).unwrap();
      if args.len() != 1 { return err("sleep needs 1 argument"); }
      let time = arg_extract_float(&args, 0).unwrap()*1000.0;

      use std::thread::sleep_ms;
      sleep_ms(time as u32);

      Sexps::Str("success".to_string())
   };
   env.borrow_mut().table_add(0, "sleep", Callable::BuiltIn(0, Box::new(sleep)));

   //TODO: add this to core language
   let do_ = |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      if let Sexps::Err(ref s) = args_ { return err(s); }

      let args = arg_extractor(&args_).unwrap();

      let mut result = err("empty do");
      for arg in args {
         result = eval(&arg, root, table);
      }
      result
   };
   env.borrow_mut().table_add(0, "do", Callable::BuiltIn(0, Box::new(do_)));

   env
}

fn main() {
   let mut game = Game::new();

   //use std::sync::mpsc;
   use std::thread::Builder;

   let (tx, rx) : (CmdSender, CmdReceiver) = channel();

   let child = Builder::new().stack_size(8*32*1024*1024).spawn(move || {
      use lambda_oxide::main;

      let env = setup_game_script_env(tx);
      main::interpreter(Some(env));
   }).unwrap();

   //engine_main();
   loop {
      let script_cmd_res = rx.try_recv();
      if let Ok(script_cmd) = script_cmd_res {
         use GameCmd::*;

         match script_cmd {
            Obj(shape_name, shape_type, color_or_texture) => {
               let mut model_builder = Model::new();

               let shape = match &*shape_type {
                  "triangle" => Shape::new_builtin(BuiltInShape::Triangle),
                  _ => panic!("unsuported shape")
               };
               model_builder.shape(shape);

               let color_opt = match &*color_or_texture {
                  "red"    => Some((1.0, 0.0, 0.0, 1.0)),
                  "green"  => Some((0.0, 1.0, 0.0, 1.0)),
                  "blue"   => Some((0.0, 0.0, 1.0, 1.0)),
                  _        => None
               };
               if let Some(color) = color_opt {
                  model_builder.color(color);
               } else {
                  model_builder.img_path(color_or_texture);
               }
               let model = model_builder.finalize(&mut game.shader_manager, &game.display);
               let game_object = GameObject::new(GameObjectType::Model(model));
               game.root.items.push(game_object);

               game.script_objs.insert(shape_name, game.root.items.len()-1);
            },
            Move(shape_name, p) => {
               let index = game.script_objs.get(&shape_name).unwrap();
               game.root.items[*index].cam.translate(&p);
            },
            Rotate(shape_name, degrees) => {
               let index = game.script_objs.get(&shape_name).unwrap();
               game.root.items[*index].cam.rotate(degrees);
            },
            Scale(shape_name, p) => {
               let index = game.script_objs.get(&shape_name).unwrap();
               game.root.items[*index].cam.scale(&p);
            }
            Exit => break,
            //_ => panic!("unsuported command")
         }
      }
      game.draw();
   }
   child.join().unwrap();
}

