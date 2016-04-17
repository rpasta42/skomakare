#![feature(type_ascription)]
#[macro_use]
extern crate glium;
extern crate image;

use std::io::Cursor;

use model::*;
use data::*;
use types::*;

use glium::texture::*;

mod types;
mod data;
mod model;

implement_vertex!(ColorVertex, pos, tex_pos);

fn img_path_to_image<'a>(img_path : &str) -> RawImage2d<'a, u8> {
   let c = Cursor::new(&include_bytes!("../data/opengl.png")[..]);

   //load() -> ImageResult<DynamicImage>
   //to_rgba() -> RgbImage
   let image = image::load(c, image::PNG).unwrap().to_rgba();

   let image_dimensions = image.dimensions();

   //image type RgbImage = ImageBuffer<Rgb<u8>, Vec<u8>>;
   //image.into_raw() -> Vec<u8>
   //from_raw_rgba_reversed -> RawImage2d<'a, u8>
   let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
   image
}

fn draw(m : &Shape, img_path : &str) {
   use glium::{DisplayBuild, Surface};
   let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

   let image = img_path_to_image(img_path);
   let texture = Texture2d::new(&display, image).unwrap(); 

   let vertex_buffer = glium::VertexBuffer::new(&display, &m.vertices).unwrap();
   let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


   let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

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

fn main_old() {
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

   let m = Model::new().shape(Shape::new_builtin(BuiltInShape::Triangle)).finalize();
   draw(&m.shape.unwrap(), "data/opengl.png");
}

