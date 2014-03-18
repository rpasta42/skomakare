/* Code for loading Blender models (http://www.blender.org/).
 * Thanks to Jeroen Bakker for writing http://www.atmind.nl/blender/mystery_ot_blend.html
 * TODO: SORTING!
 */
#include "utils.h"
#include "blend_format.h"
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

void find_simple_types(struct b_file *b_file);
void find_ptrs(struct b_file *b_file);
void b_parse_file_blocks(struct b_file* b_file);
void find_name_arrs(struct b_file* b_file);

int debug_kk;
FILE* sexps_parsed_data_file;

#define skmk_align(addr, boundary) (((long)addr % boundary) + (long)addr)
static void b_read_file_header(struct b_file *b_file)
{
  debug_kk = 0;
  strncpy(b_file->id, b_file->it, 7);
  b_file->it += 7;
  b_file->ptr_size = *b_file->it;
  b_file->it += 1;
  b_file->endianness = *b_file->it;
  b_file->it += 1;
  strncpy(b_file->version, b_file->it, 3);
  b_file->it += 3;

  if (b_file->ptr_size == '_')
    b_file->ptr_size = 4;
  else if (b_file->ptr_size == '-')
    b_file->ptr_size = 8;
  else fprintf(stderr, "incorrect pointer size in blend file: %c\n", b_file->ptr_size);

}
static void b_print_file_header(struct b_file *b_file)
{
  printf("%.7s\n", b_file->id);
  printf("%c\n", b_file->ptr_size);
  printf("%c\n", b_file->endianness);
  printf("%.3s\n", b_file->version);
}
static void b_read_file_blocks(struct b_file *b_file)
{
  uint curr_block = 0, len_blocks = 30;
  struct b_file_block *blocks = malloc(len_blocks * sizeof(struct b_file_block));

  for (;; curr_block++) {
    if (curr_block >= len_blocks) {
      len_blocks *= 2;
      blocks = realloc(blocks, len_blocks * sizeof(struct b_file_block));
    }

    strncpy(blocks[curr_block].name, b_file->it, 4);
    //TODO DELETEME printf("\n%.4s", blocks[curr_block].name);
    b_file->it += 4;
    blocks[curr_block].size = *(int*)b_file->it;
    b_file->it += 4;
    blocks[curr_block].old_ptr = *(void**)b_file->it;
    b_file->it += b_file->ptr_size;
    blocks[curr_block].sdna_index = *(int*)b_file->it;
    b_file->it += 4;
    blocks[curr_block].count_structs = *(int*)b_file->it;
    b_file->it += 4;
    blocks[curr_block].data = b_file->it;
    b_file->it += blocks[curr_block].size;
    //printf("%p ", (void*)blocks[curr_block].data);
    if (0 == strncmp(blocks[curr_block].name, "ENDB", 4)) {
      len_blocks = curr_block+1;
      blocks = realloc(blocks, len_blocks * sizeof(struct b_file_block));
      b_file->num_blocks = len_blocks;
      b_file->blocks = blocks;
      return;
    }
  }
}
static void b_print_file_blocks(struct b_file *b_file)
{
  printf("\n***blocks***\nnumber of blocks: %i\n\n", b_file->num_blocks);
}
static void b_parse_sdna(struct b_file *b_file)
{
  int i, j;
  struct b_sdna *dna = malloc(sizeof(struct b_sdna));
  char *ptr = b_file->it;

  strncpy(dna->id, ptr, 4);
  assert(0 == strncmp(dna->id, "SDNA", 4));
  ptr += 4;
  strncpy(dna->name_id, ptr, 4);
  assert(0 == strncmp(dna->name_id, "NAME", 4));
  ptr += 4;
  dna->num_names = *(int*)ptr;
  ptr += 4;

  dna->names = malloc(dna->num_names * sizeof(char*));
  dna->each_name_len = malloc(dna->num_names);
  for (i = 0; i < dna->num_names; i++) {
    dna->names[i] = ptr;
    dna->each_name_len[i] = strlen(dna->names[i]);
    ptr += dna->each_name_len[i]+1;
  }
  //ptr = skmk_align(ptr, 2);
  ptr = skmk_align(ptr, 4);
  strncpy(dna->type_id, ptr, 4);
  assert(0 == strncmp(dna->type_id, "TYPE", 4));
  ptr += 4;
  dna->num_types = *(int*)ptr;
  ptr += 4;

  dna->types = malloc(dna->num_types * sizeof(char*));
  dna->each_type_len = malloc(dna->num_types);
  for (i = 0; i < dna->num_types; i++) {
    dna->types[i] = ptr;
    dna->each_type_len[i] = strlen(dna->types[i]);
    ptr += dna->each_type_len[i]+1;
  }

  ptr = skmk_align(ptr, 4);
  strncpy(dna->tlen_id, ptr, 4);
  //printf("%.4s", dna->tlen_id);
  //assert(0 == strncmp(dna->tlen_id, "TLEN", 4));
  ptr += 4;

  dna->tlens = malloc(dna->num_types * sizeof(short));
  for (i = 0; i < dna->num_types; i++) {
    dna->tlens[i] = *(short*)ptr;
    ptr += 2;
  }

  /************************************/
  ptr = skmk_align(ptr, 4);
  strncpy(dna->strc_id, ptr, 4);
  //assert(0 == strncmp(dna->strc_id, "STRC", 4));
  //printf("%.4s", dna->strc_id);

  ptr += 4;
  dna->num_strcs = *(int*)ptr;
  ptr += 4;

  dna->struct_type_in     = malloc(dna->num_strcs * 2);
  dna->struct_num_fields  = malloc(dna->num_strcs * 2);
  dna->field_type_in      = malloc(dna->num_strcs * sizeof(short*));
  dna->field_name_in      = malloc(dna->num_strcs * sizeof(short*));

  for (i = 0; i < dna->num_strcs; i++) {
    dna->struct_type_in[i]    = *(short*)ptr;
    ptr += 2;
    dna->struct_num_fields[i] = *(short*)ptr;
    ptr += 2;

    dna->field_type_in[i] = malloc(dna->struct_num_fields[i] * 2);
    dna->field_name_in[i] = malloc(dna->struct_num_fields[i] * 2);

    for (j = 0; j < dna->struct_num_fields[i]; j++) {
      dna->field_type_in[i][j] = *(short*)ptr;
      ptr += 2;
      dna->field_name_in[i][j] = *(short*)ptr;
      ptr += 2;
    }
  }

  b_file->it = ptr;
  b_file->sdna = dna;
}
static void b_print_sdna(struct b_file *b_file)
{
  printf("\n***SDNA***\n");
  printf("#names %u\t#types %u\n", b_file->sdna->num_names, b_file->sdna->num_types);
}

/* Writes the b_file's SDNA to a file called
 * sexps_sdna.lisp, as a human-readable, LISP-style sexps.
 * TODO: convert to string and add array dimensions + ptr info.
 */
void b_write_sdna(struct b_file *b_file)
{
  int i, j;
  struct b_sdna *sdna = b_file->sdna;
  FILE* f = open_file_for_write("data/sexps_sdna.lisp");

  fprintf(f, "(");
  for (i = 0; i < sdna->num_strcs; i++) {
      fprintf(f, "\n\n(%s", sdna->types[sdna->struct_type_in[i]]);
      for (j = 0; j < sdna->struct_num_fields[i]; j++)
        fprintf(f, "\n  (%s %s)",
                sdna->types[sdna->field_type_in[i][j]],
                sdna->names[sdna->field_name_in[i][j]]);
      //fprintf(f, "\n) ;end \"%s\"", sdna->types[sdna->struct_type_in[i]]);
      fprintf(f, ")");
  }
  fprintf(f, ")");
}

char* get_name_by_index(struct b_sdna *sdna, int index)
{ /* assert(index < num_names) */
  return sdna->names[index];
}
int get_index_by_name(struct b_sdna *sdna, char *name)
{
  int i;
  for (i = 0; i < sdna->num_names; i++) {
    if (0 == strcmp(name, sdna->names[i]))
      return i;
  }
  return -1;
}
char* get_type_by_index(struct b_sdna *sdna, int index)
{ /* assert(index < num_names) */
  return sdna->types[index];
}
int get_index_by_type(struct b_sdna *sdna, char *name)
{
  int i;
  for (i = 0; i < sdna->num_types; i++) {
    if (0 == strcmp(name, sdna->types[i]))
      return i;
  }
  exit(43);
}


Skmk_object* load_blend_file(const char* file_path)
{
  debug_kk = 0;
  sexps_parsed_data_file = open_file_for_write("data/sexps_blend_data.lisp");

  int i, j;
  struct b_file      *b_file    = malloc(sizeof(struct b_file));

  b_file->raw_data_ptr = NULL;
  read_file(file_path, &b_file->raw_data_ptr);
  b_file->it = b_file->raw_data_ptr;

  b_read_file_header(b_file);
  b_print_file_header(b_file);

  b_read_file_blocks(b_file);

  b_print_file_blocks(b_file);

  for (i = 0; i < b_file->num_blocks; i++) {
    if (0 == strncmp(b_file->blocks[i].name, "DNA1", 4)) {
      b_file->it = b_file->blocks[i].data;
      break;
    }
  }
  b_parse_sdna(b_file);
  b_print_sdna(b_file);
  b_write_sdna(b_file);

  find_name_arrs(b_file);
  find_ptrs(b_file);

  b_parse_file_blocks(b_file);

  struct b_sdna      *sdna      =  b_file->sdna;
  struct b_misc_info *misc_info = &b_file->misc_info;

  /**/for (i = 0; i < b_file->num_blocks; i++) {
    int sdna_in = b_file->blocks[i].sdna_index;
    printf("%i %.4s\t%s\n", i, b_file->blocks[i].name,
           sdna->types[sdna->struct_type_in[sdna_in]]);
  }/**/

  FILE* sexps_blocks = open_file_for_write("data/sexps_blocks.lisp");
  fprintf(sexps_blocks, "(\n");
  for (i = 0; i < b_file->num_blocks; i++) {
      struct b_file_block *blk = &(b_file->blocks[i]);
      fprintf(sexps_blocks, "(%.4s %i %p %i %i)\n",
              blk->name, blk->size, blk->old_ptr,
              blk->sdna_index, blk->count_structs);
  }
  fprintf(sexps_blocks, "\n)\n");

  return NULL;
}

#if 0
void b_parse_type1(struct b_file* b_file, int tabbing, short* ptrs,
                  int sdna_in, const char* name, int name_in)
{
  int i, j;
  struct b_sdna *sdna = b_file->sdna;
  CHECK(debug_kk, "infinite loop?");
  if (sdna_in == 1)
    goto here;

  int type_in         = sdna->struct_type_in[sdna_in];
  char* type_str      = get_type_by_index(sdna, type_in);
  int num_fields      = sdna->struct_num_fields[sdna_in];
  short* field_types  = sdna->field_type_in[sdna_in];
  short* field_names  = sdna->field_name_in[sdna_in];

  bool is_ptr = false;
  if (name_in != -1) {
    for (i = 1; i < b_file->misc_info.num_ptrs; i++) {
      if (name_in == b_file->misc_info.ptrs[i]) is_ptr = true;
    }
  }
  bool  is_simple = false;
  for (i = 0; i < b_file->misc_info.num_simple_types; i++) {
    if (type_in == b_file->misc_info.simple_types[i]) is_simple = true;
  }

  const char *str = malloc(1000);
  //TODO: free
here:;
  //printf("\n%.4s\t(%i) %s", blk->name, i, type_str);
  if (is_simple || is_ptr || sdna_in == -1) {
    sprintf(str, "(%s %s)", type_str, name);
    tab("\n", str, tabbing);
    free(str);
    return;
  }

  sprintf(str, "(%s %s", type_str, name);
  tab("\n", str, tabbing++);
  //free(str);

  for (i = 0; i < num_fields; i++) {
    /*if (field_types[i] == type_in)
      sprintf(str, "(%s %s)", type_str, name);
      tab("\n", str, tabbing);
      free(str);
      continue;
    }*/
    int field_sdna_in = -1;
    for (j = 0; j < sdna->num_strcs; j++) {
      if (sdna->struct_type_in[j] == field_types[i])
        field_sdna_in = j;
    }
    b_parse_type(b_file, tabbing, ptrs, field_sdna_in, sdna->names[field_names[i]], field_names[i]);
  }
  fprintf(sexps_parsed_data_file, ")");
}
#endif

void b_parse_type_helper_rec
     (int curr, struct b_file* b_file, int tabbing, int type_in, int name_in, bool is_ptr)
{
  int i;
  struct b_sdna *sdna           = b_file->sdna;
  struct b_misc_info *misc_info = &b_file->misc_info;
  int  num_D                    = misc_info->num_dimensions[name_in];
  int* arr_D                    = misc_info->arr_dimensions[name_in];

  if (curr == num_D || num_D == 0) //>=
    b_parse_type(b_file, type_in, tabbing, is_ptr);
  else {
    tab(sexps_parsed_data_file, "\n", "(", tabbing);
    //printf(",%i,", arr_D[curr]);
    for (i = 0; i < arr_D[curr]; i++)
      b_parse_type_helper_rec(curr+1, b_file, tabbing, type_in, name_in, is_ptr);
    fprintf(sexps_parsed_data_file, ")\n");
  }
}

/* TODO: add ptrs to db, and print them only 1th in a different file */
void b_parse_type(struct b_file* b_file, int type_in, int tabbing, bool ptr)
{
  CHECK(500000, debug_kk, "nope");
  //fprintf(sexps_parsed_data_file, " ?%s? ", b_file->sdna->types[type_in]);
  fflush(sexps_parsed_data_file);
  /*kkdel
  if (ptr)
    fprintf(sexps_parsed_data_file, "!ptr!");*/
#if 1
  int i, j;
  struct b_sdna      *sdna      = b_file->sdna;
  struct b_misc_info *misc_info = &b_file->misc_info;

  char *type_str = sdna->types[type_in];
  if (ptr)
    fprintf(sexps_parsed_data_file, "(p:%p)", *(void**)b_file->it);
  else if (0 == strcmp(type_str, "int"))
    fprintf(sexps_parsed_data_file, "%i"/*"(i:%i)"*/, *(int*)b_file->it);
  else if (0 == strcmp(type_str, "char")) {
    char c = *(char*)b_file->it;
    fprintf(sexps_parsed_data_file, "\"%c\"", c);
  }
  else if (0 == strcmp(type_str, "short"))
    fprintf(sexps_parsed_data_file, "%hu"/*"(s:%i)"*/, *(int*)b_file->it);
  /*else if (0 == strcmp(type_str, "void"))
    fprintf(sexps_parsed_data_file, "v:%p", *(void**)b_file->it);*/
  else if (0 == strcmp(type_str, "float"))
    fprintf(sexps_parsed_data_file, /*"f:%f"*/"%f", *(float*)b_file->it);
  else if (0 == strcmp(type_str, "double"))
    fprintf(sexps_parsed_data_file, /*"d:%f"*/"%f", *(double*)b_file->it);
  else if (0 == strcmp(type_str, "int64_t"))
    fprintf(sexps_parsed_data_file, "%i"/*"(i64:%i)"*/, *(int64_t*)b_file->it);
  else if (0 == strcmp(type_str, "long"))
    fprintf(sexps_parsed_data_file, "%l"/*"(l:%i)"*/, *(long*)b_file->it);
  else if (0 == strcmp(type_str, "uchar"))
    fprintf(sexps_parsed_data_file, "(uc:%u)", *(unsigned char*)b_file->it);
  else if (0 == strcmp(type_str, "ushort"))
    fprintf(sexps_parsed_data_file, "(us:%u)", *(ushort*)b_file->it);
  else if (0 == strcmp(type_str, "uint64_t"))
    fprintf(sexps_parsed_data_file, "(ui64:%u)", *(uint64_t*)b_file->it);
  else if (0 == strcmp(type_str, "ulong"))
    fprintf(sexps_parsed_data_file, "(ul:%u)", *(unsigned long*)b_file->it);
#endif
  else {
    tab(sexps_parsed_data_file, "\n", "(", tabbing++);

    int sdna_in = -1;
    for (i = 0; i < sdna->num_strcs; i++) {
      if (sdna->struct_type_in[i] == type_in)
        sdna_in = i;
    }
    assert(sdna_in != -1);
    int num_fields = sdna->struct_num_fields[sdna_in];

    for (i = 0; i < num_fields; i++) { /* each field */
      int field_name_in = sdna->field_name_in[sdna_in][i];
      int field_type_in = sdna->field_type_in[sdna_in][i];

      bool is_field_ptr = false;
      for (j = 0; j < misc_info->num_ptrs; j++) {
        if (field_name_in == misc_info->ptrs[j]) {
          is_field_ptr = true;
          break;
        }
      }
      b_parse_type_helper_rec(0, b_file, tabbing, field_type_in, field_name_in, is_field_ptr);
    }
    fprintf(sexps_parsed_data_file, ")\n");

    if (ptr)
      b_file->it -= b_file->ptr_size;
    else b_file->it -= b_file->sdna->tlens[type_in];
  }
  fprintf(sexps_parsed_data_file, " ");
  if (ptr)
    b_file->it += b_file->ptr_size;
  else b_file->it += b_file->sdna->tlens[type_in];
}

void b_parse_file_blocks(struct b_file* b_file)
{
  int i, j, k, l, m, n = 0;
  struct b_sdna *sdna = b_file->sdna;
  //short* ptrs = find_ptrs(b_file->sdna);

  fprintf(sexps_parsed_data_file, "(");
  for (i = 0; i < b_file->num_blocks; i++) {
    struct b_file_block *blk    = &(b_file->blocks[i]);
    char                *name   = blk->name;
    int                 sdna_in = blk->sdna_index;
    int                 type_in = sdna->struct_type_in[sdna_in];

    b_file->it = blk->data;
    /* skip invalid */
    if (sdna_in == 0 &&
        (!strncmp(name, "DNA1", 4) || !strncmp(name, "ENDB", 4) ||
         !strncmp(name, "REND", 4) || !strncmp(name, "TEST", 4)))
    {
      //k fprintf(sexps_parsed_data_file, "\n(NOTYPE %.4s%i)", name, i);
      fprintf(sexps_parsed_data_file, "\n(EMPTY)");
      continue;
    }
    else {
      //k sprintf(str, "%.4s%i", name, i);
      //b_parse_type(b_file, 0, ptrs, sdna_in, str, -1);
      assert(blk->count_structs != 0);
      if (blk->count_structs == 1)
        b_parse_type(b_file, type_in, 0, false);
      else {
        fprintf(sexps_parsed_data_file, "\n(a %i", blk->count_structs);
        for (j = 0; j < blk->count_structs; j++)
          b_parse_type(b_file, type_in, 1, false);
        fprintf(sexps_parsed_data_file, ")\n");
      }
      //k free(str);
    }
    //printf("DONE!!!!!!!!!!!!!!!!\n");
    //fflush(sexps_parsed_data_file);
  }
  fprintf(sexps_parsed_data_file, ")");

}



/* find_arrs returns length of array through first element of returned array */
void find_name_arrs(struct b_file* b_file)
{
  int i, j;
  struct b_sdna *sdna = b_file->sdna;

  char buff[100];
  int curr_buff;

  int*  num_D = malloc(sdna->num_names * sizeof(int));
  int** arr_D = malloc(sdna->num_names * sizeof(int*));

  for (i = 0; i < sdna->num_names; i++) {
    char* name   = sdna->names[i];
    int name_len = sdna->each_name_len[i];

    num_D[i]  = 0;
    arr_D[i]  = NULL;
    if (']' != name[name_len-1])
      continue;
    else {
      curr_buff = 99;
      for (j = name_len-2; j > 0; j--) {
#ifdef SKMK_EXTRA_CHECKS
        assert(curr_buff > 0 && curr_buff <= 99);
        assert(name[j] != ']');
#endif // SKMK_EXTRA_CHECKS
        if (is_num(name[j]))
          buff[curr_buff--] = name[j];
        else if (name[j] == '[') {
          num_D[i]++;
          arr_D[i] = realloc(arr_D[i], num_D[i] * sizeof(int));
          arr_D[i][num_D[i]-1] = strtol(&buff[curr_buff+1], &buff[99], 10);
          j--;
          curr_buff = 99;
        }
        else break;
      }
    }
  }
  b_file->misc_info.num_dimensions = num_D;
  b_file->misc_info.arr_dimensions = arr_D;
}

void find_simple_types(struct b_file *b_file)
{
  struct b_sdna *sdna = b_file->sdna;
  int i, curr = 0, len = 50; /* TODO: curr -> curr_len, len -> max_len */
  short* indices = malloc(len * sizeof(short));

  for (i = 0; i < sdna->num_types; i++) {
    char *type_str = sdna->types[i];
    if (!strcmp(type_str, "int")      || !strcmp(type_str, "char")    ||
        !strcmp(type_str, "short")    || !strcmp(type_str, "void")    ||
        !strcmp(type_str, "float")    || !strcmp(type_str, "double")  ||
        !strcmp(type_str, "int64_t")  || !strcmp(type_str, "long")    ||
        !strcmp(type_str, "uchar")    || !strcmp(type_str, "ushort")  ||
        !strcmp(type_str, "uint64_t") || !strcmp(type_str, "ulong"))
    {
      if (curr >= len) {
        len *= 2;
        indices = realloc(indices, len * sizeof(short));
      }
      indices[curr++] = i;
    }
  }
  /* resize to fit if we there's extra space left */
  len = curr + 1;
  indices = realloc(indices, len * sizeof(short));

  b_file->misc_info.simple_types = indices;
  b_file->misc_info.num_simple_types = len;
}

void find_ptrs(struct b_file *b_file)
{
  struct b_sdna *sdna = b_file->sdna;
  int i, curr = 0, len = 50;
  int* ptr_indexes = malloc(len * sizeof(int));

  for (i = 0; i < sdna->num_names; i++) {
    if (sdna->names[i][0] == '*') {
      if (curr >= len) {
        len *= 2;
        ptr_indexes = realloc(ptr_indexes, len * sizeof(int));
      }
      ptr_indexes[curr++] = i;
    }
  }
  /* resize to fit if we there's extra space left */
  len = curr + 1;
  ptr_indexes = realloc(ptr_indexes, len * sizeof(int));

  b_file->misc_info.ptrs = ptr_indexes;
  b_file->misc_info.num_ptrs = len;
}


