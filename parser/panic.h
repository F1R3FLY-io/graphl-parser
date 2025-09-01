#ifdef __wasm__
#include "wasm.h"
#define PANIC(s) panic(s)
#else
#define PANIC(s) exit(1)
#endif
