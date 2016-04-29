
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
