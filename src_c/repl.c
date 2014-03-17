#include "repl.h"
#include "gl.h"
#include "gio.h"
#include "utils.h"
#include "ogre.h"
#include <libguile.h>
#include <readline/readline.h>

/* Wrappers */

/* Move 3D object */
void script_move(SCM object, SCM x, SCM y, SCM z)
{
   g_move(scm_to_int32(x),
          scm_to_int32(y),
          scm_to_int32(z));
}

uint mylen(char* s)
{
  uint len = 0;
  for (; s[len] != '\n'; len++) ;
  return len;
}

SCM script_ogre_get_scene_mgr()
{
  uint64_t ret = ogre_get_scene_mgr();
  return scm_from_uint64(ret);
}
SCM script_ogre_get_root_scene_node()
{
  uint64_t ret = ogre_get_root_scene_node();
  return scm_from_uint64(ret);
}
SCM script_ogre_create_light(SCM s_name)
{
  const char *name = scm_to_locale_string(s_name);
  uint64_t ret = ogre_create_light(name);
  return scm_from_uint64(ret);
}
SCM script_ogre_create_entity(SCM s_name, SCM s_mesh_path)
{
  const char *name      = scm_to_locale_string(s_name),
             *mesh_path = scm_to_locale_string(s_mesh_path);

  uint64_t ret = ogre_create_entity(name, mesh_path);
  return scm_from_uint64(ret);
}
SCM script_ogre_create_child_scene_node(SCM s_parent, SCM s_name)
{
  void *parent = (void*)scm_to_uint64(s_parent);
  const char *name = scm_to_locale_string(s_name);

  uint64_t ret = ogre_create_child_scene_node(parent, name);
  return scm_from_uint64(ret);
}
SCM script_ogre_attach_object(SCM s_parent, SCM s_child)
{
  void *parent = (void*)scm_to_uint64(s_parent),
       *child  = (void*)scm_to_uint64(s_child);

  ogre_attach_object(parent, child);
  return scm_from_int32(1);
}
SCM script_ogre_set_position(SCM s_obj, SCM s_x, SCM s_y, SCM s_z)
{
  void *obj = (void*)scm_to_uint64(s_obj);

  float x = scm_to_double(s_x), y = scm_to_double(s_y), z = scm_to_double(s_z);
  ogre_set_position(obj, x, y, z);
  return scm_from_int32(1);
}


static void
start_guile(void* closure, int argc, char **argv)
{
  scm_c_define_gsubr("get-scene-mgr",           0, 0, 0, script_ogre_get_scene_mgr);
  scm_c_define_gsubr("get-root-scene-node",     0, 0, 0, script_ogre_get_root_scene_node);
  scm_c_define_gsubr("create-light",            1, 0, 0, script_ogre_create_light);
  scm_c_define_gsubr("create-entity",           2, 0, 0, script_ogre_create_entity);
  scm_c_define_gsubr("create-child-scene-node", 2, 0, 0, script_ogre_create_child_scene_node);
  scm_c_define_gsubr("attach-object",           2, 0, 0, script_ogre_attach_object);
  scm_c_define_gsubr("set-position",            4, 0, 0, script_ogre_set_position);

   //scm_c_eval_string("(debug-set! stack 0)");
   //scm_c_eval_string("(use-modules (srfi srfi-1))");
   scm_c_define_gsubr("move", 4, 0, 0, script_move);

   init_gio();

   scm_c_eval_string("(load \"src_scm/db.scm\")");
   scm_c_eval_string("(load \"src_scm/graphics.scm\")");
   //scm_c_eval_string("(load \"src_scm/blend_format.scm\")");
   scm_c_eval_string("(load \"src_scm/ogre.scm\")");

  printf("\nSkomakare game engine...");
   /* TODO: replace this using scm_c_eval_string(). */
  /*while (1) {
    char* input = readline("\n$ ");
    //uint len = mylen(input);
    //printf("%i\n", len);
    //input[len-1] = '\0';
    //input = realloc(input, mylen(input) + 1);

    if (0 == strcmp(input, "q"))
      break;
    printf("%s", input);

    scm_c_eval_string(input));
    //printf("%s", scm_to_locale_string(scm_c_eval_string(input)));
  }*/
   scm_shell(argc, argv);
}

//scm_boot_guile(nargs, args, inner_main, 0);
void* repl(void* data) { scm_boot_guile(0, NULL, start_guile, 0); }









