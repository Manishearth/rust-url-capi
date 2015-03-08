#include <stdio.h>
#include <string.h>
#include <assert.h>
#include "../src/rust-url-capi.h"

#define TEST_GET(func, expected)                   \
{                                                  \
  rust_cstring part = func;                        \
  int equals = strcmp(part.data, expected) == 0;   \
  printf("%s = %s \n", #func, part.data);          \
  free_rust_cstring(part);                         \
  assert(equals);                                  \
}                                                  \

#define TEST_SET(func, expected)                   \
{                                                  \
  int32_t code = func;                             \
  printf("%s -> code %d\n", #func, code);          \
  assert(code == expected);                        \
}                                                  \


int main() {
  // Create URL
  rusturl_ptr url = rusturl_new("http://example.com/path/some/file.txt",
                                strlen("http://example.com/path/some/file.txt"));
  assert(url); // Check we have a URL

  TEST_GET(rusturl_get_spec(url), "http://example.com/path/some/file.txt");
  TEST_SET(rusturl_set_host(url, "test.com", strlen("test.com")), 0);
  TEST_GET(rusturl_get_host(url), "test.com");
  TEST_GET(rusturl_get_path(url), "/path/some/file.txt");
  TEST_SET(rusturl_set_path(url, "hello/../else.txt", strlen("hello/../else.txt")), 0);
  TEST_GET(rusturl_get_path(url), "/else.txt");
  TEST_GET(rusturl_get_scheme(url), "http");
  TEST_SET(rusturl_set_username(url, "user", strlen("user")), 0);
  TEST_GET(rusturl_get_username(url), "user");
  TEST_GET(rusturl_get_spec(url), "http://user@test.com/else.txt");
  TEST_SET(rusturl_set_password(url, "pass", strlen("pass")), 0);
  TEST_GET(rusturl_get_password(url), "pass");
  TEST_GET(rusturl_get_spec(url), "http://user:pass@test.com/else.txt");
  TEST_SET(rusturl_set_username(url, "", strlen("")), 0);
  TEST_SET(rusturl_set_password(url, "", strlen("")), 0);
  TEST_GET(rusturl_get_spec(url), "http://:@test.com/else.txt"); // XXX should rust-url remove unneeded :@ ?
  TEST_SET(rusturl_set_host_and_port(url, "example.org:1234", strlen("example.org:1234")), 0);
  TEST_GET(rusturl_get_host(url), "example.org");
  assert(rusturl_get_port(url) == 1234);
  TEST_SET(rusturl_set_port(url, "9090", strlen("9090")), 0);
  assert(rusturl_get_port(url) == 9090);
  TEST_SET(rusturl_set_query(url, "x=1", strlen("x=1")), 0);
  TEST_GET(rusturl_get_query(url), "x=1");
  TEST_SET(rusturl_set_fragment(url, "fragment", strlen("fragment")), 0);
  TEST_GET(rusturl_get_fragment(url), "fragment");
  TEST_GET(rusturl_get_spec(url), "http://:@example.org:9090/else.txt?x=1#fragment"); // XXX should rust-url remove unneeded :@ ?
  // Free the URL
  rusturl_free(url);

  printf("SUCCESS\n");
  return 0;
}