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
use glium::glutin::Event;

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
   script_objs : HashMap<String, usize>,
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
         script_objs : HashMap::new(),
      };
      game.shader_manager.add_defaults(&game.display);
      game
   }
   fn draw(&mut self) -> Vec<Event> {
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

      let mut events = Vec::new();
      for ev in self.display.poll_events() {
         match ev {
            Event::Closed => panic!("exiting application"), //return,
            e => {
               //println!("{:?}", e);
               if let Event::KeyboardInput(_, _, Some(key)) = e {
                   events.push(e);
               }
            }
         }
      }
      events
   }
}

use std::sync::mpsc::{Sender, Receiver, channel};
use lambda_oxide::types::{Sexps, arg_extractor, EnvId, err, print_compact_tree, arg_extract_str, arg_extract_float, Root};
use lambda_oxide::main::{Env, eval, Callable};
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
type EventSender = Sender<Vec<Event>>;
type EventReceiver = Receiver<Vec<Event>>;

#[allow(unused_variables)]
fn setup_game_script_env(sender : CmdSender, event_rec : EventReceiver) -> RefCell<Env> {
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

   let check_events = move |args : Sexps, root : Root, table : EnvId| -> Sexps {
      //let e_res = event_rec.try_recv();
      let e_res = event_rec.try_recv();

      let mut c = "nil".to_string();
      println!("checking events");
      if let Ok(events) = e_res {
         println!("got events {:?}", events);
         for e in events {
            if let Event::KeyboardInput(_, _, Some(key)) = e {
               c = format!("{:?}", key).to_lowercase();
               /*println!("{}", s);
               match key {
                  glium::glutin::VirtualKeyCode::W => c = "w",
                  glium::glutin::VirtualKeyCode::S => c = "s",
                  glium::glutin::VirtualKeyCode::A => c = "a",
                  glium::glutin::VirtualKeyCode::D => c = "d",
                  _ => {}
               }*/
            }
         }
      }
      Sexps::Str(c)
   };
   env.borrow_mut().table_add_f("check_events", check_events);

   env
}

fn main() {
   let mut game = Game::new();

   //use std::sync::mpsc;
   use std::thread::Builder;

   let (cmd_t, cmd_r) : (CmdSender, CmdReceiver) = channel();
   let (event_t, event_r) : (EventSender, EventReceiver) = channel();

   let child = Builder::new().stack_size(8*32*1024*1024).spawn(move || {
      use lambda_oxide::main;

      let env = setup_game_script_env(cmd_t, event_r);
      main::interpreter(Some(env));
   }).unwrap();

   //engine_main();
   loop {
      let script_cmd_res = cmd_r.try_recv();
      if let Ok(script_cmd) = script_cmd_res {
         use GameCmd::*;

         match script_cmd {
            Obj(shape_name, shape_type, color_or_texture) => {
               let mut model_builder = Model::new();

               let shape = match &*shape_type {
                  "triangle" => Shape::new_builtin(BuiltInShape::Triangle),
                  "circle" => Shape::new_builtin(BuiltInShape::Circle),
                  "square" => Shape::new_builtin(BuiltInShape::Rectangle),
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
      let events = game.draw();
      if events.len() > 0 {
         event_t.send(events);
      }
   }
   child.join().unwrap();
}

