#ifndef __RUST_URL_CAPI
#define __RUST_URL_CAPI
#include <stdlib.h>

struct rusturl;
typedef struct rusturl* rusturl_ptr;


// Rust allocated string.
// .data is a NULL terminated string
// .len is the length of the string
// .raw_ptr is only used by rust, to free the structure.
typedef struct rust_cstring {
  void * raw_ptr;
  char * data;
  size_t len;
} rust_cstring;

void free_rust_cstring(rust_cstring part);


rusturl_ptr rusturl_new(const char *spec, size_t src_len);
void rusturl_free(rusturl_ptr url);

rust_cstring rusturl_get_spec(rusturl_ptr url);
rust_cstring rusturl_get_scheme(rusturl_ptr url);
rust_cstring rusturl_get_username(rusturl_ptr url);
rust_cstring rusturl_get_password(rusturl_ptr url);
rust_cstring rusturl_get_host(rusturl_ptr url);
int32_t      rusturl_get_port(rusturl_ptr url);
rust_cstring rusturl_get_path(rusturl_ptr url);
rust_cstring rusturl_get_query(rusturl_ptr url);
rust_cstring rusturl_get_fragment(rusturl_ptr url);


int32_t rusturl_set_scheme(rusturl_ptr url, const char *scheme, size_t len);
int32_t rusturl_set_username(rusturl_ptr url, const char *user, size_t len);
int32_t rusturl_set_password(rusturl_ptr url, const char *pass, size_t len);
int32_t rusturl_set_host_and_port(rusturl_ptr url, const char *hostport, size_t len);
int32_t rusturl_set_host(rusturl_ptr url, const char *host, size_t len);
int32_t rusturl_set_port(rusturl_ptr url, const char *port, size_t len);
int32_t rusturl_set_path(rusturl_ptr url, const char *path, size_t len);
int32_t rusturl_set_query(rusturl_ptr url, const char *path, size_t len);
int32_t rusturl_set_fragment(rusturl_ptr url, const char *path, size_t len);

#endif // __RUST_URL_CAPI