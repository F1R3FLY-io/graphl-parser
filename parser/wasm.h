#ifndef WASM_HEADER
#define WASM_HEADER

#include <stddef.h>

#include "nanoprintf.h"

#define snprintf npf_snprintf

void* malloc(size_t size);
void* realloc(void *ptr, size_t new_size);
void free(void *ptr);

void* memcpy(void* restrict dest, const void* restrict src, size_t count);
void* memset(void* dest, int ch, size_t count);
size_t strlen(const char* str);
char* strdup(const char* s);
char* strncpy(char* restrict dest, const char* restrict src, size_t count);
int isspace(int c);

void __attribute__((noreturn)) panic(const char* s);

#endif
