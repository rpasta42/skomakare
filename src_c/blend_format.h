/* Code for loading Blender models (http://www.blender.org/).
 * Thanks to Jeroen Bakker for writing http://www.atmind.nl/blender/mystery_ot_blend.html
 * TODO: SORTING!
 */
#ifndef BLEND_FORMAT_H_INCLUDED
#define BLEND_FORMAT_H_INCLUDED

#include "object.h"
#include <stdint.h>

/* Cast to either Skmk_model or Skmk_file. */
Skmk_object* load_blend_file(const char* file_path);


/* TODO: convert ptrs and simple_types to be
 * the same way as num_dimensions.
 */
struct b_misc_info {
  int* ptrs; /* name indices which contain * (STAR) */
  int num_ptrs;

  /* index in sdna->types of fundamental types such as int, float, char, etc. */
  int* simple_types;
  int num_simple_types;

  /* for each sdna->names; scalars are 0-D */
  int*  num_dimensions; /* e.g. "arr[6][100][23]" = 3 */
  int** arr_dimensions; /* each dimension, e.g. "arr[324][6][10]" = {324, 6, 10} */
};

/* Blend file */
struct b_file {
  /* Instead of passing various pointers, iterator it is often passed to functions */
  char *raw_data_ptr, *it;

  /* file header */
  char      id[7];      /* id = "BLENDER" */
  char      ptr_size;   /* '_' means 4 bytes; '-' means 8 bytes */
  char      endianness; /* 'v' means little; 'V' means big */
  char      version[3]; /* blender version in which file was created; something like "268" */
  /* end file header */

  struct b_file_block *blocks;
  uint num_blocks;
  struct b_sdna *sdna;
  struct b_misc_info misc_info;
};

struct b_file_block {
  char      name[4];
  int       size;
  void*     old_ptr;
  int       sdna_index;
  int       count_structs;
  char*     data;
};

struct b_sdna {
  char      id[4];        /* "SDNA" */
  char      name_id[4];   /* "NAME" */
  int       num_names;
  char**    names;
  uint8_t*  each_name_len;

  char      type_id[4];   /* "TYPE" */
  int       num_types;
  char**    types;
  uint8_t*  each_type_len; /* type string len (not type itself) */

  /* type len in bytes */
  char tlen_id[4];        /* "TLEN" */
  short* tlens;

  /* structures info */
  char strc_id[4];        /* "STRC" */
  int num_strcs;
  short*  struct_type_in;
  short*  struct_num_fields;
  short** field_type_in;
  short** field_name_in;
};

#endif // BLEND_FORMAT_H_INCLUDED



