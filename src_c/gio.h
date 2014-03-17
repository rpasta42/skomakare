#ifndef GIO_H_INCLUDE
#define GIO_H_INCLUDE

#include "utils.h"

void init_gio(void);

void* tpircs_open_file(const char* name, uint size);

#endif // GIO_H_INCLUDE
