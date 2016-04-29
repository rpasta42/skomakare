extern crate image;

use types::*;
use glium::texture::{Texture2d, RawImage2d};
use std::io::Cursor;

#[allow(dead_code)]
pub fn char_at(s : &str, n : usize) -> Option<char> {
   for (i, c) in s.chars().enumerate() {
      if i == n { return Some(c) }
   } return None
}

/*
#[allow(dead_code)]
pub fn s_to_f(s : &str) -> f32 {
   s.parse::<f32>().unwrap()
}

fn display_file(path : &str) {
   let data = read_file(path).unwrap();
   let m = parse_obj(&data);
   m.print();
   draw(&m);
}
pub fn read_bin_file(path_str : &str) -> [u8; 12] {
   use std::fs::File;
   use std::io::Read;

   let mut file = File::open(path_str).unwrap();
   let mut buf = [0u8; 12];
   file.read(&mut buf).unwrap();
   return buf;
}*/

#[allow(dead_code)]
pub fn read_file(path_str : &str) -> Option<String> {
   use std::io::prelude::*;
   use std::fs::File;
   use std::path::Path;
   use std::error::Error;

   println!("loading file {}", path_str);
   let path = Path::new(path_str);
   match File::open(&path) {
      Ok(mut file) => {
         let mut file_content = String::new();
         match file.read_to_string(&mut file_content) {
            Ok(_) => Some(file_content.to_string()),
            Err(why) => { panic!("{}", Error::description(&why)) }
         }
      }
      Err(why) => { panic!("{}", Error::description(&why)) }
   }
}


//&include_bytes!("path")[..]
fn img_path_to_image<'a>(img_path : &str) -> RawImage2d<'a, u8> {
   use std::fs::File;
   let mut c = File::open(img_path).unwrap();
   //let c = Cursor::new(&include_bytes!("../data/opengl.png")[..]);

   let image = image::load(c, image::PNG).unwrap().to_rgba();
   //let image = image::load(c, image::JPEG).unwrap().to_rgba();

   let image_dimensions = image.dimensions();
   RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions)
}

pub fn img_path_to_texture(img_path : String, display : &Display) -> Texture2d {
   let image = img_path_to_image(&*img_path);
   Texture2d::new(display, image).unwrap()
}

