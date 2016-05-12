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

   let (pixels, height, width) = raster_text(&*text, font_path);
   /*let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);
   for (x_, y_, pixel) in imgbuf.enumerate_pixels_mut() {
      //let x = x_ as usize;
      //let y = y_ as f32;
      *pixel = image::Luma([(pixels[(x_ * y_ + y_) as usize]*256.0) as u8]);
   }
   let image = image::ImageLuma8(imgbuf).to_rgb();
   let image_d = image.dimensions();
   println!("dimenx: {} dimeny: {}", image_d.0, image_d.1);
   println!("width: {}, height: {}", width, height);
   let raw_image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_d);
   Texture2d::new(display, raw_image).unwrap()*/
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
fn raster_text(text : &str, font_path : &str) //-> (Vec<f32>, usize, usize)
-> (Vec<Vec<u8>>, usize, usize)
{
   use rusttype::{FontCollection, Scale, point, PositionedGlyph};
   use std::io::Write;

   let mut font_data = Vec::new();
   read_bin_file(font_path, &mut font_data);
   let collection = FontCollection::from_bytes(&font_data as &[u8]);
   let font = collection.into_font().unwrap();

   //let height: f32 = 12.4; // to get 80 chars across (fits most terminals); adjust as desired
   //kk version
   let height: f32 = 500.0; // to get 80 chars across (fits most terminals); adjust as desired

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

   //println!("width: {}, height: {}", width, pixel_height);

   //KK in opengl, 0 to 1 color
   // Rasterise directly into ASCII art.
   let mut pixel_data = vec![0.0; width * pixel_height];
   //let mut pixel_data = vec![b'@'; width * pixel_height];

   //let mapping = b"0123456789"; // The approximation of greyscale
   //let mapping = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];//b"\0\1\2\3\4\5\6\7\8\9";
   let mapping = &float_range(0.0, 0.01, 1.0);
   let mapping_scale = (mapping.len()-1) as f32;
   for g in glyphs {
      if let Some(bb) = g.pixel_bounding_box() {
         g.draw(|x, y, v| {
            // v should be in the range 0.0 to 1.0
            let i = (v*mapping_scale + 0.5) as usize;
            // so something's wrong if you get $ in the output.
            let c = mapping.get(i).cloned().unwrap_or(0.0);//(b'$');
            let x = x as i32 + bb.min.x;
            let y = y as i32 + bb.min.y;
            // There's still a possibility that the glyph clips the boundaries of the bitmap
            if x >= 0 && x < width as i32 && y >= 0 && y < pixel_height as i32 {
               let x = x as usize;
               let y = y as usize;
               //pixel_data[(x + y * width)] = v; //c;
               //pixel_data[(x + y * width)] = v + 0.001; //c;
               /*println!("{}", c);
               let n = match c as char {
                  '0' => 0.0, '1' => 0.1, '2' => 0.2,
                  '3' => 0.3, '4' => 0.4, '5' => 0.5,
                  '6' => 0.6, '7' => 0.7, '8' => 0.8, '9' => 0.9,
                  _ => 1.0
               };*/

               //pixel_data[(y * width + width - x - 1)] = v + 0.00; // c
               //pixel_data[(y * width + width - x - 1)] = (i as f32/200.0 - 0.1) * 256.0; //n * 256.0;
               //pixel_data[(y*width + width-x-1)] = c * 256.0; //n * 256.0;
               let z = if v < 0.0 { 0.0 } else if v > 0.9 { 0.99 } else { v };

               pixel_data[(y*width + width-x-1)] =  z * 255.0; //n * 256.0;

               //println!("x: {}, y: {}, v: {}", x, y, v);

            }
         })
      }
   }

   // Print it out
   /*let stdout = ::std::io::stdout();
   let mut handle = stdout.lock();
   for j in 0..pixel_height {
      handle.write(&pixel_data[j*width..(j+1)*width]).unwrap();
      handle.write(b"\n").unwrap();
   }*/
   let mut vec_tex: Vec<Vec<u8>> = Vec::with_capacity(pixel_height);
   for y in 0..pixel_height {
      vec_tex.push(Vec::with_capacity(width));
      for x in 0..width {
         vec_tex[y].push((pixel_data[y*width + x] * 256.0) as u8);
      }
   }

   //(pixel_data, pixel_height, width)
   (vec_tex, pixel_height, width)
}


