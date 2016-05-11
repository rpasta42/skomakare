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

//https://botbot.me/mozilla/rust-gamedev/2015-07-26/?page=1
//https://tomaka.github.io/glium/glium/texture/struct.RawImage2d.html
//https://tomaka.github.io/glium/glium/texture/trait.Texture2dDataSource.html
//http://www.piston.rs/image/image/enum.DynamicImage.html
//http://www.piston.rs/image/image/enum.DynamicImage.html
//https://github.com/PistonDevelopers/image
pub fn text_to_texture(text : String, display : &Display) -> Texture2d
{
   let font_path = "/usr/share/fonts/truetype/msttcorefonts/comic.ttf";
   /*let (pixels, height, width) = raster_text(&*text, font_path);

   let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);
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
   Texture2d::new(display, raw_image).unwrap()
   //RawImage2d::from_raw(pixels, height, width);*/
   let pixels = raster_text(&*text, font_path);
   Texture2d::new(display, pixels).unwrap()
}

// (pixel_data, pixel_height, width)
fn raster_text(text : &str, font_path : &str) -> Vec<Vec<u8>>
//-> (Vec<f32>, usize, usize)
{
   use rusttype::{FontCollection, Scale, point, PositionedGlyph};
   use std::io::Write;

   let mut font_data = Vec::new();
   read_bin_file(font_path, &mut font_data);
   let collection = FontCollection::from_bytes(&font_data as &[u8]);
   let font = collection.into_font().unwrap();

   let height: f32 = 12.4; // to get 80 chars across (fits most terminals); adjust as desired
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

   println!("width: {}, height: {}", width, pixel_height);

   //KK in opengl, 0 to 1 color
   // Rasterise directly into ASCII art.
   let mut pixel_data = vec![0.5; width * pixel_height];
   //let mut pixel_data = vec![b'@'; width * pixel_height];

   let mapping = b"@%#x+=:-. "; // The approximation of greyscale
   let mapping_scale = (mapping.len()-1) as f32;
   for g in glyphs {
      /*if let Some(bb) = g.pixel_bounding_box() {
         g.draw(|x, y, v| {
            // v should be in the range 0.0 to 1.0
            let i = (v*mapping_scale + 0.5) as usize;
            // so something's wrong if you get $ in the output.
            let c = mapping.get(i).cloned().unwrap_or(b'$');
            let x = x as i32 + bb.min.x;
            let y = y as i32 + bb.min.y;
            // There's still a possibility that the glyph clips the boundaries of the bitmap
            if x >= 0 && x < width as i32 && y >= 0 && y < pixel_height as i32 {
               let x = x as usize;
               let y = y as usize;
               //pixel_data[(x + y * width)] = v; //c;
               pixel_data[(x + y * width)] = v + 0.01; //c;
               println!("x: {}, y: {}, v: {}", x, y, v);

            }
         })
      }*/
      let bitmap = g.bitmap();
      let width = bitmap.width() as usize;
      let height = bitmap.rows() as usize();
      let data = bitmap.buffer();
      let mut vec_tex : Vec<Vec<u8>> = Vec::with_capacity(height); //TODO: Convert &[u8] into a Vec<Vec<u8>>
      for y in 0..height {
         vec_tex.push(Vec::with_capacity(width));
         for x in 0..width {
            vec_tex[y].push(data[y*width + x]);
         }
      }
      return vec_tex;
   }

   // Print it out
   /*let stdout = ::std::io::stdout();
   let mut handle = stdout.lock();
   for j in 0..pixel_height {
      handle.write(&pixel_data[j*width..(j+1)*width]).unwrap();
      handle.write(b"\n").unwrap();
   }*/
   (pixel_data, pixel_height, width)
}


