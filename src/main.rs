#![feature(type_ascription, libc)]
#[macro_use]
extern crate glium;
extern crate image;
extern crate lambda_oxide;
extern crate libc;

use std::collections::HashMap;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::Event;

use std::sync::mpsc::{Sender, Receiver, channel};
use lambda_oxide::types::{Sexps, arg_extractor, EnvId, err, print_compact_tree, arg_extract_str, arg_extract_float, Root};
use lambda_oxide::main::{Env, eval, Callable};
use std::cell::RefCell;

use types::*;
use utils::*;
use scene::*;
use model::*;
use game::Game;

mod game;
mod types;
mod shaders;
mod model;
mod scene;
mod camera;
mod utils;

implement_vertex!(ColorVertex, pos, tex_pos);


type ObjId = i64;
//Obj(name, <triangle|square|circle>, <color|pic_path>)
#[derive(Clone)]
enum GameCmd {
   Obj(ObjId, String, String), Move(ObjId, Point),
   Rotate(ObjId, Coord), Scale(ObjId, Point), Exit
}
type CmdSender = Sender<GameCmd>;
type CmdReceiver = Receiver<GameCmd>;
type EventSender = Sender<Vec<Event>>;
type EventReceiver = Receiver<Vec<Event>>;

type IdSender = Sender<ObjId>;
type IdReceiver = Receiver<ObjId>;

#[allow(unused_variables)]
fn setup_game_script_env(sender : CmdSender, event_r : EventReceiver, id_r : IdReceiver) -> RefCell<Env> {
   let env = lambda_oxide::main::setup_env();

   let sender_shape = sender.clone();
   let shape_ = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      if let Sexps::Err(ref s) = args_ { return err(s); }

      let args = arg_extractor(&args_).unwrap();
      if args.len() != 2 { return err("shape needs 2 arguments"); }

      let id = id_r.recv().unwrap();
      let shape_type = arg_extract_str(&args, 0).unwrap(); //TODO: check types
      let color_type = arg_extract_str(&args, 1).unwrap(); //and notify user if wrong

      let cmd = GameCmd::Obj(id, shape_type, color_type);
      sender_shape.send(cmd).unwrap();

      Sexps::Int(id)
   };
   env.borrow_mut().table_add(0, "shape", Callable::BuiltIn(0, Box::new(shape_)));

   let sender_move = sender.clone();
   //object_name, x, y
   let move_ = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      if let Sexps::Err(ref s) = args_ { return err(s); }

      let args = arg_extractor(&args_).unwrap();
      if args.len() != 3 { return err(&*format!("move needs 3 arguments but {} were given", args.len())); }

      let shape_id = arg_extract_float(&args, 0).unwrap() as i64;
      let x = arg_extract_float(&args, 1).unwrap(); //TODO: check types
      let y = arg_extract_float(&args, 2).unwrap(); //and notify user if wrong

      let cmd = GameCmd::Move(shape_id, [x as f32, y as f32]);
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

      let shape_id = arg_extract_float(&args, 0).unwrap() as i64; //TODO: check types
      let degrees = arg_extract_float(&args, 1).unwrap();  //and notify user if wrong

      let cmd = GameCmd::Rotate(shape_id, degrees as Coord);
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

      let shape_id = arg_extract_float(&args, 0).unwrap() as i64; //TODO: check types
      let x = arg_extract_float(&args, 1).unwrap(); //and notify user if wrong
      let y = arg_extract_float(&args, 2).unwrap();

      let cmd = GameCmd::Scale(shape_id, [x as Coord, y as Coord]);
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
      let e_res = event_r.try_recv();

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


   let (id_t, id_r) : (IdSender, IdReceiver) = channel();
   for i in 0..100 { id_t.send(i); }

   let (cmd_t, cmd_r) : (CmdSender, CmdReceiver) = channel();
   let (event_t, event_r) : (EventSender, EventReceiver) = channel();

   let child = Builder::new().stack_size(8*32*1024*1024).spawn(move || {
      use lambda_oxide::main;

      let env = setup_game_script_env(cmd_t, event_r, id_r);
      main::interpreter(Some(env));
   }).unwrap();

   //engine_main();
   loop {
      let script_cmd_res = cmd_r.try_recv();
      if let Ok(script_cmd) = script_cmd_res {
         use GameCmd::*;

         match script_cmd {
            Obj(shape_id, shape_type, color_or_texture) => {
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
            },
            Move(shape_id, p) => {
               //let index = game.script_objs.get(&shape_name).unwrap();
               //game.root.items[*index].cam.translate(&p);
               game.root.items[shape_id as usize].cam.translate(&p);
            },
            Rotate(shape_id, degrees) => {
               //let index = game.script_objs.get(&shape_name).unwrap();
               //game.root.items[*index].cam.rotate(degrees);
               game.root.items[shape_id as usize].cam.rotate(degrees);
            },
            Scale(shape_id, p) => {
               //let index = game.script_objs.get(&shape_name).unwrap();
               game.root.items[shape_id as usize].cam.scale(&p);
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

