
pub static VERTEX_SHADER_SRC : &'static str = r#"
   #version 140
   in vec2 pos;
   in vec2 tex_pos;
   out vec2 v_tex_coords;

   uniform mat4 matrix;

   void main() {
      v_tex_coords = tex_pos; 
      gl_Position = matrix * vec4(pos, 0.0, 1.0); }
"#;

pub static FRAGMENT_SHADER_SRC : &'static str = r#"
   #version 140
   in vec2 v_tex_coords;
   out vec4 color;

   uniform sampler2D tex;

   void main() {
      color = texture(tex, v_tex_coords);
   }
"#;


