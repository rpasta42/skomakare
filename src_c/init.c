#include "config.h"
#include "init.h"
#include "gl.h"

/*extern const char *vertex_shader_src;
extern const char *fragment_shader_src;*/
extern GLuint positionBufferObject;
/*float vertexPositions[] = {
	0.75f, 0.75f, 0.0f, 1.0f,
	0.75f, -0.75f, 0.0f, 1.0f,
	-0.75f, -0.75f, 0.0f, 1.0f,
};*/

extern GLuint vao;


const char**
read_shaders(void)
{
   int i = 5;
   const char** srcs = malloc(2 * sizeof(char**));

  read_file("shaders/shader.vert", srcs);
  read_file("shaders/shader.frag", &srcs[1]);

   return srcs;
}

void init_ogl(struct ogl_state *ogl_state)
{
  ogl_state->script_lock = spin_lock_new(0);

  GLenum* types = malloc(sizeof(GLenum)*2);
  types[0] = GL_VERTEX_SHADER;
  types[1] = GL_FRAGMENT_SHADER;

  const char** srcs = read_shaders();

  /*malloc(2 * sizeof(char**));
  srcs[0] = vertex_shader_src;
  srcs[1] = fragment_shader_src;*/

  err_t status =
    init_shaders(ogl_state, 2, types, srcs);

  //offsetLocation = glGetUniformLocation(ogl_state->ogl_program, "offset");

  ///Init Vertex Buffer
  ogl_state->buffs = malloc(sizeof(struct ogl_buffer));
  ogl_state->num_buffs = 1;

  struct ogl_buffer *buff = ogl_state->buffs;

  buff->data =
    read_file_floats("data/floats.file", &buff->size);

  int i;
  /*printf("%i\n", buff->size);
  for (i = 0; i < buff->size; i++)
    printf("%f ", ((float*)(buff->data))[i]);
  printf("\n\n");*/
  buff->size *= sizeof(float);

  glGenBuffers(1, &positionBufferObject);

  glBindBuffer(GL_ARRAY_BUFFER, positionBufferObject);
  glBufferData(GL_ARRAY_BUFFER, buff->size, buff->data, GL_STATIC_DRAW/*DYNAMIC*/);
  glBindBuffer(GL_ARRAY_BUFFER, 0);
  ///end

  glGenVertexArrays(1, &vao);
  glBindVertexArray(vao);

 //glViewport(0, 0, (GLsizei)1000, (GLsizei)1000);
}

#ifdef SKMK_GLUT
void reshape(int w, int h)
{
   glViewport(0, 0, w, h);
}

void keyboard(char key, int x, int y)
{
	switch (key) {
   case 27:
      glutLeaveMainLoop();
      return;
      break;
	}
}
#endif

void init_graphics(int nargs, char** args, const char* title_bar, int x, int y)
{
/*KKCODEBLOCKSSUCKS*/#define SKMK_GLUT
#ifdef SKMK_GLUT
   glutInit(&nargs, args);
   glutInitDisplayMode(GLUT_DOUBLE | GLUT_ALPHA | GLUT_DEPTH | GLUT_STENCIL);
   //kk glutInitContextVersion(3, 3);
   glutInitContextVersion(3, 1);
   glutInitContextProfile(GLUT_CORE_PROFILE);
#ifdef SKMK_DEBUG
   glutInitContextFlags(GLUT_DEBUG);
#endif
   glutInitWindowSize(x, y);
   glutInitWindowPosition(300, 300);
   int window = glutCreateWindow(title_bar);

   ogl_load_funcs();
   glutSetOption(GLUT_ACTION_ON_WINDOW_CLOSE, GLUT_ACTION_CONTINUE_EXECUTION);

   skmk.ogl_state = malloc(sizeof(struct ogl_state));
   init_ogl(skmk.ogl_state); //!!

   glutDisplayFunc(ogl_draw);
   glutReshapeFunc(reshape);
   glutKeyboardFunc(keyboard);

   glutMainLoop();
#elif SKMK_SDL2
   //SDL_GLContext glcontext = SDL_GL_CreateContext(SDL_WINDOW);
   {
   SDL_Init(SDL_INIT_EVERYTHING); //TODO: Check errors
   K_SDL_WINDOW =
      SDL_CreateWindow(title_bar, 0, 0, x, y,
                       SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE);

   SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
   SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 24);
   K_GL_CONTEXT = SDL_GL_CreateContext(K_SDL_WINDOW);
   ogl_init();
   }
#else
   #error You have to either use GLUT or SDL.
#endif
}


