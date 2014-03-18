/* Notes:
 * -2 main threads: 1 for GLUT, and 1 for Guile REPL.
 * -functions exported to Guile begin with script_*
 * -graphics-related functions begin with g_*
 * -functions for interfacing Skomakare or cpp (preprocessor) begin with skmk_*

 * init_(graphics/gio/etc) or graphics_init ?
 * spin_lock_new() or new_spin_lock?
 abbreviations: curr current, len length, num number
 */
#include "repl.h" /* contains main code for REPL */
#include "init.h"
#include "blend_format.h"
#include <pthread.h>

/* Make a thread for REPL, and launch graphics in main thread. */
int main1(int nargs, char** args)
{
  pthread_t repl_thread;
  int ret_val = pthread_create(&repl_thread, NULL, repl, NULL);

  //ret_val++;
  /* Window which displays OpenGL drawings. */
  init_graphics(nargs, args, "Skomakare", 500, 500);

  pthread_join(repl_thread, NULL);

  return 0;
}






