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



/*/

  ///
  short* arrs = find_arrs(b_file.sdna);
  short num_arrs = arrs[0];

  for (i = 1; i < num_arrs; i++) {
    printf("%s\n", b_file.sdna->names[arrs[i]]);
  }
/
    /for (i = 0; i < b_file.num_blocks; i++) {
    struct b_file_block *blk = &b_file.blocks[i];
    if (blk->sdna_index == 0)
        printf("\nindex: %i...%.4s\tsize:%i\n", i, blk->name, blk->size);
    if (0 == strncmp(blk->name, "DNA1", 4))
      printf("\n\nindex: %i\n", blk->sdna_index);
  } NEW_LINE_NEW_COMMENT int a = 0;
  for (i = 0; i < b_file.sdna->num_names && a<num_arrs; i++) {
    //if (arrs[a] > i) ; else if (arrs[a] == i) printf("%s\n", b_file.sdna->names[arrs[a++]]); else while (arrs[a] < i) a++;
    if (arrs[a] == i)
      printf("%s\n", b_file.sdna->names[arrs[a++]]);

    //printf("\n%i: %s", i, b_file.sdna->names[i]);
  }


struct b_sdna_struct {
  short type_index;

  short num_fields;
  short* field_type_index/[num_fields]/;
  short* field_name_index/[num_fields]/;
};

struct b_sdna_struct*
get_sdna_by_index(struct b_sdna *sdna, int index)
{
  int i;
  struct b_sdna_struct* strc = malloc(sizeof(struct b_sdna_struct));

  strc->type_index = sdna->type_in[index];
  strc->num_fields = sdna->num_fields[index];
  for (i = 0; i < strc->num_fields; i++)
   = sdna->type_in[index];
  //for (i = 0; i < sdna->num_strcs; i++)
  strc->type_index
}
///
get_ _by_ () {}
struct b_file_block* get_by_type_name(const char *name) {}
struct b_file_block* get_by_type_index(const char *name) {}
struct b_file_block* get_by_sdna_type(const char *name) {}
struct b_file_block* get_by_sdna_index(const char *name) {}
///
//watch for: Object Scene

struct SDNA {
  char     id[4]      = "sdna";

  char     name_id[4] = "name";
  int      num_names;
  char*    names[num_names];

  char     type_id[4] = "type";
  int      num_types;
  char*    types[num_types];

  char     tlen_id[4] = "tlen";
  uint16_t tlens[num_types];

  char      strcs_id[4] = "strc";
  int       num_strcs;

  for (int i = 0; i < num_strcs; i++) {
    uint16_t  strc_type; //index_in_type_contains_name_strc;
    int       num_fields_in_strc;

    for (int j = 0; j < num_fields_in_strc; j++) {
          uint16_t index_in_type;
          uint16_t index_in_name;
    }
  }
};
///
int unroll_sdna_struct_static(struct b_sdna *sdna,
                              struct b_sdna_struct *sdna_struct)
{
  int size = 0;

  if (str(sdna_struct->type_index)) {
  case
  }
}

void* unroll_sdna_struct_dynamic()

/ stuff like abc[123], a[1], arr[500][900] /
int parse_name_arrs(struct b_sdna *sdna)
{
  short* arrs = find_name_arrs(sdna);
  short num_arrs = arrs[0];

  for (i = 1; i < num_arrs; i++) {
    printf("%s\n", b_file.sdna->names[arrs[i]]);
  }
}



short* flatten(struct b_sdna *sdna) {}
///
void b_parse_file_block(struct b_sdna *sdna, uint block_num, int type_index)
{
  int i, j, k;
  for (i = 0; i < sdna->type_in)
}
///
val = (name/blk_name_index type num_elem? ([val]|[int]|[char]|[short]|[...]))

(count int 123), (arr float 3 0.1 0.5 32.3)

struct person {
  char* name[3];
  int16_t age;
  uint16_t looks, intellegence, sexiness, funniness, overall;
} oldk = {{"kost",     "kov",   NULL},   15, 45, 99, 30, 50, 80},
  newk = {{"kostel",   NULL,    NULL},   16, 80, 95, 87, 75, 85},
  bob  = {{"bob",      "smith", "mike"}, 22, 20, 85, 20, 30, 75},
  vic  = {{"victoria", NULL,    NULL},   16, 90, 79, 88, 97, 88};

(type person (char* 3 name) (int16_t age) (uint16_t looks) (uint16_t intellegence)
             (uint16_t sexiness) (uint16_t funniness) (uint16_t overall))
(person ("victoria" Null Null) 16 90 79 88 97 88)

struct life {
  struct person person;
  struct school {
    char* name; short rating; struct location loc;
  } school;
  struct friend *friends;
  date important_dates[10];
} = {bob, {"Hagerty_HS", 9, {"Oviedo", FL, US}}, 192843543, {4/8/1992, 7/7/2010, 18/11/2059}};
(type life (person person) (life/school school) (friend* friends) (date 10 important_dates))
(life bob ("Hagerty_HS" 9 ("Oviedo" FL US)) 192843543 ((4 8 1992) (7 7 2010) (18 11 2059)))
void b_write_sexps(const char *msg) {}*/







