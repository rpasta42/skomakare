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

pub fn s_to_f(s : &str) -> f32 {
   s.parse::<f32>().unwrap()
}
pub fn s_to_usize(s : &str) -> usize {
   s.parse::<usize>().unwrap()
}
/*#[allow(dead_code)]
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

//texture
//https://doc.rust-lang.org/std/io/trait.Read.html
pub fn read_bin_file(path_str : &str, mut ret : &mut Vec<u8>) {
   use std::fs::File;
   use std::io::Read;

   let mut file = File::open(path_str).unwrap();
   file.read_to_end(&mut ret).unwrap();
}

/*pub fn text_to_texture_freetype(text : String, display : &Display) -> Texture2d
{
   // FREETYPE TEMP TEST CODE
   let font = "/usr/share/fonts/truetype/msttcorefonts/comic.ttf";

   let character = 'A' as usize;
   let library = freetype::Library::init().unwrap();
   let face = library.new_face(font, 0).unwrap();


   face.set_char_size(40 * 64, 0, 50, 0).unwrap();
   face.load_char(character, freetype::face::RENDER).unwrap();
   let glyph = face.glyph();
   // TODO(zac): Move this into a separate function.
   let bitmap = glyph.bitmap();
   let width = bitmap.width() as usize;
   let height = bitmap.rows() as usize;
   let data = bitmap.buffer();


   let mut vec_tex: Vec<Vec<u8>> = Vec::with_capacity(height); // TODO: Convert &[u8] into a Vec<Vec<u8>>.
   for y in 0..height {
      vec_tex.push(Vec::with_capacity(width));
      for x in 0..width {
         vec_tex[0].push(data[y*width + x]);
      }
   }

   Texture2d::new(&self.display, vec_tex).unwrap();
}*/


//https://botbot.me/mozilla/rust-gamedev/2015-07-26/?page=1
//https://tomaka.github.io/glium/glium/texture/struct.RawImage2d.html
//https://tomaka.github.io/glium/glium/texture/trait.Texture2dDataSource.html
//http://www.piston.rs/image/image/enum.DynamicImage.html
//http://www.piston.rs/image/image/enum.DynamicImage.html
//https://github.com/PistonDevelopers/image
pub fn text_to_texture(text : String, display : &Display) -> Texture2d
{
   //let font_path = "/usr/share/fonts/truetype/msttcorefonts/comic.ttf";
   let font_path = "examples-data-repo/text/comic.ttf";

   let (pixels, height, width) = raster_text(&*text, font_path, None);
   //println!("height: {}, width: {}", height, width);

   Texture2d::new(display, pixels).unwrap()
}

fn float_range(start : f32, step : f32, end : f32) -> Vec<f32> {
   let mut i = start;
   let mut ret = Vec::new();
   while i < end {
      ret.push(i);
      i += step;
   }
   ret
}

// (pixel_data, pixel_height, width)
fn raster_text(text : &str, font_path : &str, height_opt : Option<f32>) -> (Vec<Vec<u8>>, usize, usize)
{
   use rusttype::{FontCollection, Scale, point, PositionedGlyph};
   use std::io::Write;

   let mut font_data = Vec::new();
   read_bin_file(font_path, &mut font_data);
   let collection = FontCollection::from_bytes(&font_data as &[u8]);
   let font = collection.into_font().unwrap();

   let height: f32 = if let Some(x) = height_opt { x } else { 500.0 } ;
   let pixel_height = height.ceil() as usize;

   // 2x scale in x direction to counter the aspect ratio of monospace characters.
   let scale = Scale { x: height*2.0, y: height };

   let v_metrics = font.v_metrics(scale);
   let offset = point(0.0, v_metrics.ascent);

   // Glyphs to draw for "RustType". Feel free to try other strings.
   let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, offset).collect();

   // Find the most visually pleasing width to display
   let width = glyphs.iter().rev()
      .filter_map(|g| g.pixel_bounding_box()
                  .map(|b| b.min.x as f32 + g.unpositioned().h_metrics().advance_width))
      .next().unwrap_or(0.0).ceil() as usize;

   let extra = 7;
   let mut pixel_data = vec![0.0; width * pixel_height + extra];

   for g in glyphs {
      if let Some(bb) = g.pixel_bounding_box() {
         g.draw(|x, y, v| {
            let x = (x as i32 + extra as i32 + bb.min.x) as usize;
            let y = (y as i32 + bb.min.y) as usize;
            pixel_data[(y*width + width-x-1)] =  v /* *255.0*/;
         })
      }
   }

   let mut vec_tex: Vec<Vec<u8>> = Vec::with_capacity(pixel_height);
   for y in 0..pixel_height {
      vec_tex.push(Vec::with_capacity(width));
      for x in 0..width {
         vec_tex[y].push((pixel_data[y*width + x] * 255.0) as u8);
      }
   }

   (vec_tex, pixel_height, width)
}


