#include "gio.h"
#include "utils.h"
//#include <stdio.h>
//#include <stdlib.h>
#include <assert.h>
#include <libguile.h>

/* A small library for converting raw memory to guile SCM.
 * Pointers are exchanged between Guile and C using uint64.
 *
 * Functions prefixed with "script_" will be exported to Guile (with prefix "c/").
 * Arguments prefixed with "s_" have boxed SCM type, and need to be converted to raw C type before usage.
 *
 * TODO: floats, error handling, GOOPS
 *
 * I was gonna do this the "correct" way, (without re-inventing the wheel) but I'd
 * rather write a new scheme implementation than try to read guile documentation again...
 * http://www.gnu.org/software/guile/manual/html_node/Void-Pointers-and-Byte-Access.html#Void-Pointers-and-Byte-Access
 * http://www.gnu.org/software/guile/manual/html_node/Foreign-Pointers.html#Foreign-Pointers
 * http://www.gnu.org/software/guile/manual/html_node/Bytevectors-as-Strings.html#Bytevectors-as-Strings
 */

/* Allocate s_num_bytes; read data from port; return pointer to data read.
 * TODO: proper error handling, return bytes read (?) */
static SCM
script_read(SCM port, SCM s_num_bytes)
{
   uint num_bytes = scm_to_uint32(s_num_bytes);

   void* buffer = malloc(num_bytes);
   uint num_read = scm_c_read(port, buffer, num_bytes);

   assert(num_read == num_bytes);

   return scm_from_uint64(buffer);
   //return scm_cons(s_num_bytes, scm_from_uint64(buffer));
}

/* Convert s_ptr to signed integer of size s_size. */
static SCM
script_ptr_to_int(SCM s_ptr, SCM s_size)
{
   void* ptr = scm_to_uint64(s_ptr);
   int size = scm_to_int32(s_size);

   switch (size) {
   case 1:
      return scm_from_int8(*(int8_t*)ptr); break;
   case 2:
      return scm_from_int16(*(int16_t*)ptr); break;
   case 4:
      return scm_from_int32(*(int32_t*)ptr); break;
   case 8:
      return scm_from_int64(*(int64_t*)ptr); break;
   default:
      return scm_from_int32(*(int32_t*)ptr); break; //exit(42); //TODO
   break;
   }
}

/* Same as above, but for unsigned ints. */
static SCM
script_ptr_to_uint(SCM s_ptr, SCM s_size)
{
   void* ptr = scm_to_uint64(s_ptr);
   int size = scm_to_int32(s_size);

   switch (size) {
   case 1:
      return scm_from_uint8(*(int8_t*)ptr); break;
   case 2:
      return scm_from_uint16(*(int16_t*)ptr); break;
   case 4:
      return scm_from_uint32(*(int32_t*)ptr); break;
   case 8:
      return scm_from_uint64(*(int64_t*)ptr); break;
   default:
      return scm_from_uint32(*(int32_t*)ptr); break; //exit(42); //TODO
   break;
   }
}

/* Same as above, but for floats. */
static SCM
script_ptr_to_float(SCM s_ptr, SCM s_size)
{
   void* ptr = scm_to_uint64(s_ptr);
   int size = scm_to_int32(s_size);

   switch (size) {
   default:
      return scm_from_double(*(float*)ptr); break; //exit(42); //TODO
   break;
   }
}

/* Convert s_ptr to string of length s_len.
 * If s_len is zero, assume string is null terminated.
 * s_len is optional argument. */
static SCM
script_ptr_to_str(SCM s_ptr, SCM s_len)
{
   void* ptr = scm_to_uint64(s_ptr);
   char* p = ptr; //DELETEME

   /* check if we received s_len */
   uint len = SCM_UNBNDP(s_len) ? strlen(ptr)+1 : scm_to_int32(s_len);

   if (len == 0)
      len = strlen(ptr)+1;

   SCM ret = scm_from_locale_stringn(ptr, len);
   //return (no_len) ? scm_cons(scm_from_int32(len), ret) : ret;
   //SCM ret = scm_from_locale_stringn("hello", 4);
   return ret;
}

static SCM
script_ptr_to_char(SCM s_ptr, SCM s_TODO_remove_me)
{
   void* ptr = scm_to_uint64(s_ptr);
   return scm_from_char(*(char*)ptr);
}


static SCM
script_str_to_ptr(SCM s_str, SCM s_size)
{
  uint len = scm_to_uint32(s_size);
  return scm_from_uint64(scm_to_locale_stringn(s_str, &len));
}

static SCM
script_uint32_to_uint64(SCM s_num)
{
  return scm_from_uint64(scm_to_uint32(s_num));
}

static SCM
script_uint64_to_uint64(SCM s_num)
{
  return scm_from_uint64(scm_to_uint64(s_num));
}

static SCM
script_file_to_ptr(SCM s_path)
{
  const char *path = scm_to_locale_string(s_path);

  char* buff = NULL;
  read_file(path, &buff);

  return scm_from_uint64(buff);
}

//(do ((i 0 (+ i 1)) (s $1 (cdr s))) ((string=? (car s) "Scene\x00")) (p i " "))
//(do ((name init (next))) ((term)) (action))

/* initialize gio so guile will see it */
void init_gio(void)
{
   scm_c_define_gsubr("c/read-port-to-ptr",  2, 0, 0, script_read);
   scm_c_define_gsubr("c/ptr->int",          2, 0, 0, script_ptr_to_int);
   scm_c_define_gsubr("c/ptr->uint",         2, 0, 0, script_ptr_to_uint);
   scm_c_define_gsubr("c/ptr->float",        2, 0, 0, script_ptr_to_float);

   scm_c_define_gsubr("c/ptr->str",          1, 1, 0, script_ptr_to_str);
   scm_c_define_gsubr("c/ptr->char",         2, 0, 0, script_ptr_to_char);

   scm_c_define_gsubr("c/str->ptr",          2, 0, 0, script_str_to_ptr);
   scm_c_define_gsubr("c/file",              1, 0, 0, script_file_to_ptr);

   scm_c_define_gsubr("u32->u64",          1, 0, 0, script_uint32_to_uint64);
   scm_c_define_gsubr("u64->u64",          1, 0, 0, script_uint64_to_uint64);

   scm_c_eval_string("(load \"src_scm/gio.scm\")");
}


