#ifndef GL_H_INCLUDED
#define GL_H_INCLUDED

#include "init.h"
#include "utils.h"

struct ogl_buffer {
   spin_lock* sl;
   void* data; uint size;
};

struct ogl_shader {
   GLuint shader;
   GLenum type;

   const char* src;
};

struct ogl_program {
   GLuint program;

   struct ogl_shader *shaders;
   uint num_shaders;
};

struct ogl_state {
  spin_lock *script_lock;

  struct ogl_program *ogl_program;

  struct ogl_buffer *buffs;
  uint num_buffs;
};


void ogl_draw();

err_t
init_shaders(struct ogl_state *ogl_state,
             uint num_shaders, GLenum* shader_types, const char **shader_srcs);


void ogl_swap_buffers();


#endif // GL_H_INCLUDE
