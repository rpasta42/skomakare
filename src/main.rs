#![feature(type_ascription, libc)]
#[macro_use]
extern crate glium;
extern crate image;
extern crate lambda_oxide;
extern crate libc;

use glium::glutin::Event;

use std::sync::mpsc::{Sender, Receiver, channel};
use lambda_oxide::types::{Sexps, arg_extractor, EnvId, err, arg_extract_str, arg_extract_num, Root};
use lambda_oxide::main::Env;
use std::cell::RefCell;

use types::*;
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
   Obj(ObjId, String, String),
   Move(ObjId, Point), Rotate(ObjId, Coord), Scale(ObjId, Point),
   SetPos(ObjId, Point), SetRotation(ObjId, Coord), SetSize(ObjId, Point),
   Exit
}
type CmdSender = Sender<GameCmd>;
type CmdReceiver = Receiver<GameCmd>;

//type EventSender = Sender<Vec<Event>>;
//type EventReceiver = Receiver<Vec<Event>>;
use game::GameEvents;

type EventSender = Sender<GameEvents>;
type EventReceiver = Receiver<GameEvents>;

type IdSender = Sender<ObjId>;
type IdReceiver = Receiver<ObjId>;

#[allow(unused_variables)]
fn setup_game_script_env(sender : CmdSender, event_r : EventReceiver, id_r : IdReceiver) -> RefCell<Env> {
   let env = lambda_oxide::main::setup_env();

   let sender_shape = sender.clone();
   let shape_ = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      let args = arg_extractor(&args_).unwrap();
      if args.len() != 2 { return err("shape needs 2 arguments"); }

      let id = id_r.recv().unwrap();
      let shape_type = arg_extract_str(&args, 0).unwrap(); //TODO: check types
      let color_type = arg_extract_str(&args, 1).unwrap(); //and notify user if wrong

      let cmd = GameCmd::Obj(id, shape_type, color_type);
      sender_shape.send(cmd).unwrap();

      Sexps::Int(id)
   };
   env.borrow_mut().table_add_f("shape", shape_);

   let sender_move = sender.clone();
   //object_name, x, y
   let move_ = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      let args = arg_extractor(&args_).unwrap();
      if args.len() != 3 { return err(&*format!("move needs 3 arguments but {} were given", args.len())); }

      let shape_id = arg_extract_num(&args, 0).unwrap() as i64;
      let x = arg_extract_num(&args, 1).unwrap(); //TODO: check types
      let y = arg_extract_num(&args, 2).unwrap(); //and notify user if wrong

      let cmd = GameCmd::Move(shape_id, [x as f32, y as f32]);
      sender_move.send(cmd.clone()).unwrap();

      Sexps::Str("success".to_string())
   };
   env.borrow_mut().table_add_f("move", move_);

   let sender_set_pos = sender.clone();
   //object_name, x, y
   let set_pos = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      let args = arg_extractor(&args_).unwrap();
      if args.len() != 3 { return err(&*format!("pos needs 3 arguments but {} were given", args.len())); }

      let shape_id = arg_extract_num(&args, 0).unwrap() as i64;
      let x = arg_extract_num(&args, 1).unwrap(); //TODO: check types
      let y = arg_extract_num(&args, 2).unwrap(); //and notify user if wrong

      let cmd = GameCmd::SetPos(shape_id, [x as f32, y as f32]);
      sender_set_pos.send(cmd.clone()).unwrap();
      Sexps::Str("success".to_string())
   };
   env.borrow_mut().table_add_f("pos", set_pos);

   let sender_rotate = sender.clone();
   //object_name, degrees
   let rotate = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      let args = arg_extractor(&args_).unwrap();
      if args.len() != 2 { return err("rotate needs 2 arguments"); }

      let shape_id = arg_extract_num(&args, 0).unwrap() as i64; //TODO: check types
      let degrees = arg_extract_num(&args, 1).unwrap();  //and notify user if wrong

      let cmd = GameCmd::Rotate(shape_id, degrees as Coord);
      sender_rotate.send(cmd.clone()).unwrap();

      Sexps::Str("success".to_string())
   };
   env.borrow_mut().table_add_f("rotate", rotate);

   let sender_scale = sender.clone();
   //object_name, degrees
   let resize = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      let args = arg_extractor(&args_).unwrap();
      if args.len() != 3 { return err("resize needs 3 arguments"); }

      let shape_id = arg_extract_num(&args, 0).unwrap() as i64; //TODO: check types
      let x = arg_extract_num(&args, 1).unwrap(); //and notify user if wrong
      let y = arg_extract_num(&args, 2).unwrap();

      let cmd = GameCmd::Scale(shape_id, [x as Coord, y as Coord]);
      sender_scale.send(cmd.clone()).unwrap();

      Sexps::Str("success".to_string())
   };
   env.borrow_mut().table_add_f("resize", resize);

   let sender_halt = sender.clone();
   let halt_ = move |args_ : Sexps, root : Root, table : EnvId| -> Sexps {
      sender_halt.send(GameCmd::Exit).unwrap();
      //kkerr sender.send(cmd).unwrap();
      err("halting")
   };
   env.borrow_mut().table_add_f("exit", halt_);

   let check_events = move |args : Sexps, root : Root, table : EnvId| -> Sexps {
      use lambda_oxide::main::run;
      //let e_res = event_rec.try_recv();
      let events = event_r.try_recv();

      let mut c = "nil".to_string();
      //println!("checking events");
      if let Ok((key_events, mouse_events)) = events {
         println!("got events {:?}", key_events);

         for e in key_events {
            if let Event::KeyboardInput(_, _, Some(key)) = e {
               let key_event_str = format!("(cons \"key\" \"{:?}\")", key).to_lowercase();
               return run(root, &*key_event_str).unwrap();
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
         for (x, y) in mouse_events {
            let mouse_event_str = format!("(cons \"mouse\" (cons {} {}))", x, y);
            return run(root, &*mouse_event_str).unwrap();
         }
      }
      run(root, "nil").unwrap()
      //Sexps::Var("nil".to_string())
   };
   env.borrow_mut().table_add_f("check_events", check_events);

   use std::str::from_utf8;
   use lambda_oxide::add_lisp_to_binary;
   let graphics_code = from_utf8(&include_bytes!("../graphics.lo")[..]).unwrap().to_string();
   add_lisp_to_binary(&graphics_code, &env);
   env
}

fn main() {
   let mut game = Game::new();

   use std::thread::Builder;

   let (id_t, id_r) : (IdSender, IdReceiver) = channel();
   for i in 0..1000 { id_t.send(i).unwrap(); }

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
            Obj(_, shape_type, color_or_texture) => {
               let mut model_builder = Model::new();

               let shape = match &*shape_type {
                  "triangle" => Shape::new_builtin(BuiltInShape::Triangle),
                  "circle" => Shape::new_builtin(BuiltInShape::Circle),
                  "square" => Shape::new_builtin(BuiltInShape::Rectangle),
                  shape_path => {
                     //panic!("unsuported shape")
                     let mut s = Shape::new();
                     s.from_obj_file(shape_path);
                     s
                  }
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
               game.root.items[shape_id as usize].cam.translate(&p);
            },
            Rotate(shape_id, rad) => {
               game.root.items[shape_id as usize].cam.rotate(rad);
            },
            Scale(shape_id, p) => {
               game.root.items[shape_id as usize].cam.scale(&p);
            },
            SetPos(shape_id, p) => {
               game.root.items[shape_id as usize].cam.set_position(&p);
            },
            SetRotation(shape_id, rad) => {
               game.root.items[shape_id as usize].cam.set_rotation(rad);
            },
            SetSize(shape_id, p) => {
               game.root.items[shape_id as usize].cam.set_size(&p);
            },
            Exit => break,
            //_ => panic!("unsuported command")
         }
      }
      //let (key_events, mouse_events) = game.draw();
      let events = game.draw();
      if events.0.len() > 0 || events.1.len() > 0 {
         event_t.send(events).unwrap();
      }
   }
   child.join().unwrap();
}

