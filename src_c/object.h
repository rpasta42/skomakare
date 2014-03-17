/* IDEAS:
 * Automatic type-casting.
 * Return same type, but they can be interpreted differently.
 */
#ifndef OBJECT_H_INCLUDED
#define OBJECT_H_INCLUDED

struct skmk_object_info {
  uint16_t type;
  void* history;
  uint history_len;
};

typedef struct skmk_object {
  struct skmk_object_info info;
} Skmk_object;

typedef struct skmk_model {
  struct skmk_object_info info;
} Skmk_model;

typedef struct skmk_file {
  struct skmk_object_info info;
} Skmk_file;



#endif // OBJECT_H_INCLUDED
