#ifndef CONFIG_H_INCLUDED
#define CONFIG_H_INCLUDED

//#define SKMK_OPTIMIZE 1
//#define SKMK_EXTRA_CHECKS 1

/* Which API should we use for initializing
 * windows, Open GL, sound?
 * Currently only GLUT.
 * In future maybe SDL1, SDL2, Waffle, GTK, Qt.
 */

//#define SKMK_SDL1 1
//#define SKMK_SDL2 1
#define SKMK_GLUT 1

/* Which API should be use for loading OpenGL functions? */
#define SKMK_GL_LOAD 1

struct ogl_state;

struct skomakare {
   struct ogl_state *ogl_state;
};

struct skomakare skmk;

#endif // CONFIG_H_INCLUDED
