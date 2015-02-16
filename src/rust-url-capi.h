#ifndef __RUST_URL_CAPI
#define __RUST_URL_CAPI
#include <stdlib.h>

struct rusturl;
typedef struct rusturl* rusturl_ptr;

typedef struct url_part {
  void * cstring_ptr;
  char * content;
  size_t size;
} rusturl_part;

rusturl_ptr rusturl_new(const char *spec, size_t src_len);
void rusturl_free(rusturl_ptr url);


rusturl_part rusturl_get_host(rusturl_ptr url);
void rusturl_set_host(rusturl_ptr url, const char *host, size_t len);


void rusturl_free_part(rusturl_part part);

#endif // __RUST_URL_CAPI