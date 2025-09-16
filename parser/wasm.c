#include <stddef.h>
#include <limits.h>

#define NANOPRINTF_IMPLEMENTATION
#include "nanoprintf.h"

#include "wasm.h"

void *memcpy(void *restrict dest, const void *restrict src, size_t count)
{
  return __builtin_memcpy(dest, src, count);
}

void *memset(void *dest, int ch, size_t count)
{
  return __builtin_memset(dest, ch, count);
}

size_t strlen(const char *str)
{
  const char *s;
  for (s = str; *s; ++s)
    ;
  return (s - str);
}

char *strdup(const char *s)
{
  size_t len = strlen(s) + 1;
  void *new = malloc(len);
  if (new == NULL)
    return NULL;
  return (char *)memcpy(new, s, len);
}

char *strncpy(char *restrict dest, const char *restrict src, size_t count)
{
  size_t src_len = strlen(src);
  size_t len = src_len < count ? src_len : count;

  memcpy(dest, src, len);
  memset(dest + len, '\0', count - len);

  return dest;
}

extern void __attribute__((noreturn)) rust_panic(const char *prefix, const char *s);

void __attribute__((noreturn)) panic(const char *prefix, const char *s) { rust_panic(prefix, s); }

int isspace(int c)
{
  return (c == ' ' || c == '\t' || c == '\n' || c == '\v' || c == '\f' ||
          c == '\r');
}
