#include "config.h"
#include "init.h"
#include "gl.h"
#include "stdio.h"


/*float vertexPositions[] = {
	0.75f, 0.75f, 0.0f, 1.0f,
	0.75f, -0.75f, 0.0f, 1.0f,
	-0.75f, -0.75f, 0.0f, 1.0f,
};*/

/* KK deleteme
const char *vertex_shader_src =
   //"#version 330\n"
   "#version 140\n"
   "#extension GL_ARB_explicit_attrib_location : require\n"
   "layout(location = 0) in vec4 position;\n"
   //"uniform vec3 offset;\n"
   "void main()\n"
   //"{\gl_Position = position; \n"
   "{gl_Position = position; \n"
   //"   gl_Position = position + vec4(offset.x, offset.y, offset.z, 0);\n"
   "}\n";

const char *fragment_shader_src =
   //"#version 330\n"
   "#version 140\n"
   "out vec4 outputColor;\n"
   "void main()\n"
   "{\n"
   "   outputColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);\n"
   "}\n";
*/

GLuint positionBufferObject;
GLuint vao;
//GLuint offsetLocation;
float offset_x, offset_y, offset_z;

/* max distance is 10,000 */
void g_move(struct ogl_state *ogl_state, int x, int y, int z)
{
   spin_lock_get(ogl_state->script_lock);
   offset_x += (float)x/10000.0;
   offset_y += (float)y/10000.0,
   offset_z += (float)z/10000.0;
   spin_lock_release(ogl_state->script_lock);
}

//use glutSwapBuffers and glutPostRedisplay() to continiously update.
void ogl_draw()
{
	glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
	glClear(GL_COLOR_BUFFER_BIT);

  glEnable(GL_VERTEX_PROGRAM_POINT_SIZE);

	glUseProgram(skmk.ogl_state->ogl_program->program);

	spin_lock_get(skmk.ogl_state->script_lock);
   //glUniform3f(offsetLocation, offset_x, offset_y, offset_z);
  spin_lock_release(skmk.ogl_state->script_lock);

	glBindBuffer(GL_ARRAY_BUFFER, positionBufferObject);
	glEnableVertexAttribArray(0);
	glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3, 0);

	glDrawArrays(/*GL_TRIANGLES*/GL_POINTS, 0, skmk.ogl_state->buffs->size /2 /3 /4); //3);

	glDisableVertexAttribArray(0);
	glUseProgram(0);

  usleep(30000);
	ogl_swap_buffers();
}

void new_shader(struct ogl_shader *ogl_shader,
                const char *src, GLenum type)
{
   ogl_shader->type = type;
   ogl_shader->src = src;

   ogl_shader->shader = glCreateShader(type);

	glShaderSource(ogl_shader->shader, 1, &src, NULL);
	glCompileShader(ogl_shader->shader);

	GLint status;
	glGetShaderiv(ogl_shader->shader, GL_COMPILE_STATUS, &status);
	if (status == GL_FALSE) /* TODO: make a separate file for error crap */
		fprintf(stderr, "bad shader\n");
}

void delete_shader(struct ogl_shader *ogl_shader)
{
   glDeleteShader(ogl_shader->shader);
}

void new_shader_program(struct ogl_program *ogl_program,
                        struct ogl_shader *shaders, uint num_shaders)
{
   GLuint i, program = glCreateProgram();

	for(i = 0; i < num_shaders; i++)
		glAttachShader(program, shaders[i].shader);

	glLinkProgram(program);

	GLint status;
	glGetProgramiv(program, GL_LINK_STATUS, &status);
	if (status == GL_FALSE)
		fprintf(stderr, "can't link shader\n");

	for(i = 0; i < num_shaders; i++)
		glDetachShader(program, shaders[i].shader);

   ogl_program->shaders = shaders;
   ogl_program->num_shaders = num_shaders;
   ogl_program->program = program;
}


/* Note: you have to delete individual shaders yourself, 'cause re-use. */
void delete_shader_program(struct ogl_program *ogl_program)
{
   glDeleteProgram(ogl_program->program);
   free(ogl_program->shaders);
}

err_t
init_shaders(struct ogl_state *ogl_state,
             uint num_shaders, GLenum* shader_types, const char **shader_srcs)
{
   int i;

   struct ogl_shader *shaders = malloc(num_shaders * sizeof(struct ogl_shader));
   for (i = 0; i < num_shaders; i++)
      new_shader(&shaders[i], shader_srcs[i], shader_types[i]);

   ogl_state->ogl_program = malloc(sizeof(struct ogl_program));
   new_shader_program(ogl_state->ogl_program, shaders, num_shaders);

   return 0;
}

void un_init_shaders(struct ogl_state *ogl_state)
{
   uint i;
   struct ogl_program *ogl_program = ogl_state->ogl_program;

   for (i = 0; i < ogl_program->num_shaders; i++)
      delete_shader(&ogl_program->shaders[i]);
   delete_shader_program(ogl_program);
   free(ogl_state->ogl_program);
   ogl_state->ogl_program = NULL;
}

void ogl_swap_buffers()
{
#ifdef SKMK_GLUT
   glutSwapBuffers();
   glutPostRedisplay();
#elif SKMK_SDL2
   SDL_GL_SwapWindows(SDL_WINDOW);
#endif
}


