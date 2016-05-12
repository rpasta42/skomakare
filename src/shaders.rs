use std::collections::HashMap;
use types::*;
use glium::Program;

pub struct ShaderManager {
   pub shaders : HashMap<String, Program>,
}
impl ShaderManager {
   pub fn new() -> ShaderManager {
      ShaderManager {
         shaders : HashMap::new()
      }
   }
   pub fn add_defaults(&mut self, display : &Display) {
      self.add_shader(display, "texture".to_string(),
                      VERT_SH_TEXTURE, FRAG_SH_TEXTURE);

      self.add_shader(display, "text".to_string(),
                      VERT_SH_TEXTURE, FRAG_SH_TEXT);

      //self.add_shader(display, "color-red".to_string(), VERT_SH_COLOR, FRAG_SH_COLOR_R);
   }

   pub fn add_shader(&mut self, display : &Display, name : String,
                     vert_sh_src : &str, frag_sh_src : &str)
   {
      let p = Program::from_source(display, vert_sh_src, frag_sh_src, None);
      self.shaders.insert(name, p.unwrap());
   }
}

pub static VERT_SH_COLOR : &'static str = r#"
   #version 140
   in vec2 pos;
   in vec2 tex_pos;
   out vec2 v_tex_coords;

   uniform mat4 matrix;

   void main() {
      v_tex_coords = tex_pos;
      gl_Position = matrix * vec4(pos, 0.0, 1.0);
   }
"#;

pub fn frag_shader_color(color : Color) -> String {
   let src = format!(r#"
      #version 140
      in vec2 v_tex_coords;
      out vec4 color;

      void main() {{
         color = vec4({}, {}, {}, {});
      }}
   "#, color.0, color.1, color.2, color.3);
   src
}

pub static VERT_SH_TEXTURE : &'static str = r#"
   #version 140
   in vec2 pos;
   in vec2 tex_pos;
   out vec2 v_tex_coords;

   uniform mat4 matrix;

   void main() {
      v_tex_coords = tex_pos;
      gl_Position = matrix * vec4(pos, 0.0, 1.0);
   }
"#;

pub static FRAG_SH_TEXTURE : &'static str = r#"
   #version 140
   in vec2 v_tex_coords;
   out vec4 color;

   uniform sampler2D tex;

   void main() {
      color = texture(tex, v_tex_coords);
   }
"#;

pub static FRAG_SH_TEXT : &'static str = r#"
   #version 140
   in vec2 v_tex_coords;
   out vec4 color;

   uniform sampler2D tex;

   void main() {
      //color = texture(tex, v_tex_coords);
      vec4 c = texture(tex, v_tex_coords);
      if (c[0] > 0.5)
         color = vec4(1, 1, 1, 1);
      else
         color = vec4(0, 0, 0, 0);
   }
"#;

