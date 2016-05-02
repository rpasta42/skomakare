use types::*;
use shaders::*;
use scene::*;
use camera::Camera;
use model::*;
use glium::glutin::Event;

pub type MousePos = (u16, u16);
pub type GameEvents = (Vec<Event>, Vec<MousePos>);

pub struct Game {
   pub display : Display,
   pub root : Scene,
   pub shader_manager : ShaderManager,
   pub cam : Camera,
   pub clear_color : Color,
   pub mouse_pos : MousePos
}

impl Game {
   pub fn new() -> Game {
      use glium::{DisplayBuild, Surface};
      use glium::glutin::WindowBuilder;

      let display_ = WindowBuilder::new().build_glium().unwrap();
      let mut game = Game {
         display : display_,
         cam : Camera::new(),
         root : Scene::new(),
         shader_manager : ShaderManager::new(),
         clear_color : (0.0, 0.0, 0.0, 1.0), //white
         mouse_pos : (0, 0)
      };
      game.shader_manager.add_defaults(&game.display);
      game
   }
   pub fn draw(&mut self) -> GameEvents {
      use glium::Surface;
      //self.root.draw();
      let init_m = self.cam.get_m();
      let mut target = self.display.draw();
      let cc = self.clear_color;
      target.clear_color(cc.0, cc.1, cc.2, cc.3);

      for game_obj in &self.root.items {
         let obj_m = game_obj.cam.get_m();
         let final_m = mul_matrices(&obj_m, &init_m);
         //let final_m = game_obj.cam.get_rot_m();


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

      let mut key_events = Vec::new();
      let mut click_events = Vec::new();
      for ev in self.display.poll_events() {
         match ev {
            Event::Closed => panic!("exiting application"), //return,
            e => {
               //println!("{:?}", e);
               use glium::glutin::ElementState::Released;
               use glium::glutin::MouseButton::Left;

               match e {
                  Event::MouseMoved(x, y) => self.mouse_pos = (x as u16, y as u16),
                  Event::KeyboardInput(_, _, Some(key)) => key_events.push(e),
                  Event::MouseInput(Released, Left) => click_events.push(self.mouse_pos),
                  _ => {}
               }
            }
         }
      }
      (key_events, click_events)
   }
}


