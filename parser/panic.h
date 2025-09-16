#pragma once

#ifdef __wasm__
#include "wasm.h"
#define PANIC_ORIGINAL(prefix, s) panic(prefix, s)
#else
#include <stdio.h>
#include <stdlib.h>
#define PANIC_ORIGINAL(prefix, s)           \
    do                                      \
    {                                       \
        fprintf(stderr, "%s%s", prefix, s); \
        fflush(stderr);                     \
        exit(1);                            \
    } while (0)
#endif

#define STRINGIFY(x) #x
#define TOSTRING(x) STRINGIFY(x)

#define PANIC(s) PANIC_ORIGINAL("PANIC at " __FILE__ ":" TOSTRING(__LINE__) " - ", s)
