/* Boring code for initializing OpenGL, SDL, GLUT, etc. */

#ifndef INIT_H_INCLUDED
#define INIT_H_INCLUDED

#include "config.h"

#ifdef SKMK_GL_LOAD
   #include "../external/glload/gl_3_3_comp.h"
   //#include "../external/glload/gl_3_3.h" //or 3.2
   #include "../external/glload/gll.h"
#endif

#ifdef SKMK_GLUT
   #include <GL/freeglut.h>
#elif SKMK_SDL2
   #include <SDL2/SDL.h>
#endif

void init_graphics(int nargs, char** args, const char* title_bar, int x, int y);

static inline void ogl_load_funcs() {
#ifdef SKMK_GL_LOAD
   LoadFunctions(); /* C version */
#endif
}

#endif // INIT_H_INCLUDED
