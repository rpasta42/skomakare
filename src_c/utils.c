#include "utils.h"
#include <stdlib.h>
#include <stdio.h>

spin_lock* spin_lock_new(uint delay)
{
   spin_lock* sl = malloc(sizeof(spin_lock));

   sl->locked = false;

   sl->wait = delay;
   if (sl->wait == 0)
      sl->wait = 100000; /* 1/10th second */

   return sl;
}

/* fixme */
void spin_lock_get(spin_lock* sl)
{
  while (true) {
    while (sl->locked)
      usleep(sl->wait);
    if (!sl->locked) {
      sl->locked = true;
      break;
    }
  }
}

void spin_lock_release(spin_lock* sl)
{
   sl->locked = false;
}

void spin_lock_is_locked(spin_lock* sl)
{
  return sl->locked;
}

/* till unlocked */
void spin_lock_wait(spin_lock* sl, uint how_much)
{
  if (how_much == 0)
    how_much = sl->wait;
  while (sl->locked)
    usleep(how_much);
}


/* returns num bytes read */
uint read_file(const char *name, char** buff)
{
  FILE *f = fopen(name, "r");
  if (f == NULL) goto err;

  uint len, n_read;

  //get file size
  fseek(f, 0, SEEK_END);
  len = ftell(f);
  rewind(f);

  *buff = malloc(1 + len);
  if (*buff == NULL) goto err;

  n_read = fread(*buff, 1, len, f);
  if (n_read != len) goto err;

  (*buff)[len] = '\0'; /* null-terminated (UGLY) */

  fclose(f);
  return len;

err:
  fprintf(stderr, "couldn't read file \"%s\"\n", name);
  return 0;
}

/* returns floats. function returns number flaots read through num. */
float* read_file_floats(const char *name, uint *pnum)
{
  int num = 0;
  uint len = 40;
  float *buff = malloc(sizeof(float) * len);

  FILE *f = fopen(name, "r");
  if (f == NULL) goto err;

  while (1) {
    if (num >= len) {
      len *= 2;
      buff = realloc(buff, len * sizeof(float));
    }

    int num_read = fscanf(f, "%f", &buff[num++]);
    if (feof(f))
      break;
    else if (num_read != 1 || ferror(f))
      goto err;
  }
  len = num + 1;
  buff = realloc(buff, len * sizeof(float));
  *pnum = len;

  fclose(f);
  return buff;

err:
  fprintf(stderr, "couldn't read vertices/floats \"%s\"\n", name);
  free(buff);
  fclose(f);
  return 0;
}


FILE* open_file_for_write(const char *path)
{
   FILE *f = fopen(path, "w");
   if (f == NULL) goto err;
  return f;

err:
  fprintf(stderr, "couldn't open file %s\n", path);
  fclose(f);
  return NULL;
}

void write_file(FILE* f, const char* str)
{
  fprintf(f, "%s", str);
}

void tab(FILE* f, const char *before_str, const char *after_str, int n)
{
  int i;
  fprintf(f, "%s", before_str);
  for (i = 0; i < n; i++) fprintf(f, "%s", "  ");
  fprintf(f, "%s", after_str);
}


/* copies new_str at the end of str. expands str's size as needed */
void str_add(char** str, uint* len, uint* size, char* new_str, uint new_len)
{
  int old_len = *len;
  *len += new_len;
  if (*len >= *size) {
    *size = 2*(*len + new_len) + 20;
    *str = realloc(*str, *size);
  }
  int i;
  for (i = 0; i < new_len; i++) {
    (*str)[old_len + i] = new_str[i];
  }
}
/*test:
int main()
{
  char* str = malloc(1);
  int len = 0, size = 1;

  printf("\n1:%.*s\t\tcurrent len: %i\t\tcurrent size: %i", len, str, len, size);
  str_add(&str, &len, &size, "hello", strlen("hello"));
  printf("\n1:%.*s\t\tcurrent len: %i\t\tcurrent size: %i", len, str, len, size);
  str_add(&str, &len, &size, " bye", strlen(" bye"));
  printf("\n1:%.*s\tcurrent len: %i\t\tcurrent size: %i", len, str, len, size);
  str_add(&str, &len, &size, " \nGOODBYE CRUEEEEEEEEEEL WORLD!!!!!!!!!!", strlen(" \nGOODBYE CRUEEEEEEEEEEL WORLD!!!!!!!!!!"));
  printf("\n1:%.*s\t\tcurrent len: %i\t\tcurrent size: %i", len, str, len, size);

  return 0;
}*/

