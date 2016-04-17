pub fn char_at(s : &str, n : usize) -> Option<char> {
   for (i, c) in s.chars().enumerate() {
      if i == n { return Some(c) }
   } return None
}

pub fn s_to_f(s : &str) -> f32 {
   s.parse::<f32>().unwrap()
}

/*fn display_file(path : &str) {
   let data = read_file(path).unwrap();
   let m = parse_obj(&data);
   m.print();
   draw(&m);
}*/


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
            Err(why) => { panic!("{}", Error::description(&why)); None }
         }
      }
      Err(why) => { panic!("{}", Error::description(&why)); None }
   }
}
