#ifndef UTILS_H_INCLUDED
#define UTILS_H_INCLUDED

#include <stdint.h>
#include <stdio.h>

typedef unsigned int uint;
typedef uint err_t;

typedef int8_t  s8;
typedef int16_t s16;
typedef int32_t s32;
typedef int64_t s64;

typedef uint8_t  u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

//#include <stdbool.h>
typedef char bool;
#define true 1
#define false 0


typedef struct spin_lock {
   bool locked;
   uint wait; /* 1/1,000,000 second */
} spin_lock;

spin_lock* spin_lock_new(uint delay);
void spin_lock_get(spin_lock* sl);
void spin_lock_release(spin_lock* sl);

uint read_file(const char *name, char **buff);
FILE* open_file_for_write(const char *path);

float* read_file_floats(const char *name, uint *pnum);

/* dirty... for debugging */
#define CHECK(num, x, msg) if(x++ > num) { printf(msg); exit(-30); }

static inline bool is_num(char c)
{
  return c >= '0' && c <= '9';
}

#endif // UTILS_H_INCLUDED
