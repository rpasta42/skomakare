use types::*;
use shaders::*;
use scene::*;
use camera::Camera;
use model::*;
use glium::glutin::Event;

pub type MousePos = (f32, f32);
pub type GameEvents = (Vec<Event>, Vec<MousePos>);

pub struct Game {
   //pub window : Window,
   pub display : Display,
   pub root : Scene,
   pub shader_manager : ShaderManager,
   pub cam : Camera,
   pub clear_color : Color,
   pub mouse_pos : MousePos
}

impl Game {
   pub fn new() -> Game {
      use glium::DisplayBuild;
      use glium::glutin::WindowBuilder;

      let win_b = WindowBuilder::new();
      let display = win_b.build_glium();
      //let window = &win_b.build().unwrap();

      let mut game = Game {
         //window : win_b.build().unwrap(),
         display : display.unwrap(),
         cam : Camera::new(),
         root : Scene::new(),
         shader_manager : ShaderManager::new(),
         clear_color : (0.0, 0.0, 0.0, 1.0), //white
         mouse_pos : (0.0, 0.0)
      };
      game.shader_manager.add_defaults(&game.display);
      game
   }

#[allow(usused_variables)]
   pub fn draw(&mut self) -> GameEvents {
      use glium::Surface;

      let init_m = self.cam.get_m();
      let mut target = self.display.draw();
      let cc = self.clear_color;
      target.clear_color(cc.0, cc.1, cc.2, cc.3);

      for game_obj in &self.root.items {
         let obj_m = game_obj.cam.get_m();
         let final_m = mul_matrices(&obj_m, &init_m);

         if let GameObjectType::Model(ref m) = game_obj.data {
            let shape = m.shape.clone().unwrap();

            use glium::VertexBuffer as VB;
            let vert_buff = VB::new(&self.display,
                                    &shape.vertices)
                                   .unwrap();

            use glium::index::NoIndices;
            let indices = NoIndices(shape.primitive_type.unwrap());
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

      let polled_events = self.display.poll_events();
      for ev in polled_events {
         match ev {
            Event::Closed => panic!("exiting application"), //return,
            e => {
               //println!("{:?}", e);
               use glium::glutin::ElementState::Released;
               use glium::glutin::MouseButton::Left;
      
               match e {
                  //Event::MouseMoved((x, y)) => {
                  Event::MouseMoved(x, y) => {
                     //let (w_w, w_h) = self.window.get_inner_size_pixels().unwrap();
                     //self.mouse_pos.0 = (x as f32) / (w_w as f32);
                     //self.mouse_pos.1 = (y as f32) / (w_h as f32);
                     let screen_size = self.display.get_framebuffer_dimensions();
                     let (screen_width, screen_height) = (screen_size.0 as f32, screen_size.1 as f32);
                     self.mouse_pos.0 = x as f32 / screen_width; //657.0;
                     self.mouse_pos.1 = y as f32 / screen_height; //533.0;
                  },
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


