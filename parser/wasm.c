#include "wasm.h"

void *rust_alloc(size_t size);
void *rust_realloc(void *ptr, size_t new_size);
void rust_free(void *ptr, size_t size);

void *malloc(size_t size)
{
  if (size == 0)
  {
    return NULL;
  }

  size_t *ret = rust_alloc(size + sizeof(size_t));
  if (ret == NULL)
  {
    return NULL;
  }

  *ret = size;
  return ret + 1;
}

void *realloc(void *ptr, size_t new_size)
{
  if (ptr == NULL)
  {
    return malloc(new_size);
  }

  if (new_size == 0)
  {
    free(ptr);
    return NULL;
  }

  size_t *ret = rust_alloc(new_size + sizeof(size_t));
  if (ret == NULL)
  {
    return NULL;
  }

  *ret = new_size;

  size_t size = *(((size_t *)ptr) - 1);
  memcpy(ret + 1, ptr, size);
  free(ptr);

  return ret + 1;
}

void free(void *ptr)
{
  if (ptr == NULL)
  {
    return;
  }

  size_t *orig = ((size_t *)ptr) - 1;
  size_t size = *orig;
  rust_free(orig, size);
}

void *memcpy(void *restrict dest, const void *restrict src, size_t count)
{
  unsigned char *d = dest;
  const unsigned char *s = src;

  while (count--)
  {
    *d++ = *s++;
  }
  return dest;
}

void *memset(void *dest, int ch, size_t count)
{
  unsigned char *p = dest;
  while (count--)
  {
    *p++ = (unsigned char)ch;
  }
  return dest;
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
  memset(dest + len, '\0', len - count);

  return dest;
}

void __attribute__((noreturn)) rust_panic(const char *s);

void __attribute__((noreturn)) panic(const char *s) { rust_panic(s); }

int isspace(int c)
{
  return (c == ' ' || c == '\t' || c == '\n' || c == '\v' || c == '\f' ||
          c == '\r');
}
